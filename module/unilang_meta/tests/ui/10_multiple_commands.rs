//! T10: Multiple #[unilang_meta::command] macros in the same compilation unit.
//!
//! Verifies that two macro invocations generate non-colliding static/wrapper/register
//! identifiers and both registration functions return the expected definitions.

#[ unilang_meta::command( name = ".alpha" ) ]
fn alpha() -> String
{
  "alpha".to_string()
}

#[ unilang_meta::command( name = ".beta", description = "Beta command" ) ]
fn beta( x : i64 ) -> String
{
  x.to_string()
}

fn main()
{
  let alpha_def = __unilang_register_alpha();
  let beta_def  = __unilang_register_beta();

  assert_eq!( alpha_def.name().as_str(), ".alpha" );
  assert!( alpha_def.arguments().is_empty() );

  assert_eq!( beta_def.name().as_str(), ".beta" );
  assert_eq!( beta_def.description(), "Beta command" );
  assert_eq!( beta_def.arguments().len(), 1 );
}
