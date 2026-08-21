//! Test explicit command naming validation (FR-REG-6)
//! 
//! Tests that the framework enforces explicit dot prefixes and rejects
//! commands that don't follow the naming requirements.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::print_literal ) ]
#![ allow( clippy::single_char_pattern ) ]


use unilang::{ CommandDefinition, CommandRegistry, ExecutionContext, VerifiedCommand, OutputData, ErrorData };

fn dummy_handler(_cmd: VerifiedCommand, _ctx: ExecutionContext) -> Result< OutputData, ErrorData >
{
  Ok( OutputData { content: "test".to_string(), format: "text".to_string(), execution_time_ms: None } )
}

/// Test that construction rejects commands without dot prefix (Phase 2 fail-fast)
///
/// **Phase 2 Update:** Validation moved from registration to construction time.
/// Invalid names now panic during `CommandDefinition::former().name()` call.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_reject_commands_without_dot_prefix()
{
  // Phase 2: This panics at construction time, before registration
  let _invalid_cmd = CommandDefinition::former()
    .name( "chat" ) // ❌ Missing dot prefix - panics here
    .description( "This should be rejected" )
    .end();
}

#[test] 
fn test_reject_invalid_namespace()
{
    let mut registry = CommandRegistry::new();
  
  // This should be REJECTED - namespace without dot prefix
  let mut invalid_cmd = CommandDefinition::former()
    .name( ".list" ) // ✅ Correct name
    .description( "This should be rejected" )
    .end();

  // Manually set invalid namespace after creation
  invalid_cmd.namespace = "session".to_string(); // ❌ Namespace missing dot
  
    let result = registry.register_with_routine(&invalid_cmd, Box::new(dummy_handler));
  
  // Should fail with explicit error message
  assert!(result.is_err(), "Namespace without dot prefix should be rejected");
  
  let error_msg = format!("{:?}", result.unwrap_err());
  assert!(error_msg.contains("namespace"), 
         "Error should mention namespace: {}", error_msg);
  assert!(error_msg.contains("must start with dot prefix"), 
         "Error should mention dot prefix requirement: {}", error_msg);
  
  println!("✅ Correctly rejected invalid namespace");
}

#[test]
fn test_accept_correctly_formatted_commands()
{
    let mut registry = CommandRegistry::new();
  
  // Root-level command - should be accepted
  let root_cmd = CommandDefinition::former()
    .name( ".test_chat" ) // ✅ Correct dot prefix
    .description( "Correctly formatted root command" )
    .end(); // ✅ Empty namespace for root
  
    let result = registry.register_with_routine(&root_cmd, Box::new(dummy_handler));
  assert!(result.is_ok(), "Correctly formatted root command should be accepted");
  println!("✅ Accepted correctly formatted root command");
  
  // Namespaced command - should be accepted
  let mut namespaced_cmd = CommandDefinition::former()
    .name( ".list" ) // ✅ Correct dot prefix
    .description( "Correctly formatted namespaced command" )
    .end();

  // Set valid namespace
  namespaced_cmd.namespace = ".session".to_string(); // ✅ Correct namespace with dot
  
    let result2 = registry.register_with_routine(&namespaced_cmd, Box::new(dummy_handler));
  assert!(result2.is_ok(), "Correctly formatted namespaced command should be accepted");
  println!("✅ Accepted correctly formatted namespaced command");
}

/// Test that valid commands are accepted (Minimum Implicit Magic principle)
///
/// **Principle:** Commands are registered exactly as specified,
/// with no automatic transformations or prefix additions.
#[test]
fn test_principle_minimum_implicit_magic()
{
  println!("\n🎯 TESTING GOVERNING PRINCIPLE: Minimum Implicit Magic");
  println!("   - Commands registered exactly as specified");
  println!("   - No automatic transformations or prefix additions");
  println!("   - Explicit validation with clear error messages");
  println!("   - What you register is exactly what gets executed\n");

    let mut registry = CommandRegistry::new();

  // Test valid command with explicit dot prefix
  let cmd = CommandDefinition::former()
    .name( ".chat" )
    .description( "Testing name: .chat" )
    .end();

    let result = registry.register_with_routine(&cmd, Box::new(dummy_handler));

  assert!(result.is_ok(), "Command '.chat' should be accepted");
  println!("   {} Command '.chat' correctly accepted", "✅");

  println!("\n🎉 Principle successfully enforced!");
}

