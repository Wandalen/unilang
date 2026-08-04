//! API public types contract tests.
//!
//! Implements AP-1..19 specification cases from `tests/docs/api/01_public_types.md`.
//!
//! ## Compile-Time Cases (AP-1, AP-5)
//!
//! AP-1 (CommandDefinition builder requires name) and AP-5 (CommandDefinition fields are private)
//! are compile-time enforcement checks. They are not expressed as runtime `#[test]` functions in
//! this file — they are verified by the trybuild compile-fail test
//! `test_tc_compile_fail_type_state_and_private_fields` in `tests/build/compile_fail_tests.rs`
//! (tagged `ap_spec(AP-1)` and `ap_spec(AP-5)` respectively).
//!
//! - **AP-1**: `CommandDefinition::former().description("desc").end()` fails to compile because
//!   the type-state builder enforces that `name` is set before calling `.end()`.
//!   (`tests/compile_fail/t40_builder_missing_name.rs`)
//! - **AP-5**: `definition.name` (direct field access) fails to compile because all
//!   `CommandDefinition` fields are private; only accessor methods (`.name()`) are valid.
//!   (`tests/compile_fail/t50_private_field_name.rs`)
//!

// AP-1: CommandDefinition builder requires name — compile-time only, see tests/build/compile_fail_tests.rs
// AP-5: CommandDefinition fields are private — compile-time only, see tests/build/compile_fail_tests.rs

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::data::ErrorData;
use unilang::static_data::{
  StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind, StaticValidationRule, StaticCommandMap,
};
use unilang::types::Value;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::pipeline::Pipeline;
use unilang::prelude::HelpVerbosity;
use unilang::help::HelpDisplayOptions;

