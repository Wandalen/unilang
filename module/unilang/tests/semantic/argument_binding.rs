//! Argument Binding Unit Tests
//!
//! ## Scope
//! Tests the semantic analyzer's ability to bind parsed arguments to command definitions.
//! This covers the critical logic that maps parser output to typed command arguments
//! with validation and type conversion.
//!
//! ## FR Coverage
//! - FR-ARG-3 (FT-1): named `param::value` binding extracts correct typed value
//! - FR-ARG-2 (FT-5): positional binding assigns value by position when no name given
//! - FR-ARG-5 (FT-3): default value is used when optional argument is absent
//! - FR-ARG-1 (FT-6): type coercion converts token into typed `Kind` value
//!
//! ## Coverage
//! - Basic argument binding (named, positional)
//! - Type conversion and validation
//! - Optional and required argument handling
//! - Default value assignment
//! - Validation rule enforcement
//! - Alias resolution and binding
//! - Error conditions and edge cases
//!
//! ## Related
//! - `unit/semantic/multiple_parameters.rs` - Multiple parameter collection
//! - `unit/parser/argument_parsing.rs` - Parser argument extraction
//! - `unit/data/types.rs` - Value types and conversions


use std::collections::HashMap;
use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind, OutputData, ValidationRule };
use unilang::error::Error;
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::interpreter::ExecutionContext;
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Simple test routine for argument binding tests
/// Returns minimal successful output - actual execution not tested here
#[allow(clippy::unnecessary_wraps)]
fn test_routine( _cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData >
{
  Ok( OutputData
  {
    content : "Test executed successfully".to_string(),
    format : "text".to_string(),
      execution_time_ms : None,
  })
}

/// Helper to create command with specific argument configuration
fn create_binding_test_command( name : &str, arguments : Vec< ArgumentDefinition > ) -> CommandDefinition
{
  CommandDefinition::former()
    .name( name )
    .description( "Test command for argument binding validation" )
    .arguments( arguments )
    .end()
}

/// Helper to parse and analyze a command
fn parse_and_bind( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, String >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( input )
    .map_err( |e| format!( "Parse error: {e:?}" ) )?;

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  analyzer.analyze().map_err( |e| format!( "Binding error: {e:?}" ) )
}

/// Helper to parse and analyze a command, preserving the raw `Error` (not stringified).
/// Used where a test must assert on the precise `ErrorCode` rather than a message substring.
fn parse_and_bind_raw( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, Error >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( input ).expect( "Parse should succeed for well-formed test input" );

  let instructions_array = [instruction];
  let analyzer = SemanticAnalyzer::new( &instructions_array, registry );
  analyzer.analyze()
}

/// FT-1: Named binding with `param::value` syntax extracts correct value.
// test_kind: ft_spec(FT-1)  [feature/02_argument_system]
#[test]
fn test_basic_named_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "param".to_string(),
      description : "Test parameter".to_string(),
      kind : Kind::String,
      hint : "String parameter".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test param::"value""# ).expect( "Binding should succeed" );

  assert_eq!( verified_commands.len(), 1 );
  let verified_cmd = &verified_commands[0];

  let param_value = verified_cmd.arguments.get( "param" ).expect( "param should be bound" );
  match param_value {
    Value::String( s ) => assert_eq!( s, "value" ),
    _ => panic!( "Expected String value" ),
  }
}

/// FT-5: Positional binding assigns value by position when no name given.
// test_kind: ft_spec(FT-5)  [feature/02_argument_system]
#[test]
fn test_positional_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "first".to_string(),
      description : "First parameter".to_string(),
      kind : Kind::String,
      hint : "First string".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "second".to_string(),
      description : "Second parameter".to_string(),
      kind : Kind::Integer,
      hint : "Second integer".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test "hello" 42"# ).expect( "Positional binding should succeed" );

  let verified_cmd = &verified_commands[0];

  let first_value = verified_cmd.arguments.get( "first" ).expect( "first should be bound" );
  match first_value {
    Value::String( s ) => assert_eq!( s, "hello" ),
    _ => panic!( "Expected String value for first" ),
  }

  let second_value = verified_cmd.arguments.get( "second" ).expect( "second should be bound" );
  match second_value {
    Value::Integer( i ) => assert_eq!( *i, 42 ),
    _ => panic!( "Expected Integer value for second" ),
  }
}

#[test]
fn test_mixed_named_and_positional_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "pos1".to_string(),
      description : "Positional 1".to_string(),
      kind : Kind::String,
      hint : "Position 1".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "named".to_string(),
      description : "Named parameter".to_string(),
      kind : Kind::String,
      hint : "Named value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    },
    ArgumentDefinition {
      name : "pos2".to_string(),
      description : "Positional 2".to_string(),
      kind : Kind::String,
      hint : "Position 2".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![],
      tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test "first" named::"middle" "last""# ).expect( "Mixed binding should succeed" );

  let verified_cmd = &verified_commands[0];

  let pos1_value = verified_cmd.arguments.get( "pos1" ).unwrap();
  assert_eq!( pos1_value, &Value::String( "first".to_string() ) );

  let named_value = verified_cmd.arguments.get( "named" ).unwrap();
  assert_eq!( named_value, &Value::String( "middle".to_string() ) );

  let pos2_value = verified_cmd.arguments.get( "pos2" ).unwrap();
  assert_eq!( pos2_value, &Value::String( "last".to_string() ) );
}

