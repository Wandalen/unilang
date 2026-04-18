//! Tests for `ShellArgv` and `ReplInput` marker newtypes and their
//! corresponding type-safe parser entry points `parse_cli` and `parse_repl`.
//!
//! ## Design Notes
//!
//! ### argv[0] is part of the command path
//! `parse_cli` passes the full `ShellArgv` slice (including `argv[0]`, the
//! program name) to `parse_from_argv`. The parser treats every token as a
//! potential command-path segment or argument — it does NOT strip `argv[0]`.
//! Tests that exercise `parse_cli` therefore include `"prog"` as the first
//! element; `"prog"` becomes the first command-path segment in the result.
//! Callers responsible for stripping `argv[0]` should do so before wrapping
//! into `ShellArgv` if their command model does not expect the program name.
//!
//! ### Delegation chain
//! `parse_cli` → `parse_from_argv` → `parse_repl_input` (internal).
//! `parse_repl` → `parse_repl_input` (direct).
//! `parse_single_instruction` (deprecated) → `parse_repl_input` (forwarding shim).
//! All three type-safe entry points are thin wrappers; behaviour tests live in
//! the existing parser test suite.  These tests verify:
//!   (a) the delegation wire is connected correctly (result equality checks),
//!   (b) semantic content survives the wrapper (named args, help flag),
//!   (c) errors propagate correctly through both wrappers, and
//!   (d) constructor and Clone behaviour of the newtypes themselves.
//!
//! ## Test Matrix
//!
//! | Scenario | Input Type | Entry Point | Expected |
//! |----------|-----------|-------------|---------|
//! | Valid argv via parse_cli | `ShellArgv::from_vec` | `parse_cli` | Ok |
//! | Valid string via parse_repl | `ReplInput::new` | `parse_repl` | Ok |
//! | ShellArgv From<Vec<String>> | `Vec::into()` | constructors | compiles |
//! | ReplInput From<&str> | `&str.into()` | constructors | compiles |
//! | ReplInput From<String> | `String.into()` | constructors | compiles |
//! | Empty argv | `ShellArgv::from_vec([])` | `parse_cli` | Ok (empty result) |
//! | parse_repl == parse_repl_input | `ReplInput` | delegation | identical result |
//! | parse_cli == parse_from_argv | `ShellArgv` | delegation | identical result |
//! | deprecated shim == parse_repl_input | `&str` | deprecated | identical result |
//! | Named arg key/value via parse_cli | `ShellArgv` | `parse_cli` | named_args["key"]=="value" |
//! | Named arg key/value via parse_repl | `ReplInput` | `parse_repl` | named_args["key"]=="value" |
//! | Help flag via parse_repl | `ReplInput("cmd ?")` | `parse_repl` | help_requested==true |
//! | Help flag via parse_cli | `ShellArgv(["prog",".cmd","?"])` | `parse_cli` | help_requested==true |
//! | Empty ReplInput | `ReplInput::new("")` | `parse_repl` | Ok (empty instruction) |
//! | Whitespace-only ReplInput | `ReplInput::new("   ")` | `parse_repl` | Ok (empty instruction) |
//! | Unclosed quote via parse_repl | `ReplInput(unclosed)` | `parse_repl` | Err |
//! | Single-colon arg via parse_cli | `ShellArgv(["bad:x"])` | `parse_cli` | Err |
//! | ShellArgv clone | `original.clone()` | constructors | same content |
//! | ReplInput clone | `original.clone()` | constructors | same content |

use unilang_parser ::{ Parser, UnilangParserOptions, ShellArgv, ReplInput };

// ---------------------------------------------------------------------------
// Constructor / round-trip tests
// ---------------------------------------------------------------------------

#[ test ]
fn shell_argv_round_trips_via_parse_cli()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let argv = ShellArgv::from_vec(
    vec![ "prog".into(), ".cmd".into(), "key::value".into() ]
  );
  let result = parser.parse_cli( &argv );
  assert!( result.is_ok(), "parse_cli must succeed for valid argv, got: {:?}", result );
}

#[ test ]
fn repl_input_round_trips_via_parse_repl()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ReplInput::new( ".cmd key::value" );
  let result = parser.parse_repl( &input );
  assert!( result.is_ok(), "parse_repl must succeed for valid string, got: {:?}", result );
}

