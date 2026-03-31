use crate::data::{ CommandDefinition, ErrorData, OutputData };
use crate::interpreter::ExecutionContext;

/// Type alias for a command routine.
/// A routine takes a `VerifiedCommand` and an `ExecutionContext`, and returns a `Result` of `OutputData` or `ErrorData`.
pub type CommandRoutine = Box< dyn Fn( crate::semantic::VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static >;

/// Registry operation mode for hybrid command lookup optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RegistryMode {
  /// Only static commands are used (compile-time optimized lookup only)
  StaticOnly,
  /// Only dynamic commands are used (HashMap lookup only)
  DynamicOnly,
  /// Hybrid mode with both static and dynamic commands (default)
  #[default]
  Hybrid,
  /// Automatic mode selection based on usage patterns
  Auto,
}

/// Helper function to format help text for a command definition.
///
/// This function generates a standardized help text format that includes:
/// - Command header (name, description, hint, version, status)
/// - Arguments section with details about each parameter
/// - Examples section
/// - Aliases section
/// - Usage patterns
///
/// Used by both `CommandRegistry` and `StaticCommandRegistry` to ensure consistent help formatting.
pub(super) fn format_command_help( cmd_def : &CommandDefinition ) -> String
{
  let mut help = String::new();

  // Command header
  help.push_str( &format!( "Command: {}\n", cmd_def.name().as_str() ) );
  help.push_str( &format!( "Description: {}\n", cmd_def.description() ) );

  if !cmd_def.hint().is_empty()
  {
    help.push_str( &format!( "Hint: {}\n", cmd_def.hint() ) );
  }

  if cmd_def.show_version_in_help() && !cmd_def.version().as_str().is_empty()
  {
    help.push_str( &format!( "Version: {}\n", cmd_def.version().as_str() ) );
  }

  // Status is now an enum, show if not Active
  match cmd_def.status()
  {
    crate::data::CommandStatus::Active => {},
    status => help.push_str( &format!( "Status: {:?}\n", status ) ),
  }

  // Arguments section
  if !cmd_def.arguments().is_empty()
  {
    help.push_str( "\nArguments:\n" );
    for arg in cmd_def.arguments()
    {
      let required = if arg.attributes.optional { "optional" } else { "required" };
      help.push_str( &format!( "  {} ({}, {})", arg.name, arg.kind, required ) );

      if let Some( default ) = &arg.attributes.default
      {
        help.push_str( &format!( " [default: {}]", default ) );
      }

      help.push_str( &format!( "\n    {}\n", arg.description ) );

      if !arg.aliases.is_empty()
      {
        help.push_str( &format!( "    Aliases: {}\n", arg.aliases.join( ", " ) ) );
      }
    }
  }

  // Examples section
  if !cmd_def.examples().is_empty()
  {
    help.push_str( "\nExamples:\n" );
    for example in cmd_def.examples()
    {
      help.push_str( &format!( "  {}\n", example ) );
    }
  }

  // Aliases section
  if !cmd_def.aliases().is_empty()
  {
    help.push_str( &format!( "\nAliases: {}\n", cmd_def.aliases().join( ", " ) ) );
  }

  // Usage patterns
  help.push_str( "\nUsage:\n" );
  help.push_str( &format!( "  {}  # Execute command\n", cmd_def.name().as_str() ) );
  help.push_str( &format!( "  {}.help  # Show this help\n", cmd_def.name().as_str() ) );
  help.push_str( &format!( "  {} ??  # Alternative help access\n", cmd_def.name().as_str() ) );

  help
}

/// Common trait for command registries to enable interoperability.
///
/// This trait defines the minimal interface required by components like
/// Pipeline, SemanticAnalyzer, and Interpreter to work with any registry type.
pub trait CommandRegistryTrait {
  /// Get a command definition by name.
  fn command(&self, name: &str) -> Option<crate::data::CommandDefinition>;

  /// Get all commands as a HashMap.
  fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition>;

  /// Get a command routine for execution.
  fn get_routine(&self, name: &str) -> Option<&CommandRoutine>;

  /// Get formatted help text for a command.
  fn get_help_for_command(&self, command_name: &str) -> Option<String>;
}