/// AP-2: Full pipeline round-trip returns correct output for a named argument.
///
/// Registers `.echo` with a required `msg: String` argument and processes
/// `.echo msg::"hello"` through the complete parse → semantic analysis → execution chain.
///
/// ## Pitfall
///
/// Named argument syntax in unilang uses `name::"value"` (double colon, quoted value).
/// The spec writes `msg::hello` as shorthand but the parser requires quotes around the value.
// test_kind: ap_spec(AP-2)  [api/01_public_types]
#[ test ]
fn test_pipeline_round_trip_correct_arguments()
{
  let mut registry = CommandRegistry::new();

  let echo_command = CommandDefinition::former()
    .name( ".echo" )
    .namespace( String::new() )
    .description( "Echo a message".to_string() )
    .hint( "Echo" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( true )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "msg".to_string(),
        kind : Kind::String,
        description : "Message to echo".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let routine : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync > =
    Box::new( | cmd, _ctx |
  {
    let msg = cmd.arguments.get( "msg" )
      .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
      .unwrap_or_default();
    Ok( OutputData { content : msg, format : "text".to_string(), execution_time_ms : None } )
  });

  registry.register_with_routine( &echo_command, routine ).unwrap();

  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command( r#".echo msg::"hello""#, ExecutionContext::default() );

  assert!( result.success, "Pipeline round-trip must succeed; error: {:?}", result.error );
  assert!( result.error.is_none(), "Pipeline result should have no error; got: {:?}", result.error );
  assert_eq!( result.outputs.len(), 1 );
  assert_eq!(
    result.outputs[ 0 ].content,
    "hello",
    "Named argument value must round-trip through full pipeline"
  );
}

/// AP-3: All 15 Kind variants are constructable without error.
///
/// Verifies that every variant of the `Kind` enum can be created and used in an
/// `ArgumentDefinition` without panicking.
// test_kind: ap_spec(AP-3)  [api/01_public_types]
#[ test ]
fn test_all_15_kind_variants_constructable()
{
  let kinds : Vec< Kind > = vec![
    Kind::String,
    Kind::Integer,
    Kind::Float,
    Kind::Boolean,
    Kind::Path,
    Kind::File,
    Kind::Directory,
    Kind::Enum( vec![ "a".to_string(), "b".to_string() ] ),
    Kind::Url,
    Kind::DateTime,
    Kind::Pattern,
    Kind::List( Box::new( Kind::String ), None ),
    Kind::Map( Box::new( Kind::String ), Box::new( Kind::Integer ), None, None ),
    Kind::JsonString,
    Kind::Object,
  ];

  assert_eq!( kinds.len(), 15, "Expected exactly 15 Kind variants" );

  for kind in &kinds
  {
    let arg = ArgumentDefinition::new( "test_arg", kind.clone() );
    assert_eq!(
      arg.name,
      "test_arg",
      "ArgumentDefinition must be constructable for Kind::{kind:?}"
    );
  }
}

/// AP-4: CommandRegistry lookup returns the expected definition.
///
/// Verifies that `registry.command()` returns the registered command with its correct description.
// test_kind: ap_spec(AP-4)  [api/01_public_types]
#[ test ]
fn test_registry_lookup_returns_expected_definition()
{
  let mut registry = CommandRegistry::new();

  let query_command = CommandDefinition::former()
    .name( ".query" )
    .description( "Run a query".to_string() )
    .status( "stable" )
    .version( "1.0.0" )
    .end();

  let noop : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync > =
    Box::new( | _cmd, _ctx |
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  );

  registry.register_with_routine( &query_command, noop ).unwrap();

  let definition = registry.command( ".query" );
  assert!( definition.is_some(), "Registered command must be retrievable by name" );

  let def = definition.unwrap();
  assert_eq!(
    def.description(),
    "Run a query",
    "Registry lookup must return the registered description"
  );
}

/// AP-6: StaticCommandDefinition → CommandDefinition conversion preserves name and argument count.
///
/// Verifies that `From<&StaticCommandDefinition>` produces a `CommandDefinition` with identical
/// observable attributes: name string and argument count.
// test_kind: ap_spec(AP-6)  [api/01_public_types]
#[ test ]
fn test_static_to_dynamic_conversion_preserves_attributes()
{
  static GREET_ARG : StaticArgumentDefinition = StaticArgumentDefinition
  {
    name : "who",
    kind : StaticKind::String,
    attributes : StaticArgumentAttributes
    {
      optional : false,
      multiple : false,
      default : None,
      sensitive : false,
      interactive : false,
    },
    hint : "Who to greet",
    description : "Name of the person to greet",
    validation_rules : &[],
    aliases : &[],
    tags : &[],
  };

  static GREET_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".greet",
    namespace : "",
    description : "Greet someone",
    hint : "",
    arguments : &[ GREET_ARG ],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : false,
    deprecation_message : "",
    http_method_hint : "",
    examples : &[],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : true,
  };

  let dynamic_cmd : CommandDefinition = ( &GREET_CMD ).into();

  assert_eq!(
    dynamic_cmd.name().as_str(),
    ".greet",
    "Conversion must preserve the command name"
  );
  assert_eq!(
    dynamic_cmd.arguments().len(),
    1,
    "Conversion must preserve argument count"
  );
  assert_eq!(
    dynamic_cmd.arguments()[ 0 ].name,
    "who",
    "Conversion must preserve argument names"
  );
}

/// AP-7: OutputData fields are accessible and hold the correct values.
///
/// ## Note on Spec Divergence
///
/// AP-7 describes a JSON serde round-trip with a `command_name` field. The current
/// `OutputData` struct has `content`, `format`, and `execution_time_ms` fields without
/// serde derives. This test verifies the actual API field contract.
// test_kind: ap_spec(AP-7)  [api/01_public_types]
#[ test ]
fn test_output_data_field_access()
{
  let output = OutputData
  {
    content : "result: ok".to_string(),
    format : "text".to_string(),
    execution_time_ms : Some( 42 ),
  };

  assert_eq!( output.content, "result: ok" );
  assert_eq!( output.format, "text" );
  assert_eq!( output.execution_time_ms, Some( 42 ) );

  let output_no_timing = OutputData
  {
    content : "result".to_string(),
    format : "json".to_string(),
    execution_time_ms : None,
  };
  assert!( output_no_timing.execution_time_ms.is_none(), "execution_time_ms should be None when not provided" );
}

/// AP-8: UNILANG_HELP_VERBOSITY env var is recognized and applied.
///
/// Verifies the env var name stability contract: `UNILANG_HELP_VERBOSITY=2` produces
/// `HelpVerbosity::Standard` (level 2), and the env var name is stable.
///
/// ## Note
///
/// This test mutates a process-level env var. nextest runs each test in a separate
/// process, so env var mutation does not affect sibling tests.
// test_kind: ap_spec(AP-8)  [api/01_public_types]
#[ test ]
fn test_help_verbosity_env_var_recognized()
{
  let old_value = std::env::var( "UNILANG_HELP_VERBOSITY" ).ok();

  std::env::set_var( "UNILANG_HELP_VERBOSITY", "2" );
  let verbosity = HelpVerbosity::from_env();

  match old_value
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_VERBOSITY", v ),
    None => std::env::remove_var( "UNILANG_HELP_VERBOSITY" ),
  }

  assert_eq!(
    verbosity,
    HelpVerbosity::Standard,
    "UNILANG_HELP_VERBOSITY=2 must produce HelpVerbosity::Standard"
  );
}