#[ test ]
fn shell_argv_from_vec_conversion()
{
  let argv : ShellArgv = vec![ "prog".to_owned(), ".cmd".to_owned() ].into();
  assert_eq!( argv.as_slice().len(), 2, "from Vec conversion must preserve all elements" );
}

#[ test ]
fn repl_input_from_str_conversion()
{
  let input : ReplInput = ".cmd key::val".into();
  assert_eq!( input.as_str(), ".cmd key::val", "from &str must preserve content" );
}

#[ test ]
fn repl_input_from_string_conversion()
{
  let input : ReplInput = ".cmd key::val".to_owned().into();
  assert_eq!( input.as_str(), ".cmd key::val", "from String must preserve content" );
}

#[ test ]
fn shell_argv_empty_yields_empty_instruction()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let argv = ShellArgv::from_vec( vec![] );
  let result = parser.parse_cli( &argv );
  assert!( result.is_ok(), "empty argv must produce Ok result" );
  let instr = result.unwrap();
  assert!( instr.command_path_slices.is_empty(), "empty argv must yield empty command path" );
}

// ---------------------------------------------------------------------------
// Delegation wire tests — result equality confirms wrappers delegate correctly
// ---------------------------------------------------------------------------

/// `parse_repl` must be a transparent wrapper around `parse_repl_input`.
/// Any divergence would indicate the unwrapping logic is broken.
#[ test ]
fn parse_repl_delegates_to_parse_repl_input()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let raw = "cmd key::value";
  let expected = parser.parse_repl_input( raw ).expect( "parse_repl_input must succeed" );
  let actual = parser.parse_repl( &ReplInput::new( raw ) ).expect( "parse_repl must succeed" );
  assert_eq!(
    actual, expected,
    "parse_repl must produce identical result to parse_repl_input"
  );
}

/// `parse_cli` must be a transparent wrapper around `parse_from_argv`.
/// Any divergence would indicate the `as_slice()` unwrapping is broken.
#[ test ]
fn parse_cli_delegates_to_parse_from_argv()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let argv : Vec< String > = vec![ "prog".into(), ".cmd".into(), "key::value".into() ];
  let expected = parser.parse_from_argv( &argv ).expect( "parse_from_argv must succeed" );
  let shell_argv = ShellArgv::from_vec( argv );
  let actual = parser.parse_cli( &shell_argv ).expect( "parse_cli must succeed" );
  assert_eq!(
    actual, expected,
    "parse_cli must produce identical result to parse_from_argv"
  );
}

/// The deprecated forwarding shim must produce an identical result to the
/// canonical method.  This proves the delegation body `parse_single_instruction
/// → parse_repl_input` was not accidentally changed.
#[ test ]
#[ allow( deprecated ) ]
fn deprecated_shim_matches_parse_repl_input()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let raw = "cmd key::value";
  let expected = parser.parse_repl_input( raw ).expect( "parse_repl_input must succeed" );
  let actual = parser.parse_single_instruction( raw )
    .expect( "deprecated shim must succeed" );
  assert_eq!(
    actual, expected,
    "deprecated shim must produce identical result to parse_repl_input"
  );
}

// ---------------------------------------------------------------------------
// Semantic correctness — named args and help flag survive the wrappers
// ---------------------------------------------------------------------------

/// Named argument key and value must be correctly parsed through `parse_cli`.
#[ test ]
fn parse_cli_extracts_named_argument()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let argv = ShellArgv::from_vec(
    vec![ "prog".into(), ".cmd".into(), "key::value".into() ]
  );
  let instr = parser.parse_cli( &argv ).expect( "parse_cli must succeed" );
  assert!(
    instr.named_arguments.contains_key( "key" ),
    "named argument 'key' must be present, got args: {:?}", instr.named_arguments
  );
  assert_eq!(
    instr.named_arguments[ "key" ][ 0 ].value, "value",
    "named argument 'key' must have value 'value'"
  );
}

