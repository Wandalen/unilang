use crate::data::OutputData;
use crate::error::Error;
use crate::interpreter::{ ExecutionContext, Interpreter };
use crate::registry::CommandRegistry;
use crate::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };
use super::result::CommandResult;


///
/// A high-level pipeline processor that combines parsing, semantic analysis, and execution.
///
/// This struct provides convenient methods for processing commands through the
/// complete Unilang pipeline, handling common patterns and error scenarios.
#[ allow( missing_debug_implementations ) ]
pub struct Pipeline
{
  pub( in super ) parser : Parser,
  pub( in super ) registry : CommandRegistry,
  pub( in super ) help_detection : bool,
}

impl Pipeline
{
  ///
  /// Creates a new pipeline with the given command registry and help detection enabled.
  ///
  #[ must_use ]
  pub fn new( registry : CommandRegistry ) -> Self
  {
    Self
    {
      parser : Parser::new( UnilangParserOptions::default() ),
      registry,
      help_detection : true,
    }
  }

  ///
  /// Sets whether unquoted `??` tokens are intercepted as help requests
  /// (default: `true`).
  ///
  /// With detection disabled, `??` reaches argument binding as an ordinary
  /// literal value. Quoting a single value (`param::"??"`) is the per-value
  /// opt-out that works without disabling detection globally.
  #[ must_use ]
  pub fn with_help_detection( mut self, help_detection : bool ) -> Self
  {
    self.help_detection = help_detection;
    self
  }

  ///
  /// Creates a new pipeline from a static command registry.
  ///
  /// This constructor enables using compile-time optimized static registries
  /// with the Pipeline API. The static registry is converted to a CommandRegistry
  /// internally using the existing `From<StaticCommandRegistry>` implementation.
  ///
  /// # Performance
  ///
  /// While the static registry provides 10-50x faster command lookups during
  /// registration, the conversion to CommandRegistry happens once at pipeline
  /// creation time. Subsequent command processing maintains the performance
  /// benefits of pre-registered commands.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # #[cfg(feature = "static_registry")]
  /// # {
  /// use unilang::pipeline::Pipeline;
  /// use unilang::registry::StaticCommandRegistry;
  ///
  /// let static_registry = StaticCommandRegistry::new();
  /// let pipeline = Pipeline::from_static(static_registry);
  /// # }
  /// ```
  ///
  /// # Feature Gate
  ///
  /// Requires the `static_registry` feature to be enabled.
  #[ must_use ]
  #[ cfg( feature = "static_registry" ) ]
  pub fn from_static( static_registry : crate::registry::StaticCommandRegistry ) -> Self
  {
    let registry = CommandRegistry::from( static_registry );
    Self::new( registry )
  }

  ///
  /// Creates a new pipeline with custom parser options.
  ///
  #[ must_use ]
  pub fn with_parser_options( registry : CommandRegistry, parser_options : UnilangParserOptions ) -> Self
  {
    Self
    {
      parser : Parser::new( parser_options ),
      registry,
      help_detection : true,
    }
  }

  ///
  /// Gets a reference to the command registry.
  ///
  #[ must_use ]
  pub fn registry( &self ) -> &CommandRegistry
  {
    &self.registry
  }

  ///
  /// Gets a mutable reference to the command registry.
  ///
  pub fn registry_mut( &mut self ) -> &mut CommandRegistry
  {
    &mut self.registry
  }

