//! Parameter Typo Suggestion Tests
//!
//! ## Root Cause (from manual testing - Issue #2)
//! Manual testing revealed that when a user provides an unknown parameter that's
//! a typo of a required parameter, the system was showing "The required argument 'parameter'
//! is missing" instead of "Unknown parameter 'paramter'. Did you mean 'parameter'?"
//!
//! The root cause was argument validation order in `semantic.rs::bind_arguments()`:
//! - OLD: Check each arg → if missing, error immediately → never reach unknown param check
//! - NEW: Collect all binding attempts → check unknown params FIRST → then check missing
//!
//! ## Why Not Caught
//! No automated test verified typo suggestions when the misspelled parameter was for a
//! **required** argument. Existing tests only checked optional parameters or provided
//! all required arguments correctly.
//!
//! ## Fix Applied
//! Restructured `bind_arguments()` in `src/semantic.rs` to use three-pass validation:
//! 1. Pass 1: Try to bind all arguments (collect missing, don't error yet)
//! 2. Pass 2: Check for unknown parameters FIRST (provides "Did you mean" suggestions)
//! 3. Pass 3: Handle missing required arguments
//!
//! This ordering ensures users get helpful typo suggestions instead of generic
//! "missing argument" errors when they make typos.
//!
//! ## Prevention
//! Test parameter typo suggestions for both optional and required arguments.
//! Verify unknown parameter check happens BEFORE missing argument check.
//!
//! ## FR Coverage
//! - FR-ARG-8 (FT-2): unknown parameter with close Levenshtein distance produces suggestion
//!   (specifically: covers required-argument typos that previously showed "missing arg" instead)
//!
//! ## Pitfall
//! When designing validation logic, checking for user errors (unknown parameters)
//! should happen BEFORE checking for missing required data. This provides better
//! user experience by surfacing the actual mistake (typo) rather than downstream
//! effects (missing required arg).

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

fn create_load_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".load" )
    .description( "Load a file" )
    .hint( "Load file" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "file" )
        .description( "Path to file" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes { optional: false, ..Default::default() } )
        .hint( "File path" )
        .end(),
    ])
    .end()
}

fn create_build_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".build" )
    .description( "Build project" )
    .hint( "Build" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "verbose" )
        .description( "Verbose output" )
        .kind( Kind::Boolean )
        .attributes( ArgumentAttributes { optional: true, ..Default::default() } )
        .hint( "Verbose flag" )
        .end(),
    ])
    .end()
}

fn create_copy_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".copy" )
    .description( "Copy files" )
    .hint( "Copy" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "source" )
        .description( "Source path" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes { optional: false, ..Default::default() } )
        .hint( "Source" )
        .end(),
      ArgumentDefinition::former()
        .name( "destination" )
        .description( "Destination path" )
        .kind( Kind::String )
        .attributes( ArgumentAttributes { optional: false, ..Default::default() } )
        .hint( "Destination" )
        .end(),
    ])
    .end()
}

fn create_verbose_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .hint( "Test" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition::former()
        .name( "verbose" )
        .description( "Verbose output" )
        .kind( Kind::Boolean )
        .attributes( ArgumentAttributes { optional: true, ..Default::default() } )
        .hint( "Verbose flag" )
        .end(),
    ])
    .end()
}

/// FT-2: Unknown parameter produces error with Levenshtein suggestion.
// test_kind: ft_spec(FT-2)  [feature/02_argument_system]
#[test]
fn test_typo_suggestion_for_required_parameter()
{
  // Create command with required parameter "file"
  let mut registry = CommandRegistry::new();
  registry.register( create_load_command() ).expect( "Registration should succeed" );

  // User makes typo: "fiel" instead of "file"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".load fiel::"data.txt""# ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion, NOT "missing argument" error
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error_msg = format!( "{:?}", result.unwrap_err() );

  // Verify we get typo suggestion
  assert!(
    error_msg.contains( "fiel" ) && error_msg.contains( "Did you mean" ) && error_msg.contains( "file" ),
    "Should suggest correct parameter name, got: {error_msg}"
  );

  // Verify we DON'T get generic "missing argument" error
  assert!(
    !error_msg.contains( "missing" ),
    "Should not show 'missing argument' error when parameter is typo, got: {error_msg}"
  );
}

#[test]
fn test_typo_suggestion_for_optional_parameter()
{
  // Create command with optional parameter "verbose"
  let mut registry = CommandRegistry::new();
  registry.register( create_build_command() ).expect( "Registration should succeed" );

  // User makes typo: "verbos" instead of "verbose"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".build verbos::true"# ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error_msg = format!( "{:?}", result.unwrap_err() );

  // Verify we get typo suggestion
  assert!(
    error_msg.contains( "verbos" ) && error_msg.contains( "Did you mean" ) && error_msg.contains( "verbose" ),
    "Should suggest correct parameter name, got: {error_msg}"
  );
}

#[test]
fn test_typo_suggestion_with_multiple_parameters()
{
  // Create command with multiple parameters
  let mut registry = CommandRegistry::new();
  registry.register( create_copy_command() ).expect( "Registration should succeed" );

  // User makes typo in one parameter: "sorce" instead of "source"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".copy sorce::"a.txt" destination::"b.txt""# ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion for the typo, not "missing source" error
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error_msg = format!( "{:?}", result.unwrap_err() );

  // Verify we get typo suggestion
  assert!(
    error_msg.contains( "sorce" ) && error_msg.contains( "Did you mean" ) && error_msg.contains( "source" ),
    "Should suggest correct parameter name, got: {error_msg}"
  );

  // Should NOT complain about missing "source" parameter
  assert!(
    !error_msg.contains( "missing" ),
    "Should not show 'missing' error when parameter is typo, got: {error_msg}"
  );
}

#[test]
fn test_no_suggestion_for_distant_typo()
{
  // Create command with parameter "verbose"
  let mut registry = CommandRegistry::new();
  registry.register( create_verbose_command() ).expect( "Registration should succeed" );

  // User provides completely different parameter (not a typo)
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".test foo::true"# ).unwrap();

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get error but NO suggestion (Levenshtein distance too high)
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error_msg = format!( "{:?}", result.unwrap_err() );

  // Should mention the unknown parameter
  assert!(
    error_msg.contains( "foo" ),
    "Should mention unknown parameter, got: {error_msg}"
  );

  // Should NOT suggest "verbose" (too different)
  assert!(
    !error_msg.contains( "Did you mean" ),
    "Should not suggest when typo is too different, got: {error_msg}"
  );
}

/// The help hint in unknown-parameter errors uses the full invocable command
/// name with a single leading dot — never the historical `..cmd` double-dot.
#[test]
fn test_help_hint_uses_full_command_name()
{
  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".fetch" )
    .namespace( ".net" )
    .description( "Fetch a resource" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "url".to_string(),
        description : "Resource URL".to_string(),
        kind : Kind::String,
        hint : String::new(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();
  registry.register( cmd ).expect( "Registration should succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( ".net.fetch bogus::x" ).unwrap();
  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let error_msg = format!( "{:?}", analyzer.analyze().unwrap_err() );

  assert!(
    error_msg.contains( "'.net.fetch ??'" ),
    "Hint must use the full invocable name; got: {error_msg}"
  );
  assert!(
    !error_msg.contains( "..net" ) && !error_msg.contains( "..fetch" ),
    "Hint must not render a double dot; got: {error_msg}"
  );
}
