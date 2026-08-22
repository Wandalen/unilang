use crate::data::{ CommandDefinition, ErrorData, ErrorCode };
use crate::error::Error;
use std::collections::HashMap;
use super::metrics::PerformanceMetrics;
use super::traits::{ CommandRoutine, RegistryMode };
use super::map::DynamicCommandMap;
use super::builder::CommandRegistryBuilder;

///
/// A registry for commands, responsible for storing and managing all
/// available command definitions.
///
/// Uses a hybrid model: static commands are stored in a compile-time optimized registry for zero overhead,
/// while dynamic commands are stored in an optimized `DynamicCommandMap` with
/// intelligent caching for runtime flexibility and performance.
///
pub struct CommandRegistry
{
  /// Optimized dynamic command storage with intelligent caching
  pub( super ) dynamic_commands : DynamicCommandMap,
  /// A map of command names to their executable routines.
  pub( super ) routines : HashMap< String, CommandRoutine >,
  /// Opt-in fallback command name for empty-path invocations that carry arguments (FR-REG-10).
  pub( super ) default_command : Option< String >,
  // NOTE: help_conventions_enabled field removed - help is now mandatory for all commands
}

impl std::fmt::Debug for CommandRegistry
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    f.debug_struct( "CommandRegistry" )
      .field( "dynamic_commands", &self.dynamic_commands )
      .field( "routines_count", &self.routines.len() ) // CommandRoutine is Box<dyn Fn>, which doesn't impl Debug
      .field( "default_command", &self.default_command )
      .finish()
  }
}

impl CommandRegistry
{
  ///
  /// Creates a new, empty `CommandRegistry` for runtime command registration.
  ///
  /// ## ⚠️ Performance Notice
  ///
  /// Runtime command registration has **10-50x slower performance** than compile-time registration.
  ///
  /// ## When to Use This Constructor
  ///
  /// ✅ **Appropriate for:**
  /// - REPL applications requiring interactive command definition
  /// - Plugin systems with runtime command loading
  /// - Prototyping and rapid development workflows
  ///
  /// ⚡ **For production CLIs, use instead:**
  /// ```text
  /// StaticCommandRegistry::from_commands(&STATIC_COMMANDS)  // 50x faster
  /// ```
  ///
  /// ## Recommended Alternative for Production
  ///
  /// ```text
  /// // In build.rs:
  /// let aggregator = MultiYamlAggregator::new(config);
  /// aggregator.write_static_registry(&output_path)?;
  ///
  /// // In your application:
  /// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
  /// ```
  ///
  #[ must_use ]
  pub fn new() -> Self
  {
    let mut registry = Self
    {
      dynamic_commands : DynamicCommandMap::new(RegistryMode::default()),
      routines : HashMap::new(),
      default_command : None,
    };

    // MANDATORY GLOBAL HELP COMMAND - NO FLEXIBILITY
    // Every registry MUST have a global .help command - this is non-negotiable
    registry.register_mandatory_global_help_command();

    registry
  }

  ///
  /// Retrieves a command definition by name using hybrid lookup.
  ///
  /// This is the backward-compatible version that doesn't update metrics
  /// or use caching to maintain immutable access.
  ///
  #[ must_use ]
  pub fn command( &self, name : &str ) -> Option< CommandDefinition >
  {
    // CommandRegistry only handles dynamic commands
    // For static command support, use StaticCommandRegistry instead
    self.dynamic_commands.lookup( name )
  }

  ///
  /// Retrieves a command definition by name using optimized hybrid lookup with metrics.
  ///
  /// This version updates performance metrics and uses intelligent caching.
  /// The lookup strategy depends on the registry mode:
  /// - StaticOnly: Only check static registry
  /// - DynamicOnly: Only check dynamic commands
  /// - Hybrid: Check static first, then dynamic (default)
  /// - Auto: Use usage patterns to optimize lookup order
  ///
  #[ must_use ]
  pub fn command_optimized( &mut self, name : &str ) -> Option< CommandDefinition >
  {
    // CommandRegistry only handles dynamic commands
    // For static command support with optimized lookup, use StaticCommandRegistry instead
    self.dynamic_commands.get( name )
  }

