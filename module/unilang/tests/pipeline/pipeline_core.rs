//! Core pipeline processing tests.
//!
//! Tests for `Pipeline::process_command`, `process_batch`, `validate_command`,
//! and the `process_single_command` / `validate_single_command` convenience functions.
//!
//! ## Spec Coverage
//!
//! | Spec File | Cases | Description |
//! |-----------|-------|-------------|
//! | `feature/03_pipeline` | FT-1..5 | Pipeline orchestration: single, batch, sequence, argv, error |
//! | `feature/05_repl_interactive` | FT-1, FT-5 | Stateless REPL and empty input handling |

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::types::Value;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::pipeline::{ Pipeline, process_single_command, validate_single_command };

fn create_test_registry() -> CommandRegistry
{
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

  registry.register_with_routine( &test_command, test_routine ).unwrap();
  registry
}

/// FT-1: Pipeline processes a valid command and returns expected output.
// test_kind: ft_spec(FT-1)  [feature/03_pipeline]
#[ test ]
fn test_ft1_pipeline_processes_valid_command()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  let result = pipeline.process_command( ".test world", context );

  assert!( result.success, ".test world must succeed" );
  assert!( result.error.is_none(), "no error expected for successful command" );
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

  assert!( !result.success, "invalid syntax must not succeed" );
  assert!( result.error.is_some(), "invalid syntax must produce an error" );
  assert!( result.error.as_ref().unwrap().contains( "Parse error" ), "error must mention Parse error" );
}

#[ test ]
fn test_pipeline_process_command_semantic_error()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  let result = pipeline.process_command( "nonexistent_command", context );

  assert!( !result.success, "unknown command must not succeed" );
  assert!( result.error.is_some(), "unknown command must produce an error" );
  assert!( result.error.as_ref().unwrap().contains( "Semantic analysis error" ), "error must mention Semantic analysis error" );
}

/// FT-2: Batch mode processes all commands regardless of individual failures.
// test_kind: ft_spec(FT-2)  [feature/03_pipeline]
#[ test ]
fn test_ft2_batch_mode_processes_all_commands()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );
  let context = ExecutionContext::default();

  let commands = vec![ ".test hello", ".test world", "nonexistent" ];
  let batch_result = pipeline.process_batch( &commands, context );

  assert_eq!( batch_result.total_commands, 3 );
  assert_eq!( batch_result.successful_commands, 2 );
  assert_eq!( batch_result.failed_commands, 1 );
  assert!( !batch_result.all_succeeded(), "batch with one failure must not report all_succeeded" );
  assert!( batch_result.any_failed(), "batch with one failure must report any_failed" );
  assert!( ( batch_result.success_rate() - 66.666_666 ).abs() < 0.001, "success rate must be ~66.7% for 2/3 commands" );
}

#[ test ]
fn test_pipeline_validate_command()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );

  assert!( pipeline.validate_command( ".test hello" ).is_ok(), "valid command must pass validation" );
  assert!( pipeline.validate_command( "nonexistent_command" ).is_err(), "unknown command must fail validation" );
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
// test_kind: ft_spec(FT-3)  [feature/03_pipeline]
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
  assert!( result.success, "process_single_command must succeed for valid command" );
  assert_eq!( result.outputs[ 0 ].content, "hello" );

  assert!( validate_single_command( ".test hello", &registry ).is_ok(), "valid command must pass validate_single_command" );
  assert!( validate_single_command( "nonexistent", &registry ).is_err(), "unknown command must fail validate_single_command" );
}

