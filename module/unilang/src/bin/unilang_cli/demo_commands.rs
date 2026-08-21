//! Demo command definitions for the unilang CLI binary.
//!
//! Provides the set of example commands registered in the demo registry,
//! illustrating math, system, file, and search command patterns.
//!
//! Printing contract: routines only RETURN their output via `OutputData.content`;
//! `main.rs` is the single printer of returned outputs. Routines must not
//! `println!` themselves, or every command double-prints.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, OutputData };
use unilang::data::Kind as ArgumentKind;
use unilang::registry::{ CommandRegistry, CommandRoutine };
use unilang::types::Value;

fn cmd_math_add() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".add" )
  .namespace( ".math".to_string() )
  .description( "Adds two numbers.".to_string() )
  .hint( "Adds two numbers." )
  .status( "stable" )
  .version( "1.0.0".to_string() )
  .aliases( vec![ "sum".to_string(), "plus".to_string() ] )
  .tags( vec![ "math".to_string(), "calculation".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "a" )
    .kind( ArgumentKind::Integer )
    .hint( "First number." )
    .end(),
    ArgumentDefinition::former()
    .name( "b" )
    .kind( ArgumentKind::Integer )
    .hint( "Second number." )
    .end(),
  ])
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let a = cmd.arguments.get( "a" ).unwrap();
    let b = cmd.arguments.get( "b" ).unwrap();
    if let ( Value::Integer( val_a ), Value::Integer( val_b ) ) = ( a, b )
    {
      let result = val_a + val_b;
      return Ok( OutputData
      {
        content : format!( "Result: {result}" ),
        format : "text".to_string(),
        execution_time_ms : None,
      });
    }
    unreachable!( "cmd_math_add: arguments 'a' and 'b' must both be Integer — guaranteed by Kind::Integer argument definition" );
  });

  ( def, routine )
}

fn cmd_math_sub() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".sub" )
  .namespace( ".math".to_string() )
  .description( "Subtracts two numbers.".to_string() )
  .hint( "Subtracts two numbers." )
  .status( "beta" )
  .version( "0.9.0".to_string() )
  .aliases( vec![ "minus".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "x" )
    .kind( ArgumentKind::Integer )
    .hint( "Minuend." )
    .end(),
    ArgumentDefinition::former()
    .name( "y" )
    .kind( ArgumentKind::Integer )
    .hint( "Subtrahend." )
    .end(),
  ])
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let x = cmd.arguments.get( "x" ).unwrap();
    let y = cmd.arguments.get( "y" ).unwrap();
    if let ( Value::Integer( val_x ), Value::Integer( val_y ) ) = ( x, y )
    {
      let result = val_x - val_y;
      return Ok( OutputData
      {
        content : format!( "Result: {result}" ),
        format : "text".to_string(),
        execution_time_ms : None,
      });
    }
    unreachable!( "cmd_math_sub: arguments 'x' and 'y' must both be Integer — guaranteed by Kind::Integer argument definition" );
  });

  ( def, routine )
}

fn cmd_greet() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".greet" )
  .namespace( String::new() )
  .description( "Greets the specified person.".to_string() )
  .hint( "Greets the specified person." )
  .status( "stable" )
  .version( "1.0.0".to_string() )
  .aliases( vec![ "hi".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![ "greet name::\"John\"".to_string(), "greet".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "name" )
    .kind( ArgumentKind::String )
    .hint( "Name of the person to greet." )
    .attributes( ArgumentAttributes
    {
      optional : true,
      multiple : false,
      default : Some( "World".to_string() ),
      sensitive : false,
      interactive : false,
    })
    .end()
  ])
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let name = match cmd.arguments.get( "name" )
    {
      Some( Value::String( s ) ) => s.clone(),
      _ => "World".to_string(),
    };
    let result = format!( "Hello, {name}!" );
    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  ( def, routine )
}

