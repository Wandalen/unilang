//! Tests for `validation_core` module.
//!
//! ## Test Matrix
//!
//! | Test | Function | Input | Expected | Status |
//! |------|----------|-------|----------|--------|
//! | `test_validate_command_name_core_valid` | `validate_command_name_core` | ".hello" | Ok(()) | Pass |
//! | `test_validate_command_name_core_invalid` | `validate_command_name_core` | "hello" | Err | Pass |
//! | `test_validate_command_name_core_empty` | `validate_command_name_core` | "" | Err | Pass |
//! | `test_validate_namespace_core_valid` | `validate_namespace_core` | ".session" | Ok(()) | Pass |
//! | `test_validate_namespace_core_empty` | `validate_namespace_core` | "" | Ok(()) | Pass |
//! | `test_validate_namespace_core_invalid` | `validate_namespace_core` | "session" | Err | Pass |
//! | `test_validate_version_core_valid` | `validate_version_core` | "1.0.0" | Ok(()) | Pass |
//! | `test_validate_version_core_empty` | `validate_version_core` | "" | Err | Pass |
//! | `test_compute_full_name_core` | `compute_full_name_core` | Various | String | Pass |
//! | `test_validate_command_definition_core` | `validate_command_definition_core` | Various | Result | Pass |
//!
//! ## Design Rationale
//!
//! These tests verify the core validation logic that is shared between
//! runtime and build.rs. The `validation_core` module has no dependencies
//! on crate types, allowing it to be included in build.rs.
//!
//! ## Related Hypotheses
//!
//! - H27: build.rs does NOT validate command names
//! - H37: build.rs does NOT use validation functions

use unilang::validation_core::{
  validate_command_name_core,
  validate_namespace_core,
  validate_version_core,
  validate_full_name_core,
  compute_full_name_core,
  validate_command_definition_core,
};

// ============================================================================
// Command Name Validation
// ============================================================================

#[test]
fn test_validate_command_name_core_valid()
{
  assert!( validate_command_name_core( ".hello" ).is_ok() );
  assert!( validate_command_name_core( ".video.search" ).is_ok() );
  assert!( validate_command_name_core( ".a" ).is_ok() );
  assert!( validate_command_name_core( ".test.nested.deep" ).is_ok() );
}

#[test]
fn test_validate_command_name_core_invalid_missing_dot()
{
  let result = validate_command_name_core( "hello" );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "dot prefix" ), "Error should mention dot prefix: {err}" );
}

#[test]
fn test_validate_command_name_core_invalid_empty()
{
  let result = validate_command_name_core( "" );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "cannot be empty" ), "Error should mention empty: {err}" );
}

#[test]
fn test_validate_command_name_core_invalid_no_leading_dot()
{
  let result = validate_command_name_core( "hello.world" );
  assert!( result.is_err() );
}

// ============================================================================
// Namespace Validation
// ============================================================================

#[test]
fn test_validate_namespace_core_valid_with_dot()
{
  assert!( validate_namespace_core( ".session" ).is_ok() );
  assert!( validate_namespace_core( ".video.youtube" ).is_ok() );
}

#[test]
fn test_validate_namespace_core_valid_empty()
{
  // Empty namespace is allowed for root-level commands
  assert!( validate_namespace_core( "" ).is_ok() );
}

// test_kind: in_spec(IN-3b)  [invariant/05_command_naming]
#[test]
fn test_validate_namespace_core_invalid_missing_dot()
{
  let result = validate_namespace_core( "session" );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "dot prefix" ), "Error should mention dot prefix: {err}" );

  // IN-3b: namespace lacking a dot prefix must be rejected by the real production
  // pipeline (validate_command_definition_core) BEFORE compute_full_name_core() is
  // ever reached. We prove "never reached" with evidence, not a comment:
  //
  // If validate_namespace_core's early-return were skipped (bug: falls through to
  // full-name construction), compute_full_name_core( "math", ".add" ) would silently
  // produce "math..add", and validate_command_definition_core would instead fail with
  // the full-name-stage message ("Invalid full command name '...'") referencing that
  // constructed value. By asserting the error is the namespace-stage message (not the
  // full-name-stage message) AND does not reference the would-be-constructed full name,
  // we confirm compute_full_name_core was never invoked with a used result.
  // `name` is given its own valid dot prefix (".add") so that validate_command_name_core
  // passes and validate_namespace_core is the actual isolated point of failure — matching
  // the sibling test test_validate_command_definition_core_invalid_namespace's pattern.
  let pipeline_result = validate_command_definition_core( ".add", "math", "1.0.0", "test.yaml" );
  assert!( pipeline_result.is_err(), "Pipeline must reject 'math' namespace before full-name construction" );
  let pipeline_err = pipeline_result.unwrap_err();

  // Evidence 1: the error is the namespace-validation-stage message, not the
  // full-name-validation-stage message that would fire if compute_full_name_core
  // had already run and produced a still-dotless full name.
  assert!(
    pipeline_err.contains( "Invalid namespace" ),
    "Error must originate from validate_namespace_core (namespace-stage), proving \
    compute_full_name_core was never reached: {pipeline_err}"
  );
  assert!(
    !pipeline_err.contains( "Invalid full command name" ),
    "Error must NOT be the full-name-stage message; its presence would mean \
    compute_full_name_core ran and validate_full_name_core rejected its output: {pipeline_err}"
  );

  // Evidence 2: the value compute_full_name_core WOULD have produced for this exact
  // input ("math..add", per its pure concatenation logic) never appears in the
  // pipeline's error output, confirming that value was never constructed/used on
  // this rejected path.
  let would_be_full_name = compute_full_name_core( "math", ".add" );
  assert_eq!( would_be_full_name, "math..add" );
  assert!(
    !pipeline_err.contains( &would_be_full_name ),
    "Error must not reference the full name compute_full_name_core would have built \
    ('{would_be_full_name}'), proving it was never constructed for this rejected entry: {pipeline_err}"
  );
}

