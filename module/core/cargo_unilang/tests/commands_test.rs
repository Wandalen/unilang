//!
//! Tests for command implementation modules
//!

use cargo_unilang::commands::{ validate_verbosity, parse_bool, validate_path, CheckParams, validate_project_name };
use std::path::PathBuf;

// Tests for check command
#[test]
fn test_validate_verbosity()
{
  assert_eq!( validate_verbosity( "0" ).unwrap(), 0 );
  assert_eq!( validate_verbosity( "5" ).unwrap(), 5 );
  assert!( validate_verbosity( "6" ).is_err() );
  assert!( validate_verbosity( "abc" ).is_err() );
}

#[test]
fn test_parse_bool()
{
  assert_eq!( parse_bool( "true" ).unwrap(), true );
  assert_eq!( parse_bool( "false" ).unwrap(), false );
  assert_eq!( parse_bool( "1" ).unwrap(), true );
  assert_eq!( parse_bool( "0" ).unwrap(), false );
  assert_eq!( parse_bool( "yes" ).unwrap(), true );
  assert_eq!( parse_bool( "no" ).unwrap(), false );
  assert!( parse_bool( "invalid" ).is_err() );
}

#[test]
fn test_validate_path_nonexistent()
{
  let path = PathBuf::from( "/nonexistent/path" );
  assert!( validate_path( &path ).is_err() );
}

#[test]
fn test_validate_path_valid()
{
  let temp = assert_fs::TempDir::new().unwrap();
  assert!( validate_path( temp.path() ).is_ok() );
}

#[test]
fn test_params_parse_minimal()
{
  let args = vec![];
  let params = CheckParams::parse( &args ).unwrap();
  assert_eq!( params.path, PathBuf::from( "." ) );
  assert_eq!( params.verbosity, 2 );
  assert_eq!( params.fix, false );
}

// Tests for new command
#[test]
fn test_validate_project_name_valid()
{
  assert!( validate_project_name( "my-cli" ).is_ok() );
  assert!( validate_project_name( "my_cli" ).is_ok() );
  assert!( validate_project_name( "mycli" ).is_ok() );
  assert!( validate_project_name( "_private" ).is_ok() );
}

#[test]
fn test_validate_project_name_empty()
{
  assert!( validate_project_name( "" ).is_err() );
}

#[test]
fn test_validate_project_name_too_long()
{
  let long_name = "a".repeat( 65 );
  assert!( validate_project_name( &long_name ).is_err() );
}

#[test]
fn test_validate_project_name_path_traversal()
{
  assert!( validate_project_name( "../etc" ).is_err() );
  assert!( validate_project_name( "foo/../bar" ).is_err() );
  assert!( validate_project_name( "/absolute" ).is_err() );
  assert!( validate_project_name( "foo\\bar" ).is_err() );
}

#[test]
fn test_validate_project_name_invalid_chars()
{
  assert!( validate_project_name( "my@cli" ).is_err() );
  assert!( validate_project_name( "my cli" ).is_err() );
  assert!( validate_project_name( "my.cli" ).is_err() );
}

#[test]
fn test_validate_project_name_must_start_with_letter_or_underscore()
{
  assert!( validate_project_name( "1cli" ).is_err() );
  assert!( validate_project_name( "-cli" ).is_err() );
}
