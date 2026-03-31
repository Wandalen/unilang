//!
//! Pipeline utilities for common Unilang workflows.
//!
//! This module provides convenient helper functions that combine multiple
//! Unilang components to handle common use cases, making it easier to
//! integrate Unilang into applications.
//!
//! # REPL Implementation Insights
//!
//! The Pipeline is specifically designed for REPL (Read-Eval-Print Loop) applications:
//!
//! ## Stateless Operation
//! - **Critical**: All components (Parser, `SemanticAnalyzer`, Interpreter) are completely stateless
//! - Each `process_command` call is independent - no state accumulation between calls
//! - Memory usage remains constant regardless of session length
//! - Safe for long-running REPL sessions without memory leaks
//!
//! ## Command Pipeline Performance Analysis
//! - Component reuse provides 20-50% performance improvement over creating new instances
//! - Static command registry lookups are zero-cost even with millions of commands
//! - Parsing overhead is minimal and constant-time for typical command lengths
//!
//! ## Error Isolation
//! - Command failures are isolated - one failed command doesn't affect subsequent commands
//! - Parse errors, semantic errors, and execution errors are all safely contained
//! - REPL sessions can continue indefinitely even with frequent command failures
//!
//! ## Interactive Argument Handling
//! - The `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error is designed to be caught by REPL loops
//! - Interactive prompts should be handled at the REPL level, not within the pipeline
//! - Secure input (passwords, API keys) should never be logged or stored in pipeline state

mod core;
mod batch;
mod argv;

/// Internal namespace (placeholder for mod_interface compatibility).
mod private {}

mod_interface::mod_interface!
{
  exposed use core::UnilangError;
  exposed use core::CommandResult;
  exposed use batch::BatchResult;
  exposed use core::Pipeline;
  exposed use core::process_single_command;
  exposed use core::validate_single_command;

  prelude use core::UnilangError;
  prelude use core::CommandResult;
  prelude use batch::BatchResult;
  prelude use core::Pipeline;
  prelude use core::process_single_command;
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
  use crate::types::Value;
  use crate::registry::CommandRegistry;
  use crate::interpreter::ExecutionContext;
  use crate::data::OutputData;

  fn create_test_registry() -> CommandRegistry
  {
    #[ allow( deprecated ) ]
    let mut registry = CommandRegistry::new();

    // Add a simple test command
    let test_command = CommandDefinition::former()
    .name( ".test" )
    .namespace( String::new() )
    .description( "Test command".to_string() )
    .hint( "Test command" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( true )
    .deprecation_message( String::new() )
    .http_method_hint( "GET".to_string() )
    .examples( vec![] )
    .arguments( vec!
    [
      ArgumentDefinition::former()
      .name( "message" )
      .description( "Test message".to_string() )
      .kind( Kind::String )
      .hint( "Message to echo" )
      .attributes
      (
        ArgumentAttributes
        {
          optional: true,
          multiple: false,
          default: Some( "hello".to_string() ),
          sensitive: false,
          interactive: false,
        }
      )
      .validation_rules( vec![] )
      .aliases( vec![] )
      .tags( vec![] )
      .end()
    ])
    .end();

    let test_routine = Box::new( | cmd : crate::semantic::VerifiedCommand, _ctx |
    {
      let default_message = "hello".to_string();
      let message = cmd.arguments.get( "message" )
      .and_then( | v | if let Value::String( s ) = v { Some( s ) } else { None } )
      .unwrap_or( &default_message );

      Ok( OutputData
      {
        content : message.clone(),
        format : "text".to_string(),
        execution_time_ms : None,
      })
    });

    #[ allow( deprecated ) ]
    registry.command_add_runtime( &test_command, test_routine ).unwrap();
    registry
  }

  #[ test ]
  fn test_pipeline_process_command_success()
  {
    let registry = create_test_registry();
    let pipeline = Pipeline::new( registry );
    let context = ExecutionContext::default();

    let result = pipeline.process_command( ".test world", context );

    assert!( result.success );
    assert!( result.error.is_none() );
    assert_eq!( result.outputs.len(), 1 );
    assert_eq!( result.outputs[ 0 ].content, "world" );
  }

  #[ test ]
  fn test_pipeline_process_command_parse_error()
  {
    let registry = create_test_registry();
    let pipeline = Pipeline::new( registry );
    let context = ExecutionContext::default();

    // This should cause a parse error (invalid syntax)
    let result = pipeline.process_command( "invalid..syntax", context );

    assert!( !result.success );
    assert!( result.error.is_some() );
    assert!( result.error.as_ref().unwrap().contains( "Parse error" ) );
  }

  #[ test ]
  fn test_pipeline_process_command_semantic_error()
  {
    let registry = create_test_registry();
    let pipeline = Pipeline::new( registry );
    let context = ExecutionContext::default();

    // This should cause a semantic error (command not found)
    let result = pipeline.process_command( "nonexistent_command", context );

    assert!( !result.success );
    assert!( result.error.is_some() );
    assert!( result.error.as_ref().unwrap().contains( "Semantic analysis error" ) );
  }

  #[ test ]
  fn test_pipeline_process_batch()
  {
    let registry = create_test_registry();
    let pipeline = Pipeline::new( registry );
    let context = ExecutionContext::default();

    let commands = vec![ ".test hello", ".test world", "nonexistent" ];
    let batch_result = pipeline.process_batch( &commands, context );

    assert_eq!( batch_result.total_commands, 3 );
    assert_eq!( batch_result.successful_commands, 2 );
    assert_eq!( batch_result.failed_commands, 1 );
    assert!( !batch_result.all_succeeded() );
    assert!( batch_result.any_failed() );
    assert!( ( batch_result.success_rate() - 66.666_666 ).abs() < 0.001 );
  }

  #[ test ]
  fn test_pipeline_validate_command()
  {
    let registry = create_test_registry();
    let pipeline = Pipeline::new( registry );

    // Valid command
    assert!( pipeline.validate_command( ".test hello" ).is_ok() );

    // Invalid command
    assert!( pipeline.validate_command( "nonexistent_command" ).is_err() );
  }

  #[ test ]
  fn test_convenience_functions()
  {
    let registry = create_test_registry();
    let context = ExecutionContext::default();

    // Test process_single_command
    let result = process_single_command( ".test hello", &registry, context );
    assert!( result.success );
    assert_eq!( result.outputs[ 0 ].content, "hello" );

    // Test validate_single_command
    assert!( validate_single_command( ".test hello", &registry ).is_ok() );
    assert!( validate_single_command( "nonexistent", &registry ).is_err() );
  }
}
