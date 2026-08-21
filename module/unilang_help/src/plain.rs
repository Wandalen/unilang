//! Plain-text help renderer with five verbosity levels.
//!
//! The command-page output of this renderer is a line-faithful port of the
//! original `unilang` `HelpGenerator` format functions — consumers migrating
//! from that implementation get byte-identical output for the same data.

use core::fmt::Write;
use crate::model::{ HelpCommandData, HelpParamData };
use crate::verbosity::{ HelpVerbosity, HelpDisplayOptions };

/// Renders help pages as plain text, controlled by a verbosity level and
/// global display options.
///
/// ```
/// use unilang_help::{ PlainRenderer, HelpCommandData, HelpVerbosity };
///
/// let mut cmd = HelpCommandData::default();
/// cmd.name = ".greet".into();
/// cmd.description = "Print a greeting.".into();
///
/// let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Minimal );
/// assert_eq!( renderer.render( &cmd ), ".greet - Print a greeting." );
/// ```
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
pub struct PlainRenderer
{
  /// Verbosity level selecting which command-page format to produce.
  pub verbosity : HelpVerbosity,
  /// Global toggles for metadata visibility.
  pub options : HelpDisplayOptions,
}

impl PlainRenderer
{
  /// Create a renderer with default verbosity (Standard) and default options.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set the verbosity level.
  #[ must_use ]
  pub fn with_verbosity( mut self, verbosity : HelpVerbosity ) -> Self
  {
    self.verbosity = verbosity;
    self
  }

  /// Set the display options.
  #[ must_use ]
  pub fn with_options( mut self, options : HelpDisplayOptions ) -> Self
  {
    self.options = options;
    self
  }

  /// Render a command help page at the configured verbosity level.
  #[ must_use ]
  pub fn render( &self, data : &HelpCommandData ) -> String
  {
    match self.verbosity
    {
      HelpVerbosity::Minimal => self.format_minimal( data ),
      HelpVerbosity::Basic => self.format_basic( data ),
      HelpVerbosity::Standard => self.format_standard( data ),
      HelpVerbosity::Detailed => self.format_detailed( data ),
      HelpVerbosity::Comprehensive => self.format_comprehensive( data ),
    }
  }

  /// Render a single-parameter help page (plain-text detail page).
  ///
  /// Verbosity does not apply here — a parameter page is already the most
  /// specific help surface; all known facts about the parameter are shown.
  #[ must_use ]
  pub fn render_param( &self, command : &HelpCommandData, param : &HelpParamData ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "Parameter: {}", param.name ).unwrap();
    writeln!( &mut help, "  {} {}::<{}>", command.name, param.name, param.kind_compact ).unwrap();

    let desc_text = if param.description.is_empty() { &param.hint } else { &param.description };
    if !desc_text.is_empty()
    {
      writeln!( &mut help ).unwrap();
      writeln!( &mut help, "{desc_text}" ).unwrap();
      if !param.hint.is_empty() && param.hint != *desc_text
      {
        writeln!( &mut help, "{}", param.hint ).unwrap();
      }
    }

