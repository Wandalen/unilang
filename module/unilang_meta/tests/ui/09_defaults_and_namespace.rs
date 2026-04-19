//! T09: Default field values when optional attributes are omitted.
//!
//! Verifies that when `description` is absent the generated CommandDefinition
//! uses the `name` value as description, and when `namespace` is absent the
//! field is an empty string.  Also verifies a zero-parameter command.

#[ unilang_meta::command( name = ".defaults" ) ]
fn defaults() -> String
{
  "ok".to_string()
}

fn main()
{
  let def = __unilang_register_defaults();

  // description defaults to name
  assert_eq!( def.description(), ".defaults" );

  // namespace defaults to empty string
  assert_eq!( def.namespace.as_str(), "" );

  // zero parameters
  assert!( def.arguments().is_empty() );
}
