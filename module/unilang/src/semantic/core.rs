use crate::data::{ ErrorData, ErrorCode };
use crate::error::Error;
use crate::registry::CommandRegistry;
use crate::types::Value;
use unilang_parser::GenericInstruction;
use std::collections::HashMap;

///
/// Represents a command that has been verified against the command registry.
///
/// This struct holds the command's definition and the arguments provided
/// by the user, ensuring that the command is valid and ready for execution.
#[ derive( Debug, Clone ) ]
pub struct VerifiedCommand
{
  /// The definition of the command.
  pub definition : crate::data::CommandDefinition,
  /// The arguments provided for the command, parsed and typed.
  pub arguments : HashMap< String, Value >,
}

impl VerifiedCommand
{
  /// Extracts a string argument by name, returning None if not found or wrong type.
  ///
  /// # Examples
  /// ```
  /// # use unilang::prelude::*;
  /// # use std::collections::HashMap;
  /// # let mut args = HashMap::new();
  /// # args.insert("name".to_string(), Value::String("Alice".to_string()));
  /// # let cmd_name = CommandName::new(".test").unwrap();
  /// # let definition = CommandDefinition::new(cmd_name, "Test".to_string());
  /// # let cmd = VerifiedCommand { definition, arguments: args };
  /// let name = cmd.get_string("name").unwrap_or("World");
  /// assert_eq!(name, "Alice");
  /// ```
  #[ must_use ]
  pub fn get_string( &self, name : &str ) -> Option< &str >
  {
    self.arguments.get( name ).and_then( | v |
      if let Value::String( s ) = v { Some( s.as_str() ) } else { None }
    )
  }

  /// Extracts a required string argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  ///
  /// # Examples
  /// ```
  /// # use unilang::prelude::*;
  /// # use std::collections::HashMap;
  /// # let mut args = HashMap::new();
  /// # args.insert("name".to_string(), Value::String("Alice".to_string()));
  /// # let cmd_name = CommandName::new(".test").unwrap();
  /// # let definition = CommandDefinition::new(cmd_name, "Test".to_string());
  /// # let cmd = VerifiedCommand { definition, arguments: args };
  /// let name = cmd.require_string("name")?;
  /// assert_eq!(name, "Alice");
  /// # Ok::<(), unilang::Error>(())
  /// ```
  pub fn require_string( &self, name : &str ) -> Result< &str, Error >
  {
    self.get_string( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected string value for argument '{}'", name ),
      ))
    )
  }

  /// Extracts and trims an optional string argument.
  ///
  /// Returns `Some("")` for whitespace-only input; callers use `.filter( | s | !s.is_empty() )`
  /// if a non-empty value is required. No allocation — borrows from self.
  ///
  /// # Examples
  /// ```
  /// # use unilang::prelude::*;
  /// # use std::collections::HashMap;
  /// # let mut args = HashMap::new();
  /// # args.insert("name".to_string(), Value::String("  Alice  ".to_string()));
  /// # let cmd_name = CommandName::new(".test").unwrap();
  /// # let definition = CommandDefinition::new(cmd_name, "Test".to_string());
  /// # let cmd = VerifiedCommand { definition, arguments: args };
  /// let name = cmd.get_string_normalized("name");
  /// assert_eq!(name, Some("Alice"));
  /// # Ok::<(), unilang::Error>(())
  /// ```
  #[ must_use ]
  pub fn get_string_normalized< 'a >( &'a self, name : &str ) -> Option< &'a str >
  {
    self.get_string( name ).map( | s | s.trim() )
  }

  /// Extracts and trims a required string argument.
  ///
  /// Same trim semantics as `get_string_normalized()`. Returns `Ok("")` for whitespace-only
  /// input; `require` only checks presence, not non-emptiness.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  ///
  /// # Examples
  /// ```
  /// # use unilang::prelude::*;
  /// # use std::collections::HashMap;
  /// # let mut args = HashMap::new();
  /// # args.insert("name".to_string(), Value::String("  Alice  ".to_string()));
  /// # let cmd_name = CommandName::new(".test").unwrap();
  /// # let definition = CommandDefinition::new(cmd_name, "Test".to_string());
  /// # let cmd = VerifiedCommand { definition, arguments: args };
  /// let name = cmd.require_string_normalized("name")?;
  /// assert_eq!(name, "Alice");
  /// # Ok::<(), unilang::Error>(())
  /// ```
  pub fn require_string_normalized< 'a >( &'a self, name : &str ) -> Result< &'a str, Error >
  {
    self.require_string( name ).map( | s | s.trim() )
  }

  /// Extracts an integer argument by name, returning None if not found or wrong type.
  #[ must_use ]
  pub fn get_integer( &self, name : &str ) -> Option< i64 >
  {
    self.arguments.get( name ).and_then( | v |
      if let Value::Integer( i ) = v { Some( *i ) } else { None }
    )
  }

  /// Extracts a required integer argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  pub fn require_integer( &self, name : &str ) -> Result< i64, Error >
  {
    self.get_integer( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected integer value for argument '{}'", name ),
      ))
    )
  }

  /// Extracts a float argument by name, returning None if not found or wrong type.
  #[ must_use ]
  pub fn get_float( &self, name : &str ) -> Option< f64 >
  {
    self.arguments.get( name ).and_then( | v |
      if let Value::Float( f ) = v { Some( *f ) } else { None }
    )
  }

  /// Extracts a required float argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  pub fn require_float( &self, name : &str ) -> Result< f64, Error >
  {
    self.get_float( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected float value for argument '{}'", name ),
      ))
    )
  }

  /// Extracts a boolean argument by name, returning None if not found or wrong type.
  #[ must_use ]
  pub fn get_boolean( &self, name : &str ) -> Option< bool >
  {
    self.arguments.get( name ).and_then( | v |
      if let Value::Boolean( b ) = v { Some( *b ) } else { None }
    )
  }

  /// Extracts a required boolean argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  pub fn require_boolean( &self, name : &str ) -> Result< bool, Error >
  {
    self.get_boolean( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected boolean value for argument '{}'", name ),
      ))
    )
  }

  /// Extracts a path argument by name, returning None if not found or wrong type.
  #[ must_use ]
  pub fn get_path( &self, name : &str ) -> Option< &std::path::Path >
  {
    self.arguments.get( name ).and_then( | v |
      match v
      {
        Value::Path( p ) | Value::File( p ) | Value::Directory( p ) => Some( p.as_path() ),
        _ => None,
      }
    )
  }

  /// Extracts a required path argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  pub fn require_path( &self, name : &str ) -> Result< &std::path::Path, Error >
  {
    self.get_path( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected path value for argument '{}'", name ),
      ))
    )
  }

  /// Extracts a list argument by name, returning None if not found or wrong type.
  #[ must_use ]
  pub fn get_list( &self, name : &str ) -> Option< &Vec< Value > >
  {
    self.arguments.get( name ).and_then( | v |
      if let Value::List( l ) = v { Some( l ) } else { None }
    )
  }

  /// Extracts a required list argument, returning an error if not found or wrong type.
  ///
  /// # Errors
  /// Returns `Error::Execution` if argument is missing or has wrong type.
  pub fn require_list( &self, name : &str ) -> Result< &Vec< Value >, Error >
  {
    self.get_list( name ).ok_or_else( ||
      Error::Execution( ErrorData::new(
        ErrorCode::ArgumentTypeMismatch,
        format!( "Argument Error: Expected list value for argument '{}'", name ),
      ))
    )
  }

  /// Returns true if the argument exists (regardless of type).
  #[ must_use ]
  pub fn has_argument( &self, name : &str ) -> bool
  {
    self.arguments.contains_key( name )
  }

  /// Gets a raw Value reference for an argument, returning None if not found.
  #[ must_use ]
  pub fn get_value( &self, name : &str ) -> Option< &Value >
  {
    self.arguments.get( name )
  }
}

