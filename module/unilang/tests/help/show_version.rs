//!
//! # Show Version In Help Tests
//!
//! ## What This Tests
//!
//! This test suite validates the `show_version_in_help` field functionality:
//! - Per-command control of version display in help output
//! - Conversion preservation from `StaticCommandDefinition` to `CommandDefinition`
//! - Default behavior (version shown when true)
//! - Hide behavior (version hidden when false)
//!
//! ## Why This Matters
//!
//! Users may want to hide default version "1.0.0" from help output when they haven't
//! explicitly set a version. The `show_version_in_help` field provides opt-out capability.
//!
//! ## Related
//!
//! - `HelpDisplayOptions` for global version hiding
//! - `src/help.rs` for help formatting implementation

use unilang::static_data::StaticCommandDefinition;
use unilang::data::CommandDefinition;
use unilang::help::HelpDisplayOptions;

//
// Test: show_version_in_help defaults to true
//

/// Verifies that `show_version_in_help` defaults to true in `CommandDefinition`.
#[ test ]
fn command_definition_show_version_defaults_to_true()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end();

  assert!( cmd.show_version_in_help(), "show_version_in_help should default to true" );
}

//
// Test: show_version_in_help can be set to false
//

/// Verifies that `show_version_in_help` can be set to false via builder.
#[ test ]
fn command_definition_show_version_can_be_false()
{
  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command".to_string() )
    .end()
    .with_show_version_in_help( false );

  assert!( !cmd.show_version_in_help(), "show_version_in_help should be false when set" );
}

//
// Test: static to dynamic conversion preserves show_version_in_help true
//

/// Verifies that conversion preserves `show_version_in_help` = true.
#[ test ]
fn from_static_preserves_show_version_true()
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

  assert!( dynamic_cmd.show_version_in_help(), "show_version_in_help=true should be preserved" );
}

//
// Test: static to dynamic conversion preserves show_version_in_help false
//

/// Verifies that conversion preserves `show_version_in_help` = false.
#[ test ]
fn from_static_preserves_show_version_false()
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
    show_version_in_help : false,
  };

  let dynamic_cmd : CommandDefinition = ( &STATIC_CMD ).into();

  assert!( !dynamic_cmd.show_version_in_help(), "show_version_in_help=false should be preserved" );
}

//
// Test: StaticCommandDefinition::new defaults to true
//

/// Verifies that `StaticCommandDefinition::new()` defaults `show_version_in_help` to true.
#[ test ]
fn static_command_new_defaults_show_version_true()
{
  let static_cmd = StaticCommandDefinition::new( ".test", "", "Test command" );

  assert!( static_cmd.show_version_in_help, "new() should default show_version_in_help to true" );
}

//
// Test: StaticCommandDefinition builder method works
//

/// Verifies that `with_show_version_in_help` builder method works.
#[ test ]
fn static_command_with_show_version_in_help()
{
  let static_cmd = StaticCommandDefinition::new( ".test", "", "Test command" )
    .with_show_version_in_help( false );

  assert!( !static_cmd.show_version_in_help, "with_show_version_in_help(false) should set to false" );
}

//
// Test: help output respects show_version_in_help = true
//

/// Verifies that help output includes version when `show_version_in_help` is true.
#[ test ]
fn help_output_includes_version_when_true()
{
  use unilang::registry::CommandRegistry;

  let cmd = CommandDefinition::former()
    .name( ".test_version_shown" )
    .description( "Test command".to_string() )
    .version( "2.5.0".to_string() )
    .end()
    .with_show_version_in_help( true );

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help = registry.help_for_command( ".test_version_shown" );
  assert!( help.is_some(), "Help should be generated" );

  let help_text = help.unwrap();
  assert!( help_text.contains( "2.5.0" ), "Help should include version when show_version_in_help=true" );
}

//
// Test: help output respects show_version_in_help = false
//

/// Verifies that help output excludes version when `show_version_in_help` is false.
#[ test ]
fn help_output_excludes_version_when_false()
{
  use unilang::registry::CommandRegistry;

  let cmd = CommandDefinition::former()
    .name( ".test_version_hidden" )
    .description( "Test command".to_string() )
    .version( "3.0.0".to_string() )
    .end()
    .with_show_version_in_help( false );

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help = registry.help_for_command( ".test_version_hidden" );
  assert!( help.is_some(), "Help should be generated" );

  let help_text = help.unwrap();
  assert!( !help_text.contains( "3.0.0" ), "Help should NOT include version when show_version_in_help=false" );
}