fn cmd_config_set() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".set" )
  .namespace( ".config".to_string() )
  .description( "Sets a configuration value.".to_string() )
  .hint( "Sets a configuration value." )
  .status( "experimental" )
  .version( "0.1.0".to_string() )
  .aliases( vec![] )
  .permissions( vec![] )
  .idempotent( false )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "key" )
    .kind( ArgumentKind::String )
    .hint( "Configuration key." )
    .end(),
    ArgumentDefinition::former()
    .name( "value" )
    .kind( ArgumentKind::String )
    .hint( "Configuration value." )
    .attributes( ArgumentAttributes
    {
      optional : false,
      multiple : false,
      default : None,
      sensitive : true,
      interactive : true,
    })
    .end(),
  ])
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let key = cmd.arguments.get( "key" ).unwrap();
    let value = cmd.arguments.get( "value" ).unwrap();
    let result = format!( "Setting config: {key} = {value}" );
    Ok( OutputData
    {
      content : result,
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  ( def, routine )
}

fn cmd_echo() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".echo" )
  .namespace( ".system".to_string() )
  .description( "Echoes a message".to_string() )
  .hint( "Echoes back the provided arguments.".to_string() )
  .status( "stable".to_string() )
  .version( "1.0.0".to_string() )
  .tags( vec![ "utility".to_string() ] )
  .aliases( vec![ "e".to_string() ] )
  .permissions( vec![ "admin".to_string() ] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![ "system.echo \"Hello\"".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "arg1" )
    .kind( ArgumentKind::String )
    .hint( "The first argument to echo." )
    .attributes( ArgumentAttributes
    {
      optional : true,
      multiple : false,
      default : None,
      sensitive : false,
      interactive : false,
    })
    .end(),
  ])
  .routine_link( Some( ".system.echo".to_string() ) )
  .end();

  let routine : CommandRoutine = Box::new( | _cmd, _ctx |
  {
    Ok( OutputData
    {
      content : "Echo command executed!".to_string(),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  ( def, routine )
}

fn cmd_cat() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".cat" )
  .namespace( ".files".to_string() )
  .description( "Read and display file contents".to_string() )
  .hint( "Print file contents to stdout".to_string() )
  .status( "stable".to_string() )
  .version( "1.0.0".to_string() )
  .tags( vec![ "filesystem".to_string() ] )
  .aliases( vec![ "type".to_string() ] )
  .permissions( vec![ "read_file".to_string() ] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec![ "files.cat path::/etc/hosts".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "path" )
    .description( "The path to the file to read".to_string() )
    .hint( "File path".to_string() )
    .kind( ArgumentKind::String )
    .aliases( vec![ "p".to_string() ] )
    .tags( vec![ "required".to_string() ] )
    .attributes( ArgumentAttributes
    {
      optional : false,
      multiple : false,
      default : None,
      sensitive : false,
      interactive : false,
    })
    .end()
  ])
  .routine_link( Some( ".files.cat".to_string() ) )
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let path = cmd.arguments.get( "path" ).unwrap();
    if let Value::String( path_str ) = path
    {
      if let Ok( contents ) = std::fs::read_to_string( path_str )
      {
        Ok( OutputData
        {
          content : contents,
          format : "text".to_string(),
          execution_time_ms : None,
        })
      }
      else
      {
        let error_msg = format!( "Failed to read file: {path_str}" );
        Err( unilang::data::ErrorData::new(
          unilang::data::ErrorCode::InternalError,
          error_msg,
        ))
      }
    }
    else
    {
      Err( unilang::data::ErrorData::new(
        unilang::data::ErrorCode::ArgumentTypeMismatch,
        "Path must be a string".to_string(),
      ))
    }
  });

  ( def, routine )
}

fn cmd_video_search() -> ( CommandDefinition, CommandRoutine )
{
  let def = CommandDefinition::former()
  .name( ".search" )
  .namespace( ".video".to_string() )
  .description( "Search for videos with query and optional filters".to_string() )
  .hint( "Search video content with multi-word query support".to_string() )
  .status( "stable".to_string() )
  .version( "1.0.0".to_string() )
  .tags( vec![ "video".to_string(), "search".to_string() ] )
  .aliases( vec![] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( String::new() )
  .examples( vec!
  [
    ".video.search query::\"rust programming\"".to_string(),
    ".video.search query::\"llm rust\" title::\"Tutorial\"".to_string()
  ])
  .arguments( vec!
  [
    ArgumentDefinition::former()
    .name( "query" )
    .description( "Search query (supports multi-word quoted values)".to_string() )
    .hint( "The search query text".to_string() )
    .kind( ArgumentKind::String )
    .aliases( vec![ "q".to_string() ] )
    .tags( vec![ "search".to_string() ] )
    .attributes( ArgumentAttributes
    {
      optional : false,
      multiple : false,
      default : None,
      sensitive : false,
      interactive : false,
    })
    .end(),
    ArgumentDefinition::former()
    .name( "title" )
    .description( "Optional title filter".to_string() )
    .hint( "Filter by title".to_string() )
    .kind( ArgumentKind::String )
    .aliases( vec![ "t".to_string() ] )
    .tags( vec![ "filter".to_string() ] )
    .attributes( ArgumentAttributes
    {
      optional : true,
      multiple : false,
      default : None,
      sensitive : false,
      interactive : false,
    })
    .end(),
  ])
  .routine_link( Some( ".video.search".to_string() ) )
  .end();

  let routine : CommandRoutine = Box::new( | cmd, _ctx |
  {
    let query = cmd.arguments.get( "query" ).unwrap();
    let title = cmd.arguments.get( "title" );
    if let Value::String( query_str ) = query
    {
      let mut result = format!( "Query: {query_str}" );
      if let Some( Value::String( title_str ) ) = title
      {
        use core::fmt::Write;
        write!( &mut result, "\nTitle: {title_str}" ).unwrap();
      }
      Ok( OutputData
      {
        content : result,
        format : "text".to_string(),
        execution_time_ms : None,
      })
    }
    else
    {
      Err( unilang::data::ErrorData::new(
        unilang::data::ErrorCode::ArgumentTypeMismatch,
        "Query must be a string".to_string(),
      ))
    }
  });

  ( def, routine )
}

/// Builds the demo command registry with all example commands registered.
pub fn build_registry() -> Result< CommandRegistry, unilang::error::Error >
{
  let mut registry = CommandRegistry::new();

  let ( def, routine ) = cmd_math_add();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_math_sub();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_greet();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_config_set();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_echo();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_cat();
  registry.register_with_routine( &def, routine )?;

  let ( def, routine ) = cmd_video_search();
  registry.register_with_routine( &def, routine )?;

  Ok( registry )
}