///
/// The semantic analyzer, responsible for validating the parsed program.
///
/// The analyzer checks the program against the command registry to ensure
/// that commands exist, arguments are correct, and types match.
#[ derive() ] // Removed Debug
#[ allow( missing_debug_implementations ) ]
pub struct SemanticAnalyzer< 'a >
{
  pub( in super ) instructions : & 'a [ GenericInstruction ],
  pub( in super ) registry : & 'a CommandRegistry,
}

impl< 'a > SemanticAnalyzer< 'a >
{
  ///
  /// Creates a new `SemanticAnalyzer`.
  ///
  #[ must_use ]
  pub fn new( instructions : & 'a [ GenericInstruction ], registry : & 'a CommandRegistry ) -> Self
  {
    Self { instructions, registry }
  }

  ///
  /// Analyzes the program and returns a list of verified commands or an error.
  ///
  /// This is the main entry point for semantic analysis, processing each
  /// statement in the program.
  ///
  /// # Errors
  ///
  /// Returns an error if any command is not found, if arguments are invalid,
  /// or if any other semantic rule is violated.
  pub fn analyze( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    // Catch panics and convert them to user-friendly errors
    let result = std::panic::catch_unwind( core::panic::AssertUnwindSafe( || {
      self.analyze_internal()
    }));

    match result
    {
      Ok( analysis_result ) => analysis_result,
      Err( _panic_info ) => Err( Error::Execution( ErrorData::new(
        ErrorCode::InternalError,
        "Internal Error: An unexpected system error occurred during command analysis. This may indicate a bug in the framework.".to_string(),
      )))
    }
  }

