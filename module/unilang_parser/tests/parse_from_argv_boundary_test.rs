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
//! | T07 | help token after named value | `[".add", "a::1", "??"]` | a="1", 1 positional `??` unquoted |
//! | T08 | help token after `name::??` | `[".add", "a::??", "??"]` | a="??", 1 positional `??` unquoted |
//! | T09 | `?`-bearing multiword + `??` near-miss | `[".cmd", "message::what", "time?", "??x"]` | message="what time? ??x" |
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
//!
//! ---
//!
//! ## Second Defect: `??` help token absorbed into named value (manual-test-2026-08-20)
//!
//! ### Root Cause
//!
//! The absorption loop broke on `::`, dot-prefixed tokens, and path-bearing values,
//! but a standalone `??` argv element — the positional help token — matched none of
//! those, so `app .cmd a::1 ??` glued into `a::"1 ??"` and the help request surfaced
//! as a coercion error on `a` instead of the command help page.
//!
//! ### Why Not Caught
//!
//! All `??`-routing tests exercised the in-process string path (`pipeline.run(".cmd a::1 ??")`),
//! where the tokenizer naturally yields `??` as its own token. The argv path has its own
//! absorption heuristic that the string path lacks, and no argv test combined a named
//! value with a trailing `??` — only CLI binary probes hit the divergence.
//!
//! ### Fix Applied
//!
//! Added a stop condition in the absorption loop: an argv element that is exactly `??`
//! always breaks absorption and becomes its own (positional, unquoted) token.
//! Location: `src/parser_engine/mod.rs` absorption loop (after `starts_with('.')` check).
//!
//! ### Prevention
//!
//! Every semantic-layer token with positional meaning (`??`) must have an argv-path
//! parity test, not just string-path coverage — the two tokenizations are separate code.
//!
//! ### Pitfall
//!
//! Only the exact `??` breaks. A `?` inside multiword text (`message::what time?`) and
//! near-misses (`??x`) must continue absorbing — they are ordinary value fragments.
//! A shell-quoted literal (`'"??"'`) arrives with inner quote characters, never equals
//! bare `??`, and also continues absorbing.

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

// test_kind: bug_reproducer(manual-test-2026-08-20)
#[test]
fn test_parse_from_argv_help_token_after_named_value()
{
  // T07: standalone `??` after a named value must not be absorbed —
  // it is the positional help token and must survive as its own unquoted positional.
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "a::1".to_string(),
    "??".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let a = instruction.named_arguments.get( "a" )
    .expect( "a param must exist" );
  assert_eq!(
    a[ 0 ].value, "1",
    "a must be exactly '1' — the trailing `??` help token must not be absorbed"
  );

  assert_eq!(
    instruction.positional_arguments.len(), 1,
    "`??` must become its own positional argument (the help token)"
  );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "??" );
  assert!(
    !instruction.positional_arguments[ 0 ].was_quoted,
    "the `??` positional must stay unquoted so help detection recognizes it"
  );
}

// test_kind: bug_reproducer(manual-test-2026-08-20)
#[test]
fn test_parse_from_argv_help_token_after_named_help_value()
{
  // T08: `a::??` followed by standalone `??` — the named `??` value stays intact
  // and the trailing `??` still becomes its own positional help token.
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".add".to_string(),
    "a::??".to_string(),
    "??".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let a = instruction.named_arguments.get( "a" )
    .expect( "a param must exist" );
  assert_eq!( a[ 0 ].value, "??", "a must keep its `??` help value" );
  assert!(
    !a[ 0 ].was_quoted,
    "the named `??` value must stay unquoted so parameter-help detection recognizes it"
  );

  assert_eq!(
    instruction.positional_arguments.len(), 1,
    "the trailing `??` must become its own positional argument"
  );
  assert_eq!( instruction.positional_arguments[ 0 ].value, "??" );
}

// test_kind: regression_prevention(manual-test-2026-08-20)
#[test]
fn test_parse_from_argv_question_fragments_still_absorb()
{
  // T09: only the exact `??` breaks absorption — a `?`-suffixed word and a `??x`
  // near-miss are ordinary value fragments and must continue multi-word absorption.
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "message::what".to_string(),
    "time?".to_string(),
    "??x".to_string(),
  ]);

  assert!( result.is_ok(), "parse_from_argv must succeed: {:?}", result.err() );
  let instruction = result.unwrap();

  let message = instruction.named_arguments.get( "message" )
    .expect( "message param must exist" );
  assert_eq!(
    message[ 0 ].value, "what time? ??x",
    "`?`-bearing fragments that are not exactly `??` must keep absorbing"
  );

  assert_eq!( instruction.positional_arguments.len(), 0, "no positionals expected" );
}
