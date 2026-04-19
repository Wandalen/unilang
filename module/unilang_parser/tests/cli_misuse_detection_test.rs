//! CLI misuse detection tests for `parse_cli_args` and `CliParser<C>::parse`.
//!
//! Tests T01-T08 from TSK-086 test matrix — verify that common misuse patterns
//! from clap/shell users are caught early with actionable hint errors.
//!
//! ## Test Matrix
//!
//! | # | Input | API Path | Expected |
//! |---|-------|----------|----------|
//! | T01 | `["scope:local"]` | `parse_cli_args` | `Err` "Parameters must use '::' separator" |
//! | T02 | `["scope:local"]` | `CliParser::parse` | same error as T01 (consistency — was missing) |
//! | T03 | `["timeout=5000"]` | `parse_cli_args` | `Err` hint: use `::` not `=` |
//! | T04 | `["timeout=5000"]` | `CliParser::parse` | same hint as T03 |
//! | T05 | `["--verbose"]` | `parse_cli_args` | `Err` hint: use named params not `--flag` |
//! | T06 | `["-v"]` | `parse_cli_args` | `Err` hint: use named params not `-f` |
//! | T07 | `["scope::local"]` | both paths | parses successfully, scope = "local" |
//! | T08 | `["path::tests/file.md"]` | both paths | parses successfully, path value preserved |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Single-colon `name:value` rejected with `::` hint (both paths)
//! - ✅ Equals-sign `name=value` rejected with `::` hint (both paths)
//! - ✅ Double-dash `--flag` rejected with named-param hint
//! - ✅ Single-dash `-f` rejected with named-param hint
//! - ✅ Valid `name::value` parses without false positive
//! - ✅ Valid `path::value/with/slashes` parses without false positive
//!
//! ## Root Cause
//!
//! Users migrating from clap, shell scripts, or Python argparse instinctively reach for
//! `--flag`, `-f`, `name=value`, and `name:value` syntax. All four were silently treated
//! as positional/message tokens, producing confusing downstream errors that pointed nowhere
//! near the actual problem (wrong separator). Additionally, `CliParser<C>::parse()` lacked
//! the single-colon check that `parse_cli_args` already had, making the same wrong input
//! produce different outcomes depending on which API the caller picked.
//!
//! ## Why Not Caught Initially
//!
//! Existing tests only validated correct `name::value` syntax. No tests covered the common
//! wrong-syntax paths. The inconsistency between `parse_cli_args` and `CliParser<C>::parse()`
//! was never noticed because the two paths were not cross-validated in a single test file.
//!
//! ## Fix Applied
//!
//! Added 4 early-exit detection guards in both `parse_cli_args` and `CliParser<C>::parse()`:
//! - `name:value` (single colon) → "Parameters must use '::' separator"
//! - `name=value` (equals sign) → "use 'name::value' syntax instead of 'name=value'"
//! - `--flag` (double dash) → "use named parameters: 'flag::true' or 'flag::value'"
//! - `-f` (single dash) → "use named parameters instead of '-flag' short flags"
//!
//! Location: `src/cli_parser.rs` in the `parsing_params` block of both parse paths.
//!
//! ## Prevention
//!
//! Always add detection for ALL common wrong-syntax patterns at BOTH API entry points.
//! Write cross-path consistency tests (T01 vs T02, T03 vs T04) to catch gaps where one
//! path has validation and the other doesn't.
//!
//! ## Pitfall
//!
//! Detection must only fire in the **parameter phase** (before message collection begins).
//! A `-` character as a message fragment (stdin sentinel, path element) must not be
//! rejected once the parser has entered message mode. Apply guards BEFORE the
//! `parsing_params` phase switches to message mode.

use std::collections::BTreeSet;
use unilang_parser::cli_parser ::
{
  parse_cli_args, CliParams, CliParseResult,
  CliParser, CliParamsAdvanced, CliParseResultAdvanced,
};

