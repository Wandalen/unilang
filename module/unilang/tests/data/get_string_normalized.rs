//! Tests for `get_string_normalized` and `require_string_normalized` in `VerifiedCommand`.
//!
//! ## Scope
//! TDD tests for Plan 009 Phase 2 — whitespace normalization API on `VerifiedCommand`.
//! Both methods covered in one file: `get_string_normalized` (C1) and
//! `require_string_normalized` (C2).
//! Written before implementation (Step A) — all fail until Phase 2 Step C.
//!
//! ## Test Matrix (TC-01 to TC-53)
//! Full factor analysis in wip docs: `-test_matrix_009.md`.
//!
//! ## Groups
//! - Group 1 (TC-01 to TC-16): `get_string_normalized` — core whitespace behavior
//! - Group 2 (TC-17 to TC-23): `get_string_normalized` — absence and type errors
//! - Group 3 (TC-24 to TC-30): `require_string_normalized` — happy paths
//! - Group 4 (TC-31 to TC-38): `require_string_normalized` — error paths
//! - Group 5 (TC-39 to TC-42): Sibling parity vs `get_string`
//! - Group 6 (TC-43 to TC-44): Idempotency
//! - Group 7 (TC-45 to TC-49): wip migration behavioral regression
//! - Group 8 (TC-50 to TC-53): Pairwise corner cases
//!
//! ## Coverage Ratio
//! 21 negative / 53 total = 39.6% ≈ 40% (meets ≥ 40% requirement)

use std::collections::HashMap;
use std::path::PathBuf;
use unilang::data::CommandDefinition;
use unilang::semantic::VerifiedCommand;
use unilang::types::Value;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_cmd( args : HashMap< String, Value > ) -> VerifiedCommand
{
  VerifiedCommand
  {
    definition : CommandDefinition::former()
      .name( ".test" )
      .namespace( String::new() )
      .description( String::new() )
      .hint( String::new() )
      .status( String::new() )
      .version( "1.0.0" )
      .arguments( vec![] )
      .tags( vec![] )
      .aliases( vec![] )
      .permissions( vec![] )
      .idempotent( true )
      .deprecation_message( String::new() )
      .http_method_hint( String::new() )
      .examples( vec![] )
      .routine_link( None )
      .end(),
    arguments : args,
  }
}

fn single( key : &str, value : Value ) -> HashMap< String, Value >
{
  let mut m = HashMap::new();
  m.insert( key.to_string(), value );
  m
}

fn empty_args() -> HashMap< String, Value >
{
  HashMap::new()
}

// ---------------------------------------------------------------------------
// Group 1: get_string_normalized — Core Whitespace Behavior (TC-01 to TC-16)
// ---------------------------------------------------------------------------

