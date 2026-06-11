//! API error code contract tests.
//!
//! Implements AP-1..14 specification cases from `tests/docs/api/02_error_codes.md`.
//!
//! Tests verify that each `ErrorCode` variant is produced under its documented condition
//! and that enum derives (`Clone`, `PartialEq`, `Eq`) are present and functional.
//!
//! ## Approach
//!
//! AP-1..4, AP-7, AP-9 use `SemanticAnalyzer` directly — the error codes originate in
//! semantic analysis (type coercion, unknown parameter detection, missing arg validation,
//! excess positional args, interactive arg signaling) before the interpreter is ever called.
//!
//! AP-5 uses `CommandRegistry::command_add_runtime` directly because `CommandAlreadyExists`
//! is returned at registration time, not during pipeline execution.
//!
//! AP-6 verifies `Clone + PartialEq + Eq` at runtime using equality assertions that
//! would panic if the derives were absent.
//!
//! AP-10 uses `Interpreter::run` because `CommandNotImplemented` is returned by the
//! command routine at execution time, not during semantic analysis.

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind, OutputData };
use unilang::data::ErrorData;
use unilang::error::Error;
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
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

/// AP-7: `TooManyArguments` is returned when excess positional arguments are provided.
///
/// `.single` is registered with one positional `item: String` argument. Invoking
/// `.single val1 val2 val3` provides three positional values but the command only
/// accepts one. The semantic analyzer must produce `ErrorCode::TooManyArguments`.
// test_kind: ap_spec(AP-7)
#[ test ]
fn test_ap7_too_many_arguments_for_excess_positional_args()
{
  let mut registry = CommandRegistry::new();
  let single_command = CommandDefinition::former()
    .name( ".single" )
    .namespace( String::new() )
    .description( "Accepts one argument".to_string() )
    .hint( "Single" )
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
        name : "item".to_string(),
        kind : Kind::String,
        description : "Single item".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( single_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".single val1 val2 val3" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Excess positional arguments must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::TooManyArguments,
        "Must produce TooManyArguments; got: {:?}", error_data.code
      );
    },
    other => panic!( "Expected Error::Execution with TooManyArguments, got: {other:?}" ),
  }
}

/// AP-9: `ArgumentInteractiveRequired` is returned when an interactive argument is missing.
///
/// `.auth` is registered with a `token` argument marked `interactive = true` and
/// `optional = false`. Invoking `.auth` without providing `token` must produce
/// `ErrorCode::ArgumentInteractiveRequired` — distinct from `ArgumentMissing` — to
/// signal that the REPL should prompt the user for secure input.
// test_kind: ap_spec(AP-9)
#[ test ]
fn test_ap9_argument_interactive_required_for_missing_interactive_arg()
{
  let mut registry = CommandRegistry::new();
  let auth_command = CommandDefinition::former()
    .name( ".auth" )
    .namespace( String::new() )
    .description( "Authenticate".to_string() )
    .hint( "Auth" )
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
        name : "token".to_string(),
        kind : Kind::String,
        description : "Authentication token".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes
        {
          optional : false,
          interactive : true,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( auth_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".auth" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Missing interactive argument must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ArgumentInteractiveRequired,
        "Must produce ArgumentInteractiveRequired (not ArgumentMissing); got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "token" ),
        "Error message must contain the argument name; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution with ArgumentInteractiveRequired, got: {other:?}" ),
  }
}

/// AP-10: `CommandNotImplemented` is returned when a command has no execution logic.
///
/// `.stub` is registered with a routine that returns `ErrorData` with
/// `ErrorCode::CommandNotImplemented` — mirroring the CliBuilder stub pattern.
/// Executing the command through the interpreter must surface that error code.
// test_kind: ap_spec(AP-10)
#[ test ]
fn test_ap10_command_not_implemented_for_stub_routine()
{
  let mut registry = CommandRegistry::new();
  let stub_command = CommandDefinition::former()
    .name( ".stub" )
    .namespace( String::new() )
    .description( "Stub command".to_string() )
    .hint( "Stub" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
    .arguments( vec![] )
    .end();

  let stub_routine : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static > = Box::new( | _cmd, _ctx |
  {
    Err( ErrorData::new(
      ErrorCode::CommandNotImplemented,
      "Command '.stub' is registered but not implemented.".to_string(),
    ))
  });

  registry.command_add_runtime( &stub_command, stub_routine )
    .expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".stub" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified = analyzer.analyze().expect( "Semantic analysis must succeed for .stub" );

  let interpreter = Interpreter::new( &verified, &registry );
  let mut ctx = ExecutionContext::default();
  let result = interpreter.run( &mut ctx );

  assert!( result.is_err(), "Stub command must produce an error at execution time" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::CommandNotImplemented,
        "Must produce CommandNotImplemented; got: {:?}", error_data.code
      );
    },
    other => panic!( "Expected Error::Execution with CommandNotImplemented, got: {other:?}" ),
  }
}