  ///
  /// Processes a single command string through the complete pipeline.
  ///
  /// This method handles parsing, semantic analysis, and execution in one call,
  /// returning a structured result with outputs or error information.
  ///
  /// # Arguments
  /// * `command_str` - The command string to process
  /// * `context` - The execution context (will be moved and consumed)
  ///
  /// # Examples
  /// ```rust
  /// use unilang::pipeline::Pipeline;
  /// use unilang::registry::CommandRegistry;
  /// use unilang::interpreter::ExecutionContext;
  ///
  /// let registry = CommandRegistry::new();
  /// let pipeline = Pipeline::new(registry);
  /// let context = ExecutionContext::default();
  ///
  /// let result = pipeline.process_command(".help", context);
  /// # drop(result); // Suppress unused variable warning
  /// ```
  #[ allow( clippy::needless_pass_by_value ) ]
  #[ must_use ]
  pub fn process_command( &self, command_str : &str, mut context : ExecutionContext ) -> CommandResult
  {
    let command = command_str.to_string();

    // Step 1: Parsing
    let instruction = match self.parser.parse_repl_input( command_str )
    {
      Ok( instruction ) => instruction,
      Err( error ) =>
      {
        return CommandResult
        {
          command,
          outputs : vec![],
          success : false,
          error : Some( format!( "Parse error: {error}" ) ),
        };
      }
    };

    // Step 2: Semantic Analysis
    let instructions = [ instruction ];
    let analyzer = SemanticAnalyzer::new( &instructions, &self.registry )
      .with_help_detection( self.help_detection );
    let verified_commands = match analyzer.analyze()
    {
      Ok( commands ) => commands,
      Err( error ) =>
      {
        // Check if this is a help request - if so, treat it as successful output
        if let crate::error::Error::Execution( error_data ) = &error
        {
          if error_data.code == crate::data::ErrorCode::HelpRequested
          {
            return CommandResult
            {
              command,
              outputs : vec![ crate::data::OutputData
              {
                content : error_data.message.clone(),
                format : "text".to_string(),
                execution_time_ms : None,
              }],
              success : true,
              error : None,
            };
          }
        }

        return CommandResult
        {
          command,
          outputs : vec![],
          success : false,
          error : Some( format!( "Semantic analysis error: {error}" ) ),
        };
      }
    };

    // Step 3: Execution
    let interpreter = Interpreter::new( &verified_commands, &self.registry );
    match interpreter.run( &mut context )
    {
      Ok( outputs ) => CommandResult
      {
        command,
        outputs,
        success : true,
        error : None,
      },
      Err( error ) => CommandResult
      {
        command,
        outputs : vec![],
        success : false,
        error : Some( format!( "Execution error: {error}" ) ),
      },
    }
  }

  ///
  /// Processes a single command string with a default execution context.
  ///
  /// This is a convenience method that creates a default execution context
  /// for simple use cases.
  #[ must_use ]
  pub fn process_command_simple( &self, command_str : &str ) -> CommandResult
  {
    self.process_command( command_str, ExecutionContext::default() )
  }

  ///
  /// Validates a command string without executing it.
  ///
  /// This method runs the command through parsing and semantic analysis
  /// but does not execute it, useful for validation scenarios.
  ///
  /// # Returns
  /// - `Ok(())` if the command is valid and would be executable
  /// - `Err(Error)` if the command has syntax or semantic errors
  #[ allow( clippy::missing_errors_doc ) ]
  pub fn validate_command( &self, command_str : &str ) -> Result< (), Error >
  {
    // Step 1: Parsing
    let instruction = self.parser.parse_repl_input( command_str )?;

    // Step 2: Semantic Analysis
    let instructions = [ instruction ];
    let analyzer = SemanticAnalyzer::new( &instructions, &self.registry )
      .with_help_detection( self.help_detection );
    analyzer.analyze()?;

    Ok(())
  }

  ///
  /// Validates multiple command strings without executing them.
  ///
  /// Returns a vector of validation results, one for each command.
  /// This is useful for batch validation scenarios.
  #[ must_use ]
  pub fn validate_batch( &self, commands : &[ &str ] ) -> Vec< Result< (), Error > >
  {
    commands.iter()
    .map( | &cmd_str | self.validate_command( cmd_str ) )
    .collect()
  }

