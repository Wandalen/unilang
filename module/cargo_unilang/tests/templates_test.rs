//!
//! Tests for template generation functions
//!

use cargo_unilang::templates::*;

#[test]
fn test_cargo_toml_minimal()
{
  let content = cargo_toml( "my-cli", None, None );
  assert!( content.contains( "name = \"my-cli\"" ) );
  assert!( content.contains( "unilang = \"0.46\"" ) );
  assert!( content.contains( "Do NOT create build.rs" ) );
  assert!( content.contains( "license = \"MIT\"" ) );
}

#[test]
fn test_cargo_toml_with_author()
{
  let content = cargo_toml( "my-cli", Some( "John Doe <john@example.com>" ), None );
  assert!( content.contains( "authors = [ \"John Doe <john@example.com>\" ]" ) );
}

#[test]
fn test_cargo_toml_with_license()
{
  let content = cargo_toml( "my-cli", None, Some( "Apache-2.0" ) );
  assert!( content.contains( "license = \"Apache-2.0\"" ) );
}

#[test]
fn test_commands_yaml_minimal_has_example_command()
{
  let content = commands_yaml_minimal();
  assert!( content.contains( ".greet" ) );
  assert!( content.contains( ".help" ) );
  assert!( content.contains( "Unilang does ALL of this automatically" ) );
}

#[test]
fn test_commands_yaml_full_has_multiple_commands()
{
  let content = commands_yaml_full();
  assert!( content.contains( ".greet" ) );
  assert!( content.contains( ".echo" ) );
  assert!( content.contains( ".help" ) );
}

#[test]
fn test_main_rs_minimal_contains_key_elements()
{
  let content = main_rs_minimal();
  assert!( content.contains( "StaticCommandRegistry::from_commands" ) );
  assert!( content.contains( "include!" ) );
  assert!( content.contains( "STATIC_COMMANDS" ) );
  assert!( content.contains( "NO custom build.rs" ) );
}

#[test]
fn test_main_rs_full_has_error_handling()
{
  let content = main_rs_full();
  assert!( content.contains( "process::exit" ) );
  assert!( content.contains( "match run()" ) );
}