/// FT-6: Type coercion — integer token parsed into Kind::Integer value.
/// FT-11: Type coercion — float token parsed into Kind::Float value.
// test_kind: ft_spec(FT-6, FT-11)  [feature/02_argument_system]
#[test]
fn test_type_conversion_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "string_val".to_string(),
      description : "String value".to_string(),
      kind : Kind::String,
      hint : "String".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "int_val".to_string(),
      description : "Integer value".to_string(),
      kind : Kind::Integer,
      hint : "Integer".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "bool_val".to_string(),
      description : "Boolean value".to_string(),
      kind : Kind::Boolean,
      hint : "Boolean".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "float_val".to_string(),
      description : "Float value".to_string(),
      kind : Kind::Float,
      hint : "Float".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test string_val::"hello" int_val::42 bool_val::true float_val::3.15"# )
    .expect( "Type conversion binding should succeed" );

  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "string_val" ).unwrap(), &Value::String( "hello".to_string() ) );
  assert_eq!( verified_cmd.arguments.get( "int_val" ).unwrap(), &Value::Integer( 42 ) );
  assert_eq!( verified_cmd.arguments.get( "bool_val" ).unwrap(), &Value::Boolean( true ) );
  assert_eq!( verified_cmd.arguments.get( "float_val" ).unwrap(), &Value::Float( 3.15 ) );
}

#[test]
fn test_optional_argument_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "required".to_string(),
      description : "Required parameter".to_string(),
      kind : Kind::String,
      hint : "Required value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "optional".to_string(),
      description : "Optional parameter".to_string(),
      kind : Kind::String,
      hint : "Optional value".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test with only required argument
  let verified_commands = parse_and_bind( &registry, r#".test required::"value""# ).expect( "Should bind with only required" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "required" ).unwrap(), &Value::String( "value".to_string() ) );
  assert!( !verified_cmd.arguments.contains_key("optional"), "Optional argument should not be present" );

  // Test with both arguments
  let verified_commands = parse_and_bind( &registry, r#".test required::"req" optional::"opt""# ).expect( "Should bind both arguments" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "required" ).unwrap(), &Value::String( "req".to_string() ) );
  assert_eq!( verified_cmd.arguments.get( "optional" ).unwrap(), &Value::String( "opt".to_string() ) );
}

/// FT-3: Default value is used when optional argument is absent.
// test_kind: ft_spec(FT-3)  [feature/02_argument_system]
#[test]
fn test_default_value_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "param".to_string(),
      description : "Parameter with default".to_string(),
      kind : Kind::String,
      hint : "String with default".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        default : Some( "default_value".to_string() ),
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test without providing the parameter (should use default)
  let verified_commands = parse_and_bind( &registry, r".test" ).expect( "Should bind with default value" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "param" ).unwrap(), &Value::String( "default_value".to_string() ) );

  // Test with providing the parameter (should override default)
  let verified_commands = parse_and_bind( &registry, r#".test param::"custom""# ).expect( "Should bind with custom value" );
  let verified_cmd = &verified_commands[0];

  assert_eq!( verified_cmd.arguments.get( "param" ).unwrap(), &Value::String( "custom".to_string() ) );
}