  ///
  /// Internal analysis implementation that can panic.
  ///
  pub( in super ) fn analyze_internal( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    let mut verified_commands : Vec< VerifiedCommand > = Vec::new();

    for instruction in self.instructions
    {
      // Fix(issue-003): Reject empty command path with attached named/positional arguments instead of silently falling back to the help listing
      // Root cause: analyze_internal returned the help listing unconditionally whenever command_path_slices was empty, without checking for attached arguments
      // Pitfall: Easy to reintroduce by "simplifying" this branch back to an unconditional help listing — the bare "." → help case looks like the whole story
      // Handle special case: single dot "." (no path, no arguments) should show help.
      // An empty path with attached named/positional arguments is NOT a help request —
      // it means the parser excluded a `name::value` token (or bare token) from the path
      // because it's actually an argument, and that argument is unknown (no command was
      // resolved to validate it against). This must be rejected per FR-ARG-8, not silently
      // routed to the help listing.
      if instruction.command_path_slices.is_empty()
      {
        if instruction.named_arguments.is_empty() && instruction.positional_arguments.is_empty()
        {
          return self.generate_help_listing();
        }

        return Err( Error::Execution( Self::unknown_parameter_error_for_empty_path( instruction ) ) );
      }

      let command_path_refs : Vec< &str > = instruction.command_path_slices.iter().map( std::string::String::as_str ).collect();
      let command_name = crate::interner::intern_command_name( &command_path_refs );

      let command_def = self.registry.command( command_name ).ok_or_else( || ErrorData::new(
        ErrorCode::CommandNotFound,
        format!( "Command Error: The command '{command_name}' was not found. Use '.' to see all available commands or check for typos." ),
      ))?;

      // Check for double question mark parameter (alternative help access)
      let has_double_question_mark = instruction.positional_arguments.iter()
        .any( | arg | arg.value == "??" ) ||
        instruction.named_arguments.values()
        .flatten()
        .any( | arg | arg.value == "??" );

      // Check if help was requested for this command (via ? operator or ?? parameter)
      if instruction.help_requested || has_double_question_mark
      {
        // Generate help for this specific command (respects UNILANG_HELP_VERBOSITY env var)
        let help_generator = crate::help::HelpGenerator::from_env( self.registry );
        let help_content = help_generator.command( command_name )
          .unwrap_or( format!( "No help available for command '{command_name}'" ) );

        return Err( Error::Execution( ErrorData::new(
          ErrorCode::HelpRequested,
          help_content,
        )));
      }

      let arguments = Self::bind_arguments( instruction, &command_def )?;
      verified_commands.push( VerifiedCommand
      {
        definition : command_def,
        arguments,
      });
    }
    Ok( verified_commands )
  }

  ///
  /// Builds an `UnknownParameter` error for an instruction whose command path is empty
  /// but which carries at least one named or positional argument.
  ///
  /// No command was resolved in this case (the path is empty), so there is no
  /// `CommandDefinition` to validate against — the argument(s) are unconditionally
  /// unknown. This mirrors the error format used by `check_unknown_named_arguments()`
  /// in `validation.rs`, but is self-contained since it has no valid-parameter set to
  /// compare against.
  ///
  fn unknown_parameter_error_for_empty_path( instruction : &GenericInstruction ) -> ErrorData
  {
    let mut unknown_names : Vec< &str > = instruction.named_arguments.keys().map( std::string::String::as_str ).collect();

    let message = if unknown_names.is_empty()
    {
      // Only positional arguments are present; no command path means they cannot be bound.
      let values : Vec< &str > = instruction.positional_arguments.iter().map( | arg | arg.value.as_str() ).collect();
      format!(
        "Argument Error: Unknown parameter(s) provided without a command path: {}. Use '.' to see all available commands.",
        values.iter().map( | v | format!( "'{v}'" ) ).collect::< Vec< _ > >().join( ", " )
      )
    }
    else if unknown_names.len() == 1
    {
      format!(
        "Argument Error: Unknown parameter '{}'. No command was specified to validate it against. Use '.' to see all available commands.",
        unknown_names.remove( 0 )
      )
    }
    else
    {
      let params_list = unknown_names.iter().map( | p | format!( "'{p}'" ) ).collect::< Vec< _ > >().join( ", " );
      format!(
        "Argument Error: Unknown parameters: {params_list}. No command was specified to validate them against. Use '.' to see all available commands."
      )
    };

    ErrorData::new( ErrorCode::UnknownParameter, message )
  }

  ///
  /// Generates a help listing showing all available commands with descriptions.
  /// This is called when a user enters just "." as a command.
  ///
  pub( in super ) fn generate_help_listing( &self ) -> Result< Vec< VerifiedCommand >, Error >
  {
    // Use the new HelpGenerator for categorized, filtered output
    let help_gen = crate::help::HelpGenerator::new( self.registry );
    let help_content = help_gen.list_commands_filtered( None );

    // Return a special error that can be handled by the CLI to display help
    Err( Error::Execution( ErrorData::new(
      ErrorCode::HelpRequested,
      help_content,
    )))
  }
}
