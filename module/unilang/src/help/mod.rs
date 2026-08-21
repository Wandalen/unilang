//!
//! The help generation components for the Unilang framework.
//!
//! This module is a thin adapter over the `unilang_help` crate: it maps
//! `CommandDefinition` into the renderer-agnostic help model and delegates
//! rendering to `unilang_help`'s `PlainRenderer` (command pages) and
//! `CliFmtRenderer` (parameter detail pages). Verbosity levels and display
//! options are re-exports of the `unilang_help` types.
//!
//! # Help Invocation Surfaces
//!
//! - `??` alone — global command listing (mirror of bare `.`)
//! - `.cmd ??` — command help page (any position, unquoted)
//! - `.cmd param::??` — parameter detail page (unquoted; quote as `"??"` for the literal)
//! - `.cmd.help` / `.cmd.help param` — spelled equivalents of the two above
//!
//! # Verbosity Levels
//!
//! Help output can be controlled through five verbosity levels (0-4):
//! - **Level 0 (Minimal)**: Single-line output with command name and description
//! - **Level 1 (Basic)**: Add parameter list with types
//! - **Level 2 (Standard)**: Concise format with USAGE, PARAMETERS, EXAMPLES - **DEFAULT**
//! - **Level 3 (Detailed)**: Full metadata including version, aliases, tags, validation rules
//! - **Level 4 (Comprehensive)**: Extensive format with rationale and detailed explanations
//!
//! # Usage Examples
//!
//! ## Basic Usage (Default Verbosity)
//!
//! ```rust
//! use unilang::prelude::*;
//!
//! let registry = CommandRegistry::new();
//! let help_gen = HelpGenerator::new( &registry );
//!
//! // Generates help at default Level 2 (Standard)
//! if let Some( help ) = help_gen.command( ".config" )
//! {
//!   println!( "{}", help );
//! }
//! ```
//!
//! ## Environment Variable Control
//!
//! ```bash
//! # Set verbosity to Minimal (Level 0)
//! UNILANG_HELP_VERBOSITY=0 cargo run
//!
//! # Set verbosity to Comprehensive (Level 4)
//! UNILANG_HELP_VERBOSITY=4 cargo run
//! ```
//!
//! ```rust
//! use unilang::prelude::*;
//!
//! let registry = CommandRegistry::new();
//!
//! // Read verbosity from environment variable UNILANG_HELP_VERBOSITY
//! let help_gen = HelpGenerator::from_env( &registry );
//! ```
//!
//! ## Programmatic Verbosity Control
//!
//! ```rust
//! use unilang::prelude::*;
//! use unilang::help::HelpVerbosity;
//!
//! let registry = CommandRegistry::new();
//!
//! // Create with specific verbosity level
//! let help_gen = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Comprehensive );
//!
//! // Or set verbosity dynamically
//! let mut help_gen = HelpGenerator::new( &registry );
//! help_gen.set_verbosity( HelpVerbosity::Minimal );
//! ```
//!

/// Internal namespace.
mod private
{
  use crate::registry::CommandRegistry;
  use crate::data::{ ArgumentDefinition, CommandDefinition, Kind };
  use core::fmt::Write;

  pub use unilang_help::{ HelpVerbosity, HelpDisplayOptions };
  use unilang_help::{ CliFmtRenderer, HelpCommandData, HelpParamData, PlainRenderer };

  /// Maps a command definition into the renderer-agnostic `unilang_help` model.
  ///
  /// `name` is the full dotted name including namespace — every rendered usage
  /// line must be directly typeable; `status` uses the lowercase `Display`
  /// form, and per-parameter data comes from `help_param_data`.
  #[ must_use ]
  pub fn help_command_data( command : &CommandDefinition ) -> HelpCommandData
  {
    let mut data = HelpCommandData::default();
    data.name = command.full_name();
    data.description = command.description().to_string();
    data.hint = command.hint().to_string();
    data.version = command.version().as_str().to_string();
    data.status = command.status().to_string();
    data.show_version = command.show_version_in_help();
    data.aliases = command.aliases().to_vec();
    data.tags = command.tags().to_vec();
    data.examples = command.examples().to_vec();
    data.params = command.arguments().iter().map( | arg | help_param_data( command, arg ) ).collect();
    data
  }

