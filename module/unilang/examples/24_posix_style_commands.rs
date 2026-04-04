#![ allow( clippy::all ) ]
//! # POSIX-Style Command Interfaces
//!
//! **Note:** This example uses runtime registration for demonstration.
//! For production use, define commands in YAML and use compile-time generation.
//!
//! Demonstrates how to model traditional POSIX/Unix tool interfaces in unilang.
//! Named `name::value` arguments replace positional flags; single-letter aliases
//! mirror POSIX short forms.
//!
//! ## POSIX → Unilang Mapping
//!
//! | POSIX command                          | Unilang equivalent                                                     |
//! |----------------------------------------|------------------------------------------------------------------------|
//! | `ls /tmp`                              | `.fs.ls path::/tmp`                                                    |
//! | `ls -la /tmp`                          | `.fs.ls path::/tmp long::true all::true`                               |
//! | `ls -laRh /tmp`                        | `.fs.ls path::/tmp long::true all::true recursive::true human::true`   |
//! | `grep error src/`                      | `.fs.grep pattern::error path::src/`                                   |
//! | `grep -ri TODO src/`                   | `.fs.grep pattern::TODO recursive::true ignore_case::true path::src/`  |
//! | `grep -n 'fn main' src/main.rs`        | `.fs.grep pattern::'fn main' line_numbers::true path::src/main.rs`     |
//! | `grep -ric error src/`                 | `.fs.grep pattern::error recursive::true ignore_case::true count::true path::src/` |
//! | `wc src/lib.rs`                        | `.fs.wc path::src/lib.rs`                                              |
//! | `wc -l src/lib.rs`                     | `.fs.wc path::src/lib.rs lines::true`                                  |
//! | `wc -lw src/lib.rs`                    | `.fs.wc path::src/lib.rs lines::true words::true`                      |
//!
//! ## Key Design Principles
//!
//! - **Named args replace positional flags**: `-a` becomes `all::true`
//! - **Single-letter aliases mirror POSIX short flags**: argument `recursive` has alias `r`
//! - **Boolean Kind for toggle flags**: `-v` style switches use `Kind::Boolean`
//! - **Defaults match POSIX tool defaults**: `long::false` matches `ls` default behavior
//! - **Argv API for real CLIs**: use `process_command_from_argv_simple` when reading `std::env::args()`
//!
//! Run with: `cargo run --example 24_posix_style_commands`

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, Kind, ValidationRule };
use unilang::prelude::*;
use unilang::ExecutionContext;