// ---------------------------------------------------------------------------
// Minimal CliParams implementation for parse_cli_args tests
// ---------------------------------------------------------------------------

#[ derive( Default, Debug ) ]
struct SimpleParams
{
  scope : Option< String >,
  path  : Option< String >,
}

impl CliParams for SimpleParams
{
  fn process_param( &mut self, key : &str, value : &str ) -> Result< bool, String >
  {
    match key
    {
      "scope" => { self.scope = Some( value.to_string() ); Ok( true ) }
      "path"  => { self.path  = Some( value.to_string() ); Ok( true ) }
      _       => Ok( false ),
    }
  }

  fn validate( &self ) -> Result< (), String >
  {
    Ok( () )
  }
}

// ---------------------------------------------------------------------------
// Minimal CliParamsAdvanced implementation for CliParser<C>::parse() tests
// ---------------------------------------------------------------------------

struct EmptyConfig;

#[ derive( Default, Debug ) ]
struct AdvancedParams
{
  scope : Option< String >,
  path  : Option< String >,
}

impl CliParamsAdvanced< EmptyConfig > for AdvancedParams
{
  fn process_param( &mut self, key : &str, value : &str ) -> Result< Option< &'static str >, String >
  {
    match key
    {
      "scope" => { self.scope = Some( value.to_string() ); Ok( Some( "scope" ) ) }
      "path"  => { self.path  = Some( value.to_string() ); Ok( Some( "path" ) ) }
      _       => Ok( None ),
    }
  }

  fn apply_defaults( &mut self, _config : &EmptyConfig, _explicit : &BTreeSet< String > ) {}

  fn finalize( &mut self, _explicit : &BTreeSet< String >, _message : &str ) {}

  fn validate( &self ) -> Result< (), String >
  {
    Ok( () )
  }
}

// ---------------------------------------------------------------------------
// T01 — single-colon via parse_cli_args (already detected — regression guard)
// ---------------------------------------------------------------------------

// test_kind: regression_prevention(issue-086)
#[ test ]
fn t01_single_colon_parse_cli_args()
{
  // T01: scope:local via parse_cli_args must error with '::' hint
  let args = vec![ "scope:local".to_string() ];
  let result : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "scope:local must be rejected by parse_cli_args" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "::" ) || err.contains( "separator" ) || err.contains( "syntax" ),
    "error must mention '::' or separator. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T02 — single-colon via CliParser<C>::parse() (consistency — was missing)
// ---------------------------------------------------------------------------

