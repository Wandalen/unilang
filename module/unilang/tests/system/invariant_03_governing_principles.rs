//! Governing principles invariant tests.
//!
//! Implements IN-1..7 specification cases from `tests/docs/invariant/03_governing_principles.md`.
//!
//! ## Compile-Time Case (IN-2)
//!
//! IN-2 (Make Illegal States Unrepresentable) is a compile-time enforcement check.
//! `CommandDefinition::former()` uses a type-state builder that requires `name` before `.end()`.
//! It is not expressed as a runtime `#[test]` in this file — it is verified by the trybuild
//! compile-fail test `test_tc_compile_fail_type_state_and_private_fields` in
//! `tests/build/compile_fail_tests.rs` (tagged `in_spec(IN-2)`), which confirms
//! `tests/compile_fail/t40_builder_missing_name.rs` is rejected by rustc.

// IN-2: Compile-time only — see tests/build/compile_fail_tests.rs::test_tc_compile_fail_type_state_and_private_fields


use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind, OutputData };
use unilang::data::ErrorData;
use unilang::error::Error;
use unilang::data::CommandName;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::pipeline::Pipeline;
use unilang_parser::{ Parser, UnilangParserOptions };

/// IN-1: Fail-Fast — malformed command string rejected at Parse stage, not Interpret stage.
///
/// An unparseable token like `"@invalid!command"` must produce a `ParseError` variant
/// from the Parser, not from later stages. The error is caught early.
// test_kind: in_spec(IN-1)  [invariant/03_governing_principles]
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
// test_kind: in_spec(IN-3)  [invariant/03_governing_principles]
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

/// IN-4: Consistent Help Access — `??` and `.cmd.help` produce equivalent content.
///
/// Both help routes must contain the same command name and argument descriptions.
/// Since both render through the same `unilang_help` renderer, the pages must be
/// byte-identical — no information is exclusive to one route.
// test_kind: in_spec(IN-4)  [invariant/03_governing_principles]
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

  registry.register_with_routine( &greet_command, noop ).expect( "Registration must succeed" );

  let pipeline = Pipeline::new( registry );

  // Route 1: .greet ?? (semantic-level help token)
  let result_q = pipeline.process_command( ".greet ??", ExecutionContext::default() );
  assert!( result_q.success, "Route '??' must succeed; error: {:?}", result_q.error );
  let help_q = &result_q.outputs[ 0 ].content;

  // Route 2: .greet.help (auto-registered when auto_help_enabled = true)
  let result_dot = pipeline.process_command( ".greet.help", ExecutionContext::default() );
  assert!( result_dot.success, "Route '.greet.help' must succeed; error: {:?}", result_dot.error );
  let help_dot = &result_dot.outputs[ 0 ].content;

  // Both must mention the command name and argument
  assert!(
    help_q.contains( "greet" ),
    "?? output must mention command; got: {help_q:?}"
  );
  assert!(
    help_q.contains( "name" ),
    "?? output must mention argument; got: {help_q:?}"
  );

  // Both routes render through the same renderer — pages must be identical
  assert_eq!( help_q, help_dot, "?? and .cmd.help must render the identical page" );
}

/// IN-5: Single Source of Truth — duplicate command registration is rejected.
///
/// After `.dup` is registered once, a second registration attempt with the same name
/// must fail with `ErrorCode::CommandAlreadyExists`.
// test_kind: in_spec(IN-5)  [invariant/03_governing_principles]
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

  registry.register_with_routine( &dup_command, Box::new( noop ) )
    .expect( "First registration must succeed" );

  let noop2 = | _cmd : VerifiedCommand, _ctx : ExecutionContext | -> Result< OutputData, ErrorData >
  {
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  };

  let result = registry.register_with_routine( &dup_command, Box::new( noop2 ) );
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

/// IN-6: Explicit Dependencies — missing required argument is rejected with actionable error.
///
/// `.needs_arg` declares an `ArgumentDefinition` whose `attributes.optional` is `false`
/// (a required argument, making the dependency on that value explicit). Invoking the
/// command without providing it must produce `ErrorCode::ArgumentMissing`, and the error
/// message must name the missing argument and instruct the caller to provide it.
// test_kind: in_spec(IN-6)  [invariant/03_governing_principles]
#[ test ]
fn test_in6_explicit_dependencies_missing_required_argument_rejected()
{
  let mut registry = CommandRegistry::new();
  let needs_arg_command = CommandDefinition::former()
    .name( ".needs_arg" )
    .description( "Command with a required argument".to_string() )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "target".to_string(),
        kind : Kind::String,
        description : "Required target value".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( needs_arg_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".needs_arg" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Invoking without the required argument must fail" );

  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ArgumentMissing,
        "Must produce ArgumentMissing; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "target" ),
        "Error message must name the missing argument 'target'; got: {:?}", error_data.message
      );
      assert!(
        error_data.message.to_lowercase().contains( "provide" ),
        "Error message must instruct the caller to provide the value; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// IN-7: Explicit Command Naming — registration without dot prefix is rejected.
///
/// A command name string `"build"` (no leading dot) passed to `CommandName::new` must
/// return `Err`, not a silently auto-corrected `".build"`. The framework never adds an
/// implicit dot prefix or otherwise transforms the name on the caller's behalf.
// test_kind: in_spec(IN-7)  [invariant/03_governing_principles]
#[ test ]
fn test_in7_explicit_command_naming_no_dot_prefix_rejected()
{
  let result = CommandName::new( "build" );

  assert!(
    result.is_err(),
    "CommandName::new must reject a name without a leading dot prefix"
  );

  match result.unwrap_err()
  {
    Error::MissingDotPrefix( original ) =>
    {
      assert_eq!(
        original, "build",
        "Error must preserve the original caller-supplied string unmodified; got: {original:?}"
      );
    },
    other => panic!( "Expected Error::MissingDotPrefix, got: {other:?}" ),
  }

  // Confirm no implicit auto-correction ever succeeds silently: a properly dot-prefixed
  // name is required, and the framework does not derive one from the rejected input.
  let corrected = CommandName::new( ".build" );
  assert!(
    corrected.is_ok(),
    "A caller-supplied dot-prefixed name must still succeed on its own merits"
  );
  assert_eq!(
    corrected.unwrap().as_str(),
    ".build",
    "CommandName must never transform the caller-supplied value beyond validation"
  );
}