#[ allow( clippy::too_many_lines ) ]
fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== POSIX-Style Command Interfaces ===" );
  println!( "Demonstrates how POSIX flags map to unilang name::value arguments" );
  println!();

  let mut registry = CommandRegistry::new();

  // =========================================================================
  // .fs.ls  —  list directory contents (mirrors `ls`)
  //
  // POSIX flag → unilang argument mapping:
  //   -a / --all              →  all::true        (alias: a)
  //   -l                      →  long::true        (alias: l)
  //   -R / --recursive        →  recursive::true   (alias: r)
  //   -h / --human-readable   →  human::true       (alias: h)
  //   [positional path]       →  path::…           (alias: p, default: ".")
  // =========================================================================

  let ls_cmd = CommandDefinition::former()
  .name( ".ls" )
  .namespace( ".fs" )
  .description( "List directory contents".to_string() )
  .hint( "Mirrors POSIX `ls [-laRh] [path]`" )
  .examples( vec!
  [
    ".fs.ls".to_string(),
    ".fs.ls path::/tmp".to_string(),
    ".fs.ls path::/tmp long::true all::true".to_string(),
    ".fs.ls path::. long::true all::true recursive::true human::true".to_string(),
  ])
  .arguments( vec!
  [
    ArgumentDefinition
    {
      name: "path".to_string(),
      description: "Directory to list (default: current directory)".to_string(),
      kind: Kind::String,
      hint: "Directory path".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( ".".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "p".to_string() ],
      tags: vec![ "filesystem".to_string() ],
    },
    ArgumentDefinition   // mirrors -a / --all
    {
      name: "all".to_string(),
      description: "Show hidden entries whose names begin with '.'".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -a / --all".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "a".to_string() ],
      tags: vec![ "display".to_string() ],
    },
    ArgumentDefinition   // mirrors -l
    {
      name: "long".to_string(),
      description: "Long format: permissions, links, owner, size, date".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -l".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "l".to_string() ],
      tags: vec![ "display".to_string() ],
    },
    ArgumentDefinition   // mirrors -R / --recursive
    {
      name: "recursive".to_string(),
      description: "Recurse into subdirectories".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -R / --recursive".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "r".to_string() ],
      tags: vec![ "behavior".to_string() ],
    },
    ArgumentDefinition   // mirrors -h / --human-readable
    {
      name: "human".to_string(),
      description: "Show sizes as 4.2K, 1.3M instead of raw bytes".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -h / --human-readable".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "h".to_string() ],
      tags: vec![ "display".to_string() ],
    },
  ])
  .end();

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &ls_cmd, Box::new( ls_routine ) )?;

  // =========================================================================
  // .fs.grep  —  search for a pattern in files (mirrors `grep`)
  //
  // POSIX flag → unilang argument mapping:
  //   [positional pattern]    →  pattern::…        (aliases: e, p)  [required]
  //   [positional path]       →  path::…            (alias: f, default: ".")
  //   -r / --recursive        →  recursive::true    (alias: r)
  //   -i / --ignore-case      →  ignore_case::true  (alias: i)
  //   -n / --line-number      →  line_numbers::true (alias: n)
  //   -c / --count            →  count::true        (alias: c)
  // =========================================================================

  let grep_cmd = CommandDefinition::former()
  .name( ".grep" )
  .namespace( ".fs" )
  .description( "Search for a pattern in files".to_string() )
  .hint( "Mirrors POSIX `grep [-rinc] pattern [path]`" )
  .examples( vec!
  [
    ".fs.grep pattern::error path::src/".to_string(),
    ".fs.grep pattern::TODO recursive::true ignore_case::true path::src/".to_string(),
    ".fs.grep pattern::main line_numbers::true path::src/main.rs".to_string(),
    ".fs.grep pattern::fn count::true recursive::true path::src/".to_string(),
  ])
  .arguments( vec!
  [
    ArgumentDefinition   // required — mirrors first positional arg
    {
      name: "pattern".to_string(),
      description: "Pattern to search for (supports regex)".to_string(),
      kind: Kind::String,
      hint: "Search pattern (required)".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength( 1 ) ],
      aliases: vec![ "e".to_string(), "p".to_string() ],   // -e is GNU grep's explicit pattern flag
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition   // mirrors subsequent positional(s)
    {
      name: "path".to_string(),
      description: "File or directory to search (default: current directory)".to_string(),
      kind: Kind::String,
      hint: "Search target path".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( ".".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "f".to_string() ],
      tags: vec![ "filesystem".to_string() ],
    },
    ArgumentDefinition   // mirrors -r / --recursive
    {
      name: "recursive".to_string(),
      description: "Recurse into subdirectories when path is a directory".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -r / --recursive".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "r".to_string() ],
      tags: vec![ "behavior".to_string() ],
    },
    ArgumentDefinition   // mirrors -i / --ignore-case
    {
      name: "ignore_case".to_string(),
      description: "Case-insensitive pattern matching".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -i / --ignore-case".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "i".to_string() ],
      tags: vec![ "behavior".to_string() ],
    },
    ArgumentDefinition   // mirrors -n / --line-number
    {
      name: "line_numbers".to_string(),
      description: "Prefix each matching line with its line number".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -n / --line-number".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "n".to_string() ],
      tags: vec![ "output".to_string() ],
    },
    ArgumentDefinition   // mirrors -c / --count
    {
      name: "count".to_string(),
      description: "Print only the count of matching lines per file".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -c / --count".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "c".to_string() ],
      tags: vec![ "output".to_string() ],
    },
  ])
  .end();

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &grep_cmd, Box::new( grep_routine ) )?;

  // =========================================================================
  // .fs.wc  —  count lines, words, and bytes (mirrors `wc`)
  //
  // POSIX flag → unilang argument mapping:
  //   [positional path]   →  path::…       (aliases: f, p)  [required]
  //   -l / --lines        →  lines::true   (alias: l)
  //   -w / --words        →  words::true   (alias: w)
  //   -c / --bytes        →  bytes::true   (alias: c)
  //
  // POSIX default (no flags): show all three counts — mirrored here.
  // =========================================================================

  let wc_cmd = CommandDefinition::former()
  .name( ".wc" )
  .namespace( ".fs" )
  .description( "Count lines, words, and bytes in a file".to_string() )
  .hint( "Mirrors POSIX `wc [-lwc] file`" )
  .examples( vec!
  [
    ".fs.wc path::src/main.rs".to_string(),
    ".fs.wc path::src/main.rs lines::true".to_string(),
    ".fs.wc path::src/main.rs lines::true words::true".to_string(),
  ])
  .arguments( vec!
  [
    ArgumentDefinition   // required — mirrors positional file argument
    {
      name: "path".to_string(),
      description: "File to count (required)".to_string(),
      kind: Kind::String,
      hint: "Input file path".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength( 1 ) ],
      aliases: vec![ "f".to_string(), "p".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition   // mirrors -l / --lines
    {
      name: "lines".to_string(),
      description: "Count newlines".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -l / --lines".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "l".to_string() ],
      tags: vec![ "output".to_string() ],
    },
    ArgumentDefinition   // mirrors -w / --words
    {
      name: "words".to_string(),
      description: "Count whitespace-delimited words".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -w / --words".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "w".to_string() ],
      tags: vec![ "output".to_string() ],
    },
    ArgumentDefinition   // mirrors -c / --bytes
    {
      name: "bytes".to_string(),
      description: "Count bytes".to_string(),
      kind: Kind::Boolean,
      hint: "Mirrors POSIX -c / --bytes".to_string(),
      attributes: ArgumentAttributes { optional: true, default: Some( "false".to_string() ), ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "c".to_string() ],
      tags: vec![ "output".to_string() ],
    },
  ])
  .end();

  #[ allow( deprecated ) ]
  registry.command_add_runtime( &wc_cmd, Box::new( wc_routine ) )?;

  // =========================================================================
  // Demo: side-by-side POSIX ↔ unilang
  // =========================================================================

  let pipeline = Pipeline::new( registry );

  let test_cases : &[ ( &str, &str ) ] = &[
    // ── ls ─────────────────────────────────────────────────────────────────
    ( "ls /tmp",
      ".fs.ls path::/tmp" ),
    ( "ls -la /tmp",
      ".fs.ls path::/tmp long::true all::true" ),
    ( "ls -laRh /tmp",
      ".fs.ls path::/tmp long::true all::true recursive::true human::true" ),
    // ── grep ───────────────────────────────────────────────────────────────
    ( "grep error src/",
      ".fs.grep pattern::error path::src/" ),
    ( "grep -ri TODO src/",
      ".fs.grep pattern::TODO recursive::true ignore_case::true path::src/" ),
    ( "grep -n 'fn main' src/main.rs",
      ".fs.grep pattern::'fn main' line_numbers::true path::src/main.rs" ),
    ( "grep -ric error src/",
      ".fs.grep pattern::error recursive::true ignore_case::true count::true path::src/" ),
    // ── wc ─────────────────────────────────────────────────────────────────
    ( "wc src/lib.rs",
      ".fs.wc path::src/lib.rs" ),
    ( "wc -l src/lib.rs",
      ".fs.wc path::src/lib.rs lines::true" ),
    ( "wc -lw src/lib.rs",
      ".fs.wc path::src/lib.rs lines::true words::true" ),
  ];

  println!( "=== Side-by-side: POSIX ↔ unilang ===" );
  println!();
  for ( posix, unilang ) in test_cases
  {
    println!( "  POSIX   : {posix}" );
    println!( "  unilang : {unilang}" );
    let result = pipeline.process_command_simple( unilang );
    if result.success
    {
      for output in &result.outputs
      {
        println!( "  Output  : {}", output.content );
      }
    }
    else
    {
      println!( "  Error   : {}", result.error.unwrap_or_default() );
    }
    println!();
  }

  // =========================================================================
  // Real CLI entry point: process_command_from_argv_simple
  //
  // For real CLI apps, pass argv directly instead of a joined string.
  // The OS delivers tokens as separate elements — no re-tokenization needed.
  //
  // Shell:  ./my_app .fs.ls path::/tmp long::true all::true
  // OS argv: [ ".fs.ls", "path::/tmp", "long::true", "all::true" ]
  // =========================================================================

  println!( "=== Real CLI Entry Point Pattern ===" );
  println!();

  let argv : Vec< String > = vec!
  [
    ".fs.ls".to_string(),
    "path::/tmp".to_string(),
    "long::true".to_string(),
    "all::true".to_string(),
  ];

  println!( "  Shell   : ./my_app .fs.ls path::/tmp long::true all::true" );
  println!( "  Argv    : {:?}", argv );

  let result = pipeline.process_command_from_argv_simple( &argv );
  if result.success
  {
    for output in &result.outputs
    {
      println!( "  Output  : {}", output.content );
    }
  }
  else
  {
    println!( "  Error   : {}", result.error.unwrap_or_default() );
  }

  // =========================================================================
  // Summary
  // =========================================================================

  println!();
  println!( "=== Design Principles ===" );
  println!( "  Boolean flags    : all::true  long::true  →  mirrors -a  -l" );
  println!( "  Single-letter    : argument 'recursive' has alias 'r'  →  mirrors -r" );
  println!( "  Named required   : pattern::error  →  replaces first positional" );
  println!( "  Defaults match   : all::false  →  ls default (no hidden files)" );
  println!( "  Argv API         : process_command_from_argv_simple for real CLI apps" );

  Ok( () )
}