  /// Maps one argument definition into the `unilang_help` parameter model.
  ///
  /// `kind` keeps the full `Display` form (`List(String)`); `kind_compact` is the
  /// lowercased `Debug` form truncated at the first `(` so parameterized kinds
  /// render as a short token (`list`, `enum`, `map`) in usage lines. Enum choices
  /// become `choices`; validation rules are pre-rendered via `Debug`. Examples
  /// are derived: a synthesized canonical invocation first, then any command
  /// examples mentioning the parameter by `name::` or `alias::`.
  #[ must_use ]
  pub fn help_param_data( command : &CommandDefinition, arg : &ArgumentDefinition ) -> HelpParamData
  {
    let mut data = HelpParamData::default();
    data.name = arg.name.clone();
    data.kind = arg.kind.to_string();
    data.kind_compact = compact_kind( &arg.kind );
    data.description = arg.description.clone();
    data.hint = arg.hint.clone();
    data.optional = arg.attributes.optional;
    data.multiple = arg.attributes.multiple;
    data.default = arg.attributes.default.clone();
    data.choices = match &arg.kind
    {
      Kind::Enum( choices ) => choices.clone(),
      _ => vec![],
    };
    data.validation_rules = arg.validation_rules.iter().map( | rule | format!( "{rule:?}" ) ).collect();
    data.aliases = arg.aliases.clone();
    data.examples = derive_param_examples( command, arg );
    data
  }

  /// Lowercased `Debug` kind truncated at the first `(`: `List(String, None)` → `list`.
  fn compact_kind( kind : &Kind ) -> String
  {
    let debug = format!( "{kind:?}" ).to_lowercase();
    match debug.find( '(' )
    {
      Some( idx ) => debug[ ..idx ].to_string(),
      None => debug,
    }
  }

  /// Placeholder shown in a synthesized `cmd param::<placeholder>` example.
  ///
  /// Enum kinds use their first choice verbatim so the canonical example is
  /// directly runnable; every other kind gets an angle-bracket token.
  fn kind_placeholder( kind : &Kind ) -> String
  {
    match kind
    {
      Kind::Enum( choices ) => choices.first().cloned().unwrap_or_else( || "<value>".to_string() ),
      Kind::Integer => "<n>".to_string(),
      Kind::Float => "<x>".to_string(),
      Kind::String => "<string>".to_string(),
      Kind::Boolean => "<true|false>".to_string(),
      Kind::Path => "<path>".to_string(),
      Kind::File => "<file>".to_string(),
      Kind::Directory => "<directory>".to_string(),
      Kind::Url => "<url>".to_string(),
      Kind::DateTime => "<datetime>".to_string(),
      Kind::Pattern => "<pattern>".to_string(),
      Kind::List( .. ) => "<list>".to_string(),
      Kind::Map( .. ) => "<map>".to_string(),
      Kind::JsonString => "<json>".to_string(),
      Kind::Object => "<object>".to_string(),
    }
  }

  /// Derives parameter-page examples: the synthesized canonical invocation, then
  /// command examples mentioning the parameter by `name::` or any `alias::`.
  fn derive_param_examples( command : &CommandDefinition, arg : &ArgumentDefinition ) -> Vec< String >
  {
    let mut examples = vec![ format!( "{} {}::{}", command.full_name(), arg.name, kind_placeholder( &arg.kind ) ) ];

    let mut mentions = vec![ format!( "{}::", arg.name ) ];
    for alias in &arg.aliases
    {
      mentions.push( format!( "{alias}::" ) );
    }
    for example in command.examples()
    {
      if mentions.iter().any( | mention | example.contains( mention.as_str() ) )
      {
        examples.push( example.clone() );
      }
    }
    examples
  }