//
// Test: UNILANG_HELP_HIDE_VERSION env var flips HelpDisplayOptions.show_version
//

/// FT-15: `UNILANG_HELP_HIDE_VERSION` suppresses version metadata from help output.
///
/// Verifies both the `HelpDisplayOptions` field toggle and the end-to-end rendered-output
/// effect via `HelpGenerator` (the `?`/`??` access path) — `with_env_overrides()` flows
/// into `format_fns.rs`'s rendering via `HelpGenerator::display_options`.
///
/// ## Note
///
/// This test mutates a process-level env var. nextest runs each test in a separate process,
/// so env var mutation does not affect sibling tests.
// test_kind: ft_spec(FT-15)  [feature/04_help_system]
#[ test ]
fn test_ft15_hide_version_env_var_suppresses_show_version_flag()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::{ HelpGenerator, HelpVerbosity };

  let old_value = std::env::var( "UNILANG_HELP_HIDE_VERSION" ).ok();

  let cmd = CommandDefinition::former()
    .name( ".test_ft15" )
    .description( "Test command".to_string() )
    .version( "4.2.0".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  std::env::set_var( "UNILANG_HELP_HIDE_VERSION", "1" );
  let options_hidden = HelpDisplayOptions::default().with_env_overrides();
  let help_text_hidden = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".test_ft15" )
    .expect( "Command should exist" );

  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
  let options_shown = HelpDisplayOptions::default().with_env_overrides();
  let help_text_shown = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".test_ft15" )
    .expect( "Command should exist" );

  match old_value
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_HIDE_VERSION", v ),
    None => std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" ),
  }

  assert!(
    !options_hidden.show_version,
    "UNILANG_HELP_HIDE_VERSION=1 must set show_version to false"
  );
  assert!(
    options_shown.show_version,
    "Unsetting UNILANG_HELP_HIDE_VERSION must restore show_version to true"
  );
  assert!(
    !help_text_hidden.contains( "4.2.0" ),
    "UNILANG_HELP_HIDE_VERSION=1 must suppress the version string in HelpGenerator-rendered output"
  );
  assert!(
    help_text_shown.contains( "4.2.0" ),
    "Unsetting UNILANG_HELP_HIDE_VERSION must restore the version string in HelpGenerator-rendered output"
  );
}

//
// Test Matrix (task 113): HelpDisplayOptions wiring to rendering
//

/// T01: Baseline — default `HelpDisplayOptions`, no suppression active, version present.
// test_kind: ft_spec(T01)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t01_default_display_options_shows_version()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::{ HelpGenerator, HelpVerbosity };

  let cmd = CommandDefinition::former()
    .name( ".test_t01" )
    .description( "Test command".to_string() )
    .version( "1.2.3".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help_text = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".test_t01" )
    .expect( "Command should exist" );

  assert!( help_text.contains( "1.2.3" ), "Version must be present with default HelpDisplayOptions" );
}

/// T02: Registry-wide suppression via explicit `HelpDisplayOptions::default().hide_version()`.
// test_kind: ft_spec(T02)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t02_explicit_hide_version_suppresses_version()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::{ HelpGenerator, HelpVerbosity };

  let cmd = CommandDefinition::former()
    .name( ".test_t02" )
    .description( "Test command".to_string() )
    .version( "1.2.3".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help_text = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .with_display_options( HelpDisplayOptions::default().hide_version() )
    .command( ".test_t02" )
    .expect( "Command should exist" );

  assert!(
    !help_text.contains( "1.2.3" ),
    "Version must be absent when HelpDisplayOptions::hide_version() is set explicitly, even though the command's own show_version_in_help() defaults to true"
  );
}

