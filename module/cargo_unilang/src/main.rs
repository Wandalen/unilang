//! `cargo_unilang` - Scaffolding and health check tool for unilang CLI projects
//!
//! Prevents common mistakes when using unilang framework:
//! - Custom build.rs (unilang provides this automatically)
//! - Duplicate dependencies (`serde_yaml_ng`, walkdir, phf)
//! - Deprecated API (`CommandRegistry::new()`)
//!
//! This tool itself is built using unilang, demonstrating correct usage and
//! serving as a reference implementation for CLI rulebook compliance.

pub mod commands;
pub mod templates;
pub mod checks;

use std::{ env, process };

use unilang::data::{ ArgumentDefinition, CommandDefinition, ErrorCode, ErrorData, Kind, OutputData };
use unilang::interpreter::ExecutionContext;
use unilang::pipeline::Pipeline;
use unilang::registry::{ CommandRegistry, CommandRoutine };
use unilang::semantic::VerifiedCommand;

fn main()
{
  let exit_code = match run()
  {
    Ok( code ) => code,
    Err( e ) =>
    {
      eprintln!( "Error: {}", e );
      1
    }
  };

  process::exit( exit_code );
}

fn run() -> Result< i32, String >
{
  let mut args : Vec< String > = env::args().skip( 1 ).collect();

  // If no arguments, treat as help request
  if args.is_empty()
  {
    args.push( ".".to_string() );
  }

  let command_name = args.first().cloned().unwrap();

  // Handle . and .help before Pipeline: Pipeline short-circuits "." as a listing request,
  // bypassing the registered routine. Pre-dispatch preserves our custom help text.
  if command_name == "." || command_name == ".help"
  {
    println!( "{}", commands::general_help() );
    return Ok( 0 );
  }

  // Require dot prefix for all other commands
  if !command_name.starts_with( '.' )
  {
    eprintln!( "Unknown command: {}", command_name );
    eprintln!( "Run 'cargo_unilang .help' for usage information" );
    return Ok( 2 );
  }

  // Require key::value format for all parameters
  if args.len() > 1
  {
    parse_params( &args[ 1.. ] )?;
  }

  // Build registry with commands and their routines
  let mut registry = CommandRegistry::new();
  register_commands( &mut registry )?;

  // Dispatch via Pipeline
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_from_argv_simple( &args );

  if result.success
  {
    Ok( 0 )
  }
  else
  {
    let error = result.error.unwrap_or_else( || "Unknown error".to_string() );
    eprintln!( "Error: {}", error );
    Ok( 1 )
  }
}

fn register_commands( registry : &mut CommandRegistry ) -> Result< (), String >
{
  // `.new` — create new unilang project
  {
    let def = CommandDefinition::former()
      .name( ".new" )
      .description( "Create new unilang project with correct structure" )
      .arguments( vec!
      [
        ArgumentDefinition::new( "project", Kind::String ),
        ArgumentDefinition::new( "template", Kind::String ).with_optional( Some( "minimal" ) ),
        ArgumentDefinition::new( "author", Kind::String ).with_optional( None::< &str > ),
        ArgumentDefinition::new( "license", Kind::String ).with_optional( None::< &str > ),
        ArgumentDefinition::new( "verbosity", Kind::String ).with_optional( Some( "2" ) ),
      ] )
      .auto_help_enabled( false )
      .end();
    let routine : CommandRoutine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
    {
      let params = collect_args( &cmd );
      let new_params = commands::NewParams::parse( &params )
        .map_err( | e | ErrorData::new( ErrorCode::ValidationRuleFailed, format!( "Invalid parameter: {e}" ) ) )?;
      commands::new::execute( new_params )
        .map( | _code | OutputData::new( "", "text" ) )
        .map_err( | e | ErrorData::new( ErrorCode::InternalError, e ) )
    } );
    registry.register_with_routine( &def, routine )
      .map_err( | e | format!( "Failed to register .new: {e}" ) )?;
  }

  // `.new.help` — help for .new
  {
    let def = CommandDefinition::former()
      .name( ".new.help" )
      .description( "Show help for .new command" )
      .auto_help_enabled( false )
      .end();
    let routine : CommandRoutine = Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext |
    {
      println!( "{}", commands::new_help() );
      Ok( OutputData::new( "", "text" ) )
    } );
    registry.register_with_routine( &def, routine )
      .map_err( | e | format!( "Failed to register .new.help: {e}" ) )?;
  }

  // `.check` — validate existing unilang project
  {
    let def = CommandDefinition::former()
      .name( ".check" )
      .description( "Validate existing unilang project for common mistakes" )
      .arguments( vec!
      [
        ArgumentDefinition::new( "path", Kind::String ).with_optional( Some( "." ) ),
        ArgumentDefinition::new( "verbosity", Kind::String ).with_optional( Some( "2" ) ),
        ArgumentDefinition::new( "fix", Kind::String ).with_optional( Some( "false" ) ),
      ] )
      .auto_help_enabled( false )
      .end();
    let routine : CommandRoutine = Box::new( | cmd : VerifiedCommand, _ctx : ExecutionContext |
    {
      let params = collect_args( &cmd );
      let check_params = commands::CheckParams::parse( &params )
        .map_err( | e | ErrorData::new( ErrorCode::ValidationRuleFailed, format!( "Invalid parameter: {e}" ) ) )?;
      match commands::check::execute( check_params )
      {
        Ok( 0 ) => Ok( OutputData::new( "", "text" ) ),
        Ok( _ ) => Err( ErrorData::new( ErrorCode::ValidationRuleFailed, "Issues found".to_string() ) ),
        Err( e ) => Err( ErrorData::new( ErrorCode::InternalError, e ) ),
      }
    } );
    registry.register_with_routine( &def, routine )
      .map_err( | e | format!( "Failed to register .check: {e}" ) )?;
  }

  // `.check.help` — help for .check
  {
    let def = CommandDefinition::former()
      .name( ".check.help" )
      .description( "Show help for .check command" )
      .auto_help_enabled( false )
      .end();
    let routine : CommandRoutine = Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext |
    {
      println!( "{}", commands::check_help() );
      Ok( OutputData::new( "", "text" ) )
    } );
    registry.register_with_routine( &def, routine )
      .map_err( | e | format!( "Failed to register .check.help: {e}" ) )?;
  }

  Ok( () )
}

/// Extracts all String arguments from a VerifiedCommand as (name, value) pairs.
///
/// Since all command arguments are defined with `Kind::String`, all values are
/// `Value::String`. The pairs are passed directly to the existing parse functions.
fn collect_args( cmd : &VerifiedCommand ) -> Vec< ( String, String ) >
{
  cmd.arguments
    .keys()
    .map( | k | ( k.clone(), cmd.get_string( k ).unwrap_or( "" ).to_string() ) )
    .collect()
}

/// Validates that all parameters use the `key::value` format.
///
/// Returns parsed pairs on success; returns an error message on the first
/// argument that does not contain `::`.
fn parse_params( args : &[ String ] ) -> Result< Vec< ( String, String ) >, String >
{
  let mut params = Vec::new();

  for arg in args
  {
    if let Some( idx ) = arg.find( "::" )
    {
      let key = arg[ ..idx ].to_string();
      let value = arg[ idx + 2.. ].to_string();
      params.push( ( key, value ) );
    }
    else
    {
      return Err( format!( "Invalid parameter format '{}'. Use param::value format", arg ) );
    }
  }

  Ok( params )
}
