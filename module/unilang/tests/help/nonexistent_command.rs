//! Test for help request on non-existent command
//!
//! ## FR Coverage
//! - FR-HELP-7 (FT-7): `?` operator with unknown command name returns not-found error
//! - FR-HELP-3 (partial): `?` operator is detected even on non-existent commands
//! - FR-HELP-5 (partial): `??` parameter must be quoted to avoid parser error
//!
//! ## Root Cause (from manual testing)
//! Manual testing revealed that requesting help for a non-existent command using
//! incorrect syntax (`.nonexistent ??` instead of `.nonexistent ?`) resulted in
//! a parser error. This test verifies the correct behavior when using proper syntax.
//!
//! ## Why Not Caught
//! No automated test existed for help requests on non-existent commands. Existing
//! help tests (help/operator.rs) only tested help for registered commands.
//!
//! ## Fix Applied
//! Created this test to verify expected behavior with proper help syntax.
//!
//! ## Prevention
//! Test both help syntaxes (`?` operator and `"??"` parameter) for non-existent commands.
//! Document expected behavior clearly in test comments.
//!
//! ## Pitfall
//! Users may confuse the two help syntaxes:
//! - `.command ?` - single `?`, unquoted, final token (FR-HELP-3)
//! - `.command "??"` - double `??`, **must be quoted** (FR-HELP-5)
//!
//! Using `??` without quotes will cause parser error.

use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// FT-7: `?` operator with unknown command name returns not-found message.
// test_kind: ft_spec(FT-7)
#[test]
fn test_help_operator_on_nonexistent_command()
{
  // Create empty registry (no commands registered)
  let registry = CommandRegistry::new();

  // Parse help request for non-existent command using `?` operator (FR-HELP-3)
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".nonexistent ?" ).unwrap();

  // Verify help was requested
  assert!( instruction.help_requested, "Help operator should be detected even for non-existent commands" );

  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Expected behavior: Should return error indicating command not found
  // The help system should not prevent the "command not found" error
  assert!( result.is_err(), "Should return error for non-existent command" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );
  assert!(
    error_msg.contains( "nonexistent" ) || error_msg.contains( "not found" ) || error_msg.contains( "NotFound" ),
    "Error should indicate command doesn't exist, got: {error_msg}"
  );
}

#[test]
fn test_help_parameter_on_nonexistent_command()
{
  // Create empty registry (no commands registered)
  let registry = CommandRegistry::new();

  // Parse help request using `??` parameter (FR-HELP-5)
  // Note: `??` must be quoted to avoid parser conflicts
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".nonexistent "??""# ).unwrap();

  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Expected behavior: Should return error indicating command not found
  // The `??` parameter help system should not prevent the "command not found" error
  assert!( result.is_err(), "Should return error for non-existent command" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );
  assert!(
    error_msg.contains( "nonexistent" ) || error_msg.contains( "not found" ) || error_msg.contains( "NotFound" ),
    "Error should indicate command doesn't exist, got: {error_msg}"
  );
}

#[test]
fn test_unquoted_double_question_mark_fails()
{
  // Verify that using `??` without quotes causes parser error (as per FR-HELP-5)
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_repl_input( ".command ??" );

  // Should fail to parse because `??` must be quoted
  assert!(
    result.is_err(),
    "Unquoted `??` should cause parser error as per FR-HELP-5"
  );

  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!(
    error_msg.contains( "Help" ) || error_msg.contains( "help" ) || error_msg.contains( "last" ) || error_msg.contains( "token" ),
    "Error should mention help operator syntax requirement, got: {error_msg}"
  );
}