// ============================================================================
// Version Validation
// ============================================================================

#[test]
fn test_validate_version_core_valid()
{
  assert!( validate_version_core( "1.0.0" ).is_ok() );
  assert!( validate_version_core( "0.1" ).is_ok() );
  assert!( validate_version_core( "any" ).is_ok() );
  assert!( validate_version_core( "v1" ).is_ok() );
}

#[test]
fn test_validate_version_core_invalid_empty()
{
  let result = validate_version_core( "" );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "cannot be empty" ), "Error should mention empty: {err}" );
}

// ============================================================================
// Full Name Validation
// ============================================================================

#[test]
fn test_validate_full_name_core_valid()
{
  assert!( validate_full_name_core( ".test.command" ).is_ok() );
  assert!( validate_full_name_core( ".hello" ).is_ok() );
}

#[test]
fn test_validate_full_name_core_invalid_empty()
{
  let result = validate_full_name_core( "" );
  assert!( result.is_err() );
}

#[test]
fn test_validate_full_name_core_invalid_no_dot()
{
  let result = validate_full_name_core( "test.command" );
  assert!( result.is_err() );
}

// ============================================================================
// Full Name Computation
// ============================================================================

#[test]
fn test_compute_full_name_core_empty_namespace()
{
  assert_eq!( compute_full_name_core( "", ".chat" ), ".chat" );
  assert_eq!( compute_full_name_core( "", ".hello" ), ".hello" );
}

#[test]
fn test_compute_full_name_core_with_namespace()
{
  assert_eq!( compute_full_name_core( ".session", "list" ), ".session.list" );
  assert_eq!( compute_full_name_core( ".video", "search" ), ".video.search" );
  assert_eq!( compute_full_name_core( ".a.b", "c" ), ".a.b.c" );
}

// ============================================================================
// Command Definition Validation
// ============================================================================

#[test]
fn test_validate_command_definition_core_valid()
{
  let result = validate_command_definition_core(
    ".test",
    "",
    "1.0.0",
    "test.yaml"
  );
  assert!( result.is_ok(), "Expected Ok, got: {result:?}" );
}

#[test]
fn test_validate_command_definition_core_valid_with_namespace()
{
  let result = validate_command_definition_core(
    "list",
    ".session",
    "1.0.0",
    "test.yaml"
  );
  // Note: This should FAIL because "list" doesn't start with dot
  // but when combined with ".session", the full name becomes ".session.list"
  // which IS valid. Let's check the actual behavior:
  // Actually, the function validates name independently first, so this should fail
  assert!( result.is_err(), "Expected Err because 'list' lacks dot prefix" );
}

#[test]
fn test_validate_command_definition_core_invalid_name()
{
  let result = validate_command_definition_core(
    "test",
    "",
    "1.0.0",
    "test.yaml"
  );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "test.yaml" ), "Error should include file path: {err}" );
}

#[test]
fn test_validate_command_definition_core_invalid_namespace()
{
  let result = validate_command_definition_core(
    ".test",
    "invalid",
    "1.0.0",
    "commands/test.yaml"
  );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "commands/test.yaml" ), "Error should include file path: {err}" );
}

#[test]
fn test_validate_command_definition_core_invalid_version()
{
  let result = validate_command_definition_core(
    ".test",
    "",
    "",
    "test.yaml"
  );
  assert!( result.is_err() );
  let err = result.unwrap_err();
  assert!( err.contains( "version" ), "Error should mention version: {err}" );
}