/// Named argument key and value must be correctly parsed through `parse_repl`.
#[ test ]
fn parse_repl_extracts_named_argument()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ReplInput::new( "cmd key::value" );
  let instr = parser.parse_repl( &input ).expect( "parse_repl must succeed" );
  assert!(
    instr.named_arguments.contains_key( "key" ),
    "named argument 'key' must be present, got args: {:?}", instr.named_arguments
  );
  assert_eq!(
    instr.named_arguments[ "key" ][ 0 ].value, "value",
    "named argument 'key' must have value 'value'"
  );
}

/// `help_requested` must be set when `?` follows the command through `parse_repl`.
#[ test ]
fn parse_repl_sets_help_requested_flag()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ReplInput::new( "cmd ?" );
  let instr = parser.parse_repl( &input ).expect( "parse_repl must succeed" );
  assert!(
    instr.help_requested,
    "parse_repl must set help_requested for '?' operator"
  );
}

/// `help_requested` must be set when `?` is present in argv through `parse_cli`.
#[ test ]
fn parse_cli_sets_help_requested_flag()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let argv = ShellArgv::from_vec(
    vec![ "prog".into(), ".cmd".into(), "?".into() ]
  );
  let instr = parser.parse_cli( &argv ).expect( "parse_cli must succeed" );
  assert!(
    instr.help_requested,
    "parse_cli must set help_requested for '?' operator"
  );
}

// ---------------------------------------------------------------------------
// Boundary: empty and whitespace-only inputs
// ---------------------------------------------------------------------------

/// `ReplInput::new("")` → `parse_repl` must yield an empty instruction, not Err.
#[ test ]
fn repl_input_empty_string_yields_empty_instruction()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ReplInput::new( "" );
  let result = parser.parse_repl( &input );
  assert!( result.is_ok(), "empty ReplInput must produce Ok, got: {:?}", result );
  let instr = result.unwrap();
  assert!(
    instr.command_path_slices.is_empty(),
    "empty ReplInput must yield empty command path"
  );
}

/// `ReplInput` wrapping only whitespace must yield an empty instruction.
#[ test ]
fn repl_input_whitespace_only_yields_empty_instruction()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let input = ReplInput::new( "   " );
  let result = parser.parse_repl( &input );
  assert!( result.is_ok(), "whitespace-only ReplInput must produce Ok, got: {:?}", result );
  let instr = result.unwrap();
  assert!(
    instr.command_path_slices.is_empty(),
    "whitespace-only ReplInput must yield empty command path"
  );
}

// ---------------------------------------------------------------------------
// Error propagation — errors from the underlying parser must surface
// ---------------------------------------------------------------------------

/// An unclosed double-quote must propagate as Err through `parse_repl`.
/// Validates that `parse_repl` does not swallow errors from `parse_repl_input`.
#[ test ]
fn parse_repl_propagates_error_for_unclosed_quote()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  // Deliberately unclosed double quote after ::
  let input = ReplInput::new( "cmd key::\"unclosed" );
  let result = parser.parse_repl( &input );
  assert!( result.is_err(), "unclosed quote must propagate Err through parse_repl" );
}

/// An argv element with a single colon (invalid syntax) must produce Err through `parse_cli`.
/// Validates that `parse_cli` does not swallow errors from `parse_from_argv`.
#[ test ]
fn parse_cli_propagates_error_for_single_colon_arg()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  // Single colon is invalid; must use "::" for named args
  let argv = ShellArgv::from_vec(
    vec![ "prog".into(), ".cmd".into(), "bad:syntax".into() ]
  );
  let result = parser.parse_cli( &argv );
  assert!( result.is_err(), "single-colon arg must propagate Err through parse_cli" );
}

// ---------------------------------------------------------------------------
// Clone behaviour
// ---------------------------------------------------------------------------

/// `ShellArgv::clone()` must produce a copy with identical content.
#[ test ]
fn shell_argv_clone_is_independent()
{
  let original = ShellArgv::from_vec( vec![ "prog".into(), ".cmd".into() ] );
  let cloned = original.clone();
  assert_eq!(
    original.as_slice(), cloned.as_slice(),
    "clone must preserve all elements"
  );
}

/// `ReplInput::clone()` must produce a copy with identical content.
#[ test ]
fn repl_input_clone_is_independent()
{
  let original = ReplInput::new( ".cmd key::val" );
  let cloned = original.clone();
  assert_eq!(
    original.as_str(), cloned.as_str(),
    "clone must preserve content"
  );
}
