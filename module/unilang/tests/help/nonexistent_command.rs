//! Test for help request on non-existent command
//!
//! ## FR Coverage
//! - FR-HELP-7 (FT-7): `??` with unknown command name returns not-found error
//! - FR-HELP-3 (partial): unquoted `??` parses as a plain positional token
//! - FR-HELP-5 (partial): quoted `"??"` is a literal value, never a help trigger
//!
//! ## Root Cause (from manual testing)
//! Manual testing of the pre-`??` design revealed inconsistent behavior for help
//! requests on unknown commands. Under the current design there is a single help
//! token (`??`, unquoted) and help detection runs only after successful command
//! lookup — an unknown command always surfaces as "command not found", never as
//! a parser error and never as a help page for nothing.
//!
//! ## Why Not Caught
//! No automated test existed for help requests on non-existent commands. Existing
//! help tests (help/operator.rs) only tested help for registered commands.
//!
//! ## Fix Applied
//! Created this test to verify expected behavior for unknown commands with both
//! unquoted `??` (help intent) and quoted `"??"` (literal value).
//!
//! ## Prevention
//! Test both spellings (`??` unquoted and `"??"` quoted) against unknown
//! commands. Document expected behavior clearly in test comments.
//!
//! ## Pitfall
//! Quoting flips the meaning:
//! - `.command ??` - unquoted, requests help for `.command`
//! - `.command "??"` - quoted, passes the literal string `??` as a value
//!
//! Neither form is a parser error; unknown commands fail at semantic lookup.

use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

/// FT-7: `??` with unknown command name returns not-found message.
// test_kind: ft_spec(FT-7)  [feature/04_help_system]
#[test]
fn test_help_token_on_nonexistent_command()
{
  // Create empty registry (no commands registered)
  let registry = CommandRegistry::new();

  // Parse help request for non-existent command using unquoted `??`
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".nonexistent ??" ).unwrap();

  // Verify the help token arrived as an unquoted positional argument
  assert!(
    instruction.positional_arguments.iter().any( | arg | arg.value == "??" && !arg.was_quoted ),
    "Help token should parse as an unquoted positional argument"
  );

  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Expected behavior: Should return error indicating command not found.
  // Help detection runs only after successful command lookup, so an unknown
  // command is reported as not found — never rendered as a help page.
  assert!( result.is_err(), "Should return error for non-existent command" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );
  assert!(
    error_msg.contains( "nonexistent" ) || error_msg.contains( "not found" ) || error_msg.contains( "NotFound" ),
    "Error should indicate command doesn't exist, got: {error_msg}"
  );
}

#[test]
fn test_quoted_help_token_on_nonexistent_command()
{
  // Create empty registry (no commands registered)
  let registry = CommandRegistry::new();

  // Quoted "??" is a literal value, not a help request — but the command
  // still doesn't exist, so lookup fails the same way.
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".nonexistent "??""# ).unwrap();

  // Run semantic analysis
  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Expected behavior: Should return error indicating command not found
  assert!( result.is_err(), "Should return error for non-existent command" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );
  assert!(
    error_msg.contains( "nonexistent" ) || error_msg.contains( "not found" ) || error_msg.contains( "NotFound" ),
    "Error should indicate command doesn't exist, got: {error_msg}"
  );
}

#[test]
fn test_unquoted_help_token_parses_cleanly()
{
  // Unquoted `??` is ordinary token material for the parser — help routing is
  // a semantic-stage concern, so parsing must succeed anywhere it appears.
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_repl_input( ".command ??" );

  assert!(
    result.is_ok(),
    "Unquoted `??` must parse as a plain positional token, got: {:?}",
    result.err()
  );

  let instruction = result.unwrap();
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "??" );
  assert!( !instruction.positional_arguments[ 0 ].was_quoted );
}
