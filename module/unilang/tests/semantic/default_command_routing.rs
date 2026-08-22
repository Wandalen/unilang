//! FR-REG-10 (Default Command): empty-path routing to a configured default command.
//!
//! ## Scope
//! Tests the semantic analyzer's opt-in default-command fallback: an instruction with an
//! empty command path (no dot-prefixed token resolved into `command_path_slices`) but at
//! least one argument routes to a registry-configured default command instead of the
//! unconditional unknown-parameter rejection `issue-003` established.
//!
//! ## FR Coverage
//! - FR-REG-10 (FT-26): configured default receives empty-path invocations with arguments;
//!   unconfigured registries and explicit paths are unaffected
//!
//! ## Related
//! - `tests/semantic/empty_path_named_argument.rs` - the unconfigured-registry regression guard
//!   this feature must leave completely intact (see `test_empty_path_...unchanged_behavior` below)
//! - `tests/registry/default_command.rs` - registry-level getter/setter/builder validation


use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, ErrorCode, Kind };
use unilang::error::Error;
use unilang::registry::CommandRegistry;
use unilang::semantic::{ SemanticAnalyzer, VerifiedCommand };
use unilang::types::Value;
use unilang_parser::{ Parser, UnilangParserOptions };

/// Builds `.report`, taking one optional `Boolean` argument `dry`.
fn report_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".report" )
    .description( "Test command used as a default-command routing target" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "dry".to_string(),
        kind : Kind::Boolean,
        description : "Dry-run flag".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Builds `.other`, taking one optional `String` argument `label` — distinct from `.report`,
/// used to prove an explicit command path is never redirected to the configured default.
fn other_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".other" )
    .description( "Test command with its own distinct argument, unrelated to the default" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "label".to_string(),
        kind : Kind::String,
        description : "Label value".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end()
}

/// Parses `input` and runs semantic analysis, preserving the raw `Error`.
fn parse_and_analyze( registry : &CommandRegistry, input : &str ) -> Result< Vec< VerifiedCommand >, Error >
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( input ).expect( "Parse should succeed for well-formed test input" );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, registry );
  analyzer.analyze()
}

/// FR-REG-10 (FT-26): An empty-path invocation with an argument the default command declares
/// routes to that command and binds the argument normally.
///
/// `"dry::true"` (no leading dot at all) reproduces the exact originating scenario — a bare
/// `name::value` token with no command word, e.g. `wpublish dry::1` — where the parser's
/// `name::value` lookahead excludes the token from `command_path_slices` entirely.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_empty_path_with_args_routes_to_configured_default_command()
{
  let mut registry = CommandRegistry::new();
  registry.register( report_command() ).expect( "Registration should succeed" );
  registry.set_default_command( ".report" ).expect( "Valid default command name should be accepted" );

  let parser = Parser::new( UnilangParserOptions::default() );
  let instruction = parser.parse_repl_input( "dry::true" ).expect( "Parse should succeed" );
  assert!(
    instruction.command_path_slices.is_empty(),
    "Precondition failed: expected empty command_path_slices, got {:?}",
    instruction.command_path_slices
  );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );
  let verified_commands = analyzer.analyze().expect( "Empty path with a default configured should route successfully" );

  assert_eq!( verified_commands.len(), 1 );
  assert_eq!( verified_commands[ 0 ].definition.full_name(), ".report", "Should route to the configured default command" );
  assert_eq!( verified_commands[ 0 ].arguments.get( "dry" ), Some( &Value::Boolean( true ) ) );
}