    writeln!( &mut help ).unwrap();
    writeln!( &mut help, "Kind: {}", param.kind ).unwrap();
    writeln!( &mut help, "Required: {}", if param.optional { "no" } else { "yes" } ).unwrap();
    if param.multiple
    {
      writeln!( &mut help, "Multiple: yes" ).unwrap();
    }
    if let Some( default ) = &param.default
    {
      writeln!( &mut help, "Default: {default}" ).unwrap();
    }
    if !param.aliases.is_empty()
    {
      writeln!( &mut help, "Aliases: {}", param.aliases.join( ", " ) ).unwrap();
    }
    if !param.choices.is_empty()
    {
      writeln!( &mut help, "Choices: {}", param.choices.join( ", " ) ).unwrap();
    }
    if !param.validation_rules.is_empty()
    {
      writeln!( &mut help, "Validation:" ).unwrap();
      for rule in &param.validation_rules
      {
        writeln!( &mut help, "  - {rule}" ).unwrap();
      }
    }
    if !param.examples.is_empty()
    {
      writeln!( &mut help ).unwrap();
      writeln!( &mut help, "Examples:" ).unwrap();
      for example in &param.examples
      {
        writeln!( &mut help, "  {example}" ).unwrap();
      }
    }
    help
  }

  /// Format Level 0: Minimal - Just name and brief description
  fn format_minimal( &self, data : &HelpCommandData ) -> String
  {
    format!( "{} - {}", data.name, data.description )
  }

  /// Format Level 1: Basic - Add parameters list with types
  fn format_basic( &self, data : &HelpCommandData ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}", data.name, data.description ).unwrap();

    if !data.params.is_empty()
    {
      writeln!( &mut help, "\nPARAMETERS:" ).unwrap();
      for param in &data.params
      {
        writeln!( &mut help, "  {}::{}", param.name, param.kind_compact ).unwrap();
      }
    }
    help
  }

  /// Format Level 2: Standard (DEFAULT) - Concise
  fn format_standard( &self, data : &HelpCommandData ) -> String
  {
    let mut help = String::new();

    // Command header with optional version
    if data.show_version && self.options.show_version
    {
      writeln!( &mut help, "Usage: {} (v{})", data.name, data.version ).unwrap();
    }
    else
    {
      writeln!( &mut help, "Usage: {}", data.name ).unwrap();
    }
    writeln!( &mut help, "{}\n", data.description ).unwrap();

    // Status information
    if self.options.show_status
    {
      writeln!( &mut help, "Status: {}", data.status ).unwrap();
    }
    if !data.aliases.is_empty() && self.options.show_aliases
    {
      writeln!( &mut help, "Aliases: {}", data.aliases.join( ", " ) ).unwrap();
    }

    // Arguments section with improved formatting
    if !data.params.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for param in &data.params
      {
        write!( &mut help, "{}", param.name ).unwrap();
        write!( &mut help, " (Type: {})", param.kind_compact ).unwrap();

        let mut status_parts = Vec::new();
        if param.optional
        {
          status_parts.push( "Optional" );
        }
        if param.multiple
        {
          status_parts.push( "Multiple" );
        }
        if !status_parts.is_empty()
        {
          write!( &mut help, " - {}", status_parts.join( ", " ) ).unwrap();
        }
        writeln!( &mut help ).unwrap();

        // Show description, or hint if description is empty
        let desc_text : &str = if !param.description.is_empty()
        {
          &param.description
        }
        else if !param.hint.is_empty()
        {
          &param.hint
        }
        else
        {
          ""
        };

        if !desc_text.is_empty()
        {
          writeln!( &mut help, "  {desc_text}" ).unwrap();
        }

        if !param.validation_rules.is_empty()
        {
          writeln!( &mut help, "  Rules: [{}]", param.validation_rules.join( ", " ) ).unwrap();
        }

        writeln!( &mut help ).unwrap();
      }
    }

    // Examples section
    if !data.examples.is_empty()
    {
      writeln!( &mut help, "Examples:" ).unwrap();
      for ( idx, example ) in data.examples.iter().enumerate()
      {
        writeln!( &mut help, "  {}. {}", idx + 1, example ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    help
  }

  /// Format Level 3: Detailed - Full metadata
  fn format_detailed( &self, data : &HelpCommandData ) -> String
  {
    let mut help = String::new();
    if data.show_version && self.options.show_version
    {
      writeln!( &mut help, "Usage: {} (v{})", data.name, data.version ).unwrap();
    }
    else
    {
      writeln!( &mut help, "Usage: {}", data.name ).unwrap();
    }
    if !data.aliases.is_empty() && self.options.show_aliases
    {
      writeln!( &mut help, "Aliases: {}", data.aliases.join( ", " ) ).unwrap();
    }
    if !data.tags.is_empty() && self.options.show_tags
    {
      writeln!( &mut help, "Tags: {}", data.tags.join( ", " ) ).unwrap();
    }
    writeln!( &mut help, "\n  Hint: {}", data.hint ).unwrap();
    writeln!( &mut help, "  {}\n", data.description ).unwrap();
    if self.options.show_status
    {
      writeln!( &mut help, "Status: {}", data.status ).unwrap();
    }

    if !data.params.is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for param in &data.params
      {
        write!( &mut help, "{}", param.name ).unwrap();
        write!( &mut help, " (Type: {})", param.kind ).unwrap();

        let mut status_parts = Vec::new();
        if param.optional
        {
          status_parts.push( "Optional" );
        }
        if param.multiple
        {
          status_parts.push( "Multiple" );
        }
        if !status_parts.is_empty()
        {
          write!( &mut help, " - {}", status_parts.join( ", " ) ).unwrap();
        }
        writeln!( &mut help ).unwrap();

        if !param.description.is_empty()
        {
          writeln!( &mut help, "  {}", param.description ).unwrap();
          if !param.hint.is_empty() && param.hint != param.description
          {
            writeln!( &mut help, "  ({})", param.hint ).unwrap();
          }
        }
        else if !param.hint.is_empty()
        {
          writeln!( &mut help, "  {}", param.hint ).unwrap();
        }

        if !param.validation_rules.is_empty()
        {
          writeln!( &mut help, "  Rules: [{}]", param.validation_rules.join( ", " ) ).unwrap();
        }

        writeln!( &mut help ).unwrap();
      }
    }

    help
  }

  /// Format Level 4: Comprehensive - Extensive
  fn format_comprehensive( &self, data : &HelpCommandData ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}\n", data.name, data.description ).unwrap();

    // USAGE section
    write!( &mut help, "USAGE:\n  {}", data.name ).unwrap();
    if !data.params.is_empty()
    {
      for param in &data.params
      {
        if param.optional
        {
          write!( &mut help, " [{}::<value>]", param.name ).unwrap();
        }
        else
        {
          write!( &mut help, " {}::<value>", param.name ).unwrap();
        }
      }
    }
    writeln!( &mut help, "\n" ).unwrap();

    // DESCRIPTION section with metadata
    writeln!( &mut help, "DESCRIPTION:" ).unwrap();
    writeln!( &mut help, "  {}", data.description ).unwrap();
    if !data.hint.is_empty() && data.hint != data.description
    {
      writeln!( &mut help, "  {}", data.hint ).unwrap();
    }
    if self.options.show_status
    {
      if data.show_version && self.options.show_version
      {
        writeln!( &mut help, "\n  Status: {} (v{})", data.status, data.version ).unwrap();
      }
      else
      {
        writeln!( &mut help, "\n  Status: {}", data.status ).unwrap();
      }
    }
    if !data.aliases.is_empty() && self.options.show_aliases
    {
      writeln!( &mut help, "  Aliases: {}", data.aliases.join( ", " ) ).unwrap();
    }
    writeln!( &mut help ).unwrap();

    // PARAMETERS section with detailed explanations
    if !data.params.is_empty()
    {
      writeln!( &mut help, "PARAMETERS:\n" ).unwrap();
      for param in &data.params
      {
        writeln!( &mut help, "  {}::<value>", param.name ).unwrap();

        // Description with indentation
        if !param.description.is_empty()
        {
          writeln!( &mut help, "    {}", param.description ).unwrap();
        }
        if !param.hint.is_empty() && param.hint != param.description
        {
          writeln!( &mut help, "    {}", param.hint ).unwrap();
        }

        // Type and attributes
        writeln!( &mut help, "    Type: {}", param.kind ).unwrap();
        if param.optional
        {
          writeln!( &mut help, "    Optional: yes" ).unwrap();
        }
        if param.multiple
        {
          writeln!( &mut help, "    Multiple values: yes" ).unwrap();
        }

        // Validation rules
        if !param.validation_rules.is_empty()
        {
          writeln!( &mut help, "    Validation:" ).unwrap();
          for rule in &param.validation_rules
          {
            writeln!( &mut help, "      - {rule}" ).unwrap();
          }
        }

        writeln!( &mut help ).unwrap();
      }
    }

    // EXAMPLES section
    if !data.examples.is_empty()
    {
      writeln!( &mut help, "EXAMPLES:" ).unwrap();
      for example in &data.examples
      {
        writeln!( &mut help, "  {example}" ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    // TAGS section if present
    if !data.tags.is_empty() && self.options.show_tags
    {
      writeln!( &mut help, "TAGS: {}", data.tags.join( ", " ) ).unwrap();
    }

    help
  }
}
