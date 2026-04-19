//! T01: Basic attribute parsing — function with no parameters, minimal attributes.
//!
//! Verifies that the macro compiles without errors when only `name` is provided
//! and the function takes no parameters.

#[ unilang_meta::command( name = ".ping" ) ]
fn ping() -> String
{
  "pong".to_string()
}

fn main()
{
  // Registration function must be callable and return a non-null reference.
  let def = __unilang_register_ping();
  assert_eq!( def.name().as_str(), ".ping" );
}
