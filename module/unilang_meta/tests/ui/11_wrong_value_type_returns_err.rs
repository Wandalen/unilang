//! T11: Wrong Value variant for a required argument returns Err, not panic.
//!
//! When a required String argument key is present but carries an incompatible
//! Value variant (Value::Integer), the generated wrapper returns Err without
//! panicking — the same error path as a fully missing argument.

use std::collections::HashMap;
use unilang::data::{ CommandDefinition, CommandName };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::types::Value;

#[ unilang_meta::command( name = ".expect_string" ) ]
fn expect_string( label : String ) -> String
{
  format!( "got: {}", label )
}

fn make_cmd( args : HashMap< String, Value > ) -> VerifiedCommand
{
  let cn = CommandName::new( ".expect_string" ).unwrap();
  VerifiedCommand
  {
    definition : CommandDefinition::new( cn, ".expect_string".to_string() ),
    arguments  : args,
  }
}

fn main()
{
  // Argument key present but wrong Value variant (Integer instead of String)
  let mut wrong_type = HashMap::new();
  wrong_type.insert( "label".to_string(), Value::Integer( 42 ) );
  let result = __unilang_wrapper_expect_string(
    make_cmd( wrong_type ),
    ExecutionContext::default(),
  );
  assert!( result.is_err(), "wrong Value variant must return Err, not Ok" );

  // Argument key completely absent
  let result2 = __unilang_wrapper_expect_string(
    make_cmd( HashMap::new() ),
    ExecutionContext::default(),
  );
  assert!( result2.is_err(), "missing argument must return Err, not Ok" );

  // Correct Value variant → Ok
  let mut correct = HashMap::new();
  correct.insert( "label".to_string(), Value::String( "world".to_string() ) );
  let result3 = __unilang_wrapper_expect_string(
    make_cmd( correct ),
    ExecutionContext::default(),
  );
  assert!( result3.is_ok(), "correct Value variant must return Ok" );
}
