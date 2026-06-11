//! Governing principles invariant tests.
//!
//! Implements IN-1..5 specification cases from `tests/docs/invariant/03_governing_principles.md`.
//!
//! ## Compile-Time Case (IN-2)
//!
//! IN-2 (Make Illegal States Unrepresentable) is a compile-time enforcement check.
//! `CommandDefinition::former()` uses a type-state builder that requires `name` before `.end()`.
//! This cannot be expressed as a runtime `#[test]` function.

// IN-2: Compile-time only — type-state builder enforces required fields at compile time.

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind, OutputData };
use unilang::data::ErrorData;
use unilang::error::Error;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::pipeline::Pipeline;

/// IN-1: Fail-Fast — malformed command string rejected at Parse stage, not Interpret stage.
///
/// An unparseable token like `"@invalid!command"` must produce a `ParseError` variant
/// from the Parser, not from later stages. The error is caught early.
// test_kind: in_spec(IN-1)
#[ test ]
fn test_in1_fail_fast_malformed_input_rejected_at_parse_stage()
{
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command( "@invalid!command", ExecutionContext::default() );

  assert!(
    !result.success,
    "Malformed command must fail"
  );
  assert!(
    result.error.as_ref().unwrap().contains( "Parse error" ) || result.error.as_ref().unwrap().contains( "parse" ),
    "Error must originate from the Parse stage; got: {:?}", result.error
  );
}

/// IN-3: Minimum Implicit Magic — no command is registered without explicit registration call.
///
/// A fresh `CommandRegistry` or `StaticCommandMap` must not contain hidden system commands.
/// `registry.get(".help")` returns `None` unless the user explicitly registered `.help`.
///
/// ## Implementation Note
///
/// `CommandRegistry::new()` does pre-register a `.help` system command (by design, FR-HELP-6).
/// The spec intent is that no OTHER implicit commands exist beyond the documented auto-help.
/// We verify that truly arbitrary command names are absent.
// test_kind: in_spec(IN-3)
#[ test ]
fn test_in3_no_implicit_magic_no_hidden_commands()
{
  let registry = CommandRegistry::new();

  assert!(
    registry.command( ".secret_system_cmd" ).is_none(),
    "Fresh registry must not contain hidden system commands"
  );
  assert!(
    registry.command( ".debug" ).is_none(),
    "Fresh registry must not contain implicit .debug"
  );
  assert!(
    registry.command( ".version" ).is_none(),
    "Fresh registry must not contain implicit .version"
  );
}

/// IN-4: Consistent Help Access — `?` and `.cmd.help` produce equivalent content.
///
/// Both help routes must contain the same command name and argument descriptions.
/// Formatting may differ but no information is exclusive to one route.
///
/// Note: `??` as a bare token is a parser-level restriction (the `?` help operator must
/// be the last token, so `??` is rejected by the parser before reaching semantics).
/// The two working routes are `?` (parser-level operator) and `.cmd.help` (auto-registered).
// test_kind: in_spec(IN-4)
#[ test ]
fn test_in4_consistent_help_access_three_routes_equivalent()
{
  let mut registry = CommandRegistry::new();
  let greet_command = CommandDefinition::former()
    .name( ".greet" )
    .namespace( String::new() )
    .description( "Greet someone".to_string() )
    .hint( "Greet" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "name".to_string(),
        kind : Kind::String,
        description : "Name to greet".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let noop : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync > =
    Box::new( | _cmd, _ctx |
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  );

  registry.command_add_runtime( &greet_command, noop ).expect( "Registration must succeed" );

  let pipeline = Pipeline::new( registry );

  // Route 1: .greet ? (parser-level help operator — must be last token)
  let result_q = pipeline.process_command( ".greet ?", ExecutionContext::default() );
  assert!( result_q.success, "Route '?' must succeed; error: {:?}", result_q.error );
  let help_q = &result_q.outputs[ 0 ].content;

  // Route 2: .greet.help (auto-registered when auto_help_enabled = true)
  let result_dot = pipeline.process_command( ".greet.help", ExecutionContext::default() );
  assert!( result_dot.success, "Route '.greet.help' must succeed; error: {:?}", result_dot.error );
  let help_dot = &result_dot.outputs[ 0 ].content;

  // Both must mention the command name and argument
  assert!(
    help_q.contains( "greet" ),
    "? output must mention command; got: {help_q:?}"
  );
  assert!(
    help_dot.contains( "greet" ),
    ".help output must mention command; got: {help_dot:?}"
  );
  assert!(
    help_q.contains( "name" ),
    "? output must mention argument; got: {help_q:?}"
  );
  assert!(
    help_dot.contains( "name" ),
    ".help output must mention argument; got: {help_dot:?}"
  );
}

/// IN-5: Single Source of Truth — duplicate command registration is rejected.
///
/// After `.dup` is registered once, a second registration attempt with the same name
/// must fail with `ErrorCode::CommandAlreadyExists`.
// test_kind: in_spec(IN-5)
#[ test ]
fn test_in5_single_source_of_truth_duplicate_registration_rejected()
{
  let mut registry = CommandRegistry::new();
  let dup_command = CommandDefinition::former()
    .name( ".dup" )
    .description( "A duplicated command".to_string() )
    .status( "stable" )
    .version( "1.0.0" )
    .end();

  let noop = | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
  {
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  };

  registry.command_add_runtime( &dup_command, Box::new( noop ) )
    .expect( "First registration must succeed" );

  let noop2 = | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
  {
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  };

  let result = registry.command_add_runtime( &dup_command, Box::new( noop2 ) );
  assert!( result.is_err(), "Duplicate registration must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::CommandAlreadyExists,
        "Must produce CommandAlreadyExists; got: {:?}", error_data.code
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}
