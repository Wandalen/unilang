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
//! ## Pitfall
//! When designing validation logic, checking for user errors (unknown parameters)
//! should happen BEFORE checking for missing required data. This provides better
//! user experience by surfacing the actual mistake (typo) rather than downstream
//! effects (missing required arg).

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

#[test]
fn test_typo_suggestion_for_required_parameter()
{
  // Create command with required parameter "file"
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "load".to_string(),
    namespace: String::new(),
    description: "Load a file".to_string(),
    hint: "Load file".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "file".to_string(),
        description: "Path to file".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes
        {
          optional: false, // Required argument
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "File path".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // User makes typo: "fiel" instead of "file"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".load fiel::"data.txt""# ).unwrap();

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion, NOT "missing argument" error
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      // Verify we get typo suggestion
      assert!(
        error_data.message.contains( "fiel" ) && error_data.message.contains( "Did you mean" ) && error_data.message.contains( "file" ),
        "Should suggest correct parameter name, got: {}",
        error_data.message
      );

      // Verify we DON'T get generic "missing argument" error
      assert!(
        !error_data.message.contains( "missing" ),
        "Should not show 'missing argument' error when parameter is typo, got: {}",
        error_data.message
      );
    },
    unilang::error::Error::Parse( _ ) => panic!( "Should get semantic error, not parse error" ),
  }
}

#[test]
fn test_typo_suggestion_for_optional_parameter()
{
  // Create command with optional parameter "verbose"
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "build".to_string(),
    namespace: String::new(),
    description: "Build project".to_string(),
    hint: "Build".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "verbose".to_string(),
        description: "Verbose output".to_string(),
        kind: Kind::Boolean,
        attributes: ArgumentAttributes
        {
          optional: true, // Optional argument
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Verbose flag".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // User makes typo: "verbos" instead of "verbose"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".build verbos::true"# ).unwrap();

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      // Verify we get typo suggestion
      assert!(
        error_data.message.contains( "verbos" ) && error_data.message.contains( "Did you mean" ) && error_data.message.contains( "verbose" ),
        "Should suggest correct parameter name, got: {}",
        error_data.message
      );
    },
    unilang::error::Error::Parse( _ ) => panic!( "Should get semantic error, not parse error" ),
  }
}

#[test]
fn test_typo_suggestion_with_multiple_parameters()
{
  // Create command with multiple parameters
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "copy".to_string(),
    namespace: String::new(),
    description: "Copy files".to_string(),
    hint: "Copy".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "source".to_string(),
        description: "Source path".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Source".to_string(),
        aliases: vec![],
        tags: vec![],
      },
      ArgumentDefinition
      {
        name: "destination".to_string(),
        description: "Destination path".to_string(),
        kind: Kind::String,
        attributes: ArgumentAttributes
        {
          optional: false,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Destination".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // User makes typo in one parameter: "sorce" instead of "source"
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".copy sorce::"a.txt" destination::"b.txt""# ).unwrap();

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get typo suggestion for the typo, not "missing source" error
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      // Verify we get typo suggestion
      assert!(
        error_data.message.contains( "sorce" ) && error_data.message.contains( "Did you mean" ) && error_data.message.contains( "source" ),
        "Should suggest correct parameter name, got: {}",
        error_data.message
      );

      // Should NOT complain about missing "source" parameter
      assert!(
        !error_data.message.contains( "missing" ),
        "Should not show 'missing' error when parameter is typo, got: {}",
        error_data.message
      );
    },
    unilang::error::Error::Parse( _ ) => panic!( "Should get semantic error, not parse error" ),
  }
}

#[test]
fn test_no_suggestion_for_distant_typo()
{
  // Create command with parameter "verbose"
  let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name: "test".to_string(),
    namespace: String::new(),
    description: "Test command".to_string(),
    hint: "Test".to_string(),
    arguments: vec![
      ArgumentDefinition
      {
        name: "verbose".to_string(),
        description: "Verbose output".to_string(),
        kind: Kind::Boolean,
        attributes: ArgumentAttributes
        {
          optional: true,
          ..Default::default()
        },
        validation_rules: vec![],
        hint: "Verbose flag".to_string(),
        aliases: vec![],
        tags: vec![],
      }
    ],
    routine_link: None,
    auto_help_enabled: false,
    status: "stable".to_string(),
    version: "1.0.0".to_string(),
    tags: vec![],
    aliases: vec![],
    permissions: vec![],
    idempotent: false,
    deprecation_message: String::new(),
    http_method_hint: String::new(),
    examples: vec![],
  });

  // User provides completely different parameter (not a typo)
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( r#".test foo::true"# ).unwrap();

  let instructions = vec![instruction];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let result = analyzer.analyze();

  // Should get error but NO suggestion (Levenshtein distance too high)
  assert!( result.is_err(), "Should return error for unknown parameter" );

  let error = result.unwrap_err();
  match error
  {
    unilang::error::Error::Execution( error_data ) =>
    {
      // Should mention the unknown parameter
      assert!(
        error_data.message.contains( "foo" ),
        "Should mention unknown parameter, got: {}",
        error_data.message
      );

      // Should NOT suggest "verbose" (too different)
      assert!(
        !error_data.message.contains( "Did you mean" ),
        "Should not suggest when typo is too different, got: {}",
        error_data.message
      );
    },
    unilang::error::Error::Parse( _ ) => panic!( "Should get semantic error, not parse error" ),
  }
}