/// FT-8: Alias-based named binding resolves to canonical argument.
// test_kind: ft_spec(FT-8)  [feature/02_argument_system]
#[test]
fn test_alias_binding()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "parameter".to_string(),
      description : "Parameter with aliases".to_string(),
      kind : Kind::String,
      hint : "String parameter".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![],
      aliases : vec![ "param".to_string(), "p".to_string() ],
      tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test binding with canonical name
  let verified_commands = parse_and_bind( &registry, r#".test parameter::"canonical""# ).expect( "Should bind with canonical name" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "canonical".to_string() ) );

  // Test binding with first alias
  let verified_commands = parse_and_bind( &registry, r#".test param::"alias1""# ).expect( "Should bind with first alias" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "alias1".to_string() ) );

  // Test binding with second alias
  let verified_commands = parse_and_bind( &registry, r#".test p::"alias2""# ).expect( "Should bind with second alias" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "parameter" ).unwrap(), &Value::String( "alias2".to_string() ) );
}

/// FT-9: ValidationRule MinLength rejects too-short value.
/// FT-13: ValidationRule Max rejects over-limit integer value.
// test_kind: ft_spec(FT-9, FT-13)  [feature/02_argument_system]
#[test]
fn test_validation_rule_enforcement()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "min_length".to_string(),
      description : "Parameter with minimum length".to_string(),
      kind : Kind::String,
      hint : "String with min length".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![ ValidationRule::MinLength( 5 ) ],
      aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "range_value".to_string(),
      description : "Parameter with range validation".to_string(),
      kind : Kind::Integer,
      hint : "Integer in range".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![ ValidationRule::Min( 1.0 ), ValidationRule::Max( 100.0 ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test valid values (should succeed)
  let result = parse_and_bind( &registry, r#".test min_length::"valid_string" range_value::50"# );
  assert!( result.is_ok(), "Valid values should pass validation" );

  // Test invalid min length (should fail)
  let result = parse_and_bind( &registry, r#".test min_length::"bad" range_value::50"# );
  assert!( result.is_err(), "Too short string should fail validation" );

  // Test invalid range (should fail)
  let result = parse_and_bind( &registry, r#".test min_length::"valid_string" range_value::150"# );
  assert!( result.is_err(), "Out of range value should fail validation" );
}

/// FT-7: Missing required argument produces structured error.
// test_kind: ft_spec(FT-7)  [feature/02_argument_system]
#[test]
fn test_missing_required_argument_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "required".to_string(),
      description : "Required parameter".to_string(),
      kind : Kind::String,
      hint : "Required value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test without required argument (should fail)
  let result = parse_and_bind( &registry, r".test" );
  assert!( result.is_err(), "Missing required argument should fail" );

  let error_message = result.unwrap_err();
  assert!( error_message.contains( "required" ) || error_message.contains( "missing" ), "Error should mention missing required argument" );
}

#[test]
fn test_type_conversion_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "number".to_string(),
      description : "Integer parameter".to_string(),
      kind : Kind::Integer,
      hint : "Integer value".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test with invalid integer value (should fail)
  let result = parse_and_bind( &registry, r#".test number::"not_a_number""# );
  assert!( result.is_err(), "Invalid type conversion should fail" );

  let error_message = result.unwrap_err();
  assert!( error_message.contains( "number" ) || error_message.contains( "integer" ) || error_message.contains( "type" ),
           "Error should mention type conversion issue" );
}

#[test]
fn test_excess_arguments_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "only_arg".to_string(),
      description : "Only argument".to_string(),
      kind : Kind::String,
      hint : "Single string".to_string(),
      attributes : ArgumentAttributes {
        optional : false,
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test with too many positional arguments (should fail)
  let result = parse_and_bind( &registry, r#".test "arg1" "arg2""# );
  assert!( result.is_err(), "Excess arguments should fail" );

  let error_message = result.unwrap_err();
  let error_lower = error_message.to_lowercase();
  assert!( error_lower.contains( "too many" ) || error_lower.contains( "excess" ) || error_lower.contains( "unexpected" ), "Error should mention excess arguments. Got: {error_message}" );
}

#[test]
fn test_binding_performance()
{

  let mut registry = CommandRegistry::new();

  // Create command with many arguments
  let mut arguments = Vec::new();
  for i in 0..50 {
    arguments.push( ArgumentDefinition {
      name : format!( "arg{i}" ),
      description : format!( "Argument {i}" ),
      kind : Kind::String,
      hint : "String argument".to_string(),
      attributes : ArgumentAttributes {
        optional : true,
        default : Some( format!( "default{i}" ) ),
        ..Default::default()
      },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    });
  }

  let cmd = create_binding_test_command( ".perf", arguments );
  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Test with all default values (many arguments to bind)
  let result = parse_and_bind( &registry, r".perf" );

  assert!( result.is_ok(), "Performance test should succeed" );

  let verified_cmd = &result.unwrap()[0];
  assert_eq!( verified_cmd.arguments.len(), 50, "All default arguments should be bound" );
}

/// FT-10: ValidationRule Pattern rejects non-matching value.
// test_kind: ft_spec(FT-10)  [feature/02_argument_system]
#[test]
fn test_pattern_validation_rejects_non_matching()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "email".to_string(),
      description : "Email address".to_string(),
      kind : Kind::String,
      hint : "Valid email".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ ValidationRule::Pattern( r"^[a-z]+@[a-z]+\.[a-z]+$".to_string() ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Valid email passes
  let result = parse_and_bind( &registry, r#".test email::"user@example.com""# );
  assert!( result.is_ok(), "Valid email should pass pattern validation" );

  // Invalid email fails
  let result = parse_and_bind( &registry, r#".test email::"INVALID""# );
  assert!( result.is_err(), "Non-matching value should fail pattern validation" );
}

/// FT-27: ValidationRule::Pattern with a syntactically-invalid regex fails closed, not open.
// test_kind: ft_spec(FT-27)  [feature/02_argument_system]
#[test]
fn test_pattern_validation_malformed_regex_fails_closed()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "code".to_string(),
      description : "Code matching a pattern".to_string(),
      kind : Kind::String,
      hint : "Pattern-validated code".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      // "[unclosed" is not a valid regex — the rule string is never eagerly
      // compile-checked when attached to the argument definition, only at
      // validation time inside `apply_validation_rule`.
      validation_rules : vec![ ValidationRule::Pattern( "[unclosed".to_string() ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // A malformed rule regex must reject the value (fail-closed) rather than
  // silently accept it (fail-open) or panic.
  let result = parse_and_bind( &registry, r#".test code::"anything""# );
  assert!( result.is_err(), "Malformed Pattern rule regex should reject the value, not silently accept it" );
}

/// FT-12: Type coercion — path token parsed into Kind::Path value.
// test_kind: ft_spec(FT-12)  [feature/02_argument_system]
#[test]
fn test_path_type_coercion()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "file".to_string(),
      description : "File path".to_string(),
      kind : Kind::Path,
      hint : "Path to file".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let verified_commands = parse_and_bind( &registry, r#".test file::"/tmp/data.csv""# )
    .expect( "Path coercion should succeed" );

  let verified_cmd = &verified_commands[0];
  let file_value = verified_cmd.arguments.get( "file" ).expect( "file should be bound" );

  match file_value
  {
    Value::Path( p ) => assert_eq!( p.to_string_lossy(), "/tmp/data.csv" ),
    _ => panic!( "Expected Path value, got: {file_value:?}" ),
  }
}

/// FT-14: `Kind::Enum` accepts only predefined choices.
// test_kind: ft_spec(FT-14)  [feature/02_argument_system]
#[test]
fn test_ft14_enum_accepts_only_predefined_choices()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "level".to_string(),
      description : "Severity level".to_string(),
      kind : Kind::Enum( vec![ "low".to_string(), "medium".to_string(), "high".to_string() ] ),
      hint : "One of low/medium/high".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Out-of-choice value is rejected as a type mismatch.
  let result = parse_and_bind_raw( &registry, r#".test level::"extreme""# );
  assert!( result.is_err(), "Value outside the enum choices must fail" );
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

  // In-choice value binds to Value::Enum with no error.
  let verified_commands = parse_and_bind_raw( &registry, r#".test level::"medium""# )
    .expect( "Valid enum choice should bind successfully" );
  let verified_cmd = &verified_commands[0];
  assert_eq!( verified_cmd.arguments.get( "level" ).unwrap(), &Value::Enum( "medium".to_string() ) );
}

/// FT-15: `Kind::File` and `Kind::Directory` validate filesystem existence and category.
// test_kind: ft_spec(FT-15)  [feature/02_argument_system]
#[test]
fn test_ft15_file_and_directory_validate_existence_and_category()
{
  let tmp_dir = tempfile::tempdir().expect( "Should create temp dir" );
  let existing_file = tmp_dir.path().join( "existing.txt" );
  std::fs::write( &existing_file, "content" ).expect( "Should write temp file" );
  let existing_dir = tmp_dir.path().join( "existing_subdir" );
  std::fs::create_dir( &existing_dir ).expect( "Should create temp subdir" );
  let nonexistent = tmp_dir.path().join( "does_not_exist.txt" );

  let mut file_registry = CommandRegistry::new();
  let file_cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "target".to_string(),
      description : "File target".to_string(),
      kind : Kind::File,
      hint : "Path to an existing file".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  file_registry.register_with_routine( &file_cmd, Box::new( test_routine ) ).unwrap();

  // (a) Existing regular file binds Value::File with no error.
  let input_a = format!( r#".test target::"{}""#, existing_file.to_string_lossy() );
  let verified_commands = parse_and_bind_raw( &file_registry, &input_a ).expect( "Existing file should bind" );
  match verified_commands[0].arguments.get( "target" ).unwrap()
  {
    Value::File( p ) => assert_eq!( p, &existing_file ),
    other => panic!( "Expected Value::File, got: {other:?}" ),
  }

  // (b) Existing directory is rejected: expected a file, found a directory.
  let input_b = format!( r#".test target::"{}""#, existing_dir.to_string_lossy() );
  let result_b = parse_and_bind_raw( &file_registry, &input_b );
  assert!( result_b.is_err(), "Directory supplied where File expected must fail" );
  match result_b.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch );
      assert!(
        error_data.message.contains( "directory" ) || error_data.message.contains( "Directory" ),
        "Error should state a directory was found; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }

  // (c) Nonexistent path is rejected: no file found at the path.
  let input_c = format!( r#".test target::"{}""#, nonexistent.to_string_lossy() );
  let result_c = parse_and_bind_raw( &file_registry, &input_c );
  assert!( result_c.is_err(), "Nonexistent path supplied where File expected must fail" );
  match result_c.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }

  // Symmetric behavior for Kind::Directory: file/directory cases reversed.
  let mut dir_registry = CommandRegistry::new();
  let dir_cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "target".to_string(),
      description : "Directory target".to_string(),
      kind : Kind::Directory,
      hint : "Path to an existing directory".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  dir_registry.register_with_routine( &dir_cmd, Box::new( test_routine ) ).unwrap();

  // Existing directory binds Value::Directory with no error.
  let verified_commands = parse_and_bind_raw( &dir_registry, &input_b ).expect( "Existing directory should bind" );
  match verified_commands[0].arguments.get( "target" ).unwrap()
  {
    Value::Directory( p ) => assert_eq!( p, &existing_dir ),
    other => panic!( "Expected Value::Directory, got: {other:?}" ),
  }

  // Existing file is rejected for Kind::Directory: expected a directory, found a file.
  let result_reversed = parse_and_bind_raw( &dir_registry, &input_a );
  assert!( result_reversed.is_err(), "File supplied where Directory expected must fail" );
  match result_reversed.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch );
      assert!(
        error_data.message.contains( "file" ) || error_data.message.contains( "File" ),
        "Error should state a file was found; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-16: `Kind::Url` and `Kind::DateTime` parse into their typed values.
// test_kind: ft_spec(FT-16)  [feature/02_argument_system]
#[test]
fn test_ft16_url_and_datetime_parse_into_typed_values()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "endpoint".to_string(),
      description : "Endpoint URL".to_string(),
      kind : Kind::Url,
      hint : "A URL".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "when".to_string(),
      description : "A timestamp".to_string(),
      kind : Kind::DateTime,
      hint : "An RFC 3339 date-time".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Valid URL and DateTime both parse successfully.
  let verified_commands = parse_and_bind_raw(
    &registry,
    r#".test endpoint::"https://api.example.com/v1" when::"2024-01-15T10:30:00+00:00""#,
  ).expect( "Valid URL and DateTime should bind" );
  let verified_cmd = &verified_commands[0];

  match verified_cmd.arguments.get( "endpoint" ).unwrap()
  {
    Value::Url( u ) =>
    {
      assert_eq!( u.scheme(), "https" );
      assert_eq!( u.host_str(), Some( "api.example.com" ) );
      assert_eq!( u.path(), "/v1" );
    },
    other => panic!( "Expected Value::Url, got: {other:?}" ),
  }
  match verified_cmd.arguments.get( "when" ).unwrap()
  {
    Value::DateTime( dt ) =>
    {
      use chrono::{ Datelike, Timelike };
      assert_eq!( ( dt.year(), dt.month(), dt.day() ), ( 2024, 1, 15 ) );
      assert_eq!( ( dt.hour(), dt.minute(), dt.second() ), ( 10, 30, 0 ) );
    },
    other => panic!( "Expected Value::DateTime, got: {other:?}" ),
  }

  // Malformed URL produces a type-mismatch error, not a panic.
  let mut url_only_registry = CommandRegistry::new();
  let url_only_cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "endpoint".to_string(),
      description : "Endpoint URL".to_string(),
      kind : Kind::Url,
      hint : "A URL".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  url_only_registry.register_with_routine( &url_only_cmd, Box::new( test_routine ) ).unwrap();
  let result_bad_url = parse_and_bind_raw( &url_only_registry, r#".test endpoint::"not a url""# );
  assert!( result_bad_url.is_err(), "Malformed URL must fail, not panic" );
  match result_bad_url.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }

  // Malformed DateTime produces a type-mismatch error, not a panic.
  let mut dt_only_registry = CommandRegistry::new();
  let dt_only_cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "when".to_string(),
      description : "A timestamp".to_string(),
      kind : Kind::DateTime,
      hint : "An RFC 3339 date-time".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  dt_only_registry.register_with_routine( &dt_only_cmd, Box::new( test_routine ) ).unwrap();
  let result_bad_dt = parse_and_bind_raw( &dt_only_registry, r#".test when::"not-a-date""# );
  assert!( result_bad_dt.is_err(), "Malformed DateTime must fail, not panic" );
  match result_bad_dt.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-17: `Kind::Pattern` compiles input into a regular expression value.
// test_kind: ft_spec(FT-17)  [feature/02_argument_system]
#[test]
fn test_ft17_pattern_compiles_into_regex_value()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "regex".to_string(),
      description : "Regex pattern".to_string(),
      kind : Kind::Pattern,
      hint : "A regular expression".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Valid regex source compiles into Value::Pattern whose source matches the input.
  let verified_commands = parse_and_bind_raw( &registry, r#".test regex::"^[a-z]+$""# )
    .expect( "Valid regex should compile" );
  match verified_commands[0].arguments.get( "regex" ).unwrap()
  {
    Value::Pattern( r ) => assert_eq!( r.as_str(), "^[a-z]+$" ),
    other => panic!( "Expected Value::Pattern, got: {other:?}" ),
  }

  // Invalid regex source produces a type-mismatch error, not a panic.
  let result = parse_and_bind_raw( &registry, r#".test regex::"[unclosed""# );
  assert!( result.is_err(), "Invalid regex must fail, not panic" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-18: `Kind::List` and `Kind::Map` parse with default and custom delimiters.
// test_kind: ft_spec(FT-18)  [feature/02_argument_system]
#[test]
fn test_ft18_list_and_map_parse_with_default_and_custom_delimiters()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "tags".to_string(),
      description : "Comma-delimited tags".to_string(),
      kind : Kind::List( Box::new( Kind::String ), None ),
      hint : "List with default delimiter".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "tags2".to_string(),
      description : "Semicolon-delimited tags".to_string(),
      kind : Kind::List( Box::new( Kind::String ), Some( ';' ) ),
      hint : "List with custom delimiter".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    },
    ArgumentDefinition {
      name : "opts".to_string(),
      description : "Key-value options".to_string(),
      kind : Kind::Map( Box::new( Kind::String ), Box::new( Kind::String ), None, None ),
      hint : "Map with default delimiters".to_string(),
      attributes : ArgumentAttributes { optional : true, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Default ',' delimiter produces a correct 3-element list.
  let verified_commands = parse_and_bind_raw( &registry, r#".test tags::"a,b,c""# )
    .expect( "Default-delimiter list should bind" );
  match verified_commands[0].arguments.get( "tags" ).unwrap()
  {
    Value::List( items ) => assert_eq!(
      items,
      &vec![ Value::String( "a".to_string() ), Value::String( "b".to_string() ), Value::String( "c".to_string() ) ]
    ),
    other => panic!( "Expected Value::List, got: {other:?}" ),
  }

  // Custom ';' delimiter produces the same shape of 3-element list.
  let verified_commands2 = parse_and_bind_raw( &registry, r#".test tags2::"a;b;c""# )
    .expect( "Custom-delimiter list should bind" );
  match verified_commands2[0].arguments.get( "tags2" ).unwrap()
  {
    Value::List( items ) => assert_eq!(
      items,
      &vec![ Value::String( "a".to_string() ), Value::String( "b".to_string() ), Value::String( "c".to_string() ) ]
    ),
    other => panic!( "Expected Value::List, got: {other:?}" ),
  }

  // Map with default ',' entry delimiter and '=' key-value delimiter produces a 2-entry map.
  let verified_commands3 = parse_and_bind_raw( &registry, r#".test opts::"k1=v1,k2=v2""# )
    .expect( "Map should bind" );
  match verified_commands3[0].arguments.get( "opts" ).unwrap()
  {
    Value::Map( m ) =>
    {
      assert_eq!( m.len(), 2, "Map should have exactly 2 entries" );
      assert_eq!( m.get( "k1" ), Some( &Value::String( "v1".to_string() ) ) );
      assert_eq!( m.get( "k2" ), Some( &Value::String( "v2".to_string() ) ) );
    },
    other => panic!( "Expected Value::Map, got: {other:?}" ),
  }
}

/// FT-19: `Kind::JsonString` and `Kind::Object` parse and validate JSON payloads.
/// Requires the `json_parser` feature.
// test_kind: ft_spec(FT-19)  [feature/02_argument_system]
#[cfg(feature = "json_parser")]
#[test]
fn test_ft19_jsonstring_and_object_parse_and_validate_json_payloads()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "payload".to_string(),
      description : "Raw JSON string".to_string(),
      kind : Kind::JsonString,
      hint : "A JSON string".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  // Valid JSON binds Value::JsonString with the original text preserved.
  let verified_commands = parse_and_bind_raw( &registry, r#".test payload::"{\"a\":1}""# )
    .expect( "Valid JSON string should bind" );
  match verified_commands[0].arguments.get( "payload" ).unwrap()
  {
    Value::JsonString( s ) => assert_eq!( s, r#"{"a":1}"# ),
    other => panic!( "Expected Value::JsonString, got: {other:?}" ),
  }

  let mut object_registry = CommandRegistry::new();
  let object_cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "data".to_string(),
      description : "Parsed JSON object".to_string(),
      kind : Kind::Object,
      hint : "A JSON object".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);
  object_registry.register_with_routine( &object_cmd, Box::new( test_routine ) ).unwrap();

  // Valid JSON binds Value::Object as a parsed serde_json::Value.
  let verified_commands2 = parse_and_bind_raw( &object_registry, r#".test data::"{\"a\":1}""# )
    .expect( "Valid JSON object should bind" );
  match verified_commands2[0].arguments.get( "data" ).unwrap()
  {
    Value::Object( v ) => assert_eq!( v[ "a" ], 1 ),
    other => panic!( "Expected Value::Object, got: {other:?}" ),
  }

  // Malformed JSON produces a type-mismatch error for both kinds, not a panic.
  let result_json_string = parse_and_bind_raw( &registry, r#".test payload::"{not json}""# );
  assert!( result_json_string.is_err(), "Malformed JSON must fail JsonString binding, not panic" );
  match result_json_string.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }

  let result_object = parse_and_bind_raw( &object_registry, r#".test data::"{not json}""# );
  assert!( result_object.is_err(), "Malformed JSON must fail Object binding, not panic" );
  match result_object.unwrap_err()
  {
    Error::Execution( error_data ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-20: `ValidationRule::Min` rejects an under-limit numeric value.
// test_kind: ft_spec(FT-20)  [feature/02_argument_system]
#[test]
fn test_ft20_validation_rule_min_rejects_under_limit_value()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "age".to_string(),
      description : "Age in years".to_string(),
      kind : Kind::Integer,
      hint : "Non-negative age".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ ValidationRule::Min( 0.0 ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let result = parse_and_bind_raw( &registry, r#".test age::"-1""# );
  assert!( result.is_err(), "Value below the minimum must fail validation" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ValidationRuleFailed,
        "Must produce ValidationRuleFailed; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "minimum" ),
        "Error should mention the minimum constraint; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-21: `ValidationRule::MaxLength` rejects a too-long string value.
// test_kind: ft_spec(FT-21)  [feature/02_argument_system]
#[test]
fn test_ft21_validation_rule_maxlength_rejects_too_long_value()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "code".to_string(),
      description : "Short code".to_string(),
      kind : Kind::String,
      hint : "At most 4 characters".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ ValidationRule::MaxLength( 4 ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let result = parse_and_bind_raw( &registry, r#".test code::"abcdef""# );
  assert!( result.is_err(), "Value exceeding the maximum length must fail validation" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ValidationRuleFailed,
        "Must produce ValidationRuleFailed; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "maximum" ),
        "Error should mention the maximum length constraint; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-22: `ValidationRule::MinItems` rejects a list with too few elements.
// test_kind: ft_spec(FT-22)  [feature/02_argument_system]
#[test]
fn test_ft22_validation_rule_minitems_rejects_too_few_elements()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "tags".to_string(),
      description : "At least 2 tags".to_string(),
      kind : Kind::List( Box::new( Kind::String ), None ),
      hint : "A list with at least 2 items".to_string(),
      attributes : ArgumentAttributes { optional : false, ..Default::default() },
      validation_rules : vec![ ValidationRule::MinItems( 2 ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let result = parse_and_bind_raw( &registry, r#".test tags::"solo""# );
  assert!( result.is_err(), "List with fewer than the minimum items must fail validation" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ValidationRuleFailed,
        "Must produce ValidationRuleFailed; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "minimum required 2 items" ),
        "Error should mention the minimum items required; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-23: Sensitive argument attribute redacts the value in validation error messages.
///
/// Note on rule choice: the spec's example rule is `ValidationRule::MinLength(8)`, but the
/// real `format_validation_error()` implementation (`src/semantic/validation.rs`) only
/// interpolates the (possibly redacted) `value_str` into the message for the `Min`, `Max`,
/// and `Pattern` rule variants — `MinLength`/`MaxLength`/`MinItems` messages report only
/// numeric lengths/counts and never reference `value_str` (so they never leak the raw value,
/// but they also never emit the literal `"[REDACTED]"` marker). `Pattern` is used here instead
/// so the test can genuinely observe the documented `"[REDACTED]"` marker while still exercising
/// the same `sensitive` redaction code path and the same `UNILANG_VALIDATION_RULE_FAILED` code.
// test_kind: ft_spec(FT-23)  [feature/02_argument_system]
#[test]
fn test_ft23_sensitive_attribute_redacts_value_in_validation_error()
{
  let mut registry = CommandRegistry::new();

  let raw_value = "abc";
  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "password".to_string(),
      description : "Account password".to_string(),
      kind : Kind::String,
      hint : "Must match the required password pattern".to_string(),
      attributes : ArgumentAttributes { optional : false, sensitive : true, ..Default::default() },
      validation_rules : vec![ ValidationRule::Pattern( r"^[a-z]{8,}$".to_string() ) ],
      aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let result = parse_and_bind_raw( &registry, &format!( r#".test password::"{raw_value}""# ) );
  assert!( result.is_err(), "Too-short sensitive password must fail validation" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ValidationRuleFailed,
        "Must produce ValidationRuleFailed; got: {:?}", error_data.code
      );
      assert!(
        error_data.message.contains( "[REDACTED]" ),
        "Error message must contain a redaction marker; got: {:?}", error_data.message
      );
      assert!(
        !error_data.message.contains( raw_value ),
        "Error message must NOT contain the literal raw sensitive value '{}'; got: {:?}", raw_value, error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-24: Interactive argument attribute signals a distinct error instead of a missing-argument failure.
// test_kind: ft_spec(FT-24)  [feature/02_argument_system]
#[test]
fn test_ft24_interactive_attribute_signals_distinct_error()
{
  let mut registry = CommandRegistry::new();

  let cmd = create_binding_test_command( ".test", vec![
    ArgumentDefinition {
      name : "token".to_string(),
      description : "Auth token".to_string(),
      kind : Kind::String,
      hint : "Provided interactively".to_string(),
      attributes : ArgumentAttributes { optional : false, interactive : true, ..Default::default() },
      validation_rules : vec![], aliases : vec![], tags : vec![],
    }
  ]);

  registry.register_with_routine( &cmd, Box::new( test_routine ) ).unwrap();

  let result = parse_and_bind_raw( &registry, ".test" );
  assert!( result.is_err(), "Missing required interactive argument must fail" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!(
        error_data.code,
        ErrorCode::ArgumentInteractiveRequired,
        "Must produce ArgumentInteractiveRequired (not ArgumentMissing); got: {:?}", error_data.code
      );
      assert_ne!(
        error_data.code,
        ErrorCode::ArgumentMissing,
        "Interactive-required error must be distinct from the generic ArgumentMissing code"
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FT-25: `VerifiedCommand` typed extraction methods retrieve, coerce-check, and report missing arguments.
// test_kind: ft_spec(FT-25)  [feature/02_argument_system]
#[test]
fn test_ft25_verified_command_typed_extraction_methods()
{
  let mut arguments = HashMap::new();
  arguments.insert( "name".to_string(), Value::String( "Alice".to_string() ) );
  arguments.insert( "count".to_string(), Value::Integer( 3 ) );

  let verified_cmd = VerifiedCommand
  {
    definition : create_binding_test_command( ".test", vec![] ),
    arguments,
  };

  // "name": correct-type accessors succeed.
  assert_eq!( verified_cmd.get_string( "name" ), Some( "Alice" ) );
  assert_eq!( verified_cmd.require_string( "name" ).unwrap(), "Alice" );
  assert!( verified_cmd.has_argument( "name" ) );
  assert_eq!( verified_cmd.get_value( "name" ), Some( &Value::String( "Alice".to_string() ) ) );

  // "count": wrong-type access for get_string/require_string.
  assert_eq!( verified_cmd.get_string( "count" ), None, "Wrong-type get_* must return None" );
  match verified_cmd.require_string( "count" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  // Correct-type accessor for "count" succeeds.
  assert_eq!( verified_cmd.get_integer( "count" ), Some( 3 ) );
  assert_eq!( verified_cmd.require_integer( "count" ).unwrap(), 3 );

  // "missing": every get_* returns None, every require_* returns Err(ArgumentTypeMismatch).
  assert_eq!( verified_cmd.get_string( "missing" ), None );
  assert_eq!( verified_cmd.get_integer( "missing" ), None );
  assert_eq!( verified_cmd.get_float( "missing" ), None );
  assert_eq!( verified_cmd.get_boolean( "missing" ), None );
  assert_eq!( verified_cmd.get_path( "missing" ), None );
  assert_eq!( verified_cmd.get_list( "missing" ), None );

  match verified_cmd.require_string( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  match verified_cmd.require_integer( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  match verified_cmd.require_float( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  match verified_cmd.require_boolean( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  match verified_cmd.require_path( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }
  match verified_cmd.require_list( "missing" )
  {
    Err( Error::Execution( error_data ) ) => assert_eq!( error_data.code, ErrorCode::ArgumentTypeMismatch ),
    other => panic!( "Expected Err(Error::Execution) with ArgumentTypeMismatch, got: {other:?}" ),
  }

  assert!( !verified_cmd.has_argument( "missing" ) );
  assert_eq!( verified_cmd.get_value( "missing" ), None );
}

/// FT-26: Normalized string extraction trims surrounding whitespace.
// test_kind: ft_spec(FT-26)  [feature/02_argument_system]
#[test]
fn test_ft26_normalized_string_extraction_trims_whitespace()
{
  let mut padded_args = HashMap::new();
  padded_args.insert( "name".to_string(), Value::String( "  Alice  ".to_string() ) );
  let padded_cmd = VerifiedCommand
  {
    definition : create_binding_test_command( ".test", vec![] ),
    arguments : padded_args,
  };

  assert_eq!( padded_cmd.get_string_normalized( "name" ), Some( "Alice" ) );
  assert_eq!( padded_cmd.require_string_normalized( "name" ).unwrap(), "Alice" );

  // Whitespace-only value normalizes to Some("")/Ok("") — not None/error.
  let mut whitespace_args = HashMap::new();
  whitespace_args.insert( "name".to_string(), Value::String( "   ".to_string() ) );
  let whitespace_cmd = VerifiedCommand
  {
    definition : create_binding_test_command( ".test", vec![] ),
    arguments : whitespace_args,
  };

  assert_eq!( whitespace_cmd.get_string_normalized( "name" ), Some( "" ) );
  assert_eq!( whitespace_cmd.require_string_normalized( "name" ).unwrap(), "" );
}