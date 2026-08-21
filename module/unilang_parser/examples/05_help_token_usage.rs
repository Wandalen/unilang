//! Help Token Usage Example
//!
//! This example demonstrates :
//! - The unquoted `??` help token as a positional argument
//! - Distinguishing a help request from the literal string `"??"` via `was_quoted`
//! - Contextual help requests alongside named arguments
//! - Any-position placement of the help token
//!
//! The parser itself carries no help state — it surfaces `??` as an ordinary
//! argument and records whether it was quoted. A semantic layer (e.g. `unilang`)
//! maps an unquoted `??` to help output.

use unilang_parser :: { Argument, Parser, UnilangParserOptions };

/// Returns true when the instruction carries an unquoted `??` positional — the
/// convention semantic layers use to detect a help request.
fn wants_help( positionals : &[ Argument ] ) -> bool
{
  positionals.iter().any( | arg | arg.value == "??" && !arg.was_quoted )
}

fn main() -> Result< (), Box< dyn core ::error ::Error > >
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Basic command help
  println!( "=== Basic Command Help ===" );
  let cmd = parser.parse_repl_input( "file.copy ??" )?;
  println!( "Command: {:?}", cmd.command_path_slices );
  println!( "Help requested: {}", wants_help( &cmd.positional_arguments ) );

  assert!( wants_help( &cmd.positional_arguments ) );
  assert_eq!( cmd.command_path_slices, [ "file", "copy" ] );

  // Contextual help with arguments
  println!( "\n=== Contextual Help with Arguments ===" );
  let cmd2 = parser.parse_repl_input( "database.migrate version :: 1.2.0 ??" )?;
  println!( "Command: {:?}", cmd2.command_path_slices );
  println!( "Help requested: {}", wants_help( &cmd2.positional_arguments ) );
  println!( "Context arguments: {:?}", cmd2.named_arguments );

  assert!( wants_help( &cmd2.positional_arguments ) );
  assert_eq!
  (
  cmd2.named_arguments
  .get( "version" )
  .map( | arg | &arg[0].value )
  .unwrap(),
  "1.2.0"
 );

  // Quoted "??" is a literal value, never a help request
  println!( "\n=== Quoted \"??\" Stays Literal ===" );
  let cmd3 = parser.parse_repl_input( "search.find pattern :: \"??\"" )?;
  let pattern = &cmd3.named_arguments.get( "pattern" ).unwrap()[ 0 ];
  println!( "Pattern value: {:?}, was_quoted: {}", pattern.value, pattern.was_quoted );

  assert_eq!( pattern.value, "??" );
  assert!( pattern.was_quoted );

  // Help token position is free — it may precede other arguments
  println!( "\n=== Any-Position Help Token ===" );
  let cmd4 = parser.parse_repl_input( "server.deploy ?? target ::production replicas :: 5" )?;
  println!( "Command: {:?}", cmd4.command_path_slices );
  println!( "Help requested: {}", wants_help( &cmd4.positional_arguments ) );
  println!( "Context arguments: {:?}", cmd4.named_arguments );

  assert!( wants_help( &cmd4.positional_arguments ) );
  assert_eq!( cmd4.named_arguments.len(), 2 );

  println!( "\n✓ Help token usage parsing successful!" );
  Ok( () )
}