  ///
  /// Processes help requests uniformly across the framework.
  ///
  /// This method provides a standardized way to handle help requests for any registered command.
  /// It generates comprehensive help information including command description, arguments,
  /// usage examples, and metadata.
  ///
  /// # Arguments
  /// * `command_name` - The full name of the command to get help for (e.g., ".example" or ".cmd2.list")
  /// * `context` - The execution context for the help request
  ///
  /// # Returns
  /// * `Result<OutputData, Error>` - Formatted help output or error if command not found
  ///
  /// # Examples
  /// ```rust
  /// use unilang::{pipeline::Pipeline, registry::CommandRegistry, interpreter::ExecutionContext};
  ///
  /// let registry = CommandRegistry::new();
  /// let pipeline = Pipeline::new(registry);
  /// let context = ExecutionContext::default();
  ///
  /// match pipeline.process_help_request(".help", context) {
  ///     Ok(output) => println!("{}", output.content),
  ///     Err(e) => eprintln!("Help error: {}", e),
  /// }
  /// ```
  #[ allow( clippy::needless_pass_by_value ) ]
  pub fn process_help_request( &self, command_name : &str, _context : ExecutionContext ) -> Result< OutputData, Error >
  {
    match self.registry.help_for_command( command_name )
    {
      Some( help_text ) => Ok( OutputData
      {
        content : help_text,
        format : "text".to_string(),
        execution_time_ms : None,
      }),
      None => Err( Error::Registration( format!(
        "Help Error: Command '{}' not found. Use '.' to see all available commands.",
        command_name
      ))),
    }
  }
}

///
/// Convenience function to process a single command with a registry.
///
/// This is a shorthand for creating a pipeline and processing one command.
/// Useful for simple scenarios where you don't need to reuse the pipeline.
/// Note: This creates a new parser each time, so it's less efficient than reusing a Pipeline.
///
/// # Examples
/// ```rust
/// use unilang::pipeline::process_single_command;
/// use unilang::registry::CommandRegistry;
/// use unilang::interpreter::ExecutionContext;
///
/// #[allow(deprecated)]
/// let registry = CommandRegistry::new();
/// let context = ExecutionContext::default();
/// let result = process_single_command(".help", &registry, context);
/// # drop(result); // Suppress unused variable warning
/// ```
#[ must_use ]
pub fn process_single_command
(
  command_str : &str,
  registry : &CommandRegistry,
  context : ExecutionContext,
)
->
CommandResult
{
  // Create parser and process command directly without Pipeline
  let parser = Parser::new( UnilangParserOptions::default() );
  let command = command_str.to_string();

  // Step 1: Parsing
  let instruction = match parser.parse_repl_input( command_str )
  {
    Ok( instruction ) => instruction,
    Err( error ) =>
    {
      return CommandResult
      {
        command,
        outputs : vec![],
        success : false,
        error : Some( format!( "Parse error: {error}" ) ),
      };
    }
  };

  // Step 2: Semantic Analysis
  let instructions = [ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, registry );
  let verified_commands = match analyzer.analyze()
  {
    Ok( commands ) => commands,
    Err( error ) =>
    {
      // Help requests are detected by ErrorCode, not message text, and become
      // successful output — same contract as `Pipeline::process_command`.
      if let crate::error::Error::Execution( error_data ) = &error
      {
        if error_data.code == crate::data::ErrorCode::HelpRequested
        {
          return CommandResult
          {
            command,
            outputs : vec![ crate::data::OutputData
            {
              content : error_data.message.clone(),
              format : "text".to_string(),
              execution_time_ms : None,
            }],
            success : true,
            error : None,
          };
        }
      }

      return CommandResult
      {
        command,
        outputs : vec![],
        success : false,
        error : Some( format!( "Semantic analysis error: {error}" ) ),
      };
    }
  };

  // Step 3: Execution
  let interpreter = Interpreter::new( &verified_commands, registry );
  let mut exec_context = context;
  match interpreter.run( &mut exec_context )
  {
    Ok( outputs ) => CommandResult
    {
      command,
      outputs,
      success : true,
      error : None,
    },
    Err( error ) => CommandResult
    {
      command,
      outputs : vec![],
      success : false,
      error : Some( format!( "Execution error: {error}" ) ),
    },
  }
}

///
/// Convenience function to validate a single command with a registry.
///
/// This is a shorthand for creating a pipeline and validating one command.
/// Note: This creates a new parser each time, so it's less efficient than reusing a Pipeline.
#[ allow( clippy::missing_errors_doc ) ]
pub fn validate_single_command
(
  command_str : &str,
  registry : &CommandRegistry,
)
->
Result< (), Error >
{
  // Create parser and validate command directly without Pipeline
  let parser = Parser::new( UnilangParserOptions::default() );

  // Step 1: Parsing
  let instruction = parser.parse_repl_input( command_str )?;

  // Step 2: Semantic Analysis
  let instructions = [ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, registry );
  analyzer.analyze()?;

  Ok(())
}
