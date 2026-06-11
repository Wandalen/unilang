use crate::data::{ CommandDefinition, ErrorData, ErrorCode, OutputData };
use crate::error::Error;
use std::collections::HashMap;
use super::metrics::PerformanceMetrics;
use super::traits::{ CommandRoutine, RegistryMode, format_command_help };
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
  dynamic_commands : DynamicCommandMap,
  /// A map of command names to their executable routines.
  routines : HashMap< String, CommandRoutine >,
  // NOTE: help_conventions_enabled field removed - help is now mandatory for all commands
}

impl std::fmt::Debug for CommandRegistry
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    f.debug_struct( "CommandRegistry" )
      .field( "dynamic_commands", &self.dynamic_commands )
      .field( "routines_count", &self.routines.len() ) // CommandRoutine is Box<dyn Fn>, which doesn't impl Debug
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
    self.dynamic_commands.get_readonly( name )
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
  /// This ensures both `register()` and `command_add_runtime()` enforce identical invariants.
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
    // VALIDATION: Enforce same invariants as command_add_runtime (Phase 1 Fix)
    // This closes the code path divergence vulnerability where register() didn't validate
    // but command_add_runtime() did, allowing invalid commands via one path.
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

    // AUTO-GENERATE HELP (same logic as command_add_runtime)
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
  pub fn command_add_runtime( &mut self, command_def : &CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
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
  pub fn get_routine( &self, command_name : &str ) -> Option< &CommandRoutine >
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
  /// registry.set_registry_mode(RegistryMode::StaticOnly);
  /// ```
  pub fn set_registry_mode( &mut self, mode : RegistryMode )
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
  /// Returns a formatted string listing all registered commands with their descriptions.
  ///
  /// This method generates a user-friendly command listing that can be displayed
  /// in help output, showing each command name and its hint/description.
  /// The listing is automatically synchronized with the registry - all registered
  /// commands will appear, preventing the help divergence bug.
  ///
  /// # Returns
  /// * `String` - Formatted command listing with one command per line
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::new();
  /// let listing = registry.format_command_listing();
  /// println!("{}", listing);
  /// // Output:
  /// // Available commands:
  /// //   .help                         - Display help for all commands
  /// ```
  #[ must_use ]
  pub fn format_command_listing( &self ) -> String
  {
    let mut output = String::from( "Available commands:\n" );

    // Get all commands and filter out help commands
    let commands = self.commands();
    #[ allow( clippy::case_sensitive_file_extension_comparisons ) ] // .help is a command suffix, not a file extension
    let mut command_names : Vec< String > = commands
      .keys()
      .filter( | name | !name.ends_with( ".help" ) )
      .cloned()
      .collect();

    // Sort for consistent output
    command_names.sort();

    for cmd_name in command_names
    {
      if let Some( cmd ) = commands.get( &cmd_name )
      {
        // Use description for the listing (primary documentation)
        let description = &cmd.description();

        // Format: "  .command_name                  - Description"
        output.push_str( &format!( "  {:<30} - {}\n", cmd_name, description ) );
      }
    }

    output
  }

  ///
  /// Validates that all registered commands have corresponding help commands.
  ///
  /// This method checks the registry for completeness - ensuring that every
  /// command has a `.command.help` variant. This prevents the help divergence
  /// bug where commands are registered but don't appear in help.
  ///
  /// # Returns
  /// * `Ok(())` - All commands have help
  /// * `Err(Vec<String>)` - List of command names missing help
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  /// use unilang::data::CommandDefinition;
  ///
  /// let mut registry = CommandRegistry::new();
  ///
  /// // Validation should pass for properly registered commands
  /// let result = registry.validate_help_completeness();
  /// assert!(result.is_ok());
  /// ```
  pub fn validate_help_completeness( &self ) -> Result< (), Vec< String > >
  {
    let mut missing_help = Vec::new();

    let commands = self.commands();

    // Check each command (excluding help commands themselves)
    for cmd_name in commands.keys()
    {
      // Skip help commands and the global .help command
      #[ allow( clippy::case_sensitive_file_extension_comparisons ) ] // .help is a command suffix, not a file extension
      if cmd_name.ends_with( ".help" ) || cmd_name == ".help"
      {
        continue;
      }

      // Check if this command has auto_help_enabled
      if let Some( cmd ) = commands.get( cmd_name )
      {
        // Only validate if auto_help is enabled for this command
        if cmd.auto_help_enabled()
        {
          let help_name = format!( "{}.help", cmd_name );
          if !commands.contains_key( &help_name )
          {
            missing_help.push( cmd_name.clone() );
          }
        }
      }
    }

    if missing_help.is_empty()
    {
      Ok( () )
    }
    else
    {
      Err( missing_help )
    }
  }

  ///
  /// Registers a command with automatic help command generation.
  ///
  /// This method provides explicit control over help generation, registering the main command
  /// and optionally generating a `.command.help` counterpart based on the command's configuration
  /// and the registry's global help conventions setting.
  ///
  /// # Arguments
  /// * `command` - The command definition to register
  /// * `routine` - The executable routine for the command
  ///
  /// # Returns
  /// * `Result<(), Error>` - Success or registration error
  ///
  /// # Errors
  /// Returns an error if command registration fails due to invalid naming or other validation issues.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::{registry::CommandRegistry, data::{CommandDefinition, OutputData}};
  ///
  /// # fn example() -> Result<(), unilang::Error> {
  /// let mut registry = CommandRegistry::new();
  /// let cmd = CommandDefinition::former()
  ///     .name(".example")
  ///     .description("Example command".to_string())
  ///     .end()
  ///     .with_auto_help(true);
  ///
  /// let routine = Box::new(|_cmd, _ctx| {
  ///     Ok(OutputData {
  ///         content: "Success".to_string(),
  ///         format: "text".to_string(),
  ///         execution_time_ms: None,
  ///     })
  /// });
  /// registry.register_with_auto_help(cmd, routine)?;
  /// // Both ".example" and ".example.help" are now registered
  /// # Ok(())
  /// # }
  /// ```
  pub fn register_with_auto_help( &mut self, command : CommandDefinition, routine : CommandRoutine ) -> Result< (), Error >
  {
    // MANDATORY HELP ENFORCEMENT: This method now behaves identically to command_add_runtime
    // because help generation is mandatory and automatic for all commands
    #[ allow( deprecated ) ]
    self.command_add_runtime( &command, routine )
  }

  ///
  /// Retrieves formatted help text for any registered command.
  ///
  /// This method generates comprehensive help information for a given command,
  /// including its description, arguments, usage examples, and metadata.
  /// It works with both static and dynamic commands.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command (e.g., ".example" or ".cmd2.list")
  ///
  /// # Returns
  /// * `Option<String>` - Formatted help text, or None if command not found
  ///
  /// # Examples
  /// ```rust
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::new();
  /// if let Some(help_text) = registry.get_help_for_command(".example") {
  ///     println!("{}", help_text);
  /// }
  /// ```
  #[ must_use ]
  pub fn get_help_for_command( &self, command_name : &str ) -> Option< String >
  {
    self.command( command_name ).map( | cmd_def | self.format_help_text( &cmd_def ) )
  }

  ///
  /// Registers the mandatory global help command.
  ///
  /// This internal method creates and registers the global `.help` command
  /// that lists all available commands in the registry. This command is
  /// automatically registered in every new CommandRegistry instance.
  ///
  /// **MANDATORY ENFORCEMENT:** This method is called automatically during
  /// registry construction and cannot be disabled or bypassed.
  fn register_mandatory_global_help_command( &mut self )
  {
    use crate::data::{ CommandName, CommandStatus, VersionType };

    let global_help_command = CommandDefinition::new(
      CommandName::new( ".help" ).expect( "valid help command name" ),
      "Display help information for all available commands".to_string(),
    )
    .with_namespace( String::new() )
    .with_hint( "Global help system" )
    .with_status( CommandStatus::Active )
    .with_version( VersionType::new( "1.0.0" ).expect( "valid version" ) )
    .with_arguments( vec![] )
    .with_routine_link( None )
    .with_tags( vec![ "help".to_string(), "system".to_string(), "global".to_string() ] )
    .with_aliases( vec![ ".h".to_string(), ".help".to_string() ] )
    .with_permissions( vec![] )
    .with_idempotent( true )
    .with_http_method_hint( "GET" )
    .with_examples( vec![ ".help".to_string(), ".h".to_string() ] )
    .with_auto_help( false ) // Prevent recursive help for help command
    .with_category( "help" )
    .with_short_desc( "Show help for all commands" )
    // Fix(BUG-102): Hide .help from its own listing.
    // Root cause: .help registered here with hidden_from_list=false, so interpreter's
    // list_commands_filtered showed .help listing itself — self-referential noise.
    // Pitfall: command_add_runtime(.help) always fails (already registered here), so
    // the only place to set hidden_from_list is this mandatory registration.
    .with_hidden_from_list( true )
    .with_priority( 0 )
    .with_group( "" );

    let global_help_routine = Box::new( | _cmd, _ctx |
    {
      // Generate global help content listing all commands
      let mut help_content = String::new();
      help_content.push_str( "Available Commands:\n\n" );
      help_content.push_str( "Use '.command.help' to get detailed help for any specific command.\n" );
      help_content.push_str( "Examples: '.cmd1.process.help', '.cmd2.list.help'\n\n" );
      help_content.push_str( "Global Commands:\n" );
      help_content.push_str( "  .help    Display this help information\n" );

      Ok( OutputData
      {
        content : help_content,
        format : "text".to_string(),
        execution_time_ms : None,
      })
    });

    // Force-register the global help command bypassing normal validation
    // This is the only exception to the rule that all commands must have help
    self.dynamic_commands.insert( ".help".to_string(), global_help_command );
    self.routines.insert( ".help".to_string(), global_help_routine );
  }

  ///
  /// Creates a help routine for a given command.
  ///
  /// This internal method generates the executable routine that will be used
  /// for `.command.help` commands. The routine returns formatted help information
  /// about the parent command.
  ///
  /// # Arguments
  /// * `parent_command` - The command for which to create a help routine
  ///
  /// # Returns
  /// * `CommandRoutine` - An executable routine that returns help information
  fn create_help_routine( &self, parent_command : &CommandDefinition ) -> CommandRoutine
  {
    let help_text = self.format_help_text( parent_command );

    Box::new( move | _cmd, _ctx |
    {
      Ok( OutputData
      {
        content : help_text.clone(),
        format : "text".to_string(),
        execution_time_ms : None,
      })
    })
  }

  ///
  /// Formats comprehensive help text for a command definition.
  ///
  /// This internal method generates detailed, human-readable help information
  /// including command description, arguments with types and defaults,
  /// usage examples, and metadata.
  ///
  /// # Arguments
  /// * `cmd_def` - The command definition to format help for
  ///
  /// # Returns
  /// * `String` - Formatted help text
  fn format_help_text( &self, cmd_def : &CommandDefinition ) -> String
  {
    format_command_help( cmd_def )
  }

  ///
  /// Creates a new `CommandRegistry` from static commands.
  ///
  /// This method enables integration between static and dynamic command registries
  /// by converting static command definitions to dynamic ones. All commands from
  /// the provided static map will be added to the new registry's dynamic storage.
  ///
  /// # Arguments
  /// * `static_commands` - A static command map containing compile-time command definitions
  ///
  /// # Returns
  /// A new `CommandRegistry` containing all commands from the static map
  ///
  /// # Performance Note
  /// This conversion has one-time O(n) cost where n is the number of static commands.
  /// Once converted, dynamic lookup performance applies (slower than static lookups).
  /// Consider using `StaticCommandRegistry` directly for better performance.
  ///
  /// # Examples
  /// ```text
  /// use unilang::{ registry::CommandRegistry, static_data::StaticCommandMap };
  ///
  /// // Create registry from static commands
  /// let registry = CommandRegistry::from_static_commands( &STATIC_COMMANDS );
  /// ```
  #[ must_use ]
  #[ cfg( feature = "static_registry" ) ]
  pub fn from_static_commands( static_commands : &crate::static_data::StaticCommandMap ) -> Self
  {
    #[ allow( deprecated ) ]
    let mut registry = Self::new();

    // Convert each static command to dynamic and register it
    for ( _command_name, static_cmd ) in static_commands.entries()
    {
      let dynamic_cmd = crate::data::CommandDefinition::from( *static_cmd );

      // Skip .help command if it already exists (mandatory global help is registered in new())
      // The mandatory help is non-negotiable and takes precedence over static definitions
      if dynamic_cmd.full_name() == ".help" && registry.dynamic_commands.contains_key(".help")
      {
        continue;
      }

      registry.register( dynamic_cmd )
        .expect( "Static commands should always be valid - this is a build-time generation bug" );
    }

    registry
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

impl super::traits::CommandRegistryTrait for CommandRegistry {
  fn command(&self, name: &str) -> Option<crate::data::CommandDefinition> {
    self.command(name)
  }

  fn commands(&self) -> std::collections::HashMap<String, crate::data::CommandDefinition> {
    self.commands()
  }

  fn get_routine(&self, name: &str) -> Option<&CommandRoutine> {
    self.get_routine(name)
  }

  fn get_help_for_command(&self, command_name: &str) -> Option<String> {
    self.get_help_for_command(command_name)
  }
}
