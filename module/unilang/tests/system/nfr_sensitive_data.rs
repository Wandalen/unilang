//! NFR sensitive data exclusion tests.
//!
//! Implements IN-3 specification case from `tests/docs/invariant/02_non_functional_requirements.md`.
//!
//! Tests verify that sensitive argument values are never exposed in error messages,
//! enforcing NFR-SEC-1: no sensitive data (passwords, API keys, tokens) may appear
//! in formatted error output.
//!
//! ## Implementation Note
//!
//! Sensitive value exclusion is enforced in `src/semantic/argument_binding.rs` via
//! `coerce_arg_value()`, which checks `arg_def.attributes.sensitive` and replaces the
//! raw value with a redaction message in the error string.

#![ allow( deprecated ) ]

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
// test_kind: in_spec(IN-3)
#[ test ]
fn test_in3_sensitive_argument_value_absent_from_error_message()
{
  let mut registry = CommandRegistry::new();
  let login_command = CommandDefinition::former()
    .name( ".login" )
    .namespace( String::new() )
    .description( "Authenticate user".to_string() )
    .hint( "Login" )
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
