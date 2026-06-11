//! Bug-reproducer tests for BUG-089: `category` field loss during
//! `StaticCommandDefinition` → `CommandDefinition` conversion.
//!
//! Also guards against regression of BUG-088 (`auto_help_enabled` — same pattern).
//!
//! ## Root Cause
//!
//! The `From<&StaticCommandDefinition>` impl hardcoded `.with_category( "" )` instead
//! of copying `static_cmd.category`, discarding all YAML-configured category values.
//! Data flow: YAML → build.rs → `StaticCommandDefinition` → **From conversion** → `CommandDefinition`.
//!
//! ## Why Not Caught
//!
//! Most test commands used empty-string categories (the default). The hardcoded `""`
//! happened to match, so conversion tests passed. Only commands with explicit non-empty
//! `category` values in YAML were silently broken — and no test asserted a non-default value.
//!
//! ## Fix Applied
//!
//! Changed `.with_category( "" )` to `.with_category( static_cmd.category )` in the
//! `From<&StaticCommandDefinition>` impl. Updated `MultiYamlAggregator` codegen to emit
//! the category field. Same fix pattern as BUG-088 (`auto_help_enabled`).
//!
//! ## Prevention
//!
//! Every field added to `StaticCommandDefinition` must have a dedicated conversion test
//! asserting non-default values survive the round-trip. These tests cover: non-empty
//! category, empty category, special characters, all-fields-populated, and cross-field
//! regression (both `category` and `auto_help_enabled` preserved simultaneously).
//!
//! ## Pitfall
//!
//! **Silent Field Loss Pattern (BUG-088 + BUG-089):** When adding fields to
//! `StaticCommandDefinition`, ALL code paths must be updated: struct field, build.rs
//! extraction, PHF generation, Static→Dynamic conversion, MultiYamlAggregator generation.
//! Testing only default values masks hardcoded defaults masquerading as correct conversions.

use unilang::static_data::*;
use unilang::data::CommandDefinition;

//
// Test: from_static preserves non-empty category
//

/// Verifies that conversion preserves non-empty category value.
///
/// This prevents loss of category during conversion (was BUG-089 root cause).
// test_kind: bug_reproducer(BUG-089)
#[ test ]
fn from_static_preserves_non_empty_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "git_ops",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "git_ops" );
}

//
// Test: from_static preserves empty category
//

/// Verifies that conversion preserves empty category (not null or "uncategorized").
///
/// This prevents empty category becoming unexpected default.
#[ test ]
fn from_static_preserves_empty_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "" );
}

//
// Test: from_static with all fields preserves category
//

/// Verifies that category is preserved when all struct fields are populated.
///
/// This prevents category being overwritten by other field conversions.
#[ test ]
fn from_static_with_all_fields_preserves_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "namespace",
    description : "Test command",
    hint : "test hint",
    arguments : &[],
    routine_link : Some( "routine" ),
    status : "stable",
    version : "1.0.0",
    tags : &[ "tag1", "tag2" ],
    aliases : &[ "alias1" ],
    permissions : &[ "read" ],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "POST",
    examples : &[ "example" ],
    auto_help_enabled : false,
    category : "test_category",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "test_category" );
  assert!( !dynamic_cmd.auto_help_enabled() );
  assert_eq!( dynamic_cmd.name().as_str(), ".test" );
}

//
// Test: conversion doesn't modify category
//

/// Verifies that category value is unchanged through conversion (no trim, lowercase, etc.).
///
/// This prevents unexpected category transformations.
#[ test ]
fn conversion_doesnt_modify_category()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : true,
    category : "  MiXeD_CaSe_WiTh_SpAcEs  ",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!( dynamic_cmd.category(), "  MiXeD_CaSe_WiTh_SpAcEs  " );
}

//
// Test: BUG-088 regression - both fields preserved
//

/// Verifies that both `auto_help_enabled` AND `category` are preserved in conversion.
///
/// This prevents regression of BUG-088 fix when adding BUG-089 fix.
// test_kind: bug_reproducer(BUG-088)
// test_kind: bug_reproducer(BUG-089)
#[ test ]
fn issue_088_regression_both_fields_preserved()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test command",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : false,
    category : "test",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert!( !dynamic_cmd.auto_help_enabled(), "BUG-088 regression: auto_help_enabled not preserved" );
  assert_eq!( dynamic_cmd.category(), "test", "BUG-089: category not preserved" );
}
