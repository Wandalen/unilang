//! Core pipeline processing tests.
//!
//! Tests for `Pipeline::process_command`, `process_batch`, `validate_command`,
//! and the `process_single_command` / `validate_single_command` convenience functions.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::types::Value;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::pipeline::{ Pipeline, process_single_command, validate_single_command };

fn create_test_registry() -> CommandRegistry
{
  #[ allow( deprecated ) ]
  let mut registry = CommandRegistry::new();

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

  let test_routine = Box::new( | cmd : VerifiedCommand, _ctx |
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

  assert!( pipeline.validate_command( ".test hello" ).is_ok() );
  assert!( pipeline.validate_command( "nonexistent_command" ).is_err() );
}

/// FT-3: Sequence mode stops at first failure and does not execute subsequent commands.
///
/// `process_sequence` is fail-fast: the sequence `[".nonexistent", ".test"]` runs the
/// first command (`.nonexistent`, which fails with CommandNotFound) and then stops. The
/// second command (`.test`, which would succeed) is never executed.
///
/// ## Observability (no mocks required)
///
/// `BatchResult.results.len()` equals the number of commands actually processed.
/// If the sequence stopped at the first failure, `results.len() == 1` and
/// `total_commands == 2`, proving the second command was skipped.
// test_kind: ft_spec(FT-3)
#[ test ]
fn test_ft3_sequence_stops_at_first_failure()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  // ".nonexistent" fails (CommandNotFound); ".test" would succeed if reached
  let commands = vec![ ".nonexistent", ".test" ];
  let batch_result = pipeline.process_sequence( &commands, context );

  assert_eq!( batch_result.total_commands, 2, "Sequence must record all input commands" );
  assert_eq!( batch_result.failed_commands, 1, "Exactly one command must have failed" );
  assert_eq!( batch_result.successful_commands, 0, "No commands must have succeeded" );
  assert_eq!(
    batch_result.results.len(),
    1,
    "Only one result must be present — the second command must not have been executed"
  );
  assert!( !batch_result.results[ 0 ].success, "The result present must be the failure" );
}

#[ test ]
fn test_convenience_functions()
{
  let registry = create_test_registry();
  let context = ExecutionContext::default();

  let result = process_single_command( ".test hello", &registry, context );
  assert!( result.success );
  assert_eq!( result.outputs[ 0 ].content, "hello" );

  assert!( validate_single_command( ".test hello", &registry ).is_ok() );
  assert!( validate_single_command( "nonexistent", &registry ).is_err() );
}