// test_kind: bug_reproducer(issue-086)
#[ test ]
fn t02_single_colon_cliparser_parse()
{
  // T02: scope:local via CliParser::parse must produce same error as T01 (was: silently treated as message)
  let config = EmptyConfig;
  let args = vec![ "scope:local".to_string() ];
  let result : Result< CliParseResultAdvanced< AdvancedParams >, String > = CliParser ::new()
    .with_config( &config )
    .parse( &args );

  assert!( result.is_err(), "scope:local must be rejected by CliParser::parse for consistency with parse_cli_args" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "::" ) || err.contains( "separator" ) || err.contains( "syntax" ),
    "error must mention '::' or separator. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T03 — equals-sign via parse_cli_args
// ---------------------------------------------------------------------------

// test_kind: bug_reproducer(issue-086)
#[ test ]
fn t03_equals_sign_parse_cli_args()
{
  // T03: timeout=5000 via parse_cli_args must error with hint to use ::
  let args = vec![ "timeout=5000".to_string() ];
  let result : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "timeout=5000 must be rejected by parse_cli_args; use 'timeout::5000'" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "::" ) || err.contains( "name::value" ) || err.contains( "separator" ),
    "error must hint at '::' syntax. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T04 — equals-sign via CliParser<C>::parse()
// ---------------------------------------------------------------------------

// test_kind: bug_reproducer(issue-086)
#[ test ]
fn t04_equals_sign_cliparser_parse()
{
  // T04: timeout=5000 via CliParser::parse must produce same hint as T03
  let config = EmptyConfig;
  let args = vec![ "timeout=5000".to_string() ];
  let result : Result< CliParseResultAdvanced< AdvancedParams >, String > = CliParser ::new()
    .with_config( &config )
    .parse( &args );

  assert!( result.is_err(), "timeout=5000 must be rejected by CliParser::parse" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "::" ) || err.contains( "name::value" ) || err.contains( "separator" ),
    "error must hint at '::' syntax. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T05 — double-dash --flag via parse_cli_args
// ---------------------------------------------------------------------------

// test_kind: bug_reproducer(issue-086)
#[ test ]
fn t05_double_dash_flag_parse_cli_args()
{
  // T05: --verbose via parse_cli_args must error with named-param hint
  let args = vec![ "--verbose".to_string() ];
  let result : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "--verbose must be rejected; unilang uses named params not --flag" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "named" ) || err.contains( "::" ) || err.contains( "--" ),
    "error must hint at named parameter syntax. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T06 — single-dash -f via parse_cli_args
// ---------------------------------------------------------------------------

// test_kind: bug_reproducer(issue-086)
#[ test ]
fn t06_single_dash_flag_parse_cli_args()
{
  // T06: -v via parse_cli_args must error with named-param hint
  let args = vec![ "-v".to_string() ];
  let result : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );

  assert!( result.is_err(), "-v must be rejected; unilang uses named params not -f short flags" );
  let err = result.unwrap_err();
  assert!(
    err.contains( "named" ) || err.contains( "::" ) || err.contains( "-" ),
    "error must hint at named parameter syntax. Got: {err}"
  );
}

// ---------------------------------------------------------------------------
// T07 — valid scope::local (regression guard for both paths)
// ---------------------------------------------------------------------------

// test_kind: regression_prevention(issue-086)
#[ test ]
fn t07_valid_double_colon_both_paths()
{
  // T07: scope::local must parse successfully on both paths (no false positive)
  let args = vec![ "scope::local".to_string() ];

  let result1 : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );
  assert!( result1.is_ok(), "scope::local must parse via parse_cli_args: {:?}", result1.err() );
  assert_eq!( result1.unwrap().params.scope, Some( "local".to_string() ) );

  let config = EmptyConfig;
  let result2 : Result< CliParseResultAdvanced< AdvancedParams >, String > = CliParser ::new()
    .with_config( &config )
    .parse( &args );
  assert!( result2.is_ok(), "scope::local must parse via CliParser::parse: {:?}", result2.err() );
  assert_eq!( result2.unwrap().params.scope, Some( "local".to_string() ) );
}

// ---------------------------------------------------------------------------
// T08 — valid path value with slashes (regression guard for both paths)
// ---------------------------------------------------------------------------

// test_kind: regression_prevention(issue-086)
#[ test ]
fn t08_valid_path_value_with_slashes_both_paths()
{
  // T08: path::tests/file.md must parse successfully — '/' in value must not trigger false positive
  let args = vec![ "path::tests/file.md".to_string() ];

  let result1 : Result< CliParseResult< SimpleParams >, String > = parse_cli_args( &args );
  assert!( result1.is_ok(), "path::tests/file.md must parse via parse_cli_args: {:?}", result1.err() );
  assert_eq!( result1.unwrap().params.path, Some( "tests/file.md".to_string() ) );

  let config = EmptyConfig;
  let result2 : Result< CliParseResultAdvanced< AdvancedParams >, String > = CliParser ::new()
    .with_config( &config )
    .parse( &args );
  assert!( result2.is_ok(), "path::tests/file.md must parse via CliParser::parse: {:?}", result2.err() );
  assert_eq!( result2.unwrap().params.path, Some( "tests/file.md".to_string() ) );
}
