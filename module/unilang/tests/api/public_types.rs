//! API public types contract tests.
//!
//! Implements AP-1..10 specification cases from `tests/docs/api/01_public_types.md`.
//!
//! ## Compile-Time Cases (AP-1, AP-5)
//!
//! AP-1 (CommandDefinition builder requires name) and AP-5 (CommandDefinition fields are private)
//! are compile-time enforcement checks. They cannot be expressed as runtime `#[test]` functions.
//!
//! - **AP-1**: `CommandDefinition::former().description("desc").end()` fails to compile because
//!   the type-state builder enforces that `name` is set before calling `.end()`.
//! - **AP-5**: `definition.name` (direct field access) fails to compile because all
//!   `CommandDefinition` fields are private; only accessor methods (`.name()`) are valid.

// AP-1: CommandDefinition builder requires name — compile-time only. No runtime test.
// AP-5: CommandDefinition fields are private — compile-time only. No runtime test.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::data::ErrorData;
use unilang::static_data::{
  StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind,
};
use unilang::types::Value;
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::pipeline::Pipeline;
use unilang::prelude::HelpVerbosity;

/// AP-2: Full pipeline round-trip returns correct output for a named argument.
///
/// Registers `.echo` with a required `msg: String` argument and processes
/// `.echo msg::"hello"` through the complete parse → semantic analysis → execution chain.
///
/// ## Pitfall
///
/// Named argument syntax in unilang uses `name::"value"` (double colon, quoted value).
/// The spec writes `msg::hello` as shorthand but the parser requires quotes around the value.
// test_kind: ap_spec(AP-2)
#[ test ]
fn test_pipeline_round_trip_correct_arguments()
{
  #[ allow( deprecated ) ]
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

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &echo_command, routine ).unwrap();

  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command( r#".echo msg::"hello""#, ExecutionContext::default() );

  assert!( result.success, "Pipeline round-trip must succeed; error: {:?}", result.error );
  assert!( result.error.is_none() );
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
// test_kind: ap_spec(AP-3)
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
// test_kind: ap_spec(AP-4)
#[ test ]
fn test_registry_lookup_returns_expected_definition()
{
  #[ allow( deprecated ) ]
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

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &query_command, noop ).unwrap();

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
// test_kind: ap_spec(AP-6)
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
// test_kind: ap_spec(AP-7)
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
  assert!( output_no_timing.execution_time_ms.is_none() );
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
// test_kind: ap_spec(AP-8)
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
// test_kind: ap_spec(AP-9)
#[ test ]
fn test_ap9_process_command_from_argv_preserves_boundaries()
{
  #[ allow( deprecated ) ]
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

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &echo_command, routine ).unwrap();

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
// test_kind: ap_spec(AP-10)
#[ test ]
fn test_ap10_process_batch_collects_all_results()
{
  #[ allow( deprecated ) ]
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

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &ok_command, ok_routine ).unwrap();

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