/// T03: Registry-wide suppression via `UNILANG_HELP_HIDE_VERSION` env var through `HelpGenerator::from_env`.
// test_kind: ft_spec(T03)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t03_env_var_via_from_env_suppresses_version()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::HelpGenerator;

  let old_value = std::env::var( "UNILANG_HELP_HIDE_VERSION" ).ok();

  let cmd = CommandDefinition::former()
    .name( ".test_t03" )
    .description( "Test command".to_string() )
    .version( "5.5.5".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  std::env::set_var( "UNILANG_HELP_HIDE_VERSION", "1" );
  let help_text_hidden = HelpGenerator::from_env( &registry )
    .command( ".test_t03" )
    .expect( "Command should exist" );

  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
  let help_text_shown = HelpGenerator::from_env( &registry )
    .command( ".test_t03" )
    .expect( "Command should exist" );

  match old_value
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_HIDE_VERSION", v ),
    None => std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" ),
  }

  assert!( !help_text_hidden.contains( "5.5.5" ), "UNILANG_HELP_HIDE_VERSION=1 must suppress version via HelpGenerator::from_env" );
  assert!( help_text_shown.contains( "5.5.5" ), "Unsetting UNILANG_HELP_HIDE_VERSION must restore version via HelpGenerator::from_env" );
}

/// T04: Same env var, exercised through the `.command.help` access path (`registry.help_for_command()`).
// test_kind: ft_spec(T04)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t04_env_var_suppresses_version_in_command_help_path()
{
  use unilang::registry::CommandRegistry;

  let old_value = std::env::var( "UNILANG_HELP_HIDE_VERSION" ).ok();

  let cmd = CommandDefinition::former()
    .name( ".test_t04" )
    .description( "Test command".to_string() )
    .version( "6.6.6".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  std::env::set_var( "UNILANG_HELP_HIDE_VERSION", "1" );
  let help_hidden = registry.help_for_command( ".test_t04" ).expect( "Help should be generated" );

  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
  let help_shown = registry.help_for_command( ".test_t04" ).expect( "Help should be generated" );

  match old_value
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_HIDE_VERSION", v ),
    None => std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" ),
  }

  assert!(
    !help_hidden.contains( "6.6.6" ),
    "UNILANG_HELP_HIDE_VERSION=1 must suppress version in .command.help output identically to the ?/?? path"
  );
  assert!( help_shown.contains( "6.6.6" ), "Unsetting the env var must restore version in .command.help output" );
}

/// T05: Per-command `show_version_in_help(false)` is never overridden by a permissive registry-wide default.
// test_kind: ft_spec(T05)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t05_per_command_false_not_overridden_by_permissive_global_default()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::{ HelpGenerator, HelpVerbosity };

  let cmd = CommandDefinition::former()
    .name( ".test_t05" )
    .description( "Test command".to_string() )
    .version( "7.7.7".to_string() )
    .end()
    .with_show_version_in_help( false );

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let permissive_global_default = HelpDisplayOptions::default();
  assert!(
    permissive_global_default.show_version,
    "Registry-wide default must remain permissive (show_version: true) for this test to be meaningful"
  );

  let help_text = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .with_display_options( permissive_global_default )
    .command( ".test_t05" )
    .expect( "Command should exist" );

  assert!(
    !help_text.contains( "7.7.7" ),
    "Per-command show_version_in_help(false) must suppress version even when the registry-wide HelpDisplayOptions default is permissive"
  );
}

/// T06: Previously-dead builder methods (`hide_status`/`hide_aliases`/`hide_tags`) now wired to rendering.
// test_kind: ft_spec(T06)  [task/unilang/113_wire_help_display_options_to_rendering]
#[ test ]
fn test_t06_hide_status_aliases_tags_all_suppressed()
{
  use unilang::registry::CommandRegistry;
  use unilang::help::{ HelpGenerator, HelpVerbosity };

  let cmd = CommandDefinition::former()
    .name( ".test_t06" )
    .description( "Test command".to_string() )
    .status( "deprecated".to_string() )
    .aliases( vec![ "t06alias".to_string() ] )
    .tags( vec![ "t06tag".to_string() ] )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  let help_text = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Detailed )
    .with_display_options( HelpDisplayOptions::default().hide_status().hide_aliases().hide_tags() )
    .command( ".test_t06" )
    .expect( "Command should exist" );

  assert!( !help_text.contains( "deprecated" ), "Status must be absent when hide_status() is set" );
  assert!( !help_text.contains( "t06alias" ), "Aliases must be absent when hide_aliases() is set" );
  assert!( !help_text.contains( "t06tag" ), "Tags must be absent when hide_tags() is set" );
}