// =============================================================================
// Routines
// =============================================================================

fn ls_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let path = str_arg( &cmd, "path", "." );
  let all = bool_arg( &cmd, "all" );
  let long = bool_arg( &cmd, "long" );
  let recursive = bool_arg( &cmd, "recursive" );
  let human = bool_arg( &cmd, "human" );

  let mut flags = String::new();
  if all       { flags.push( 'a' ); }
  if long      { flags.push( 'l' ); }
  if recursive { flags.push( 'R' ); }
  if human     { flags.push( 'h' ); }

  let posix = if flags.is_empty() { format!( "ls {path}" ) } else { format!( "ls -{flags} {path}" ) };
  println!( "    [ls] {posix}  (all={all} long={long} recursive={recursive} human={human})" );
  Ok( OutputData { content: posix, format: "text".to_string(), execution_time_ms: None } )
}

fn grep_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let pattern = str_arg( &cmd, "pattern", "" );
  let path = str_arg( &cmd, "path", "." );
  let recursive = bool_arg( &cmd, "recursive" );
  let ignore_case = bool_arg( &cmd, "ignore_case" );
  let line_numbers = bool_arg( &cmd, "line_numbers" );
  let count = bool_arg( &cmd, "count" );

  let mut flags = String::new();
  if recursive    { flags.push( 'r' ); }
  if ignore_case  { flags.push( 'i' ); }
  if line_numbers { flags.push( 'n' ); }
  if count        { flags.push( 'c' ); }

  let posix = if flags.is_empty()
  {
    format!( "grep '{pattern}' {path}" )
  }
  else
  {
    format!( "grep -{flags} '{pattern}' {path}" )
  };
  println!( "    [grep] {posix}" );
  Ok( OutputData { content: posix, format: "text".to_string(), execution_time_ms: None } )
}