/// FT-2: Interactive arg absent — pipeline returns `requires_interactive_input()` signal.
///
/// A command whose argument carries `interactive: true` and `optional: false` signals the REPL
/// to prompt the user when that argument is omitted. The pipeline must not panic or return a
/// generic error; it must return a result where `requires_interactive_input() == true` and
/// `interactive_argument()` names the missing argument.
///
/// Spec: feature/005_repl_interactive.md § FT-2
// test_kind: ft_spec(FT-2)  [feature/05_repl_interactive]
#[ test ]
fn test_ft2_interactive_arg_absent_returns_interactive_required()
{
  use unilang::types::Value;

  let mut registry = CommandRegistry::new();

  let greet_cmd = CommandDefinition::former()
  .name( ".greet" )
  .namespace( String::new() )
  .description( "Greet by name".to_string() )
  .hint( "Greet" )
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
    .name( "name" )
    .description( "Name to greet".to_string() )
    .kind( Kind::String )
    .hint( "Name" )
    .attributes
    (
      ArgumentAttributes
      {
        optional: false,    // required — must be prompted when absent
        multiple: false,
        default: None,
        sensitive: false,
        interactive: true,  // triggers interactive-required signal
      }
    )
    .validation_rules( vec![] )
    .aliases( vec![] )
    .tags( vec![] )
    .end()
  ])
  .end();

  let routine = Box::new( | cmd : VerifiedCommand, _ctx |
  {
    let name = cmd.arguments.get( "name" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
    .unwrap_or_else( || "world".to_string() );
    Ok( OutputData { content : format!( "Hello, {}!", name ), format : "text".to_string(), execution_time_ms : None } )
  });

  registry.register_with_routine( &greet_cmd, routine ).unwrap();

  let pipeline = Pipeline::new( registry );

  // No argument provided — must signal interactive required
  let result = pipeline.process_command( ".greet", ExecutionContext::default() );

  assert!(
    !result.success,
    "FT-2: must not succeed when required interactive arg is absent"
  );
  assert!(
    result.requires_interactive_input(),
    "FT-2: requires_interactive_input must be true; error was: {:?}",
    result.error
  );
  assert_eq!(
    result.interactive_argument().as_deref(),
    Some( "name" ),
    "FT-2: interactive_argument must name the missing arg"
  );
}

/// FT-3: Interactive arg provided — pipeline executes normally, no prompt signal emitted.
///
/// When a command has an `interactive: true` required argument but the caller already
/// supplies the value, the pipeline must execute successfully with no interactive signal.
///
/// Spec: feature/005_repl_interactive.md § FT-3
// test_kind: ft_spec(FT-3)  [feature/05_repl_interactive]
#[ test ]
fn test_ft3_interactive_arg_provided_executes_without_prompt()
{
  use unilang::types::Value;

  let mut registry = CommandRegistry::new();

  let greet_cmd = CommandDefinition::former()
  .name( ".greet" )
  .namespace( String::new() )
  .description( "Greet by name".to_string() )
  .hint( "Greet" )
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
    .name( "name" )
    .description( "Name to greet".to_string() )
    .kind( Kind::String )
    .hint( "Name" )
    .attributes
    (
      ArgumentAttributes
      {
        optional: false,
        multiple: false,
        default: None,
        sensitive: false,
        interactive: true,
      }
    )
    .validation_rules( vec![] )
    .aliases( vec![] )
    .tags( vec![] )
    .end()
  ])
  .end();

  let routine = Box::new( | cmd : VerifiedCommand, _ctx |
  {
    let name = cmd.arguments.get( "name" )
    .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
    .unwrap_or_else( || "world".to_string() );
    Ok( OutputData { content : format!( "Hello, {}!", name ), format : "text".to_string(), execution_time_ms : None } )
  });

  registry.register_with_routine( &greet_cmd, routine ).unwrap();

  let pipeline = Pipeline::new( registry );

  // Argument provided — must execute without prompting
  let result = pipeline.process_command( ".greet name::alice", ExecutionContext::default() );

  assert!(
    result.success,
    "FT-3: must succeed when interactive arg is provided; error was: {:?}",
    result.error
  );
  assert!(
    !result.requires_interactive_input(),
    "FT-3: must not signal interactive required when arg was provided"
  );
  assert_eq!(
    result.outputs[ 0 ].content,
    "Hello, alice!",
    "FT-3: output must reflect the provided name"
  );
}

