//! T04: Full pipeline — namespace, description, arguments all present in generated definition.
//!
//! Verifies that the registration function returns a CommandDefinition whose
//! fields match all specified macro attributes and inferred parameters.

use unilang::data::Kind;

#[ unilang_meta::command(
  name        = ".list",
  namespace   = ".session",
  description = "List active sessions",
  hint        = "List sessions hint",
) ]
fn list( filter : String, limit : i64 ) -> String
{
  format!( "Listing sessions with filter='{}' limit={}", filter, limit )
}

fn main()
{
  let def = __unilang_register_list();

  // Name and namespace
  assert_eq!( def.name().as_str(), ".list" );
  assert_eq!( def.namespace, ".session" );

  // Description
  assert_eq!( def.description(), "List active sessions" );

  // Inferred arguments
  assert_eq!( def.arguments().len(), 2 );

  let filter_arg = &def.arguments()[ 0 ];
  assert_eq!( filter_arg.name, "filter" );
  assert_eq!( filter_arg.kind, Kind::String );
  assert!( !filter_arg.attributes.optional );

  let limit_arg = &def.arguments()[ 1 ];
  assert_eq!( limit_arg.name, "limit" );
  assert_eq!( limit_arg.kind, Kind::Integer );
  assert!( !limit_arg.attributes.optional );
}