/// FR-REG-10 (FT-26): Routing to the default command does not bypass FR-ARG-8 unknown-parameter
/// validation — an argument the default command doesn't declare is still rejected.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_empty_path_with_unknown_argument_against_default_still_rejected()
{
  let mut registry = CommandRegistry::new();
  registry.register( report_command() ).expect( "Registration should succeed" );
  registry.set_default_command( ".report" ).expect( "Valid default command name should be accepted" );

  let result = parse_and_analyze( &registry, "typo_param::1" );

  assert!( result.is_err(), "An argument unknown to the routed default command must still be rejected" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::UnknownParameter, "Must produce UnknownParameter; got: {:?}", error_data.code );
      assert!(
        error_data.message.contains( "typo_param" ),
        "Error message must name the unknown parameter; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FR-REG-10 (FT-26): An instruction that resolves an explicit, non-empty command path is
/// never redirected to the configured default, regardless of configuration.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_explicit_command_path_not_affected_by_configured_default()
{
  let mut registry = CommandRegistry::new();
  registry.register( report_command() ).expect( "Registration should succeed" );
  registry.register( other_command() ).expect( "Registration should succeed" );
  registry.set_default_command( ".report" ).expect( "Valid default command name should be accepted" );

  let verified_commands = parse_and_analyze( &registry, r#".other label::"x""# )
    .expect( "Explicit command path should resolve normally" );

  assert_eq!( verified_commands.len(), 1 );
  assert_eq!(
    verified_commands[ 0 ].definition.full_name(), ".other",
    "Explicit path must resolve to the command actually named, never the configured default"
  );
  assert_eq!( verified_commands[ 0 ].arguments.get( "label" ), Some( &Value::String( "x".to_string() ) ) );
}

/// FR-REG-10 (FT-26): A registry that never calls `set_default_command` reproduces the
/// pre-existing, unconfigured `issue-003` rejection unchanged — this feature is strictly opt-in.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_empty_path_with_args_unconfigured_registry_unchanged_behavior()
{
  let mut registry = CommandRegistry::new();
  registry.register( report_command() ).expect( "Registration should succeed" );
  // Deliberately not calling set_default_command.

  let result = parse_and_analyze( &registry, "dry::true" );

  assert!( result.is_err(), "Empty path with arguments and no configured default must still be rejected" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::UnknownParameter, "Must produce UnknownParameter; got: {:?}", error_data.code );
      assert!(
        error_data.message.contains( "No command was specified to validate it against" ),
        "Error message must match the pre-existing empty-path rejection wording; got: {:?}", error_data.message
      );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FR-REG-10: A default command name that was never actually registered surfaces the ordinary
/// `CommandNotFound` error at analysis time — configuration never requires prior registration.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_misconfigured_default_command_surfaces_command_not_found()
{
  let mut registry = CommandRegistry::new();
  registry.set_default_command( ".never_registered" ).expect( "Valid default command name should be accepted" );

  let result = parse_and_analyze( &registry, "dry::true" );

  assert!( result.is_err(), "A default naming an unregistered command must fail" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::CommandNotFound, "Must produce CommandNotFound; got: {:?}", error_data.code );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FR-REG-10: A bare empty path with NO arguments still returns the help listing, even when a
/// default command is configured — only an empty path WITH arguments triggers routing.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_empty_path_no_args_returns_help_listing_even_with_default_configured()
{
  let mut registry = CommandRegistry::new();
  registry.register( report_command() ).expect( "Registration should succeed" );
  registry.set_default_command( ".report" ).expect( "Valid default command name should be accepted" );

  let result = parse_and_analyze( &registry, "." );

  assert!( result.is_err(), "Bare '.' must still surface the help-listing signal (as an Err), not a routed command" );
  match result.unwrap_err()
  {
    Error::Execution( error_data ) =>
    {
      assert_eq!( error_data.code, ErrorCode::HelpRequested, "Must produce HelpRequested; got: {:?}", error_data.code );
    },
    other => panic!( "Expected Error::Execution, got: {other:?}" ),
  }
}

/// FR-REG-10: Configuring the default before the target command is even registered still works
/// once both are in place — the builder's deferred existence check is order-independent.
// test_kind: ft_spec(FT-26)  [feature/01_command_registry]
#[ test ]
fn test_default_configured_before_target_registration_still_routes()
{
  let mut registry = CommandRegistry::new();
  // Default configured FIRST, target registered SECOND — reverse of every other test above.
  registry.set_default_command( ".report" ).expect( "Valid default command name should be accepted" );
  registry.register( report_command() ).expect( "Registration should succeed" );

  let verified_commands = parse_and_analyze( &registry, "dry::false" )
    .expect( "Routing should work regardless of configuration/registration order" );

  assert_eq!( verified_commands[ 0 ].definition.full_name(), ".report" );
  assert_eq!( verified_commands[ 0 ].arguments.get( "dry" ), Some( &Value::Boolean( false ) ) );
}
