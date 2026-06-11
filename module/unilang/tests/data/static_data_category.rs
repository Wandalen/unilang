//! Bug-reproducer tests for Issue-089: `category` field loss during
//! `StaticCommandDefinition` → `CommandDefinition` conversion.

use unilang::static_data::*;

/// Reproduces category field loss during Static→Dynamic conversion.
///
/// ## Root Cause
///
/// The `From<&StaticCommandDefinition>` impl hardcoded `.with_category( "" )` instead
/// of using `static_cmd.category`, discarding all YAML-configured category values.
///
/// ## Pitfall
///
/// **Silent Field Loss Pattern (Issue-088 + Issue-089):** When adding fields to
/// `StaticCommandDefinition`, ALL code paths must be updated: struct field, build.rs
/// extraction, PHF generation, Static→Dynamic conversion, MultiYamlAggregator generation.
// test_kind: bug_reproducer(issue-089)
#[ test ]
fn test_category_conversion_preserves_non_empty_value()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".deploy",
    namespace : "ops",
    description : "Deploy application",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : false,
    deprecation_message : "",
    http_method_hint : "",
    examples : &[],
    auto_help_enabled : true,
    category : "deployment_operations",
    show_version_in_help : true,
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!(
    dynamic_cmd.category(),
    "deployment_operations",
    "Category field must be preserved during Static→Dynamic conversion"
  );
}

/// Verifies that empty category string is preserved (not converted to a different default).
///
/// ## Pitfall
///
/// **Untested Boundary Conditions:** Just because current code "works" for
/// boundary cases doesnt mean its intentional. Explicit tests prevent
/// accidental breakage and document expected behavior.
// test_kind: bug_reproducer(issue-089)
#[ test ]
fn test_category_conversion_preserves_empty_string()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".help",
    namespace : "",
    description : "Show help",
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
    http_method_hint : "",
    examples : &[],
    auto_help_enabled : false,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!(
    dynamic_cmd.category(),
    "",
    "Empty category must be preserved (indicates uncategorized command)"
  );
}

/// Verifies category field with spaces and special characters is preserved exactly.
///
/// ## Pitfall
///
/// **String Field Transformations:** Never assume string fields should be
/// normalized (trim, lowercase, sanitize). Preserve exact user input unless
/// the spec explicitly requires transformation.
// test_kind: bug_reproducer(issue-089)
#[ test ]
fn test_category_conversion_preserves_special_characters()
{
  static STATIC_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".test",
    namespace : "",
    description : "Test",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : false,
    deprecation_message : "",
    http_method_hint : "",
    examples : &[],
    auto_help_enabled : true,
    category : "Git Operations / Advanced",
    show_version_in_help : true,
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_CMD ).into();

  assert_eq!(
    dynamic_cmd.category(),
    "Git Operations / Advanced",
    "Category with spaces and special characters must be preserved exactly"
  );
}