  ///
  /// Registers a command, adding it to the dynamic registry.
  ///
  /// **Automatic Help Generation:** When `command.auto_help_enabled()` is `true` (default),
  /// this method automatically generates a `.command.help` variant, ensuring all registered
  /// commands appear in help listings.
  ///
  /// This prevents the help divergence bug where commands are registered but invisible
  /// in help output.
  ///
  /// **Validation:** This method validates the command definition before registration,
  /// checking for proper naming conventions, namespace format, and parameter storage types.
  /// This ensures both `register()` and `register_with_routine()` enforce identical invariants.
  ///
  /// **Duplicate Detection:** If a command with the same name already exists, this method
  /// returns an error. To replace an existing command, use `unregister()` first or use
  /// `register_or_replace()` for explicit overwrite behavior.
  ///
  /// Note: Static commands cannot be overwritten and will take precedence in lookups.
  ///
  /// # Errors
  ///
  /// Returns `Error::Registration` if:
  /// - Command name doesnt start with '.' prefix
  /// - Namespace is non-empty but doesnt start with '.' prefix
  /// - Parameter has `multiple:true` but non-List storage type
  /// - Command with same name already exists (duplicate)
  pub fn register( &mut self, command : CommandDefinition ) -> Result< (), Error >
  {
    // VALIDATION: Enforce same invariants as register_with_routine (Phase 1 Fix)
    // This closes the code path divergence vulnerability where register() didn't validate
    // but register_with_routine() did, allowing invalid commands via one path.
    crate::command_validation::validate_command_for_registration( &command )?;

    let full_name = command.full_name();

    // DUPLICATE DETECTION: Prevent silent overwrite (Phase 1 Fix)
    // Previously, duplicate registration silently overwrote the first command.
    // This caused production bugs where first registration disappeared without warning.
    if self.dynamic_commands.contains_key( &full_name )
    {
      return Err( Error::Registration( format!(
        "Command '{}' is already registered. Cannot register duplicate commands. \
        Use unregister() first or register_or_replace() for explicit overwrite.",
        full_name
      )));
    }

    // Register main command
    self.dynamic_commands.insert( full_name.clone(), command.clone() );

    // AUTO-GENERATE HELP (same logic as register_with_routine)
    if command.auto_help_enabled() && !crate::command_validation::is_help_command( &full_name )
    {
      let help_command = command.generate_help_command();
      let help_name = crate::command_validation::make_help_command_name( &full_name );

      if !self.dynamic_commands.contains_key( &help_name )
      {
        // Register help command definition
        self.dynamic_commands.insert( help_name.clone(), help_command );

        // Create and register help routine
        let help_routine = self.create_help_routine( &command );
        self.routines.insert( help_name, help_routine );
      }
    }

    Ok(())
  }

  ///
  /// Registers a command with its executable routine at runtime.
  ///
  /// ## ⚠️ Performance Notice
  ///
  /// Runtime command registration has **10-50x slower performance** than compile-time registration.
  ///
  /// ## When to Use This Method
  ///
  /// ✅ **Appropriate for:**
  /// - REPL applications requiring interactive command definition
  /// - Plugin systems where commands are loaded from external sources
  /// - Prototyping and rapid development workflows
  ///
  /// ⚡ **For production CLI applications:**
  /// Use static command definitions generated at build time via `build.rs` and loaded with
  /// `StaticCommandRegistry::from_commands(&STATIC_COMMANDS)` for zero-cost lookups.
  ///
  /// # Arguments
  ///
  /// * `command_def` - The command definition
  /// * `routine` - The function that executes the command logic
  ///
  /// # Errors
  ///
  /// Returns an `Error::Registration` if a command with the same name
  /// is already registered and cannot be overwritten (e.g., if it was
  /// a compile-time registered command).
  ///
  pub fn register_with_routine( &mut self, command_def : &CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // EXPLICIT COMMAND NAMING ENFORCEMENT (FR-REG-6)
    // Following the governing principle: minimum implicit magic!

    // Validate command definition using centralized validation module
    crate::command_validation::validate_command_for_registration( command_def )?;

    // Build full command name using CommandDefinition's method
    let full_name = command_def.full_name();
    // Check if command exists in dynamic registry
    // Note: Static command conflicts should be checked by StaticCommandRegistry
    if self.dynamic_commands.contains_key( &full_name )
    {
      return Err( Error::Execution( ErrorData::new(
        ErrorCode::CommandAlreadyExists,
        format!( "Registration Error: Command '{full_name}' already exists. Use a different name or remove the existing command first." ),
      )));
    }

    // Register the main command
    self.dynamic_commands.insert( full_name.clone(), command_def.clone() );
    self.routines.insert( full_name.clone(), routine );

    // AUTO HELP GENERATION - Respects auto_help_enabled field
    // Generate help command only if auto_help_enabled is true
    if command_def.auto_help_enabled() && !crate::command_validation::is_help_command( &full_name )
    {
      let help_command = command_def.generate_help_command();
      let help_routine = self.create_help_routine( command_def );

      // Register the auto-generated help command
      let help_name = crate::command_validation::make_help_command_name( &full_name );
      if !self.dynamic_commands.contains_key( &help_name )
      {
        self.dynamic_commands.insert( help_name.clone(), help_command );
        self.routines.insert( help_name, help_routine );
      }
    }

    Ok(())
  }

