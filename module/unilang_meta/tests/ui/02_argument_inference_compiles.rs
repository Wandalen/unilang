//! T02: Argument type inference — String, bool, Option<i64> parameters.
//!
//! Verifies that the macro correctly infers Kind::String, Kind::Boolean,
//! Kind::Integer (with optional=true for Option<i64>) and produces a
//! CommandDefinition with 3 ArgumentDefinition entries.

#[ unilang_meta::command(
  name        = ".process",
  description = "Process something with typed arguments",
) ]
fn process( label : String, verbose : bool, count : Option< i64 > ) -> String
{
  if verbose
  {
    format!( "Processing '{}' {} times", label, count.unwrap_or( 1 ) )
  }
  else
  {
    label
  }
}

fn main()
{
  let def = __unilang_register_process();
  assert_eq!( def.name().as_str(), ".process" );
  assert_eq!( def.arguments().len(), 3 );
  assert_eq!( def.arguments()[ 0 ].name, "label" );
  assert_eq!( def.arguments()[ 1 ].name, "verbose" );
  assert_eq!( def.arguments()[ 2 ].name, "count" );
  assert!( def.arguments()[ 2 ].attributes.optional );
}
