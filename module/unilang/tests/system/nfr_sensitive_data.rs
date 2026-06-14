//! NFR sensitive data exclusion tests.
//!
//! Implements IN-3 specification case from `tests/docs/invariant/002_non_functional_requirements.md`.
//!
//! Tests verify that sensitive argument values are never exposed in error messages,
//! enforcing NFR-SEC-1: no sensitive data (passwords, API keys, tokens) may appear
//! in formatted error output.
//!
//! ## Implementation Note
//!
//! Sensitive value exclusion is enforced at two points in the semantic analysis pipeline:
//! - **Type coercion path** (`src/semantic/argument_binding.rs`): `coerce_arg_value()` checks
//!   `arg_def.attributes.sensitive` and replaces the raw value with a redaction message.
//! - **Validation path** (`src/semantic/validation.rs`): `format_validation_error()` accepts
//!   a `sensitive` flag and emits `[REDACTED]` instead of the literal value.


use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
use unilang::error::Error;
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// IN-3: Sensitive argument value is absent from error output.
///
/// `.login` is registered with a `password` argument marked `sensitive = true` and
/// typed `Kind::Integer`. Invoking `.login password::s3cr3t` triggers a type-coercion
/// failure (the string "s3cr3t" is not a valid integer). The error message must NOT
/// contain the literal string `"s3cr3t"` under any circumstances.
///
/// ## Why Kind::Integer
///
/// Using `Kind::Integer` ensures the semantic analyzer attempts type coercion and
/// fails cleanly, producing a real `ErrorData`. The exact value `"s3cr3t"` is chosen
/// as the test secret because it is distinctive and unlikely to appear in any error
/// message for other reasons.
// test_kind: in_spec(IN-3)  [invariant/02_non_functional_requirements]
#[ test ]
fn test_in3_sensitive_argument_value_absent_from_error_message()
{
  let mut registry = CommandRegistry::new();
  let login_command = CommandDefinition::former()
    .name( ".login" )
    .description( "Authenticate user".to_string() )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "password".to_string(),
        kind : Kind::Integer, // not a real password kind, but triggers coercion failure
        description : "User password".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes
        {
          optional : false,
          sensitive : true, // marks this argument's value as confidential
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( login_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".login password::"s3cr3t""# ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Type coercion must fail for non-integer password value" );

  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      let message = &error_data.message;
      assert!(
        !message.contains( "s3cr3t" ),
        "Error message must NOT contain the sensitive value 's3cr3t'; got: {:?}", message
      );
      // The argument name is acceptable to include (not sensitive), but not the value
      assert!(
        message.contains( "password" ),
        "Error message may contain the argument name 'password' for diagnostics; got: {:?}", message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// IN-3 (validation path): Sensitive value absent from Pattern validation error.
///
/// Same `.login` shape, but with `Kind::String` and a `Pattern("^\\d+$")` rule.
/// The password "s3cr3t" fails the digits-only pattern. Without the sensitive-redaction
/// fix, `format_validation_error` would embed the literal `"s3cr3t"` in the error.
/// With the fix, `[REDACTED]` appears instead.
///
/// This covers the `format_validation_error` path, complementing the type-coercion
/// path tested by `test_in3_sensitive_argument_value_absent_from_error_message`.
// test_kind: in_spec(IN-3)  [invariant/02_non_functional_requirements]
#[ test ]
fn test_in3_sensitive_value_absent_from_validation_error()
{
  let mut registry = CommandRegistry::new();
  let login_command = CommandDefinition::former()
    .name( ".login" )
    .description( "Authenticate user".to_string() )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "password".to_string(),
        kind : Kind::String,
        description : "User password".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes
        {
          optional : false,
          sensitive : true,
          ..Default::default()
        },
        validation_rules : vec![ unilang::data::ValidationRule::Pattern( r"^\d+$".to_string() ) ],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register( login_command ).expect( "Registration must succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".login password::"s3cr3t""# ).unwrap();
  let instructions = vec![ instruction ];

  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  assert!( result.is_err(), "Pattern(digits-only) must reject 's3cr3t'" );

  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      let message = &error_data.message;
      assert!(
        !message.contains( "s3cr3t" ),
        "Validation error must NOT contain sensitive value 's3cr3t'; got: {:?}", message
      );
      assert!(
        message.contains( "[REDACTED]" ),
        "Validation error must show [REDACTED] for sensitive values; got: {:?}", message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}
