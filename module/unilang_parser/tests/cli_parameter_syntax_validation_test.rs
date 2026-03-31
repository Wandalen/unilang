//! Parameter syntax validation tests for CLI parser.
//!
//! ## Root Cause
//!
//! The unilang parameter parser does not validate the `::` separator for named parameters.
//! When a malformed parameter uses single colon (e.g., `scope:local` instead of `scope::local`),
//! the parser treats it as a positional argument instead of a named parameter, leading to
//! confusing error messages that reference downstream behavior rather than parameter syntax.
//!
//! ## Why Not Caught Initially
//!
//! Existing CLI parser tests (`cli_parser_tests.rs`) only tested valid parameter syntax
//! (`param::value`). No tests verified behavior when users accidentally use single-colon
//! syntax, which is a common mistake given other CLI tools use single colons.
//!
//! ## Fix Applied
//!
//! Added validation in `cli_parser.rs` (line ~267) to detect single-colon syntax BEFORE
//! attempting `split_once("::")`. The fix checks if an argument contains `:` but not `::`,
//! and returns a clear error message explaining the correct syntax.
//!
//! ## Prevention
//!
//! Add boundary tests for parameter syntax validation to catch malformed separators.
//! This test file serves as the regression guard.
//!
//! ## Pitfall
//!
//! Parameter parsers must validate syntax BEFORE attempting to match parameters.
//! Treating malformed syntax as positional args creates confusing error messages that
//! mislead users about the actual problem (syntax error vs downstream validation).

use unilang_parser::cli_parser::{ parse_cli_args, CliParams, CliParseResult };

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct TestParams
{
  scope: Option< String >,
  format: Option< String >,
}

impl CliParams for TestParams
{
  fn process_param( &mut self, key: &str, value: &str ) -> Result< bool, String >
  {
    match key
    {
      "scope" => { self.scope = Some( value.to_string() ); }
      "format" => { self.format = Some( value.to_string() ); }
      _ => return Ok( false ), // Unknown parameter
    }
    Ok( true )
  }

  fn validate( &self ) -> Result< (), String >
  {
    Ok( () )
  }
}

/// Test: Malformed scope parameter (single colon) should be rejected with syntax error
///
/// Bug: Currently interprets `scope:local` as positional argument instead of invalid parameter syntax
// test_kind: bug_reproducer(manual-test-2026-02-12)
#[test]
fn malformed_scope_single_colon()
{
  let args = vec![ "scope:local".to_string() ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  // CURRENT (WRONG): Treats as positional arg, parser succeeds with message="scope:local"
  // EXPECTED: Returns Err with message about parameter syntax and :: separator

  assert!( result.is_err(), "Single-colon parameter should be rejected" );

  let err = result.unwrap_err();
  assert!(
    err.contains( "parameter" ) || err.contains( "syntax" ) || err.contains( "::" ),
    "Error should mention parameter syntax or :: separator. Got: {err}"
  );
}

/// Test: Malformed format parameter (single colon) should be rejected with syntax error
// test_kind: bug_reproducer(manual-test-2026-02-12)
#[test]
fn malformed_format_single_colon()
{
  let args = vec![ "format:tree".to_string() ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "Single-colon parameter should be rejected" );

  let err = result.unwrap_err();
  assert!(
    err.contains( "parameter" ) || err.contains( "syntax" ) || err.contains( "::" ),
    "Error should mention parameter syntax or :: separator. Got: {err}"
  );
}

/// Test: Valid scope parameter (double colon) should work correctly
#[test]
fn valid_scope_double_colon()
{
  let args = vec![ "scope::local".to_string() ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  assert!( result.is_ok(), "Valid double-colon syntax should succeed" );

  let parsed = result.unwrap();
  assert_eq!( parsed.params.scope, Some( "local".to_string() ) );
  assert_eq!( parsed.message, "" );
}

/// Test: Valid format parameter (double colon) should work correctly
#[test]
fn valid_format_double_colon()
{
  let args = vec![ "format::list".to_string() ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  assert!( result.is_ok(), "Valid double-colon syntax should succeed" );

  let parsed = result.unwrap();
  assert_eq!( parsed.params.format, Some( "list".to_string() ) );
  assert_eq!( parsed.message, "" );
}

/// Test: Mixed valid parameters and message should work
#[test]
fn mixed_valid_params_and_message()
{
  let args = vec![
    "scope::local".to_string(),
    "format::tree".to_string(),
    "this".to_string(),
    "is".to_string(),
    "message".to_string(),
  ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  assert!( result.is_ok(), "Valid syntax with message should succeed" );

  let parsed = result.unwrap();
  assert_eq!( parsed.params.scope, Some( "local".to_string() ) );
  assert_eq!( parsed.params.format, Some( "tree".to_string() ) );
  assert_eq!( parsed.message, "this is message" );
}

/// Test: Malformed parameter before valid message should fail
#[test]
fn malformed_before_message()
{
  let args = vec![
    "scope:local".to_string(),
    "valid".to_string(),
    "message".to_string(),
  ];
  let result: Result< CliParseResult< TestParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "Should reject malformed parameter before message" );

  let err = result.unwrap_err();
  assert!(
    err.contains( "parameter" ) || err.contains( "syntax" ) || err.contains( "::" ),
    "Error should mention parameter syntax. Got: {err}"
  );
}
