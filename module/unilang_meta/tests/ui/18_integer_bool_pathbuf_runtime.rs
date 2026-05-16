//! T18: Runtime extraction of i32, usize, bool, and PathBuf through the wrapper.
//!
//! ## Test Matrix
//!
//! | Param type | Value variant | Expected extraction |
//! |------------|--------------|---------------------|
//! | i32 | Value::Integer(7) | 7i32 |
//! | usize | Value::Integer(3) | 3usize |
//! | bool | Value::Boolean(true) | true |
//! | PathBuf | Value::Path("/tmp") | PathBuf::from("/tmp") |
//! | i32 (wrong type) | Value::Boolean(false) | Err |
//!
//! T07 tests compile-time kind inference; this test exercises the actual runtime
//! extraction path (the `gen_value_match` generated code) for each of the
//! non-String integer variants, bool, and PathBuf.

use std::collections::HashMap;
use std::path::PathBuf;
use unilang::data::{ CommandDefinition, CommandName };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::types::Value;

#[ unilang_meta::command( name = ".mixed" ) ]
fn mixed( count : i32, size : usize, active : bool, path : PathBuf ) -> String
{
  format!( "{} {} {} {:?}", count, size, active, path )
}

fn make_cmd( args : HashMap< String, Value > ) -> VerifiedCommand
{
  let cn = CommandName::new( ".mixed" ).unwrap();
  VerifiedCommand
  {
    definition : CommandDefinition::new( cn, ".mixed".to_string() ),
    arguments  : args,
  }
}

fn main()
{
  // Happy path: all correct Value variants
  let mut args = HashMap::new();
  args.insert( "count".to_string(),  Value::Integer( 7 ) );
  args.insert( "size".to_string(),   Value::Integer( 3 ) );
  args.insert( "active".to_string(), Value::Boolean( true ) );
  args.insert( "path".to_string(),   Value::Path( PathBuf::from( "/tmp" ) ) );

  let result = __unilang_wrapper_mixed( make_cmd( args ), ExecutionContext::default() );
  assert!( result.is_ok(), "all correct types must return Ok" );
  let content = result.unwrap().content;
  assert!( content.contains( "7" ),  "i32 extraction must produce 7" );
  assert!( content.contains( "3" ),  "usize extraction must produce 3" );
  assert!( content.contains( "true" ), "bool extraction must produce true" );
  assert!( content.contains( "/tmp" ), "PathBuf extraction must produce /tmp path" );

  // Wrong type for i32 → Err (required arg missing effective value)
  let mut wrong = HashMap::new();
  wrong.insert( "count".to_string(),  Value::Boolean( false ) ); // wrong variant for i32
  wrong.insert( "size".to_string(),   Value::Integer( 3 ) );
  wrong.insert( "active".to_string(), Value::Boolean( true ) );
  wrong.insert( "path".to_string(),   Value::Path( PathBuf::from( "/tmp" ) ) );

  let result2 = __unilang_wrapper_mixed( make_cmd( wrong ), ExecutionContext::default() );
  assert!( result2.is_err(), "wrong type for required i32 must return Err" );
}