/// FT-4: Argv-based execution joins elements into a single command string.
///
/// `process_command_from_argv` accepts a `&[String]` slice where the first element
/// is the command name and subsequent elements are arguments. The pipeline must join
/// these into a single command and execute it correctly, preserving argument values
/// that contain spaces (the OS provides them as a single argv element).
// test_kind: ft_spec(FT-4)  [feature/03_pipeline]
#[ test ]
fn test_ft4_argv_execution_joins_elements()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );

  let argv : Vec< String > = vec![
    ".test".to_string(),
    "message::world".to_string(),
  ];
  let result = pipeline.process_command_from_argv( &argv, ExecutionContext::default() );

  assert!( result.success, "FT-4: argv-based execution must succeed; error: {:?}", result.error );
  assert_eq!( result.outputs.len(), 1 );
  assert_eq!( result.outputs[ 0 ].content, "world", "FT-4: argv message must reach the routine" );
}

/// FT-5: Pipeline returns CommandNotFound for an unregistered command.
///
/// Invoking a command not in the registry must produce a failed result with
/// an error message referencing the unknown command name. The pipeline must
/// not panic.
// test_kind: ft_spec(FT-5)  [feature/03_pipeline]
#[ test ]
fn test_ft5_pipeline_returns_command_not_found()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command( ".nonexistent", ExecutionContext::default() );

  assert!( !result.success, "FT-5: unregistered command must not succeed" );
  assert!( result.error.is_some(), "FT-5: must carry an error message" );
  let err_msg = result.error.as_ref().unwrap();
  assert!(
    err_msg.contains( "nonexistent" ) || err_msg.contains( "not found" ) || err_msg.contains( "CommandNotFound" ),
    "FT-5: error must reference the missing command; got: {err_msg}"
  );
}

/// FT-1: Stateless REPL — repeated calls produce no state leakage between invocations.
///
/// The Pipeline holds no mutable per-call state. Two consecutive calls to
/// `process_command` with different arguments must produce independent results.
///
/// Spec: feature/005_repl_interactive.md § FT-1
// test_kind: ft_spec(FT-1)  [feature/05_repl_interactive]
#[ test ]
fn test_ft1_stateless_repl_no_state_leakage()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );

  // First call
  let result1 = pipeline.process_command( ".test first", ExecutionContext::default() );
  assert!( result1.success, "FT-1: first call must succeed" );
  assert_eq!( result1.outputs[ 0 ].content, "first" );

  // Second call with different argument — must be independent
  let result2 = pipeline.process_command( ".test second", ExecutionContext::default() );
  assert!( result2.success, "FT-1: second call must succeed" );
  assert_eq!( result2.outputs[ 0 ].content, "second" );

  // Third call uses default — must not retain "second"
  let result3 = pipeline.process_command( ".test", ExecutionContext::default() );
  assert!( result3.success, "FT-1: third call must succeed with default arg" );
  assert_eq!( result3.outputs[ 0 ].content, "hello", "Default value must be used, not leaked state" );
}

/// FT-5: Empty REPL input handled without panic.
///
/// An empty string `""` passed to `process_command` must not cause a panic. The pipeline
/// may return an error (e.g., parse error for empty input) or succeed with no-op output,
/// but it must never unwind.
///
/// Spec: feature/005_repl_interactive.md § FT-5
// test_kind: ft_spec(FT-5)  [feature/05_repl_interactive]
#[ test ]
fn test_ft5_empty_repl_input_no_panic()
{
  let registry = create_test_registry();
  let pipeline = Pipeline::new( registry );

  // Empty string — must not panic; error or no-op is acceptable
  let result = pipeline.process_command( "", ExecutionContext::default() );

  // The pipeline did not panic (reaching this line is the primary assertion).
  // Actual behavior: empty input triggers the help system, returning a command listing
  // as successful output. This is a valid non-panic response per the spec ("Ok with
  // no-op behavior" — the help listing is the empty-input fallback, not a side effect).
  if !result.success
  {
    // Error path — also acceptable per spec
    assert!( result.error.is_some(), "FT-5: failed result must carry an error message" );
  }
}