fn wc_routine( cmd : VerifiedCommand, _ctx : ExecutionContext ) -> Result< OutputData, ErrorData >
{
  let path = str_arg( &cmd, "path", "" );
  let lines = bool_arg( &cmd, "lines" );
  let words = bool_arg( &cmd, "words" );
  let bytes = bool_arg( &cmd, "bytes" );

  // No flags → POSIX default: show all three counts
  let mut flags = String::new();
  if lines { flags.push( 'l' ); }
  if words { flags.push( 'w' ); }
  if bytes { flags.push( 'c' ); }

  let posix = if flags.is_empty() { format!( "wc {path}" ) } else { format!( "wc -{flags} {path}" ) };
  println!( "    [wc] {posix}" );
  Ok( OutputData { content: posix, format: "text".to_string(), execution_time_ms: None } )
}

// ---- argument extraction helpers ----

fn str_arg< 'a >( cmd : &'a VerifiedCommand, name : &str, default : &'a str ) -> &'a str
{
  cmd.arguments.get( name )
  .and_then( | v | if let Value::String( s ) = v { Some( s.as_str() ) } else { None } )
  .unwrap_or( default )
}

fn bool_arg( cmd : &VerifiedCommand, name : &str ) -> bool
{
  cmd.arguments.get( name )
  .and_then( | v | if let Value::Boolean( b ) = v { Some( *b ) } else { None } )
  .unwrap_or( false )
}
