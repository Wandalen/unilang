//!
//! The help generation components for the Unilang framework.
//!
//! This module provides flexible help text generation with configurable verbosity levels,
//! allowing applications to tailor help output to different user preferences and use cases.
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
  use core::fmt::Write;

  mod format_fns;

  mod verbosity;
  pub use verbosity::{ HelpVerbosity, HelpDisplayOptions };

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
    }
  }

  ///
  /// Creates a new `HelpGenerator` with specified verbosity level.
  ///
  #[ must_use ]
  pub fn with_verbosity( registry : & 'a CommandRegistry, verbosity : HelpVerbosity ) -> Self
  {
    Self { registry, verbosity }
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
  /// Generates a help string for a single command using current verbosity level.
  ///
  /// The output format depends on the verbosity level (0-4).
  #[ must_use ]
  pub fn command( &self, command_name : &str ) -> Option< String >
  {
    // Try exact match first, then try with dot prefix
    let command = self.registry.command( command_name )
    .or_else( || self.registry.command( &format!( ".{command_name}" ) ) )
    .or_else( ||
    {
      // If command_name is "echo", try ".system.echo"
      // If command_name is "cmd1.add", it should already be found.
      // This handles cases where the user provides just the command name without namespace,
      // or a partial namespace.
      // For now, a simple check for "echo" to ".system.echo"
      if command_name == "echo"
      {
        self.registry.command( ".system.echo" )
      }
      else
      {
        None
      }
    })?;

    match self.verbosity
    {
      HelpVerbosity::Minimal => Some( self.format_minimal( &command ) ),
      HelpVerbosity::Basic => Some( self.format_basic( &command ) ),
      HelpVerbosity::Standard => Some( self.format_standard( &command ) ),
      HelpVerbosity::Detailed => Some( self.format_detailed( &command ) ),
      HelpVerbosity::Comprehensive => Some( self.format_comprehensive( &command ) ),
    }
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

    // If only one category and it's empty, show flat list
    if by_category.len() == 1 && by_category.contains_key( "" )
    {
      writeln!( &mut summary, "Available commands:\n" ).unwrap();
      let mut cmds : Vec< _ > = by_category.get( "" ).unwrap().iter().collect();
      cmds.sort_by_key( |( name, cmd )| ( cmd.priority(), name.as_str() ) );

      for ( name, cmd ) in cmds
      {
        writeln!( &mut summary, "  {:<20} {}", name, Self::cmd_effective_description( cmd ) ).unwrap();
      }
    }
    else
    {
      // Show categorized output
      writeln!( &mut summary, "Available commands:\n" ).unwrap();

      for ( category, mut cmds ) in by_category
      {
        if !category.is_empty()
        {
          writeln!( &mut summary, "{}:", self.format_category_name( &category ) ).unwrap();
        }

        // Sort by priority then name
        cmds.sort_by_key( |( name, cmd )| ( cmd.priority(), name.as_str() ) );

        for ( name, cmd ) in cmds
        {
          writeln!( &mut summary, "  {:<20} {}", name, Self::cmd_effective_description( cmd ) ).unwrap();
        }
        writeln!( &mut summary ).unwrap();
      }
    }

    // Add footer with usage hints
    if prefix.is_none()
    {
      writeln!( &mut summary, "Use '<command> help' to get detailed help for a specific command." ).unwrap();
      writeln!( &mut summary, "Example: . .list help" ).unwrap();
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

  prelude use private::HelpGenerator;
  prelude use private::HelpVerbosity;
  prelude use private::HelpDisplayOptions;
}
