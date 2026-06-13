use crate::interpreter::ExecutionContext;
use super::core::Pipeline;
use super::result::CommandResult;

///
/// Result of processing multiple commands through the pipeline.
///
#[ derive( Debug, Clone ) ]
pub struct BatchResult
{
  /// Results for each individual command.
  pub results : Vec< CommandResult >,
  /// Total number of commands processed.
  pub total_commands : usize,
  /// Number of commands that succeeded.
  pub successful_commands : usize,
  /// Number of commands that failed.
  pub failed_commands : usize,
}

impl BatchResult
{
  /// Returns true if all commands in the batch succeeded.
  #[ must_use ]
  pub fn all_succeeded( &self ) -> bool
  {
    self.failed_commands == 0
  }

  /// Returns true if any commands in the batch failed.
  #[ must_use ]
  pub fn any_failed( &self ) -> bool
  {
    self.failed_commands > 0
  }

  /// Returns the success rate as a percentage.
  #[ must_use ]
  pub fn success_rate( &self ) -> f64
  {
    if self.total_commands == 0
    {
      0.0
    }
    else
    {
      ( self.successful_commands as f64 / self.total_commands as f64 ) * 100.0
    }
  }
}

impl Pipeline
{
  ///
  /// Processes multiple command strings as a batch.
  ///
  /// This method processes each command independently and returns a summary
  /// of the batch execution results. Commands are executed in order, and
  /// failure of one command does not stop execution of subsequent commands.
  ///
  /// # Arguments
  /// * `commands` - Slice of command strings to process
  /// * `context` - The execution context (will be cloned for each command)
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
  /// let commands = vec![".help"];
  /// let batch_result = pipeline.process_batch(&commands, context);
  /// println!("Success rate: {:.1}%", batch_result.success_rate());
  /// ```
  #[ allow( clippy::needless_pass_by_value ) ]
  #[ must_use ]
  pub fn process_batch( &self, commands : &[ &str ], context : ExecutionContext ) -> BatchResult
  {
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for &cmd_str in commands
    {
      let result = self.process_command( cmd_str, context.clone() );

      if result.success
      {
        successful += 1;
      }
      else
      {
        failed += 1;
      }

      results.push( result );
    }

    BatchResult
    {
      results,
      total_commands : commands.len(),
      successful_commands : successful,
      failed_commands : failed,
    }
  }

  ///
  /// Processes multiple command strings with early termination on failure.
  ///
  /// Unlike `process_batch`, this method stops processing commands as soon
  /// as one command fails, returning the results of commands processed up
  /// to that point.
  ///
  /// # Arguments
  /// * `commands` - Slice of command strings to process
  /// * `context` - The execution context (will be moved and mutated)
  #[ allow( clippy::needless_pass_by_value ) ]
  #[ must_use ]
  pub fn process_sequence( &self, commands : &[ &str ], context : ExecutionContext ) -> BatchResult
  {
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for &cmd_str in commands
    {
      let result = self.process_command( cmd_str, context.clone() );

      if result.success
      {
        successful += 1;
      }
      else
      {
        failed += 1;
        results.push( result );
        break; // Stop on first failure
      }

      results.push( result );
    }

    BatchResult
    {
      results,
      total_commands : commands.len(),
      successful_commands : successful,
      failed_commands : failed,
    }
  }
}
