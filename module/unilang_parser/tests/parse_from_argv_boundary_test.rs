//! `parse_from_argv` token-boundary absorption tests
//!
//! Tests the greedy-absorption bug (issue-087) in `parse_from_argv` where bare
//! positional argv tokens following a named parameter were silently concatenated
//! into that parameter's value.
//!
//! ## Test Matrix
//!
//! | # | Scenario | Input | Expected |
//! |---|----------|-------|----------|
//! | T01 | path value + bare positional | `[".add", "repo::Wandalen/willbe", "willbe/assistant"]` | repo="Wandalen/willbe", 1 positional |
//! | T02 | two named params (regression) | `[".add", "repo::Wandalen/willbe", "path::mydir"]` | repo="Wandalen/willbe", path="mydir" |
//! | T03 | multi-word plain value (regression) | `[".cmd", "message::hello", "world"]` | message="hello world" |
//! | T04 | path value + two bare positionals | `[".add", "repo::Wandalen/willbe", "extra1", "extra2"]` | repo="Wandalen/willbe", 2 positionals |
//! | T05 | SSH URL as single token (regression) | `[".add", "repo::git@github.com:user/repo.git"]` | repo="git@github.com:user/repo.git" |
//! | T06 | no params (regression guard) | `[".status"]` | command="status", no args |
//!
//! ## Corner Cases Covered
//!
//! - ✅ Path/URL values (containing `/`) — absorption stops immediately (fix)
//! - ✅ Plain-text values (no `/`) — multi-word absorption continues (preserved)
//! - ✅ Two named params — `::` in next arg still triggers break (regression)
//! - ✅ SSH-style URL as single argv element (no absorption possible)
//! - ✅ Multiple bare positionals after path value
//! - ✅ No absorption at all (command with no params)
//!
//! ## Root Cause
//!
//! `parse_from_argv` in `parser_engine/mod.rs` uses a `while` loop to absorb
//! subsequent argv elements into the current named parameter's value. The loop
//! only stopped on `::` (another named arg) or a `.`-prefixed arg (command).
//! Any bare token — including a separate positional path like `willbe/assistant`
//! — was silently concatenated with a space, corrupting both the named param and
//! losing the positional.
//!
//! ## Why Not Caught Initially
//!
//! - Multi-word absorption is intentional for plain-text values (`message::hello world`);
//!   path values containing `/` were never treated as a special case.
//! - No test covered the mixed scenario: named param with path value + bare positional.
//! - Silent corruption (corrupted URL) only surfaced at call-site (`git clone`), not parser.
//!
//! ## Fix Applied
//!
//! Added a stop condition inside the absorption loop: if the accumulated value
//! already contains `/`, break before absorbing the next token. Path/URL values
//! are complete in their first token; plain-text values have no `/` and continue
//! to absorb normally.
//!
//! Location: `src/parser_engine/mod.rs` absorption loop (after `starts_with('.')` check).
//!
//! ## Prevention
//!
//! - Test named params with path-like values explicitly alongside bare positionals.
//! - Treat `/` as a "self-contained token" signal when designing absorption heuristics.
//! - Any new stop condition must have both a failing test (before fix) and a regression
//!   guard confirming the preserved behavior (multi-word plain values).
//!
//! ## Pitfall
//!
//! **Multi-word absorption is intentional** — do not remove it. The fix must be
//! surgical: only stop absorption when the accumulated value signals it is
//! already complete (e.g., contains `/`). Removing the absorption loop entirely
//! would break `message::hello world` style params.

use unilang_parser::{ Parser, UnilangParserOptions };

// test_kind: bug_reproducer(issue-087)
#[test]
fn test_parse_from_argv_no_greedy_absorption_path_value()
{
  // T01: bare positional after path value must not be absorbed
  // Currently broken: produces repo="Wandalen/willbe willbe/assistant"
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "repo::Wandalen/willbe".to_string(),
    "willbe/assistant".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let repo = instruction.named_arguments.get( "repo" )
    .expect( "repo param must exist" );
  assert_eq!(
    repo[ 0 ].value, "Wandalen/willbe",
    "repo value must be exactly 'Wandalen/willbe', not greedily absorb next token"
  );

  assert_eq!(
    instruction.positional_arguments.len(), 1,
    "willbe/assistant must become a positional argument, not be absorbed into repo"
  );
  assert_eq!(
    instruction.positional_arguments[ 0 ].value, "willbe/assistant",
    "positional value must be 'willbe/assistant'"
  );
}

// test_kind: regression_prevention(issue-087)
#[test]
fn test_parse_from_argv_two_named_params()
{
  // T02: second named param with '::' must still stop absorption (regression)
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "repo::Wandalen/willbe".to_string(),
    "path::mydir".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let repo = instruction.named_arguments.get( "repo" )
    .expect( "repo param must exist" );
  assert_eq!( repo[ 0 ].value, "Wandalen/willbe", "repo must be 'Wandalen/willbe'" );

  let path = instruction.named_arguments.get( "path" )
    .expect( "path param must exist" );
  assert_eq!( path[ 0 ].value, "mydir", "path must be 'mydir'" );

  assert_eq!( instruction.positional_arguments.len(), 0, "no positionals expected" );
}

// test_kind: regression_prevention(issue-087)
#[test]
fn test_parse_from_argv_multiword_plain_value_preserved()
{
  // T03: multi-word absorption must still work for plain-text values (no '/' in value)
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "message::hello".to_string(),
    "world".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let message = instruction.named_arguments.get( "message" )
    .expect( "message param must exist" );
  assert_eq!(
    message[ 0 ].value, "hello world",
    "multi-word plain value must still be absorbed (message::hello world)"
  );

  assert_eq!( instruction.positional_arguments.len(), 0, "no positionals expected" );
}

// test_kind: bug_reproducer(issue-087)
#[test]
fn test_parse_from_argv_two_positionals_after_path_value()
{
  // T04: two bare positionals after path value — both must become separate positionals
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "repo::Wandalen/willbe".to_string(),
    "extra1".to_string(),
    "extra2".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let repo = instruction.named_arguments.get( "repo" )
    .expect( "repo param must exist" );
  assert_eq!( repo[ 0 ].value, "Wandalen/willbe", "repo must not absorb extra1/extra2" );

  assert_eq!(
    instruction.positional_arguments.len(), 2,
    "both extra1 and extra2 must be separate positional arguments"
  );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "extra1" );
  assert_eq!( instruction.positional_arguments[ 1 ].value, "extra2" );
}

// test_kind: regression_prevention(issue-087)
#[test]
fn test_parse_from_argv_ssh_url_single_token()
{
  // T05: SSH-style URL as single argv element — no absorption involved (regression)
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "repo::git@github.com:Wandalen/willbe.git".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let repo = instruction.named_arguments.get( "repo" )
    .expect( "repo param must exist" );
  assert_eq!(
    repo[ 0 ].value, "git@github.com:Wandalen/willbe.git",
    "SSH URL value must be preserved as-is"
  );

  assert_eq!( instruction.positional_arguments.len(), 0, "no positionals expected" );
}

// test_kind: regression_prevention(issue-087)
#[test]
fn test_parse_from_argv_no_params()
{
  // T06: command with no params — no absorption possible (regression guard)
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".status".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  assert_eq!( instruction.named_arguments.len(), 0, "no named args expected" );
  assert_eq!( instruction.positional_arguments.len(), 0, "no positionals expected" );
  assert!( instruction.command_path_slices.contains( &"status".to_string() ) );
}
