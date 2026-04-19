//! T08: Optional parameter variants — extraction at runtime.
//!
//! Verifies Option<String>, Option<bool>, Option<PathBuf> are inferred with
//! optional=true and the correct Kind; and that the wrapper returns Ok when
//! all optional args are absent (None) and when each is present.

use std::path::PathBuf;
use std::collections::HashMap;
use unilang::data::{ CommandDefinition, CommandName, Kind };
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::types::Value;

#[ unilang_meta::command( name = ".opt_types" ) ]
fn opt_types( label : Option< String >, flag : Option< bool >, path : Option< PathBuf > ) -> String
{
  format!( "{:?} {:?} {:?}", label, flag, path )
}

fn make_cmd( args : HashMap< String, Value > ) -> VerifiedCommand
{
  let cn = CommandName::new( ".opt_types" ).unwrap();
  VerifiedCommand
  {
    definition : CommandDefinition::new( cn, ".opt_types".to_string() ),
    arguments  : args,
  }
}

fn main()
{
  let def = __unilang_register_opt_types();
  assert_eq!( def.arguments().len(), 3 );

  let label_arg = &def.arguments()[ 0 ];
  assert_eq!( label_arg.kind, Kind::String );
  assert!( label_arg.attributes.optional, "label should be optional" );

  let flag_arg = &def.arguments()[ 1 ];
  assert_eq!( flag_arg.kind, Kind::Boolean );
  assert!( flag_arg.attributes.optional, "flag should be optional" );

  let path_arg = &def.arguments()[ 2 ];
  assert_eq!( path_arg.kind, Kind::Path );
  assert!( path_arg.attributes.optional, "path should be optional" );

  // All optional args absent → Ok (no missing-arg error)
  let absent = __unilang_wrapper_opt_types( make_cmd( HashMap::new() ), ExecutionContext::default() );
  assert!( absent.is_ok(), "all-absent optional args must return Ok" );

  // All optional args present with correct types → Ok
  let mut args = HashMap::new();
  args.insert( "label".to_string(), Value::String( "hello".to_string() ) );
  args.insert( "flag".to_string(), Value::Boolean( true ) );
  args.insert( "path".to_string(), Value::Path( PathBuf::from( "/tmp" ) ) );
  let present = __unilang_wrapper_opt_types( make_cmd( args ), ExecutionContext::default() );
  assert!( present.is_ok(), "all-present optional args with correct types must return Ok" );
}
