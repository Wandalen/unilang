//! # unilang CLI Binary Entry Point
//!
//! This is a comprehensive CLI application for the `unilang` module that demonstrates:
//! - Command registry initialization with multiple namespaces
//! - Command-line argument parsing with proper error handling
//! - Semantic analysis and command execution
//! - Help system integration
//!
//! Following Design Rulebook principles:
//! - Uses proper error handling with Result types
//! - Implements comprehensive help system
//! - Uses explicit parameter handling to avoid fragile defaults
//! - Follows proper spacing and formatting per Codestyle Rulebook

mod demo_commands;

use std::collections::HashMap;
use unilang::help::HelpGenerator;
use unilang::interpreter::{ ExecutionContext, Interpreter };
use unilang::registry::CommandRegistry;
use unilang::semantic::SemanticAnalyzer;
use unilang_parser::{ Parser, UnilangParserOptions };

fn main()
{
  if let Err( err ) = run()
  {
    eprintln!( "Error: {err}" );
    std::process::exit( 1 );
  }
}

fn build_alias_map( registry : &CommandRegistry ) -> HashMap< String, String >
{
  let mut alias_map = HashMap::new();
  for ( full_name, cmd_def ) in &registry.commands()
  {
    for alias in cmd_def.aliases()
    {
      alias_map.insert( alias.clone(), full_name.clone() );
    }
  }
  alias_map
}

/// Handles `--help` and `help [command]` dispatching. Returns `true` if handled.
///
/// Error cases (invalid help usage, command not found) call `std::process::exit(1)`.
fn handle_help_dispatch( processed_args : &[ String ], registry : &CommandRegistry ) -> bool
{
  let first = processed_args.first().map( String::as_str );
  let is_help_flag = first == Some( "--help" );
  let is_help_cmd  = first == Some( "help" );

  if !is_help_flag && !is_help_cmd
  {
    return false;
  }

  let help_generator = HelpGenerator::from_env( registry );

  if is_help_flag
  {
    println!( "{}", help_generator.list_commands() );
    return true;
  }

  // is_help_cmd == true
  if processed_args.len() > 2
  {
    eprintln!( "Error: Invalid usage of help command. Use `help` or `help <command_name>`." );
    std::process::exit( 1 );
  }
  else if let Some( command_name ) = processed_args.get( 1 )
  {
    if let Some( help_text ) = help_generator.command( command_name )
    {
      println!( "{help_text}" );
    }
    else
    {
      eprintln!( "Error: Command '{command_name}' not found for help." );
      std::process::exit( 1 );
    }
  }
  else
  {
    println!( "{}", help_generator.list_commands() );
  }
  true
}

fn run() -> Result< (), unilang::error::Error >
{
  let registry = demo_commands::build_registry()?;

  let args : Vec< String > = std::env::args().skip( 1 ).collect();

  if args.is_empty()
  {
    let help_generator = HelpGenerator::from_env( &registry );
    let help_text = help_generator.list_commands();
    println!( "{help_text}" );
    eprintln!( "Usage: unilang_cli <command> [args...]" );
    eprintln!( "Examples:" );
    eprintln!( "  unilang_cli greet name::\"Alice\"" );
    eprintln!( "  unilang_cli math.add a::10 b::20" );
    eprintln!( "  unilang_cli config.set key::\"theme\" value::\"dark\"" );
    eprintln!( "  unilang_cli help greet" );
    eprintln!( "Note: Arguments use name::value syntax. String values must be quoted." );
    return Ok( () );
  }

  let verbosity = std::env::var( "UNILANG_VERBOSITY" )
  .ok()
  .and_then( | v | v.parse::< u8 >().ok() )
  .unwrap_or( 1 );

  if verbosity > 1
  {
    eprintln!( "DEBUG: Raw shell arguments: {args:?}" );
  }

  let parser = Parser::new( UnilangParserOptions { verbosity, ..Default::default() } );

  let alias_map = build_alias_map( &registry );
  let mut processed_args = args.clone();
  if let Some( first_arg ) = processed_args.first_mut()
  {
    if let Some( canonical_name ) = alias_map.get( first_arg )
    {
      *first_arg = canonical_name.clone();
    }
  }

  if handle_help_dispatch( &processed_args, &registry )
  {
    return Ok( () );
  }

  if verbosity > 1
  {
    eprintln!( "DEBUG: Processing argv: {processed_args:?}" );
  }

  // Parse using argv-aware parser to properly handle multi-word parameter values.
  // The shell removes quotes from arguments like query::"llm rust", resulting in
  // argv = ["query::llm rust"] (one token). Using parse_from_argv() preserves these
  // token boundaries, while parse_single_instruction() would re-tokenize on spaces.
  let instruction = parser.parse_from_argv( &processed_args )?;
  let instructions = &[ instruction ][ .. ];

  let semantic_analyzer = SemanticAnalyzer::new( instructions, &registry );
  let commands = match semantic_analyzer.analyze()
  {
    Ok( commands ) => commands,
    Err( unilang::error::Error::Execution( error_data ) ) if error_data.code == unilang::data::ErrorCode::HelpRequested =>
    {
      println!( "{}", error_data.message );
      return Ok( () );
    },
    Err( e ) => return Err( e ),
  };

  let interpreter = Interpreter::new( &commands, &registry );
  let mut context = ExecutionContext::default();
  interpreter.run( &mut context )?;

  Ok( () )
}

