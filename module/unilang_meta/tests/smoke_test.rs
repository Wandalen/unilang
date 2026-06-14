//!
//! Pipeline integration tests for the `#[command]` macro.
//!
//! Verifies that commands defined with `#[unilang_meta::command]` can be
//! registered into a `CommandRegistry` and executed end-to-end through
//! the `Pipeline`.
//!

use unilang::pipeline::Pipeline;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;

// --- Test commands ---

#[ unilang_meta::command( name = ".greet", description = "Greet the user" ) ]
fn greet( name : String ) -> String
{
  format!( "Hello, {}!", name )
}

#[ unilang_meta::command( name = ".add", description = "Add two integers" ) ]
fn add( a : i64, b : i64 ) -> String
{
  format!( "{}", a + b )
}

// --- Helpers ---

fn build_pipeline() -> Pipeline
{
  let mut registry = CommandRegistry::new();
  registry.register_with_routine( __unilang_register_greet(), Box::new( __unilang_wrapper_greet ) ).unwrap();
  registry.register_with_routine( __unilang_register_add(), Box::new( __unilang_wrapper_add ) ).unwrap();
  Pipeline::new( registry )
}

// --- Tests ---

#[ test ]
fn macro_command_executes_through_pipeline()
{
  let pipeline = build_pipeline();
  let result = pipeline.process_command( ".greet name::World", ExecutionContext::default() );
  assert!( result.success, "pipeline should succeed; error: {:?}", result.error );
  assert_eq!( result.outputs[ 0 ].content, "Hello, World!" );
}

#[ test ]
fn macro_command_with_integer_params()
{
  let pipeline = build_pipeline();
  let result = pipeline.process_command( ".add a::3 b::7", ExecutionContext::default() );
  assert!( result.success, "pipeline should succeed; error: {:?}", result.error );
  assert_eq!( result.outputs[ 0 ].content, "10" );
}

#[ test ]
fn macro_command_missing_required_arg()
{
  let pipeline = build_pipeline();
  let result = pipeline.process_command( ".greet", ExecutionContext::default() );
  assert!( !result.success, "pipeline should fail when required arg is missing" );
}