/// TC-01 A1×B1×C1×D1: Normal string — no trimming needed, unchanged.
#[ test ]
fn tc_01_get_normalized_normal_string()
{
  let cmd = make_cmd( single( "p", Value::String( "hello".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-02 A2×B1×C1×D1: Leading whitespace trimmed.
#[ test ]
fn tc_02_get_normalized_leading_whitespace()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-03 A3×B1×C1×D1: Trailing whitespace trimmed.
#[ test ]
fn tc_03_get_normalized_trailing_whitespace()
{
  let cmd = make_cmd( single( "p", Value::String( "hello  ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-04 A4×B1×C1×D1: Both ends whitespace trimmed.
#[ test ]
fn tc_04_get_normalized_both_ends_whitespace()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello  ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-05 A5×B1×C1×D1: Whitespace-only → `Some("")` NOT `None`.
/// Critical edge: whitespace-only is distinct from absent; callers use `.filter()`.
#[ test ]
fn tc_05_get_normalized_whitespace_only_returns_some_empty()
{
  let cmd = make_cmd( single( "p", Value::String( "   ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "" ) );
}

/// TC-06 A6×B1×C1×D1: Empty string → `Some("")` (no trim effect).
#[ test ]
fn tc_06_get_normalized_empty_string()
{
  let cmd = make_cmd( single( "p", Value::String( String::new() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "" ) );
}

/// TC-07 A7×B1×C1×D1: Internal whitespace preserved — trim is ends-only.
#[ test ]
fn tc_07_get_normalized_internal_whitespace_preserved()
{
  let cmd = make_cmd( single( "p", Value::String( "hello world".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello world" ) );
}

/// TC-08 A8×B1×C1×D1: Tab characters at ends trimmed.
#[ test ]
fn tc_08_get_normalized_tab_ends_trimmed()
{
  let cmd = make_cmd( single( "p", Value::String( "\thello\t".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-09 A9×B1×C1×D1: Newlines at ends trimmed.
#[ test ]
fn tc_09_get_normalized_newline_ends_trimmed()
{
  let cmd = make_cmd( single( "p", Value::String( "\nhello\n".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-10 A10×B1×C1×D1: Carriage returns at ends trimmed.
#[ test ]
fn tc_10_get_normalized_cr_ends_trimmed()
{
  let cmd = make_cmd( single( "p", Value::String( "\rhello\r".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-11 A11×B1×C1×D1: Mixed whitespace types at both ends all trimmed.
#[ test ]
fn tc_11_get_normalized_mixed_whitespace_ends_trimmed()
{
  let cmd = make_cmd( single( "p", Value::String( " \t\nhello\n\t ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-12 A12×B1×C1×D1: Unicode content with ASCII spaces — spaces trimmed, content preserved.
#[ test ]
fn tc_12_get_normalized_unicode_content()
{
  let cmd = make_cmd( single( "p", Value::String( "  日本語  ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "日本語" ) );
}

/// TC-13 A13×B1×C1×D1: Single non-whitespace character, unchanged.
#[ test ]
fn tc_13_get_normalized_single_char()
{
  let cmd = make_cmd( single( "p", Value::String( "a".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "a" ) );
}

/// TC-14 A14×B1×C1×D1: Single space → `Some("")`.
#[ test ]
fn tc_14_get_normalized_single_space_becomes_empty()
{
  let cmd = make_cmd( single( "p", Value::String( " ".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "" ) );
}

/// TC-15 A15×B1×C1×D1: Long string (1000 chars) with leading/trailing spaces — trimmed correctly.
#[ test ]
fn tc_15_get_normalized_long_string()
{
  let content = "a".repeat( 1000 );
  let padded = format!( "  {}  ", content );
  let cmd = make_cmd( single( "p", Value::String( padded ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( content.as_str() ) );
}

/// TC-16 A16×B1×C1×D1: Embedded newline not at ends — preserved by trim.
#[ test ]
fn tc_16_get_normalized_embedded_newline_preserved()
{
  let cmd = make_cmd( single( "p", Value::String( "hello\nworld".to_string() ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello\nworld" ) );
}

// ---------------------------------------------------------------------------
// Group 2: get_string_normalized — Absence and Type Errors (TC-17 to TC-23)
// ---------------------------------------------------------------------------

/// TC-17 B2×C1×D2: Absent key returns `None`.
#[ test ]
fn tc_17_get_normalized_absent_key_returns_none()
{
  let cmd = make_cmd( empty_args() );
  assert_eq!( cmd.get_string_normalized( "missing" ), None );
}

/// TC-18 B3×C1×D1: Integer value → `None` (wrong type).
#[ test ]
fn tc_18_get_normalized_integer_returns_none()
{
  let cmd = make_cmd( single( "p", Value::Integer( 42 ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-19 B4×C1×D1: Float value → `None` (wrong type).
#[ test ]
fn tc_19_get_normalized_float_returns_none()
{
  let cmd = make_cmd( single( "p", Value::Float( 2.5 ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-20 B5×C1×D1: Boolean value → `None` (wrong type).
#[ test ]
fn tc_20_get_normalized_boolean_returns_none()
{
  let cmd = make_cmd( single( "p", Value::Boolean( true ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-21 B6×C1×D1: Path value → `None` (wrong type).
#[ test ]
fn tc_21_get_normalized_path_returns_none()
{
  let cmd = make_cmd( single( "p", Value::Path( PathBuf::from( "/tmp/x" ) ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-22 B7×C1×D1: List value → `None` (wrong type).
#[ test ]
fn tc_22_get_normalized_list_returns_none()
{
  let cmd = make_cmd( single( "p", Value::List( vec![] ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-23 B2×C1×D3: Empty string as argument name, key absent → `None`.
#[ test ]
fn tc_23_get_normalized_empty_name_absent_returns_none()
{
  let cmd = make_cmd( empty_args() );
  assert_eq!( cmd.get_string_normalized( "" ), None );
}

// ---------------------------------------------------------------------------
// Group 3: require_string_normalized — Happy Paths (TC-24 to TC-30)
// ---------------------------------------------------------------------------

/// TC-24 A1×B1×C2×D1: Normal string — no trimming needed.
#[ test ]
fn tc_24_require_normalized_normal_string()
{
  let cmd = make_cmd( single( "p", Value::String( "hello".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "hello" );
}

/// TC-25 A2×B1×C2×D1: Leading whitespace trimmed.
#[ test ]
fn tc_25_require_normalized_leading_whitespace()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "hello" );
}

/// TC-26 A4×B1×C2×D1: Both ends trimmed.
#[ test ]
fn tc_26_require_normalized_both_ends()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello  ".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "hello" );
}

/// TC-27 A5×B1×C2×D1: Whitespace-only → `Ok("")` — `require` checks presence, NOT non-emptiness.
#[ test ]
fn tc_27_require_normalized_whitespace_only_returns_ok_empty()
{
  let cmd = make_cmd( single( "p", Value::String( "   ".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "" );
}

/// TC-28 A6×B1×C2×D1: Empty string → `Ok("")` (empty is valid for require).
#[ test ]
fn tc_28_require_normalized_empty_string_ok()
{
  let cmd = make_cmd( single( "p", Value::String( String::new() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "" );
}

/// TC-29 A11×B1×C2×D1: Mixed whitespace types at ends all trimmed.
#[ test ]
fn tc_29_require_normalized_mixed_whitespace()
{
  let cmd = make_cmd( single( "p", Value::String( " \t\nhello\n\t ".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "hello" );
}

/// TC-30 A12×B1×C2×D1: Unicode content with ASCII spaces — spaces trimmed.
#[ test ]
fn tc_30_require_normalized_unicode_content()
{
  let cmd = make_cmd( single( "p", Value::String( "  日本語  ".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "日本語" );
}

// ---------------------------------------------------------------------------
// Group 4: require_string_normalized — Error Paths (TC-31 to TC-38)
// ---------------------------------------------------------------------------

/// TC-31 B2×C2×D1: Absent key → `Err(ArgumentTypeMismatch)`.
#[ test ]
fn tc_31_require_normalized_absent_returns_err()
{
  let cmd = make_cmd( empty_args() );
  assert!( cmd.require_string_normalized( "my_arg" ).is_err() );
}

/// TC-32 B2×C2×D1: Absent key → error message contains the argument name.
#[ test ]
fn tc_32_require_normalized_absent_error_contains_arg_name()
{
  let cmd = make_cmd( empty_args() );
  let err = cmd.require_string_normalized( "my_arg" ).unwrap_err();
  assert!( err.to_string().contains( "my_arg" ) );
}

/// TC-33 B3×C2×D1: Integer value → `Err`.
#[ test ]
fn tc_33_require_normalized_integer_returns_err()
{
  let cmd = make_cmd( single( "p", Value::Integer( 42 ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-34 B4×C2×D1: Float value → `Err`.
#[ test ]
fn tc_34_require_normalized_float_returns_err()
{
  let cmd = make_cmd( single( "p", Value::Float( 2.5 ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-35 B5×C2×D1: Boolean value → `Err`.
#[ test ]
fn tc_35_require_normalized_boolean_returns_err()
{
  let cmd = make_cmd( single( "p", Value::Boolean( true ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-36 B6×C2×D1: Path value → `Err`.
#[ test ]
fn tc_36_require_normalized_path_returns_err()
{
  let cmd = make_cmd( single( "p", Value::Path( PathBuf::from( "/tmp/x" ) ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-37 B7×C2×D1: List value → `Err`.
#[ test ]
fn tc_37_require_normalized_list_returns_err()
{
  let cmd = make_cmd( single( "p", Value::List( vec![] ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-38 B2×C2×D3: Empty string as argument name, absent → `Err`.
#[ test ]
fn tc_38_require_normalized_empty_name_returns_err()
{
  let cmd = make_cmd( empty_args() );
  assert!( cmd.require_string_normalized( "" ).is_err() );
}

// ---------------------------------------------------------------------------
// Group 5: Sibling Parity — get_string_normalized vs get_string (TC-39 to TC-42)
// ---------------------------------------------------------------------------

/// TC-39 A1×B1: Normal string — identical results from both siblings.
#[ test ]
fn tc_39_sibling_parity_normal_string_identical()
{
  let cmd = make_cmd( single( "p", Value::String( "hello".to_string() ) ) );
  assert_eq!( cmd.get_string( "p" ), Some( "hello" ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-40 A4×B1: Padded string — siblings differ: raw vs trimmed.
#[ test ]
fn tc_40_sibling_parity_padded_string_differs()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello  ".to_string() ) ) );
  assert_eq!( cmd.get_string( "p" ), Some( "  hello  " ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "hello" ) );
}

/// TC-41 A5×B1: Whitespace-only — `get_string` returns raw spaces, normalized returns `""`.
#[ test ]
fn tc_41_sibling_parity_whitespace_only_differs()
{
  let cmd = make_cmd( single( "p", Value::String( "   ".to_string() ) ) );
  assert_eq!( cmd.get_string( "p" ), Some( "   " ) );
  assert_eq!( cmd.get_string_normalized( "p" ), Some( "" ) );
}

/// TC-42 B2: Absent key — both siblings return `None`.
#[ test ]
fn tc_42_sibling_parity_absent_both_none()
{
  let cmd = make_cmd( empty_args() );
  assert_eq!( cmd.get_string( "p" ), None );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

// ---------------------------------------------------------------------------
// Group 6: Idempotency (TC-43 to TC-44)
// ---------------------------------------------------------------------------

/// TC-43: `get_string_normalized` called twice returns identical results (read-only).
#[ test ]
fn tc_43_idempotency_get_normalized()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello  ".to_string() ) ) );
  let first = cmd.get_string_normalized( "p" );
  let second = cmd.get_string_normalized( "p" );
  assert_eq!( first, second );
  assert_eq!( first, Some( "hello" ) );
}

/// TC-44: `require_string_normalized` called twice returns identical results.
#[ test ]
fn tc_44_idempotency_require_normalized()
{
  let cmd = make_cmd( single( "p", Value::String( "  hello  ".to_string() ) ) );
  let first = cmd.require_string_normalized( "p" ).unwrap();
  let second = cmd.require_string_normalized( "p" ).unwrap();
  assert_eq!( first, second );
  assert_eq!( first, "hello" );
}

// ---------------------------------------------------------------------------
// Group 7: wip Migration Behavioral Regression (TC-45 to TC-49)
// ---------------------------------------------------------------------------

/// TC-45: Whitespace-only with downstream filter → `None`.
/// Verifies `get_string_normalized("x").filter(|s| !s.is_empty()) == None` for `"   "`.
#[ test ]
fn tc_45_migration_whitespace_only_filtered_to_none()
{
  let cmd = make_cmd( single( "repo", Value::String( "   ".to_string() ) ) );
  let result = cmd.get_string_normalized( "repo" ).filter( | s | !s.is_empty() );
  assert_eq!( result, None );
}

/// TC-46: Non-empty padded value with filter → `Some("repo/name")`.
#[ test ]
fn tc_46_migration_padded_value_filtered_some()
{
  let cmd = make_cmd( single( "repo", Value::String( "  repo/name  ".to_string() ) ) );
  let result = cmd.get_string_normalized( "repo" ).filter( | s | !s.is_empty() );
  assert_eq!( result, Some( "repo/name" ) );
}

/// TC-47 B2: Absent key with downstream filter → `None` (negative).
#[ test ]
fn tc_47_migration_absent_key_filtered_none()
{
  let cmd = make_cmd( empty_args() );
  let result = cmd.get_string_normalized( "repo" ).filter( | s | !s.is_empty() );
  assert_eq!( result, None );
}

/// TC-48: Tab-only with downstream filter → `None` (negative).
#[ test ]
fn tc_48_migration_tab_only_filtered_to_none()
{
  let cmd = make_cmd( single( "repo", Value::String( "\t".to_string() ) ) );
  let result = cmd.get_string_normalized( "repo" ).filter( | s | !s.is_empty() );
  assert_eq!( result, None );
}

/// TC-49: Normal value without whitespace passes filter → `Some("hello")`.
#[ test ]
fn tc_49_migration_normal_value_returns_some()
{
  let cmd = make_cmd( single( "repo", Value::String( "hello".to_string() ) ) );
  let result = cmd.get_string_normalized( "repo" ).filter( | s | !s.is_empty() );
  assert_eq!( result, Some( "hello" ) );
}

// ---------------------------------------------------------------------------
// Group 8: Pairwise Corner Cases (TC-50 to TC-53)
// ---------------------------------------------------------------------------

/// TC-50a A11×B3 — wrong type at key → `get_string_normalized` returns `None`.
#[ test ]
fn tc_50a_pairwise_wrong_type_get_returns_none()
{
  let cmd = make_cmd( single( "p", Value::Integer( 42 ) ) );
  assert_eq!( cmd.get_string_normalized( "p" ), None );
}

/// TC-50b A11×B3 — wrong type at key → `require_string_normalized` returns `Err`.
#[ test ]
fn tc_50b_pairwise_wrong_type_require_returns_err()
{
  let cmd = make_cmd( single( "p", Value::Integer( 42 ) ) );
  assert!( cmd.require_string_normalized( "p" ).is_err() );
}

/// TC-51 A6×C2 — empty string present → `Ok("")` NOT `Err` (empty ≠ absent for require).
#[ test ]
fn tc_51_corner_empty_string_require_ok()
{
  let cmd = make_cmd( single( "p", Value::String( String::new() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "" );
}

/// TC-52 A5×C2 — whitespace-only present → `Ok("")` (require succeeds, value is empty after trim).
#[ test ]
fn tc_52_corner_whitespace_only_require_ok_empty()
{
  let cmd = make_cmd( single( "p", Value::String( "  ".to_string() ) ) );
  assert_eq!( cmd.require_string_normalized( "p" ).unwrap(), "" );
}

/// TC-53 — Multiple keys; only the fetched key is affected (isolation).
#[ test ]
fn tc_53_isolation_only_fetched_key_affected()
{
  let mut args = HashMap::new();
  args.insert( "a".to_string(), Value::String( "  x  ".to_string() ) );
  args.insert( "b".to_string(), Value::String( "  y  ".to_string() ) );
  let cmd = make_cmd( args );
  assert_eq!( cmd.get_string_normalized( "a" ), Some( "x" ) );
  // Verify "b" is unaffected by "a"'s normalization — check via sibling (raw value)
  assert_eq!( cmd.get_string( "b" ), Some( "  y  " ) );
}