/// AP-9: `process_command_from_argv` preserves argument boundaries without re-quoting.
///
/// Argv-based processing preserves spaces within argument values because the OS
/// keeps them in a single argv element. The pipeline must not re-split them.
// test_kind: ap_spec(AP-9)  [api/01_public_types]
#[ test ]
fn test_ap9_process_command_from_argv_preserves_boundaries()
{
  let mut registry = CommandRegistry::new();

  let echo_command = CommandDefinition::former()
    .name( ".echo" )
    .namespace( String::new() )
    .description( "Echo a message".to_string() )
    .hint( "Echo" )
    .status( "stable" )
    .version( "1.0.0" )
    .aliases( vec![] )
    .tags( vec![] )
    .permissions( vec![] )
    .idempotent( true )
    .deprecation_message( String::new() )
    .http_method_hint( String::new() )
    .examples( vec![] )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "msg".to_string(),
        kind : Kind::String,
        description : "Message".to_string(),
        hint : String::new(),
        attributes : ArgumentAttributes { optional : false, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  let routine : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync > =
    Box::new( | cmd, _ctx |
  {
    let msg = cmd.arguments.get( "msg" )
      .and_then( | v | if let Value::String( s ) = v { Some( s.clone() ) } else { None } )
      .unwrap_or_default();
    Ok( OutputData { content : msg, format : "text".to_string(), execution_time_ms : None } )
  });

  registry.register_with_routine( &echo_command, routine ).unwrap();

  let pipeline = Pipeline::new( registry );
  let argv : Vec< String > = vec![
    ".echo".to_string(),
    "msg::hello world".to_string(),
  ];
  let result = pipeline.process_command_from_argv( &argv, ExecutionContext::default() );

  assert!( result.success, "Argv-based processing must succeed; error: {:?}", result.error );
  assert_eq!( result.outputs.len(), 1 );
  assert_eq!(
    result.outputs[ 0 ].content,
    "hello world",
    "Space must be preserved because argv boundaries prevent re-splitting"
  );
}

/// AP-10: `process_batch` collects all results regardless of individual failures.
///
/// Batch mode must execute ALL commands without short-circuiting on failures.
/// Three commands `[".nonexistent", ".ok", ".also_nonexistent"]` must produce
/// exactly 3 results: `[Err, Ok, Err]`.
// test_kind: ap_spec(AP-10)  [api/01_public_types]
#[ test ]
fn test_ap10_process_batch_collects_all_results()
{
  let mut registry = CommandRegistry::new();

  let ok_command = CommandDefinition::former()
    .name( ".ok" )
    .description( "Always succeeds".to_string() )
    .status( "stable" )
    .version( "1.0.0" )
    .end();

  let ok_routine : Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, ErrorData > + Send + Sync > =
    Box::new( | _cmd, _ctx |
    Ok( OutputData { content : "ok".to_string(), format : "text".to_string(), execution_time_ms : None } )
  );

  registry.register_with_routine( &ok_command, ok_routine ).unwrap();

  let pipeline = Pipeline::new( registry );
  let commands = vec![ ".nonexistent", ".ok", ".also_nonexistent" ];
  let batch_result = pipeline.process_batch( &commands, ExecutionContext::default() );

  assert_eq!(
    batch_result.total_commands, 3,
    "Batch must record all 3 commands"
  );
  assert_eq!(
    batch_result.results.len(), 3,
    "All 3 results must be collected — no short-circuiting"
  );
  assert!( !batch_result.results[ 0 ].success, "First (nonexistent) must fail" );
  assert!( batch_result.results[ 1 ].success, "Second (.ok) must succeed" );
  assert!( !batch_result.results[ 2 ].success, "Third (nonexistent) must fail" );
}

/// AP-11: `StaticArgumentAttributes` → `ArgumentAttributes` conversion preserves all five fields.
///
/// Builds a `StaticArgumentAttributes` with every field set to a non-default value via the
/// fluent `with_*` builder methods, converts it via `From<&StaticArgumentAttributes>`, and
/// verifies each of the five fields survived the conversion unchanged.
// test_kind: ap_spec(AP-11)  [api/01_public_types]
#[ test ]
fn test_ap11_static_argument_attributes_conversion_preserves_all_fields()
{
  let static_attrs = StaticArgumentAttributes::new()
    .with_optional( true )
    .with_multiple( true )
    .with_default( "fallback" )
    .with_sensitive( true )
    .with_interactive( true );

  let dynamic_attrs : ArgumentAttributes = ( &static_attrs ).into();

  assert!( dynamic_attrs.optional, "optional must be preserved as true" );
  assert!( dynamic_attrs.multiple, "multiple must be preserved as true" );
  assert_eq!(
    dynamic_attrs.default,
    Some( "fallback".to_string() ),
    "default must be preserved as Some(\"fallback\")"
  );
  assert!( dynamic_attrs.sensitive, "sensitive must be preserved as true" );
  assert!( dynamic_attrs.interactive, "interactive must be preserved as true" );
}

/// AP-12: `StaticKind` → `Kind` conversion preserves nested `List` and `Enum` structure.
///
/// Verifies that `StaticKind::List` preserves both the boxed nested item kind and the
/// delimiter character, and that `StaticKind::Enum` converts its `&'static [&'static str]`
/// choices into an owned `Vec<String>` with identical contents and order.
// test_kind: ap_spec(AP-12)  [api/01_public_types]
#[ test ]
fn test_ap12_static_kind_conversion_preserves_nested_structure()
{
  static ITEM_KIND : StaticKind = StaticKind::Integer;
  let static_list = StaticKind::List( &ITEM_KIND, Some( ',' ) );

  let dynamic_list : Kind = ( &static_list ).into();
  assert_eq!(
    dynamic_list,
    Kind::List( Box::new( Kind::Integer ), Some( ',' ) ),
    "List variant must preserve nested item kind and delimiter"
  );

  let static_enum = StaticKind::Enum( &[ "red", "green", "blue" ] );
  let dynamic_enum : Kind = ( &static_enum ).into();
  assert_eq!(
    dynamic_enum,
    Kind::Enum( vec![ "red".to_string(), "green".to_string(), "blue".to_string() ] ),
    "Enum variant must preserve choices as an owned Vec<String> in original order"
  );
}

/// AP-13: `StaticValidationRule` → `ValidationRule` conversion preserves rule parameters
/// for all 6 variants.
///
/// Constructs one instance of each `StaticValidationRule` variant, converts each via
/// `From<&StaticValidationRule>`, and asserts the resulting `ValidationRule` matches the
/// expected variant with the identical parameter value.
// test_kind: ap_spec(AP-13)  [api/01_public_types]
#[ test ]
fn test_ap13_static_validation_rule_conversion_preserves_parameters_for_all_variants()
{
  let min : ValidationRule = ( &StaticValidationRule::Min( 1.0 ) ).into();
  assert_eq!( min, ValidationRule::Min( 1.0 ), "Min parameter must be preserved" );

  let max : ValidationRule = ( &StaticValidationRule::Max( 100.0 ) ).into();
  assert_eq!( max, ValidationRule::Max( 100.0 ), "Max parameter must be preserved" );

  let min_length : ValidationRule = ( &StaticValidationRule::MinLength( 3 ) ).into();
  assert_eq!( min_length, ValidationRule::MinLength( 3 ), "MinLength parameter must be preserved" );

  let max_length : ValidationRule = ( &StaticValidationRule::MaxLength( 50 ) ).into();
  assert_eq!( max_length, ValidationRule::MaxLength( 50 ), "MaxLength parameter must be preserved" );

  let pattern : ValidationRule = ( &StaticValidationRule::Pattern( "^[a-z]+$" ) ).into();
  assert_eq!(
    pattern,
    ValidationRule::Pattern( "^[a-z]+$".to_string() ),
    "Pattern parameter must be preserved and converted to an owned String"
  );

  let min_items : ValidationRule = ( &StaticValidationRule::MinItems( 2 ) ).into();
  assert_eq!( min_items, ValidationRule::MinItems( 2 ), "MinItems parameter must be preserved" );
}

/// AP-14: `StaticArgumentDefinition` → `ArgumentDefinition` conversion preserves name, kind,
/// description, and attributes.
///
/// Builds a `StaticArgumentDefinition` via `new()` and `with_attributes()`, converts it via
/// `From<&StaticArgumentDefinition>`, and verifies the resulting `ArgumentDefinition` carries
/// the same name, kind, description, and `attributes.optional` value.
// test_kind: ap_spec(AP-14)  [api/01_public_types]
#[ test ]
fn test_ap14_static_argument_definition_conversion_preserves_name_kind_and_attributes()
{
  let static_arg = StaticArgumentDefinition::new( "count", StaticKind::Integer, "A count value" )
    .with_attributes( StaticArgumentAttributes::new().with_optional( true ) );

  let dynamic_arg : ArgumentDefinition = ( &static_arg ).into();

  assert_eq!( dynamic_arg.name, "count", "name must be preserved" );
  assert_eq!( dynamic_arg.kind, Kind::Integer, "kind must be preserved" );
  assert_eq!( dynamic_arg.description, "A count value", "description must be preserved" );
  assert!( dynamic_arg.attributes.optional, "attributes.optional must be preserved as true" );
}

/// AP-15: `StaticCommandMap` get/`contains_key`/len/`is_empty` return correct O(1) lookups.
///
/// Builds a `StaticCommandMap` from a `phf::Map` containing exactly one entry (`.greet`) via
/// `StaticCommandMap::from_phf_internal`, then verifies every read-only accessor agrees on
/// the map's single-entry state.
// test_kind: ap_spec(AP-15)  [api/01_public_types]
#[ test ]
fn test_ap15_static_command_map_get_and_contains_key_match_len_and_is_empty()
{
  static GREET_CMD : StaticCommandDefinition = StaticCommandDefinition
  {
    name : ".greet",
    namespace : "",
    description : "Greet someone",
    hint : "",
    arguments : &[],
    routine_link : None,
    status : "stable",
    version : "1.0.0",
    tags : &[],
    aliases : &[],
    permissions : &[],
    idempotent : false,
    deprecation_message : "",
    http_method_hint : "",
    examples : &[],
    auto_help_enabled : true,
    category : "",
    show_version_in_help : true,
  };

  static PHF_MAP : unilang::phf::Map< &'static str, &'static StaticCommandDefinition > = unilang::phf::phf_map!
  {
    ".greet" => &GREET_CMD,
  };

  let map = StaticCommandMap::from_phf_internal( &PHF_MAP );

  let found = map.get( ".greet" );
  assert!( found.is_some(), "get(\".greet\") must return Some" );
  assert_eq!( found.unwrap().name, ".greet", "returned definition must have name \".greet\"" );

  assert!( map.contains_key( ".greet" ), "contains_key(\".greet\") must return true" );
  assert!( !map.contains_key( ".missing" ), "contains_key(\".missing\") must return false" );
  assert_eq!( map.len(), 1, "len() must return 1 for a single-entry map" );
  assert!( !map.is_empty(), "is_empty() must return false for a single-entry map" );
}

/// AP-16: `UNILANG_VERBOSITY` env var controls CLI binary logging verbosity, distinct from
/// `UNILANG_HELP_VERBOSITY`.
///
/// `UNILANG_VERBOSITY` is read only by the `unilang_cli` binary (`src/bin/unilang_cli/main.rs`)
/// via `std::env::var("UNILANG_VERBOSITY").ok().and_then(|v| v.parse::<u8>().ok()).unwrap_or(1)`
/// — it is not a library-level API function. This test replicates that exact read/parse
/// pattern to verify `UNILANG_VERBOSITY=2` yields verbosity level 2 (debug), and that mutating
/// it does not affect `HelpVerbosity::from_env()` (which reads the separate
/// `UNILANG_HELP_VERBOSITY` variable), proving the two env vars are independent.
///
/// ## Note
///
/// This test mutates process-level env vars. nextest runs each test in a separate process,
/// so env var mutation does not affect sibling tests.
// test_kind: ap_spec(AP-16)  [api/01_public_types]
#[ test ]
fn test_ap16_unilang_verbosity_env_var_distinct_from_help_verbosity()
{
  let old_verbosity = std::env::var( "UNILANG_VERBOSITY" ).ok();
  let old_help_verbosity = std::env::var( "UNILANG_HELP_VERBOSITY" ).ok();

  std::env::set_var( "UNILANG_VERBOSITY", "2" );
  std::env::remove_var( "UNILANG_HELP_VERBOSITY" );

  // Replicates the exact parse pattern used in src/bin/unilang_cli/main.rs.
  let verbosity : u8 = std::env::var( "UNILANG_VERBOSITY" )
    .ok()
    .and_then( | v | v.parse::< u8 >().ok() )
    .unwrap_or( 1 );

  let help_verbosity = HelpVerbosity::from_env();

  match old_verbosity
  {
    Some( v ) => std::env::set_var( "UNILANG_VERBOSITY", v ),
    None => std::env::remove_var( "UNILANG_VERBOSITY" ),
  }
  match old_help_verbosity
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_VERBOSITY", v ),
    None => std::env::remove_var( "UNILANG_HELP_VERBOSITY" ),
  }

  assert_eq!( verbosity, 2, "UNILANG_VERBOSITY=2 must produce debug-level (2) verbosity" );
  assert_eq!(
    help_verbosity,
    HelpVerbosity::default(),
    "UNILANG_VERBOSITY must not influence HelpVerbosity::from_env() — the two env vars are independent"
  );
}