/// Test that invalid commands are rejected at construction (Minimum Implicit Magic principle)
///
/// **Phase 2 Update:** Validation moved to construction time.
/// Invalid names panic during `CommandDefinition::former().name()` call.
#[test]
#[should_panic(expected = "MissingDotPrefix")]
fn test_principle_minimum_implicit_magic_rejects_invalid()
{
  // Phase 2: This panics at construction time
  let _invalid_cmd = CommandDefinition::former()
    .name( "chat" ) // ❌ Missing dot prefix - panics here
    .description( "Testing name: chat" )
    .end();
}
/// An `Enum` parameter whose default is not among its own choices could never
/// pass coercion — registration must reject it outright.
#[test]
fn test_enum_default_outside_choices_rejected_at_registration()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, Kind };

  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".lint_enum_bad" )
    .description( "Enum default outside choices" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "mode".to_string(),
        description : "Mode selector".to_string(),
        kind : Kind::Enum( vec![ "fast".to_string(), "slow".to_string() ] ),
        hint : String::new(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "turbo".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let result = registry.register( cmd );
  assert!( result.is_err(), "Enum default outside choices must be rejected" );
  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!(
    error_msg.contains( "not among its enum choices" ),
    "Rejection must explain the enum-default mismatch; got: {error_msg}"
  );
}

/// The same definition with a default drawn from the choices registers fine.
#[test]
fn test_enum_default_within_choices_accepted()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, Kind };

  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".lint_enum_ok" )
    .description( "Enum default within choices" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "mode".to_string(),
        description : "Mode selector".to_string(),
        kind : Kind::Enum( vec![ "fast".to_string(), "slow".to_string() ] ),
        hint : String::new(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "slow".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  assert!( registry.register( cmd ).is_ok(), "Enum default within choices must register" );
}

/// A `String` parameter whose description embeds an `a|b|c` choice list gets a
/// non-fatal warning steering toward `Kind::Enum` — registration still succeeds.
#[test]
fn test_string_choice_list_description_warns_but_registers()
{
  use unilang::data::{ ArgumentAttributes, ArgumentDefinition, Kind };
  use unilang::command_validation::validate_help_conventions;

  // Suppress the stderr side of the warning in test output
  std::env::set_var( "UNILANG_NO_LINT_WARNINGS", "1" );

  let build = | description : &str |
  {
    CommandDefinition::former()
      .name( ".lint_string_choices" )
      .description( "String with choice-list description" )
      .arguments( vec![
        ArgumentDefinition
        {
          name : "format".to_string(),
          description : description.to_string(),
          kind : Kind::String,
          hint : String::new(),
          attributes : ArgumentAttributes { optional : true, ..Default::default() },
          validation_rules : vec![],
          aliases : vec![],
          tags : vec![],
        }
      ])
      .end()
  };

  // Whole-description choice list (2 segments) warns
  let warnings = validate_help_conventions( &build( "json|yaml" ) ).unwrap();
  assert_eq!( warnings.len(), 1, "Whole-description choice list must warn" );
  assert!( warnings[ 0 ].contains( "Kind::Enum" ), "Warning must steer toward Kind::Enum" );
  assert!( warnings[ 0 ].contains( "format::??" ), "Warning must mention the help syntax" );

  // Embedded token with >= 3 segments warns
  let warnings = validate_help_conventions( &build( "Output format: json|yaml|table" ) ).unwrap();
  assert_eq!( warnings.len(), 1, "Embedded 3-segment choice list must warn" );

  // Embedded 2-segment token inside prose does NOT warn (avoids false positives)
  let warnings = validate_help_conventions( &build( "Filter expression, e.g. name|size" ) ).unwrap();
  assert!( warnings.is_empty(), "Prose with a single embedded pipe pair must not warn" );

  // Plain prose does not warn
  let warnings = validate_help_conventions( &build( "Free-form output format string" ) ).unwrap();
  assert!( warnings.is_empty(), "Plain prose must not warn" );

  // The warning is non-fatal: registration succeeds
  let mut registry = CommandRegistry::new();
  assert!( registry.register( build( "json|yaml|table" ) ).is_ok(),
    "Choice-list description warning must not block registration" );
}
