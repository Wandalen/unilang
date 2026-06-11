//! Bug-reproducer tests for Issue-088: `auto_help_enabled` field loss during
//! `StaticCommandDefinition` → `CommandDefinition` conversion.

use unilang::static_data::*;

/// Test that `auto_help_enabled` is preserved during Static→Dynamic conversion (true case)
///
/// # Root Cause
///
/// The `From<&StaticCommandDefinition> for CommandDefinition` implementation
/// hardcoded `auto_help_enabled: false` instead of reading from the source field.
///
/// # Fix Applied
///
/// 1. Added `auto_help_enabled: bool` field to `StaticCommandDefinition`
/// 2. Updated `build.rs` to extract `auto_help_enabled` from YAML (defaults to true)
/// 3. Updated conversion to copy field value instead of hardcoding
///
/// # Pitfall
///
/// **Silent Field Loss in Conversions:** Any field in `StaticCommandDefinition` that
/// isnt explicitly copied in the `From<&StaticCommandDefinition>` impl will be lost
/// or defaulted, silently breaking user YAML configuration.
// test_kind: bug_reproducer(issue-088)
#[ test ]
fn test_auto_help_enabled_conversion_preserves_true()
{
  static STATIC_CMD_WITH_HELP : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".crates.list",
    namespace : ".crates",
    description : "List all crates in workspace",
    hint : "Lists crates",
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
    examples : &[ ".crates.list" ],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_CMD_WITH_HELP ).into();

  assert!(
    dynamic_cmd.auto_help_enabled(),
    "Expected auto_help_enabled to be true (from static definition), but conversion returned false. \
     This breaks .command.help generation for all commands with auto_help_enabled: true in YAML."
  );

  assert!(
    dynamic_cmd.has_auto_help(),
    "has_auto_help() should return true when auto_help_enabled is true"
  );
}

/// Test that `auto_help_enabled: false` is preserved during conversion.
///
/// Help commands themselves should have `auto_help_enabled: false` to prevent
/// recursive help generation.
///
/// # Pitfall
///
/// See `test_auto_help_enabled_conversion_preserves_true` for detailed analysis.
// test_kind: bug_reproducer(issue-088)
#[ test ]
fn test_auto_help_enabled_conversion_preserves_false()
{
  static STATIC_HELP_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".crates.list.help",
    namespace : ".crates",
    description : "Help for .crates.list command",
    hint : "Show help",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[ "help" ],
    aliases : &[],
    permissions : &[],
    idempotent : true,
    deprecation_message : "",
    http_method_hint : "GET",
    examples : &[],
    auto_help_enabled : false,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : unilang::data::CommandDefinition = ( &STATIC_HELP_CMD ).into();

  assert!(
    !dynamic_cmd.auto_help_enabled(),
    "Expected auto_help_enabled to be false for help commands (prevents recursion)"
  );

  assert!(
    !dynamic_cmd.has_auto_help(),
    "has_auto_help() should return false when auto_help_enabled is false"
  );
}

/// Verifies that `CommandDefinition` supports `auto_help_enabled` via the builder API.
///
/// # Pitfall
///
/// **Incomplete Test Coverage:** Even comprehensive-looking tests can miss critical
/// fields. Systematic verification of ALL struct fields is required.
// test_kind: bug_reproducer(issue-088)
#[ test ]
fn test_existing_conversion_test_includes_auto_help()
{
  use unilang::data::CommandName;

  let name = CommandName::new( ".test" ).expect( "valid command name" );
  let cmd = unilang::data::CommandDefinition::new( name, "Test command".to_string() )
    .with_auto_help( true );

  assert!( cmd.auto_help_enabled(), "CommandDefinition should support auto_help_enabled via with_auto_help" );
  assert!( cmd.has_auto_help(), "has_auto_help() should return true when auto_help_enabled is true" );
}