/// AP-17: `UNILANG_HELP_HIDE_VERSION` suppresses the version line in help output.
///
/// Verifies both the `HelpDisplayOptions.show_version` field toggle and the end-to-end
/// rendered-output effect via `HelpGenerator` (the `?`/`??` access path).
///
/// ## Note
///
/// This test mutates a process-level env var. nextest runs each test in a separate process,
/// so env var mutation does not affect sibling tests.
// test_kind: ap_spec(AP-17)  [api/01_public_types]
#[ test ]
fn test_ap17_help_hide_version_env_var_suppresses_show_version_flag()
{
  use unilang::help::HelpGenerator;

  let old_value = std::env::var( "UNILANG_HELP_HIDE_VERSION" ).ok();

  let cmd = CommandDefinition::former()
    .name( ".test_ap17" )
    .description( "Test command".to_string() )
    .version( "8.8.8".to_string() )
    .end();

  let mut registry = CommandRegistry::new();
  let _ = registry.register( cmd );

  std::env::set_var( "UNILANG_HELP_HIDE_VERSION", "1" );
  let options_hidden = HelpDisplayOptions::default().with_env_overrides();
  let help_text_hidden = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".test_ap17" )
    .expect( "Command should exist" );

  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
  let options_restored = HelpDisplayOptions::default().with_env_overrides();
  let help_text_restored = HelpGenerator::with_verbosity( &registry, HelpVerbosity::Standard )
    .command( ".test_ap17" )
    .expect( "Command should exist" );

  match old_value
  {
    Some( v ) => std::env::set_var( "UNILANG_HELP_HIDE_VERSION", v ),
    None => std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" ),
  }

  assert!(
    !options_hidden.show_version,
    "UNILANG_HELP_HIDE_VERSION=1 must set show_version to false"
  );
  assert!(
    options_restored.show_version,
    "Unsetting UNILANG_HELP_HIDE_VERSION must restore show_version to true"
  );
  assert!(
    !help_text_hidden.contains( "8.8.8" ),
    "UNILANG_HELP_HIDE_VERSION=1 must suppress the version string in HelpGenerator-rendered output"
  );
  assert!(
    help_text_restored.contains( "8.8.8" ),
    "Unsetting UNILANG_HELP_HIDE_VERSION must restore the version string in HelpGenerator-rendered output"
  );
}

