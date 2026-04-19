//! T17: Optional arg with wrong Value variant → None (not Err).
//!
//! ## Test Matrix
//!
//! | Case | Argument | Expected outcome |
//! |------|----------|-----------------|
//! | optional absent | no key | None, Ok |
//! | optional correct type | Value::String("x") | Some("x"), Ok |
//! | optional wrong type | Value::Integer(1) for String param | None, Ok (silent) |
//!
//! When an optional parameter's key IS present but carries an incompatible Value
//! variant, the `.and_then(|v| match)` returns None — the wrapper does NOT
//! return Err.  This is the documented behavioral contract: type-mismatch on
//! optional args is silently treated as absent.  Required args (T11) differ:
//! they return Err for both missing key and wrong type.

use std::collections::HashMap;
use unilang::data::{ CommandDefinition, CommandName };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::types::Value;

#[ unilang_meta::command( name = ".maybe" ) ]
fn maybe( label : Option< String >, flag : Option< bool > ) -> String
{
  format!( "{:?} {:?}", label, flag )
}

fn make_cmd( args : HashMap< String, Value > ) -> VerifiedCommand
{
  let cn = CommandName::new( ".maybe" ).unwrap();
  VerifiedCommand
  {
    definition : CommandDefinition::new( cn, ".maybe".to_string() ),
    arguments  : args,
  }
}

fn main()
{
  // Optional key absent → None, Ok
  let result = __unilang_wrapper_maybe( make_cmd( HashMap::new() ), ExecutionContext::default() );
  assert!( result.is_ok(), "all-absent optionals must return Ok" );
  assert_eq!( result.unwrap().content, "None None" );

  // Optional key present with correct type → Some, Ok
  let mut correct = HashMap::new();
  correct.insert( "label".to_string(), Value::String( "hello".to_string() ) );
  correct.insert( "flag".to_string(), Value::Boolean( true ) );
  let result2 = __unilang_wrapper_maybe( make_cmd( correct ), ExecutionContext::default() );
  assert!( result2.is_ok(), "correct types must return Ok" );

  // Optional key present with wrong type → None (silent), Ok
  let mut wrong = HashMap::new();
  wrong.insert( "label".to_string(), Value::Integer( 42 ) );   // String param, Integer value
  wrong.insert( "flag".to_string(), Value::String( "oops".to_string() ) ); // bool param, String value
  let result3 = __unilang_wrapper_maybe( make_cmd( wrong ), ExecutionContext::default() );
  assert!( result3.is_ok(), "wrong type on optional must return Ok (None), not Err" );
  assert_eq!( result3.unwrap().content, "None None", "wrong type on optional must yield None" );
}
