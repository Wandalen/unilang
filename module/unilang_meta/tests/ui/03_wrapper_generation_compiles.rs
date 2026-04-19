//! T03: Wrapper function generation — verify exact signature and argument extraction.
//!
//! Verifies that the generated wrapper function has the correct interpreter signature
//! and that a missing required argument produces Err(ErrorData) without panicking.

use unilang::semantic::VerifiedCommand;
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang::data::CommandName;
use unilang::data::CommandDefinition;
use std::collections::HashMap;

#[ unilang_meta::command( name = ".greet", description = "Greet the user" ) ]
fn greet( name : String ) -> String
{
  format!( "Hello, {}!", name )
}

fn make_command( args : HashMap< String, Value > ) -> VerifiedCommand
{
  let cn = CommandName::new( ".greet" ).unwrap();
  VerifiedCommand
  {
    definition : CommandDefinition::new( cn, "Greet the user".to_string() ),
    arguments  : args,
  }
}

fn main()
{
  // Happy path: correct argument provided
  let mut args = HashMap::new();
  args.insert( "name".to_string(), Value::String( "World".to_string() ) );
  let result = __unilang_wrapper_greet( make_command( args ), ExecutionContext::default() );
  assert!( result.is_ok() );
  assert_eq!( result.unwrap().content, "Hello, World!" );

  // Error path: missing required argument → Err, not panic
  let empty = HashMap::new();
  let err_result = __unilang_wrapper_greet( make_command( empty ), ExecutionContext::default() );
  assert!( err_result.is_err() );
}