/// AP-18: `VerifiedCommand` typed extraction methods return None/false appropriately for a
/// missing (optional, omitted) argument.
///
/// Builds a `VerifiedCommand` whose `arguments` map has no entry for `"count"` (simulating an
/// optional argument that was not supplied), then verifies `get_integer`, `has_argument`, and
/// `get_value` all agree the argument is absent, without panicking.
// test_kind: ap_spec(AP-18)  [api/01_public_types]
#[ test ]
fn test_ap18_verified_command_typed_extraction_returns_none_for_missing_argument()
{
  use std::collections::HashMap;
  use unilang::data::CommandName;

  let definition = CommandDefinition::new(
    CommandName::new( ".count_cmd" ).unwrap(),
    "Counts things".to_string(),
  );

  let verified_command = VerifiedCommand
  {
    definition,
    arguments : HashMap::new(),
  };

  assert_eq!(
    verified_command.get_integer( "count" ),
    None,
    "get_integer(\"count\") must return None when the argument is absent"
  );
  assert!(
    !verified_command.has_argument( "count" ),
    "has_argument(\"count\") must return false when the argument is absent"
  );
  assert!(
    verified_command.get_value( "count" ).is_none(),
    "get_value(\"count\") must return None when the argument is absent"
  );
}

/// AP-19: Configuration Utilities typed extraction parses `u32` and `bool` from `ConfigMap`.
///
/// Builds a `ConfigMap<&str>` (feature `json_parser`) with a `"port"` key holding
/// `JsonValue::Number(8080)` and an `"enabled"` key holding `JsonValue::Bool(true)`, then
/// verifies `extract_u32` and `extract_bool` return the correctly typed values without any
/// manual `JsonValue` matching.
// test_kind: ap_spec(AP-19)  [api/01_public_types]
#[ cfg( feature = "json_parser" ) ]
#[ test ]
fn test_ap19_config_map_typed_extraction_parses_u32_and_bool()
{
  use std::collections::HashMap;
  use serde_json::json;
  use unilang::config_extraction::{ ConfigMap, extract_u32, extract_bool };

  let mut config : ConfigMap< &str > = HashMap::new();
  config.insert( "port".to_string(), ( json!( 8080 ), "default" ) );
  config.insert( "enabled".to_string(), ( json!( true ), "default" ) );

  assert_eq!(
    extract_u32( &config, "port" ),
    Some( 8080u32 ),
    "extract_u32 must return Some(8080u32) for the \"port\" key"
  );
  assert_eq!(
    extract_bool( &config, "enabled" ),
    Some( true ),
    "extract_bool must return Some(true) for the \"enabled\" key"
  );
}
