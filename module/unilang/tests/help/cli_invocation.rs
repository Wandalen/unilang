//! Tests for help generation and discovery.
//!
//! This module contains integration tests that invoke the `unilang_cli` binary
//! with help flags/commands and assert on the content and format of the generated help output.

use predicates::prelude::*;

use predicates::Predicate;

fn contains_all_unordered( expected_lines : Vec< &str > ) -> impl Predicate< str > + '_
{
  predicate::function( move | s : &str | expected_lines.iter().all( | line | s.contains( line ) ) )
}

// Test Matrix for Help Generation
//
// Factors:
// - Help Command: "--help", "help", "help <command_name>", "help <non_existent_command>"
// - Expected Output: stdout (list of commands, specific command help), stderr (error messages), exit code
//
// Combinations:
//
// | ID    | Command Invocation | Expected Stdout (contains)                               | Expected Stderr (contains)                               | Expected Exit Code | Notes                                     |
// |-------|--------------------|----------------------------------------------------------|----------------------------------------------------------|--------------------|-------------------------------------------|
// | T8.1  | `unilang_cli`      | "Available Commands:\n  echo\n  add\n  cat"             | "Usage: unilang_cli <command> [args...]"                 | 0                  | Basic echo command                        |
// | T8.2  | `unilang_cli --help` | "Available Commands:\n  echo\n  add\n  cat"             |                                                          | 0                  | Global help, lists all commands           |
// | T8.3  | `unilang_cli help` | "Available Commands:\n  echo\n  add\n  cat"             |                                                          | 0                  | Global help, lists all commands (alias)   |
// | T8.4  | `unilang_cli help echo` | "Usage: echo\n\n  Echoes a message."                 |                                                          | 0                  | Specific command help                     |
// | T8.5  | `unilang_cli help add` | "Usage: add\n\n  Adds two integers.\n\nArguments:\n  a              (Kind: Integer)\n  b              (Kind: Integer)" |                                                          | 0                  | Specific command help with arguments      |
// | T8.6  | `unilang_cli help non_existent` |                                                          | "Error: Command 'non_existent' not found for help."      | 1                  | Help for non-existent command             |
// | T8.7  | `unilang_cli help arg1 arg2` |                                                          | "Error: Invalid usage of help command."                  | 1                  | Invalid help command usage                |
// | T8.8  | `unilang_cli .math.add.help` | "Usage:", ".math.add", "Adds two numbers"               |                                                          | 0                  | Spelled help route prints a non-empty page |
// | T8.9  | `unilang_cli .math.add a::1 ??` | "Usage:", "Adds two numbers"                          |                                                          | 0                  | `??` after a named value is help, not a coercion error |
// | T8.10 | `.math.add ??` vs `.math.add.help` | identical stdout                                    |                                                          | 0                  | Both help routes are byte-identical        |
// | T8.11 | `unilang_cli` (no args) | "Use '<command> ??' or '<command>.help'", "Example: .list ??" | "Usage: unilang_cli <command> [args...]"           | 0                  | Listing footer advertises the `??` token   |

#[ test ]
fn test_cli_no_args_help()
{
  // Test Matrix Row: T8.1
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( predicate::str::contains( "Usage: unilang_cli <command> [args...]" ) );
}

