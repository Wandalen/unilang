use crate::interpreter::{ ExecutionContext, Interpreter };
use crate::semantic::SemanticAnalyzer;
use super::core::Pipeline;
use super::result::CommandResult;

impl Pipeline
{
  ///
  /// Processes a command from an argv array (OS command-line arguments).
  ///
  /// This method provides proper CLI integration by preserving the original argv structure
  /// from the operating system, avoiding information loss from string joining and re-tokenization.
  ///
  /// **Recommended for CLI applications** - This is the preferred method when building CLI
  /// applications that receive argv from `std::env::args()`. Use `process_command()` for
  /// REPL, interactive shells, or when parsing embedded DSL strings.
  ///
  /// # Algorithm
  ///
  /// The argv parser intelligently combines consecutive argv elements:
  /// 1. Elements containing `::` start named arguments (`key::value`)
  /// 2. Following elements without `::` or `.` are combined into the parameter value
  /// 3. Combining stops at the next `::` or dot-prefixed element
  ///
  /// # Arguments
  /// * `argv` - Command-line arguments from `std::env::args().skip(1).collect()`
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
  /// // Shell: ./app .command arg1 arg2
  /// // OS provides: [".command", "arg1", "arg2"]
  /// let argv: Vec<String> = vec![".help".to_string()];
  /// let result = pipeline.process_command_from_argv(&argv, context);
  /// # drop(result); // Suppress unused variable warning
  /// ```
  ///
  /// # Why Use This Method?
  ///
  /// **Problem with string-based API:**
  /// ```text
  /// // ❌ BAD: Information loss
  /// let argv = vec!["command::ls", "-la"];
  /// let command_str = argv.join(" ");  // "command::ls -la"
  /// pipeline.process_command(&command_str);  // Parse error!
  /// ```
  ///
  /// **Solution with argv-based API:**
  /// ```
  /// use unilang::pipeline::Pipeline;
  /// use unilang::registry::CommandRegistry;
  /// use unilang::interpreter::ExecutionContext;
  ///
  /// # let registry = CommandRegistry::new();
  /// # let pipeline = Pipeline::new(registry);
  /// # let context = ExecutionContext::default();
  /// // ✅ GOOD: Preserves structure
  /// let argv = vec![".help".to_string()];
  /// let result = pipeline.process_command_from_argv(&argv, context);
  /// # drop(result);
  /// ```
  ///
  /// # See Also
  ///
  /// - [`Self::process_command`] - For REPL/interactive shells
  /// - [`Self::process_command_from_argv_simple`] - Convenience method with default context
  /// - Task 080: Argv-Based API Request - Full specification
  #[ allow( clippy::needless_pass_by_value ) ]
  #[ must_use ]
  pub fn process_command_from_argv( &self, argv : &[String], mut context : ExecutionContext ) -> CommandResult
  {
    // Build a command string representation for error reporting
    let command = argv.join( " " );

    // Step 1: Parse argv using the argv-aware parser
    let instruction = match self.parser.parse_from_argv( argv )
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
    let analyzer = SemanticAnalyzer::new( &instructions, &self.registry );
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
  /// Processes a command from argv with a default execution context.
  ///
  /// This is a convenience method for CLI applications that creates a default
  /// execution context for simple use cases.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::pipeline::Pipeline;
  /// use unilang::registry::CommandRegistry;
  ///
  /// let registry = CommandRegistry::new();
  /// let pipeline = Pipeline::new(registry);
  ///
  /// // Typical CLI application pattern
  /// let argv: Vec<String> = vec![".help".to_string()];
  /// let result = pipeline.process_command_from_argv_simple(&argv);
  ///
  /// if result.success {
  ///     println!("Success");
  /// } else {
  ///     eprintln!("Error: {}", result.error.unwrap_or_default());
  /// }
  /// ```
  #[ must_use ]
  pub fn process_command_from_argv_simple( &self, argv : &[String] ) -> CommandResult
  {
    self.process_command_from_argv( argv, ExecutionContext::default() )
  }
}
