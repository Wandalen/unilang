//! API error code contract tests.
//!
//! Implements AP-1..6 specification cases from `tests/docs/api/02_error_codes.md`.
//!
//! Tests verify that each `ErrorCode` variant is produced under its documented condition
//! and that enum derives (`Clone`, `PartialEq`, `Eq`) are present and functional.
//!
//! ## Approach
//!
//! AP-1..4 use `SemanticAnalyzer` directly — the error codes originate in semantic
//! analysis (type coercion, unknown parameter detection, missing arg validation) before
//! the interpreter is ever called.
//!
//! AP-5 uses `CommandRegistry::command_add_runtime` directly because `CommandAlreadyExists`
//! is returned at registration time, not during pipeline execution.
//!
//! AP-6 verifies `Clone + PartialEq + Eq` at runtime using equality assertions that
//! would panic if the derives were absent.

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind, OutputData };
use unilang::data::ErrorData;
use unilang::error::Error;
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang_parser::{ Parser, UnilangParserOptions };

/// AP-1: `CommandNotFound` is returned for an unregistered command path.
///
/// A registry that does not contain `.unknown` must cause the semantic analyzer
/// to return `Error::Execution(ErrorData { code: CommandNotFound, ... })` with a
/// non-empty message when `.unknown` is invoked.
// test_kind: ap_spec(AP-1)
#[ test ]
fn test_ap1_command_not_found_for_unregistered_command()
{
  let registry = CommandRegistry::new();
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".unknown" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Unregistered command must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::CommandNotFound,
        "Must produce CommandNotFound; got: {:?}", error_data.code
      );
      assert!(
        !error_data.message.is_empty(),
        "Error message must be non-empty"
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// AP-2: `ArgumentMissing` is returned when a required argument is absent.
///
/// `.greet` is registered with a required `name: String` argument. Invoking
/// `.greet` without supplying `name` must produce `ErrorCode::ArgumentMissing`
/// with the argument name present in the error message.
// test_kind: ap_spec(AP-2)
#[ test ]
fn test_ap2_argument_missing_for_absent_required_arg()
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
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( greet_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".greet" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Missing required argument must produce an error" );
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
        error_data.message.contains( "name" ),
        "Error message must contain the argument name; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// AP-3: `UnknownParameter` is returned for a named argument not in the command definition.
///
/// `.greet` is registered with only a `name` argument (optional). Invoking
/// `.greet typo::"value"` must produce `ErrorCode::UnknownParameter` with the
/// unknown parameter name present in the error message.
///
/// ## Pitfall
///
/// `name` is declared optional here to isolate the `UnknownParameter` failure.
/// If `name` were required, the validation order (unknown params vs. missing args)
/// could determine which error fires first, making the test fragile.
// test_kind: ap_spec(AP-3)
#[ test ]
fn test_ap3_unknown_parameter_for_undefined_named_arg()
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

  registry.register( greet_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".greet typo::"value""# ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Unknown parameter must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::UnknownParameter,
        "Must produce UnknownParameter; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "typo" ),
        "Error message must contain the unknown parameter name; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// AP-4: `ArgumentTypeMismatch` is returned when a value cannot be coerced to the declared `Kind`.
///
/// `.add` is registered with argument `x` of `Kind::Integer`. Invoking `.add x::"not_a_number"`
/// must produce `ErrorCode::ArgumentTypeMismatch` because `"not_a_number"` cannot be parsed
/// as an integer during semantic type coercion.
// test_kind: ap_spec(AP-4)
#[ test ]
fn test_ap4_argument_type_mismatch_for_non_coercible_value()
{
  let mut registry = CommandRegistry::new();
  let add_command = CommandDefinition::former()
    .name( ".add" )
    .namespace( String::new() )
    .description( "Add a number".to_string() )
    .hint( "Add" )
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
        name : "x".to_string(),
        kind : Kind::Integer,
        description : "An integer value".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( add_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".add x::"not_a_number""# ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Type mismatch must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ArgumentTypeMismatch,
        "Must produce ArgumentTypeMismatch; got: {:?}", error_data.code
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// AP-5: `CommandAlreadyExists` is returned when registering a duplicate command name.
///
/// After `.dup` is registered once via `command_add_runtime`, a second registration
/// attempt with the same name must fail with `ErrorCode::CommandAlreadyExists`.
/// The error comes from `command_add_runtime` at registration time, not from the pipeline.
// test_kind: ap_spec(AP-5)
#[ test ]
fn test_ap5_command_already_exists_for_duplicate_registration()
{
  let mut registry = CommandRegistry::new();
  let dup_command = CommandDefinition::former()
    .name( ".dup" )
    .namespace( String::new() )
    .description( "A duplicated command".to_string() )
    .hint( "Dup" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
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
    other => panic!( "Expected Error::Execution with CommandAlreadyExists, got: {other:?}" ),
  }
}

/// AP-6: `ErrorCode` derives `Clone`, `PartialEq`, and `Eq`.
///
/// Verifies at runtime that the derive implementations produce correct results:
/// - Two identical variants compare equal (`PartialEq`)
/// - A cloned value equals the original (`Clone + PartialEq`)
/// - Different variants compare unequal (`Eq` transitivity)
// test_kind: ap_spec(AP-6)
#[ test ]
fn test_ap6_error_code_derives_clone_partial_eq_eq()
{
  let a = ErrorCode::CommandNotFound;
  let b = ErrorCode::CommandNotFound;
  let c = a.clone();

  assert_eq!( a, b, "Two identical ErrorCode variants must be equal (PartialEq)" );
  assert_eq!( a, c, "Cloned ErrorCode must equal the original (Clone + PartialEq)" );

  let d = ErrorCode::ArgumentMissing;
  assert_ne!( a, d, "Different ErrorCode variants must not be equal (Eq)" );
}