  ///
  /// Retrieves the routine for a given command name.
  ///
  #[ must_use ]
  pub fn routine( &self, command_name : &str ) -> Option< &CommandRoutine >
  {
    self.routines.get( command_name )
  }

  ///
  /// Returns a collection of all command definitions (both static and dynamic).
  ///
  /// This is provided for backward compatibility and introspection.
  /// Static commands are converted from the optimized static registry.
  ///
  #[ must_use ]
  pub fn commands( &self ) -> HashMap< String, CommandDefinition >
  {
    let mut all_commands = HashMap::new();

    // Add static commands (none available in CommandRegistry - use StaticCommandRegistry instead)
    // Static commands are only available in StaticCommandRegistry

    // Add dynamic commands (they can override static ones in this view)
    for ( name, cmd ) in self.dynamic_commands.iter()
    {
      all_commands.insert( name.clone(), cmd.clone() );
    }

    all_commands
  }

  ///
  /// Returns a builder for creating a `CommandRegistry` with a fluent API.
  ///
  #[ must_use ]
  pub fn builder() -> CommandRegistryBuilder
  {
    CommandRegistryBuilder::new()
  }

  ///
  /// Set the registry mode for optimized command lookup.
  ///
  /// This controls which command sources are checked during lookup:
  /// - StaticOnly: Only check compile-time optimized registry
  /// - DynamicOnly: Only check runtime-registered commands
  /// - Hybrid: Check both (static first, then dynamic)
  /// - Auto: Use adaptive strategies based on usage patterns
  ///
  /// # Arguments
  /// * `mode` - The registry mode to use
  ///
  /// # Examples
  /// ```rust
  /// use unilang::{CommandRegistry, RegistryMode};
  ///
  /// let mut registry = CommandRegistry::new();
  /// registry.set_mode(RegistryMode::StaticOnly);
  /// ```
  pub fn set_mode( &mut self, mode : RegistryMode )
  {
    self.dynamic_commands.set_mode( mode );
  }

  ///
  /// Get the current registry mode.
  ///
  #[ must_use ]
  pub fn registry_mode( &self ) -> RegistryMode
  {
    self.dynamic_commands.mode()
  }

  ///
  /// Get performance metrics for command lookups.
  ///
  /// Returns metrics including cache hit rates, lookup counts,
  /// and static vs dynamic usage patterns.
  ///
  #[ must_use ]
  pub fn performance_metrics( &self ) -> &PerformanceMetrics
  {
    self.dynamic_commands.metrics()
  }

  ///
  /// Clear the dynamic command cache.
  ///
  /// This forces all subsequent dynamic command lookups to go through
  /// the main IndexMap storage, useful for testing or memory management.
  ///
  pub fn clear_cache( &mut self )
  {
    self.dynamic_commands.clear_cache();
  }

  ///
  /// Returns the configured default command name, if any (FR-REG-10).
  ///
  /// The default command is the fallback an empty command path carrying at least one
  /// argument routes to (see `set_default_command`). Returns `None` when no default has
  /// been configured — the ordinary case, and the only case for a registry that never
  /// calls `set_default_command`.
  ///
  #[ must_use ]
  pub fn default_command( &self ) -> Option< &str >
  {
    self.default_command.as_deref()
  }

  ///
  /// Configures a fallback command for empty-path invocations that carry arguments (FR-REG-10).
  ///
  /// When set, an invocation whose command path is empty (no dot-prefixed token resolves
  /// into `command_path_slices`) but carries at least one named or positional argument
  /// routes to `name` instead of failing with an unknown-parameter error. An invocation
  /// that resolves any explicit command path is never affected. This is strictly opt-in:
  /// a registry that never calls this method sees no behavior change.
  ///
  /// `name` is validated with the same rules as command registration (`CommandName::new`),
  /// but existence is **not** checked here — `name` need not already be registered. An
  /// invocation that falls back to an unregistered default surfaces the ordinary
  /// `CommandNotFound` error at analysis time, not at configuration time.
  ///
  /// # Errors
  ///
  /// Returns `Error::MissingDotPrefix` or `Error::EmptyCommandName` if `name` fails
  /// `CommandName` validation.
  ///
  pub fn set_default_command( &mut self, name : &str ) -> Result< (), Error >
  {
    let validated = crate::data::CommandName::new( name )?;
    self.default_command = Some( validated.into_inner() );
    Ok(())
  }
}

impl Default for CommandRegistry
{
  fn default() -> Self
  {
    #[ allow( deprecated ) ]
    Self::new()
  }
}