  /// Finds a parameter by canonical name or alias.
  fn find_parameter< 'd >( command : & 'd CommandDefinition, param_name : &str ) -> Option< & 'd ArgumentDefinition >
  {
    command.arguments().iter()
      .find( | arg | arg.name == param_name || arg.aliases.iter().any( | alias | alias == param_name ) )
  }

  /// Renders the standard help page for a command definition, honoring
  /// `UNILANG_HELP_VERBOSITY` and display-option environment overrides.
  ///
  /// Registry-free counterpart of `HelpGenerator::command` — used by help
  /// routines that capture a command definition instead of a registry.
  #[ must_use ]
  pub fn command_help_text( command : &CommandDefinition ) -> String
  {
    PlainRenderer::default()
      .with_verbosity( HelpVerbosity::from_env() )
      .with_options( HelpDisplayOptions::default().with_env_overrides() )
      .render( &help_command_data( command ) )
  }

  /// Renders the parameter detail page for one parameter, honoring display-option
  /// environment overrides. Accepts the canonical name or any alias; `None` when
  /// the command has no such parameter.
  #[ must_use ]
  pub fn parameter_help_text( command : &CommandDefinition, param_name : &str ) -> Option< String >
  {
    let arg = find_parameter( command, param_name )?;
    let renderer = CliFmtRenderer::default().with_options( HelpDisplayOptions::default().with_env_overrides() );
    Some( renderer.render_param( &help_command_data( command ), &help_param_data( command, arg ) ) )
  }

  /// Like `parameter_help_text`, but a request for an unknown parameter yields a
  /// listing of the command's valid parameters instead of `None` — a mistyped
  /// `param::??` is never a dead end.
  #[ must_use ]
  pub fn parameter_help_or_listing( command : &CommandDefinition, param_name : &str ) -> String
  {
    if let Some( page ) = parameter_help_text( command, param_name )
    {
      return page;
    }

    let valid : Vec< &str > = command.arguments().iter().map( | arg | arg.name.as_str() ).collect();
    if valid.is_empty()
    {
      format!(
        "Parameter '{}' not found: command '{}' takes no parameters.",
        param_name, command.full_name()
      )
    }
    else
    {
      format!(
        "Parameter '{}' not found for command '{}'. Valid parameters: {}.\nUse '{} ??' for command help.",
        param_name, command.full_name(), valid.join( ", " ), command.full_name()
      )
    }
  }

///
/// Generates help information for commands.
///
/// This struct provides methods to create formatted help messages from
/// `CommandDefinition` instances, which can be displayed to the user.
#[ allow( missing_debug_implementations ) ]
pub struct HelpGenerator< 'a >
{
  registry : & 'a CommandRegistry,
  verbosity : HelpVerbosity,
  display_options : HelpDisplayOptions,
}

