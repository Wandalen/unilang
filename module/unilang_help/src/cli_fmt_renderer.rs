//! Column-aligned, colour-aware renderer backed by `cli_fmt`'s detail-page template.
//!
//! Unlike [`crate::PlainRenderer`], which reproduces the historical `unilang`
//! plain-text formats byte-for-byte, this renderer maps the help model onto
//! `cli_fmt`'s [`DetailPageData`] and inherits its column alignment, section
//! layout, and TTY-aware colouring.

use cli_fmt::help::{ CliHelpStyle, DetailPageData, DetailPageTemplate, DetailSection, ExampleEntry, OptionEntry };
use crate::model::{ HelpCommandData, HelpParamData };
use crate::verbosity::HelpDisplayOptions;

/// Renders command and parameter help pages through `cli_fmt`.
///
/// ```
/// use cli_fmt::help::CliHelpStyle;
/// use unilang_help::{ CliFmtRenderer, HelpCommandData, HelpParamData };
///
/// let mut cmd = HelpCommandData::default();
/// cmd.name = ".rollup".into();
/// let mut param = HelpParamData::default();
/// param.name = "scope".into();
/// param.kind = "Enum".into();
/// param.kind_compact = "enum".into();
/// param.description = "Discovery strategy selector.".into();
///
/// let style = CliHelpStyle { tty_detect : false, ..Default::default() };
/// let renderer = CliFmtRenderer::default().with_style( style );
/// let page = renderer.render_param( &cmd, &param );
/// assert!( page.contains( "Parameter: scope" ) );
/// assert!( page.contains( ".rollup scope::<enum>" ) );
/// ```
#[ derive( Debug, Clone, Default ) ]
pub struct CliFmtRenderer
{
  /// Layout and colour configuration passed through to `cli_fmt`.
  pub style : CliHelpStyle,
  /// Global toggles for metadata visibility on command pages.
  pub options : HelpDisplayOptions,
}

impl CliFmtRenderer
{
  /// Create a renderer with default style (TTY-detected colour) and options.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set the `cli_fmt` style.
  #[ must_use ]
  pub fn with_style( mut self, style : CliHelpStyle ) -> Self
  {
    self.style = style;
    self
  }

  /// Set the display options.
  #[ must_use ]
  pub fn with_options( mut self, options : HelpDisplayOptions ) -> Self
  {
    self.options = options;
    self
  }

  /// Render a single-parameter help page.
  ///
  /// The page shows the parameter's usage syntax, description, a fact block
  /// (kind, requiredness, default, aliases, validation), possible values for
  /// enum-like kinds, and parameter-specific examples.
  #[ must_use ]
  pub fn render_param( &self, command : &HelpCommandData, param : &HelpParamData ) -> String
  {
    let mut page = DetailPageData::default();
    page.label = "Parameter".into();
    page.name = param.name.clone();
    page.usage.push( format!( "{} {}::<{}>", command.name, param.name, param.kind_compact ) );

    let desc_text = if param.description.is_empty() { &param.hint } else { &param.description };
    if !desc_text.is_empty()
    {
      page.description.push( desc_text.clone() );
      if !param.hint.is_empty() && param.hint != *desc_text
      {
        page.description.push( param.hint.clone() );
      }
    }

    let mut facts = vec!
    [
      OptionEntry { name : "Kind".into(), desc : param.kind.clone() },
      OptionEntry
      {
        name : "Required".into(),
        desc : ( if param.optional { "no" } else { "yes" } ).into(),
      },
    ];
    if param.multiple
    {
      facts.push( OptionEntry { name : "Multiple".into(), desc : "yes".into() } );
    }
    if let Some( default ) = &param.default
    {
      facts.push( OptionEntry { name : "Default".into(), desc : default.clone() } );
    }
    if !param.aliases.is_empty()
    {
      facts.push( OptionEntry { name : "Aliases".into(), desc : param.aliases.join( ", " ) } );
    }
    if !param.validation_rules.is_empty()
    {
      facts.push( OptionEntry { name : "Validation".into(), desc : param.validation_rules.join( ", " ) } );
    }
    page.sections.push( DetailSection::new( "", facts ) );

    if !param.choices.is_empty()
    {
      let choices = param.choices.iter()
        .map( | choice | OptionEntry { name : choice.clone(), desc : String::new() } )
        .collect();
      page.sections.push( DetailSection::new( "Possible values", choices ) );
    }

    page.examples = param.examples.iter()
      .map( | example | ExampleEntry { invocation : example.clone(), desc : None } )
      .collect();

    DetailPageTemplate::new( self.style.clone(), page ).render()
  }

  /// Render a whole-command help page.
  ///
  /// Metadata rows (status, version, aliases, tags) honor both the global
  /// [`HelpDisplayOptions`] and the per-command
  /// [`HelpCommandData::show_version`] flag.
  #[ must_use ]
  pub fn render_command( &self, command : &HelpCommandData ) -> String
  {
    let mut page = DetailPageData::default();
    page.label = "Command".into();
    page.name = command.name.clone();

    let mut usage = command.name.clone();
    for param in &command.params
    {
      if param.optional
      {
        usage.push_str( &format!( " [{}::<{}>]", param.name, param.kind_compact ) );
      }
      else
      {
        usage.push_str( &format!( " {}::<{}>", param.name, param.kind_compact ) );
      }
    }
    page.usage.push( usage );

    if !command.description.is_empty()
    {
      page.description.push( command.description.clone() );
    }
    if !command.hint.is_empty() && command.hint != command.description
    {
      page.description.push( command.hint.clone() );
    }

    let mut facts = Vec::new();
    if self.options.show_status && !command.status.is_empty()
    {
      facts.push( OptionEntry { name : "Status".into(), desc : command.status.clone() } );
    }
    if command.show_version && self.options.show_version && !command.version.is_empty()
    {
      facts.push( OptionEntry { name : "Version".into(), desc : command.version.clone() } );
    }
    if !command.aliases.is_empty() && self.options.show_aliases
    {
      facts.push( OptionEntry { name : "Aliases".into(), desc : command.aliases.join( ", " ) } );
    }
    if !command.tags.is_empty() && self.options.show_tags
    {
      facts.push( OptionEntry { name : "Tags".into(), desc : command.tags.join( ", " ) } );
    }
    page.sections.push( DetailSection::new( "", facts ) );

    let params = command.params.iter().map( | param |
    {
      let desc_text = if param.description.is_empty() { &param.hint } else { &param.description };
      let mut desc = desc_text.clone();
      if param.optional
      {
        desc.push_str( " (optional)" );
      }
      if param.multiple
      {
        desc.push_str( " (multiple)" );
      }
      OptionEntry
      {
        name : format!( "{}::<{}>", param.name, param.kind_compact ),
        desc,
      }
    } ).collect();
    page.sections.push( DetailSection::new( "Parameters", params ) );

    page.examples = command.examples.iter()
      .map( | example | ExampleEntry { invocation : example.clone(), desc : None } )
      .collect();

    DetailPageTemplate::new( self.style.clone(), page ).render()
  }
}
