use crate::data::{ ErrorData, OutputData };
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
  fn routine(&self, name: &str) -> Option<&CommandRoutine>;

  /// Get formatted help text for a command.
  fn help_for_command(&self, command_name: &str) -> Option<String>;
}