impl< 'a > HelpGenerator< 'a >
{
  ///
  /// Creates a new `HelpGenerator` with default verbosity (Level 2: Standard).
  ///
  #[ must_use ]
  pub fn new( registry : & 'a CommandRegistry ) -> Self
  {
    Self
    {
      registry,
      verbosity : HelpVerbosity::default(),
      display_options : HelpDisplayOptions::default().with_env_overrides(),
    }
  }

  ///
  /// Creates a new `HelpGenerator` reading verbosity from UNILANG_HELP_VERBOSITY environment variable.
  /// Falls back to default (Level 2: Standard) if not set or invalid.
  ///
  #[ must_use ]
  pub fn from_env( registry : & 'a CommandRegistry ) -> Self
  {
    Self
    {
      registry,
      verbosity : HelpVerbosity::from_env(),
      display_options : HelpDisplayOptions::default().with_env_overrides(),
    }
  }

  ///
  /// Creates a new `HelpGenerator` with specified verbosity level.
  ///
  #[ must_use ]
  pub fn with_verbosity( registry : & 'a CommandRegistry, verbosity : HelpVerbosity ) -> Self
  {
    Self { registry, verbosity, display_options : HelpDisplayOptions::default().with_env_overrides() }
  }

  ///
  /// Sets explicit display options, overriding the environment-derived defaults.
  ///
  #[ must_use ]
  pub fn with_display_options( mut self, display_options : HelpDisplayOptions ) -> Self
  {
    self.display_options = display_options;
    self
  }

  ///
  /// Sets the verbosity level for help output.
  ///
  pub fn set_verbosity( &mut self, verbosity : HelpVerbosity )
  {
    self.verbosity = verbosity;
  }

  ///
  /// Gets the current verbosity level.
  ///
  #[ must_use ]
  pub fn verbosity( &self ) -> HelpVerbosity
  {
    self.verbosity
  }

  ///
  /// Resolves a command name to its definition using the help lookup chain:
  /// exact match, then dot-prefixed, then the legacy `echo` → `.system.echo` mapping.
  fn lookup( &self, command_name : &str ) -> Option< crate::CommandDefinition >
  {
    self.registry.command( command_name )
    .or_else( || self.registry.command( &format!( ".{command_name}" ) ) )
    .or_else( ||
    {
      // Handles the case where the user provides just the command name without
      // namespace; currently a single legacy mapping.
      if command_name == "echo"
      {
        self.registry.command( ".system.echo" )
      }
      else
      {
        None
      }
    })
  }

  ///
  /// Generates a help string for a single command using current verbosity level.
  ///
  /// The output format depends on the verbosity level (0-4).
  #[ must_use ]
  pub fn command( &self, command_name : &str ) -> Option< String >
  {
    let command = self.lookup( command_name )?;
    let renderer = PlainRenderer::default()
      .with_verbosity( self.verbosity )
      .with_options( self.display_options.clone() );
    Some( renderer.render( &help_command_data( &command ) ) )
  }

  ///
  /// Generates a parameter detail page for one parameter of a command.
  ///
  /// Uses the same lookup chain as `command()` and accepts the parameter's
  /// canonical name or any alias. When the command exists but the parameter does
  /// not, returns a listing of the command's valid parameters so a mistyped
  /// `param::??` request is never a dead end. Returns `None` only when the
  /// command itself is unknown.
  #[ must_use ]
  pub fn parameter( &self, command_name : &str, param_name : &str ) -> Option< String >
  {
    let command = self.lookup( command_name )?;
    if let Some( arg ) = find_parameter( &command, param_name )
    {
      let renderer = CliFmtRenderer::default().with_options( self.display_options.clone() );
      return Some( renderer.render_param( &help_command_data( &command ), &help_param_data( &command, arg ) ) );
    }

    Some( parameter_help_or_listing( &command, param_name ) )
  }



  ///
  /// Generates a summary list of all available commands.
  ///
  #[ must_use ]
  pub fn list_commands( &self ) -> String
  {
    self.list_commands_filtered( None )
  }

  ///
  /// Generates a summary list of commands filtered by prefix.
  ///
  /// # Arguments
  /// * `prefix` - Optional prefix filter (e.g., ".git", ".remove")
  ///
  /// # Returns
  /// Formatted string with categorized command list
  ///
  #[ must_use ]
  pub fn list_commands_filtered( &self, prefix : Option< &str > ) -> String
  {
    use std::collections::BTreeMap;
    use cli_fmt::help::{ CliHelpData, CliHelpStyle, CliHelpTemplate, CommandGroup, CommandEntry };

    let mut summary = String::new();

    // Filter commands by prefix and visibility
    let all_commands = self.registry.commands();
    let commands : Vec< ( &String, &crate::CommandDefinition ) > = all_commands
      .iter()
      .filter( |( name, cmd )|
      {
        // Apply prefix filter if provided
        let matches_prefix = prefix.map_or( true, |p| name.starts_with( p ) );

        // Hide commands marked as hidden_from_list
        let is_visible = !cmd.hidden_from_list();

        matches_prefix && is_visible
      })
      .collect();

    if commands.is_empty()
    {
      if let Some( p ) = prefix
      {
        writeln!( &mut summary, "No commands found matching prefix: {}", p ).unwrap();
      }
      else
      {
        writeln!( &mut summary, "No commands available." ).unwrap();
      }
      return summary;
    }

    // Group commands by category
    let mut by_category : BTreeMap< String, Vec< ( &String, &crate::CommandDefinition ) > > = BTreeMap::new();

    for ( name, cmd ) in commands
    {
      let category = if cmd.category().is_empty()
      {
        // Auto-detect category from command prefix
        self.auto_categorize( name )
      }
      else
      {
        cmd.category().to_string()
      };

      by_category.entry( category ).or_default().push( ( name, cmd ) );
    }

    // Delegate category-grouped, column-aligned rendering to cli_fmt.
    let mut data = CliHelpData::default();

    for ( category, mut cmds ) in by_category
    {
      // Sort by priority then name
      cmds.sort_by_key( |( name, cmd )| ( cmd.priority(), name.as_str() ) );

      let group_name = if category.is_empty() { String::new() } else { self.format_category_name( &category ) };

      let entries = cmds
        .into_iter()
        .map( |( name, cmd )| CommandEntry
        {
          name : name.clone(),
          desc : Self::cmd_effective_description( cmd ).to_string(),
        })
        .collect();

      data.groups.push( CommandGroup { name : group_name, entries } );
    }

    // Match the column layout the hand-rolled renderer used ( `"  {:<20} {}"`, zero-indent
    // category headers, no color ) so pre-existing tests asserting exact spacing keep passing.
    let style = CliHelpStyle
    {
      cmd_indent : 2,
      col_gap : 1,
      grp_indent : 0,
      tty_detect : false,
      ..CliHelpStyle::default()
    };
    let rendered = CliHelpTemplate::new( style, data ).render();
    // `render()` unconditionally prefixes a "Usage: {binary} <command>" + tagline preamble that
    // `HelpGenerator` has no binary-name concept to fill in; strip through the fixed "Commands:\n"
    // marker and keep unilang's own existing lead-in text instead.
    let body = rendered.split_once( "Commands:\n" ).map_or( rendered.as_str(), |( _, rest )| rest );
    writeln!( &mut summary, "Available commands:" ).unwrap();
    summary.push_str( body );

    // Add footer with usage hints
    if prefix.is_none()
    {
      writeln!( &mut summary, "Use '<command> ??' or '<command>.help' to get detailed help for a specific command." ).unwrap();
      writeln!( &mut summary, "Example: .list ??" ).unwrap();
    }

    summary
  }

  /// Selects `short_desc` if non-empty; falls back to `description`.
  fn cmd_effective_description( cmd: &crate::CommandDefinition ) -> &str
  {
    if cmd.short_desc().is_empty() { cmd.description() } else { cmd.short_desc() }
  }

  /// Returns empty string — categories must be explicit via `CommandDefinition::category()`.
  ///
  /// This architectural constraint is verified by tests to prevent implicit categorization.
  pub fn auto_categorize( &self, _name : &str ) -> String
  {
    String::new()
  }

  /// Transforms a snake_case category name to Title Case.
  ///
  /// `"git_operations"` → `"Git Operations"`. Uses a domain-agnostic algorithm.
  pub fn format_category_name( &self, category : &str ) -> String
  {
    category
      .split( '_' )
      .map( |word| {
        let mut chars = word.chars();
        match chars.next()
        {
          None => String::new(),
          Some( first ) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
      })
      .collect::<Vec<_>>()
      .join( " " )
  }
}
}

mod_interface::mod_interface!
{
  exposed use private::HelpGenerator;
  exposed use private::HelpVerbosity;
  exposed use private::HelpDisplayOptions;
  exposed use private::help_command_data;
  exposed use private::help_param_data;
  exposed use private::command_help_text;
  exposed use private::parameter_help_text;
  exposed use private::parameter_help_or_listing;

  prelude use private::HelpGenerator;
  prelude use private::HelpVerbosity;
  prelude use private::HelpDisplayOptions;
}
