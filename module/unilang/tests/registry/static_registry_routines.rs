//! Tests for `StaticCommandRegistry` routine management:
//! `into_routines()` and `From<StaticCommandRegistry>` routine transfer.

#![ allow( deprecated ) ]

use unilang::data::{ CommandDefinition, CommandName, OutputData };
use unilang::pipeline::Pipeline;
use unilang::registry::{ StaticCommandRegistry, CommandRoutine, CommandRegistry };

/// T01: `into_routines()` on an empty registry returns an empty HashMap.
#[ test ]
fn test_into_routines_empty()
{
  let registry = StaticCommandRegistry::new();
  let routines = registry.into_routines();
  assert!( routines.is_empty() );
}

/// T02: `into_routines()` after `register_with_routine` for N commands returns N entries.
#[ test ]
fn test_into_routines_populated()
{
  let mut registry = StaticCommandRegistry::new();

  let cmd1 = CommandDefinition::new(
    CommandName::new( ".ping" ).unwrap(),
    "Ping command".to_string(),
  );
  let r1 : CommandRoutine = Box::new( |_, _| Ok( OutputData
  {
    content : "pong".to_string(),
    format : "text".to_string(),
    execution_time_ms : None,
  }));
  registry.register_with_routine( cmd1, r1 ).unwrap();

  let cmd2 = CommandDefinition::new(
    CommandName::new( ".echo" ).unwrap(),
    "Echo command".to_string(),
  );
  let r2 : CommandRoutine = Box::new( |_, _| Ok( OutputData
  {
    content : "echo".to_string(),
    format : "text".to_string(),
    execution_time_ms : None,
  }));
  registry.register_with_routine( cmd2, r2 ).unwrap();

  let routines = registry.into_routines();
  assert_eq!( routines.len(), 2 );
  assert!( routines.contains_key( ".ping" ) );
  assert!( routines.contains_key( ".echo" ) );
}

/// T03: `From<StaticCommandRegistry>` for `CommandRegistry` transfers all routines.
#[ test ]
fn test_from_static_preserves_routines()
{
  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::new(
    CommandName::new( ".greet" ).unwrap(),
    "Greet command".to_string(),
  );
  let routine : CommandRoutine = Box::new( |_, _| Ok( OutputData
  {
    content : "hello".to_string(),
    format : "text".to_string(),
    execution_time_ms : None,
  }));
  static_reg.register_with_routine( cmd, routine ).unwrap();

  let cmd_registry : CommandRegistry = static_reg.into();
  assert!(
    cmd_registry.get_routine( ".greet" ).is_some(),
    "Routine for .greet must survive StaticCommandRegistry → CommandRegistry conversion"
  );
}

/// T04: `Pipeline::from_static` with routines dispatches and executes them.
#[ test ]
fn test_pipeline_from_static_executes_routine()
{
  let mut static_reg = StaticCommandRegistry::new();

  let cmd = CommandDefinition::new(
    CommandName::new( ".ping" ).unwrap(),
    "Ping the service".to_string(),
  );
  let routine : CommandRoutine = Box::new( |_, _| Ok( OutputData
  {
    content : "pong".to_string(),
    format : "text".to_string(),
    execution_time_ms : None,
  }));
  static_reg.register_with_routine( cmd, routine ).unwrap();

  let pipeline = Pipeline::from_static( static_reg );
  let result = pipeline.process_command_simple( ".ping" );

  assert!(
    result.is_success(),
    "Expected success from Pipeline::from_static dispatch, got: {:?}", result.error
  );
  assert_eq!( result.outputs.len(), 1 );
  assert_eq!( result.outputs[ 0 ].content, "pong" );
}

/// T05: The existing `Pipeline::new()` path continues to work after this change.
#[ test ]
fn test_pipeline_new_unaffected()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::new(
    CommandName::new( ".status" ).unwrap(),
    "Status command".to_string(),
  );
  let routine : CommandRoutine = Box::new( |_, _| Ok( OutputData
  {
    content : "ok".to_string(),
    format : "text".to_string(),
    execution_time_ms : None,
  }));
  registry.command_add_runtime( &cmd, routine ).unwrap();

  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".status" );

  assert!(
    result.is_success(),
    "Expected success from Pipeline::new dispatch, got: {:?}", result.error
  );
  assert_eq!( result.outputs[ 0 ].content, "ok" );
}