#[ test ]
fn test_cli_global_help_flag()
{
  // Test Matrix Row: T8.2
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.arg( "--help" );
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_global_help_command()
{
  // Test Matrix Row: T8.3
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.arg( "help" );
  cmd
  .assert()
  .success()
  .stdout( contains_all_unordered( vec![
    "Available commands:",
    "  .math.add            Adds two numbers.",
    "  .math.sub            Subtracts two numbers.",
    "  .greet               Greets the specified person.",
    "  .config.set          Sets a configuration value.",
  ]) )
  .stderr( "" ); // No stderr for successful help
}

#[ test ]
fn test_cli_specific_command_help_add()
{
  // Test Matrix Row: T8.5
  // Note: Using Level 2 (Standard) verbosity by default - improved readability format
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.args( vec![ "help", ".math.add" ] );
  cmd
  .assert()
  .success()
  .stdout(
    predicate::str::contains( "Usage:" )
    .and( predicate::str::contains( ".add" ) )
    .and( predicate::str::contains( "Adds two numbers" ) )
    .and( predicate::str::contains( "Arguments:" ) )
    .and( predicate::str::contains( "a" ) )
    .and( predicate::str::contains( "Type: integer" ) )
    .and( predicate::str::contains( "First number" ) )
    .and( predicate::str::contains( "b" ) )
    .and( predicate::str::contains( "Second number" ) ),
  )
  .stderr( "" );
}

#[ test ]
fn test_cli_help_non_existent_command()
{
  // Test Matrix Row: T8.6
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.args( vec![ "help", "non_existent" ] );
  cmd
  .assert()
  .failure()
  .stderr( predicate::str::contains( "Error: Command 'non_existent' not found for help." ) );
}

#[ test ]
fn test_cli_invalid_help_usage()
{
  // Test Matrix Row: T8.7
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.args( vec![ "help", "arg1", "arg2" ] );
  cmd.assert().failure().stderr( predicate::str::contains(
    "Error: Invalid usage of help command. Use `help` or `help <command_name>`.",
  ) );
}

// test_kind: bug_reproducer(manual-test-2026-08-20)
#[ test ]
fn test_cli_spelled_help_route_prints_page()
{
  // Test Matrix Row: T8.8
  // The `.command.help` routine only RETURNS its page; the binary must print the
  // interpreter's returned outputs. Before the fix this exited 0 with zero bytes.
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.arg( ".math.add.help" );
  cmd
  .assert()
  .success()
  .stdout(
    predicate::str::contains( "Usage:" )
    .and( predicate::str::contains( ".math.add" ) )
    .and( predicate::str::contains( "Adds two numbers" ) ),
  )
  .stderr( "" );
}

// test_kind: bug_reproducer(manual-test-2026-08-20)
#[ test ]
fn test_cli_help_token_after_named_value()
{
  // Test Matrix Row: T8.9
  // A standalone `??` argv element after a named value is the positional help token.
  // Before the fix, argv absorption glued it into a::"1 ??" and the binary reported
  // "Cannot coerce value for argument 'a' to Integer" instead of the help page.
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd.args( vec![ ".math.add", "a::1", "??" ] );
  cmd
  .assert()
  .success()
  .stdout(
    predicate::str::contains( "Usage:" )
    .and( predicate::str::contains( "Adds two numbers" ) )
    .and( predicate::str::contains( "Cannot coerce" ).not() ),
  )
  .stderr( "" );
}

// test_kind: regression_prevention(manual-test-2026-08-20)
#[ test ]
fn test_cli_help_routes_byte_identical()
{
  // Test Matrix Row: T8.10
  // `.math.add ??` (semantic interception, printed from the HelpRequested arm) and
  // `.math.add.help` (auto-registered routine, printed from returned OutputData)
  // must produce byte-identical stdout — both delegate to `command_help_text`.
  let out_token = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" )
  .args( vec![ ".math.add", "??" ] )
  .output()
  .expect( "running unilang_cli .math.add ?? must succeed" );
  let out_spelled = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" )
  .arg( ".math.add.help" )
  .output()
  .expect( "running unilang_cli .math.add.help must succeed" );

  assert!( out_token.status.success(), "`.math.add ??` must exit 0" );
  assert!( out_spelled.status.success(), "`.math.add.help` must exit 0" );
  assert!( !out_token.stdout.is_empty(), "`.math.add ??` must print a help page" );
  assert_eq!(
    String::from_utf8_lossy( &out_token.stdout ),
    String::from_utf8_lossy( &out_spelled.stdout ),
    "`.math.add ??` and `.math.add.help` must render byte-identical pages"
  );
}

// test_kind: bug_reproducer(manual-test-2026-08-20)
#[ test ]
fn test_cli_listing_footer_advertises_help_token()
{
  // Test Matrix Row: T8.11
  // The global listing footer must advertise the current help forms, not the
  // pre-Variant-B "<command> help" wording it carried before the fix.
  let mut cmd = assert_cmd::cargo::cargo_bin_cmd!( "unilang_cli" );
  cmd
  .assert()
  .success()
  .stdout(
    predicate::str::contains( "Use '<command> ??' or '<command>.help' to get detailed help for a specific command." )
    .and( predicate::str::contains( "Example: .list ??" ) ),
  );
}
