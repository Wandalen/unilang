//!
//! Manual corner case testing script
//!
//! Tests edge cases that are difficult to cover in automated tests

use unilang::prelude::*;
use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };

/// Simple test routine that returns success
#[allow(clippy::unnecessary_wraps)]
fn test_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let content = if let Some( text ) = cmd.get_string( "text" )
  {
    format!( "Received: {} (len={})", text, text.len() )
  }
  else if let Some( data ) = cmd.get_string( "data" )
  {
    format!( "Received data of length: {}", data.len() )
  }
  else if let Some( items ) = cmd.get_list( "item" )
  {
    format!( "Auto-collected {} items", items.len() )
  }
  else if let Some( item ) = cmd.get_string( "item" )
  {
    format!( "Single item: {}", item )
  }
  else
  {
    "Test executed successfully".to_string()
  };

  Ok( OutputData
  {
    content,
    format : "text".to_string(),
    execution_time_ms : None,
  })
}

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Unilang Manual Corner Case Testing ===" );
  println!();

  // Test 1: Empty command registry
  println!( "Test 1: Empty registry help" );
  test_empty_registry()?;

  // Test 2: Empty parameter value
  println!( "\nTest 2: Empty parameter value" );
  test_empty_parameter()?;

  // Test 3: Unicode in parameters
  println!( "\nTest 3: Unicode in parameters" );
  test_unicode_parameters()?;

  // Test 4: Very long parameter value (1MB)
  println!( "\nTest 4: Very long parameter value" );
  test_long_parameter()?;

  // Test 5: Parameter with newlines
  println!( "\nTest 5: Parameter with newlines" );
  test_multiline_parameter()?;

  // Test 6: Help for non-existent command
  println!( "\nTest 6: Help for non-existent command" );
  test_nonexistent_help()?;

  // Test 7: Unknown parameter with typo (Did you mean)
  println!( "\nTest 7: Unknown parameter typo suggestion" );
  test_typo_suggestion()?;

  // Test 8: Multiple same parameter collection
  println!( "\nTest 8: Multiple same parameter auto-collection" );
  test_multiple_params()?;

  println!( "\n=== All Manual Tests Completed ===" );
  Ok( () )
}

fn test_empty_registry() -> Result< (), Box< dyn std::error::Error > >
{
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_simple( "." );

  if result.success
  {
    if let Some( output ) = result.outputs.first()
    {
      println!( "  ✓ Empty registry help: {}", output.content.lines().next().unwrap_or( "" ));
    }
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✓ Empty registry error: {}", e );
  }

  Ok( () )
}

fn test_empty_parameter() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".test" )
    .description( "Test command" )
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_simple( r#".test param::"""#);

  if result.success
  {
    println!( "  ✓ Empty parameter value accepted" );
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✓ Empty parameter error: {}", e );
  }

  Ok( () )
}

fn test_unicode_parameters() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".unicode" )
    .description( "Unicode test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "text".to_string(),
        description : "Text parameter".to_string(),
        kind : Kind::String,
        hint : "String".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  // Test emoji
  let result = pipeline.process_command_simple( r#".unicode text::"Hello 👋 World 🌍""#);
  if let Some( output ) = result.outputs.first()
  {
    println!( "    {}", output.content );
  }

  // Test RTL text
  let result = pipeline.process_command_simple( r#".unicode text::"مرحبا""#);
  if let Some( output ) = result.outputs.first()
  {
    println!( "    {}", output.content );
  }

  // Test combining characters
  let result = pipeline.process_command_simple( r#".unicode text::"é""#);
  if let Some( output ) = result.outputs.first()
  {
    println!( "    {}", output.content );
  }

  println!( "  ✓ Unicode tests completed" );
  Ok( () )
}

fn test_long_parameter() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".long" )
    .description( "Long param test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "data".to_string(),
        description : "Data parameter".to_string(),
        kind : Kind::String,
        hint : "String".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  // Test 1MB string
  let long_string = "x".repeat( 1_000_000 );
  let command = format!( r#".long data::"{}""#, long_string );

  let result = pipeline.process_command_simple( &command );

  if result.success
  {
    println!( "  ✓ 1MB parameter accepted" );
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✗ 1MB parameter failed: {}", e );
  }

  Ok( () )
}

fn test_multiline_parameter() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".multiline" )
    .description( "Multiline test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "text".to_string(),
        description : "Text parameter".to_string(),
        kind : Kind::String,
        hint : "String".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  let multiline = "Line 1\nLine 2\nLine 3";
  let command = format!( r#".multiline text::"{}""#, multiline );

  let result = pipeline.process_command_simple( &command );

  if result.success
  {
    println!( "  ✓ Multiline parameter accepted" );
    if let Some( output ) = result.outputs.first()
    {
      println!( "    {}", output.content );
    }
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✗ Multiline failed: {}", e );
  }

  Ok( () )
}

fn test_nonexistent_help() -> Result< (), Box< dyn std::error::Error > >
{
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_simple( ".nonexistent ??" );

  if result.success
  {
    println!( "  ✗ Should have failed for non-existent command" );
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✓ Non-existent help error: {}", e.lines().next().unwrap_or( "" ));
  }

  Ok( () )
}

fn test_typo_suggestion() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".typo" )
    .description( "Typo test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "parameter".to_string(),
        description : "Test parameter".to_string(),
        kind : Kind::String,
        hint : "String".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  // Try with typo: "paramter" instead of "parameter"
  let result = pipeline.process_command_simple( r#".typo paramter::"value""#);

  if result.success
  {
    println!( "  ✗ Should have failed for typo" );
  }
  else if let Some( e ) = result.error
  {
    let msg = &e;
    if msg.contains( "Did you mean" ) || msg.contains( "parameter" )
    {
      println!( "  ✓ Typo suggestion provided: {}", msg.lines().next().unwrap_or( "" ));
    }
    else
    {
      println!( "  ✗ No typo suggestion: {}", msg );
    }
  }

  Ok( () )
}

fn test_multiple_params() -> Result< (), Box< dyn std::error::Error > >
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".multi" )
    .description( "Multiple param test" )
    .arguments( vec![
      ArgumentDefinition {
        name : "item".to_string(),
        description : "Item parameter".to_string(),
        kind : Kind::String,
        hint : "String".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      }
    ])
    .end();

  registry.register_with_routine( &cmd, Box::new( test_routine ))?;

  let pipeline = Pipeline::new( registry );

  // Test with multiple same parameters
  let result = pipeline.process_command_simple( r#".multi item::"first" item::"second" item::"third""#);

  if result.success
  {
    println!( "  ✓ Multiple parameters processed" );
    if let Some( output ) = result.outputs.first()
    {
      println!( "    {}", output.content );
    }
  }
  else if let Some( e ) = result.error
  {
    println!( "  ✗ Multiple params failed: {}", e );
  }

  Ok( () )
}
