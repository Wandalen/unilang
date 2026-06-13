//! Help, listing, and validation methods for CommandRegistry.

use super::dynamic::CommandRegistry;
use crate::data::{ CommandDefinition, OutputData };
use crate::error::Error;
use super::traits::{ CommandRoutine, format_command_help };

impl CommandRegistry
{
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
    // MANDATORY HELP ENFORCEMENT: This method now behaves identically to register_with_routine
    // because help generation is mandatory and automatic for all commands
    #[ allow( deprecated ) ]
    self.register_with_routine( &command, routine )
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
  /// if let Some(help_text) = registry.help_for_command(".example") {
  ///     println!("{}", help_text);
  /// }
  /// ```
  #[ must_use ]
  pub fn help_for_command( &self, command_name : &str ) -> Option< String >
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
  pub( super ) fn register_mandatory_global_help_command( &mut self )
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
    // Pitfall: register_with_routine(.help) always fails (already registered here), so
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
  pub( super ) fn create_help_routine( &self, parent_command : &CommandDefinition ) -> CommandRoutine
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