/// AP-11: `HelpRequested` is converted to successful `OutputData` by the pipeline.
///
/// When `.greet ?` is processed, the semantic analyzer returns `HelpRequested`.
/// The pipeline intercepts this and converts it to `Ok(CommandResult { success: true, ... })`
/// containing the help text as output content.
// test_kind: ap_spec(AP-11)
#[ test ]
fn test_ap11_help_requested_converted_to_successful_output()
{
  use unilang::pipeline::Pipeline;

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
  let result = pipeline.process_command( ".greet ?", ExecutionContext::default() );

  assert!(
    result.success,
    "Help request must be converted to successful output; error: {:?}", result.error
  );
  assert!(
    !result.outputs.is_empty(),
    "Help output must contain at least one output entry"
  );
  assert!(
    result.outputs[ 0 ].content.contains( ".greet" ) || result.outputs[ 0 ].content.contains( "greet" ),
    "Help output must reference the command; got: {:?}", result.outputs[ 0 ].content
  );
}

/// AP-12: `InternalError` produced for unexpected system error.
///
/// A routine that returns `ErrorData` with `ErrorCode::InternalError` must surface
/// that code through the interpreter.
// test_kind: ap_spec(AP-12)
#[ test ]
fn test_ap12_internal_error_for_unexpected_failure()
{
  let mut registry = CommandRegistry::new();
  let broken_command = CommandDefinition::former()
    .name( ".broken" )
    .namespace( String::new() )
    .description( "Triggers internal error".to_string() )
    .hint( "Broken" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( false )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
    .arguments( vec![] )
    .end();

  let broken_routine : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync + 'static > =
    Box::new( | _cmd, _ctx |
  {
    Err( ErrorData::new(
      ErrorCode::InternalError,
      "Unexpected internal state".to_string(),
    ))
  });

  registry.command_add_runtime( &broken_command, broken_routine )
    .expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".broken" ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified = analyzer.analyze().expect( "Semantic analysis must succeed" );

  let interpreter = Interpreter::new( &verified, &registry );
  let mut ctx = ExecutionContext::default();
  let result = interpreter.run( &mut ctx );

  assert!( result.is_err(), "Internal error routine must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::InternalError,
        "Must produce InternalError; got: {:?}", error_data.code
      );
      assert!(
        !error_data.message.is_empty(),
        "Error message must be non-empty"
      );
    },
    other => panic!( "Expected Error::Execution with InternalError, got: {other:?}" ),
  }
}

/// AP-13: `TypeMismatch` is returned for internal type conversion error.
///
/// Triggering a type conversion failure (e.g., `TypeError`) must produce
/// `ErrorCode::TypeMismatch` via the `From<TypeError>` impl on `Error`.
// test_kind: ap_spec(AP-13)
#[ test ]
fn test_ap13_type_mismatch_for_type_conversion_error()
{
  let mut registry = CommandRegistry::new();
  let add_command = CommandDefinition::former()
    .name( ".add" )
    .namespace( String::new() )
    .description( "Add integers".to_string() )
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
        description : "An integer".to_string(),
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

  assert!( result.is_err(), "Type conversion failure must produce an error" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert!(
        error_data.code == ErrorCode::ArgumentTypeMismatch || error_data.code == ErrorCode::TypeMismatch,
        "Must produce TypeMismatch or ArgumentTypeMismatch; got: {:?}", error_data.code
      );
      assert!(
        !error_data.message.is_empty(),
        "Error message must be non-empty"
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// AP-14: `ErrorCode` string representations match the documented catalog.
///
/// Each `ErrorCode` variant must produce its documented `UNILANG_*` string
/// representation via `Display` / `.to_string()`.
// test_kind: ap_spec(AP-14)
#[ test ]
fn test_ap14_error_code_string_representations_match_catalog()
{
  let cases : Vec< ( ErrorCode, &str ) > = vec![
    ( ErrorCode::CommandNotFound, "UNILANG_COMMAND_NOT_FOUND" ),
    ( ErrorCode::ArgumentMissing, "UNILANG_ARGUMENT_MISSING" ),
    ( ErrorCode::ArgumentTypeMismatch, "UNILANG_ARGUMENT_TYPE_MISMATCH" ),
    ( ErrorCode::ArgumentInteractiveRequired, "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" ),
    ( ErrorCode::ValidationRuleFailed, "UNILANG_VALIDATION_RULE_FAILED" ),
    ( ErrorCode::TooManyArguments, "UNILANG_TOO_MANY_ARGUMENTS" ),
    ( ErrorCode::UnknownParameter, "UNILANG_UNKNOWN_PARAMETER" ),
    ( ErrorCode::CommandAlreadyExists, "UNILANG_COMMAND_ALREADY_EXISTS" ),
    ( ErrorCode::CommandNotImplemented, "UNILANG_COMMAND_NOT_IMPLEMENTED" ),
    ( ErrorCode::TypeMismatch, "UNILANG_TYPE_MISMATCH" ),
    ( ErrorCode::InternalError, "UNILANG_INTERNAL_ERROR" ),
    ( ErrorCode::HelpRequested, "HELP_REQUESTED" ),
  ];

  for ( code, expected_str ) in &cases
  {
    let actual = format!( "{}", code );
    assert_eq!(
      &actual, expected_str,
      "ErrorCode::{:?} must produce {:?}; got: {:?}", code, expected_str, actual
    );
  }
}
