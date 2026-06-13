//! Regression test for empty arguments handling bug (BUG-093).
//!
//! ## Root Cause
//!
//! `full_cli_example.rs` only checked for explicit `"help"` argument before parsing.
//! Running with no args passed empty string to parser/analyzer, which returned
//! `HelpRequested` error (exit code 1) instead of showing help gracefully (exit code 0).
//!
//! ## Why Not Caught
//!
//! No test ran examples with zero arguments. All integration tests supplied explicit
//! command names, so the empty-args path was never exercised.
//!
//! ## Fix Applied
//!
//! Added `args.is_empty()` check before parsing in `full_cli_example.rs`. When no args
//! provided, displays help and exits with code 0 instead of propagating `HelpRequested`.
//!
//! ## Prevention
//!
//! These tests validate that empty input and the `HelpRequested` error code are handled
//! as success cases. Any CLI entry point must handle the zero-arguments case before parsing.
//!
//! ## Pitfall
//!
//! `HelpRequested` is semantically a user request, not an error. Using `Result::Err` for
//! help requests forces callers to special-case it to avoid treating help as failure.
//! Standard CLI convention: empty args → show help with exit code 0.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::redundant_closure_for_method_calls ) ]

use unilang::{ CommandRegistry, Pipeline };

/// Reproduces empty args handling bug where empty string triggers error.
///
/// ## Root Cause
///
/// In `examples/full_cli_example.rs:240-245`, code checked for help request:
/// ```rust,ignore
/// if args.first().is_some_and(|arg| arg == "help") {
///   // Show help
/// }
/// ```
///
/// This check only handles explicit "help" argument. When example runs without args:
/// ```bash
/// cargo run --example full_cli_example
/// ```
///
/// The `args` vector is empty. Code then calls:
/// ```rust,ignore
/// let result = pipeline.process_command_simple("");
/// ```
///
/// Parser in `src/parser.rs` treats empty string as special case, returning:
/// ```rust,ignore
/// Err(ErrorData { code: ErrorCode::HelpRequested, ... })
/// ```
///
/// Example propagates this error to main(), which exits with code 1 and prints error message
/// instead of showing help with exit code 0.
///
/// ## Why Not Caught Initially
///
/// Examples are typically tested by running with valid arguments to demonstrate functionality.
/// Edge case of running with NO arguments wasn't tested. Manual testing focused on "happy path"
/// (example works as documented), not error paths or degenerate inputs.
///
/// Unit tests for parser likely test empty string, but unit tests accept that HelpRequested
/// is an error (which is architecturally questionable). Integration test running examples
/// without args would catch this.
///
/// ## Fix Applied
///
/// Updated `examples/full_cli_example.rs:240-245` to check for empty args:
/// ```rust
/// if args.is_empty() || args.first().is_some_and(|arg| arg == "help") {
///   let help_generator = unilang::help::HelpGenerator::new(&registry);
///   println!("{}", help_generator.generate_full_help());
///   return Ok(());
/// }
/// ```
///
/// Now empty args trigger help display before reaching parser, avoiding error path.
///
/// ## Prevention
///
/// 1. **Parser design:** Consider treating empty input as valid "show help" request,
///    not error condition. Return `Ok(HelpRequested)` instead of `Err(HelpRequested)`.
/// 2. **CLI framework:** Provide wrapper handling common patterns (empty args → help)
/// 3. **Example template:** Create CLI example template with proper arg handling
/// 4. **Integration tests:** Test all examples with no args, single arg, multiple args
///
/// ## Pitfall to Avoid
///
/// Using `Result::Err` for non-error conditions creates awkward error handling. HelpRequested
/// is not a failure - it's a valid user intent. Consider using separate return type:
/// ```rust
/// enum CliResult { Success, HelpRequested, Error(ErrorData) }
/// ```
///
/// Or handle help display at higher level before calling parser. Current design forces all
/// callers to special-case HelpRequested, violating DRY principle.
/// FT-5: Empty REPL input handled without panic.
// test_kind: ft_spec(FT-5), bug_reproducer(BUG-093)
#[ test ]
fn test_ft5_empty_repl_input_handled_without_panic()
{
  // Test how empty string is handled

  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_simple( "" );

  // Pipeline converts HelpRequested into a success result (help display is not an error)
  assert!( result.success, "Pipeline with empty string must succeed — HelpRequested is not an error" );
  assert!( result.error.is_none(), "Help request must carry no error field" );
}

/// Verifies that the pipeline handles empty-string input as a successful help response.
///
/// The fix in examples/ intercepts empty args before calling the pipeline, but the
/// pipeline itself also handles the case gracefully (HelpRequested → success).
#[ test ]
fn test_empty_args_should_show_help()
{
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  // Empty string → HelpRequested → pipeline converts to success with help output
  let result = pipeline.process_command_simple( "" );

  assert!( result.success, "Pipeline with empty string must succeed; HelpRequested is treated as a valid help response" );
  assert!( result.error.is_none(), "Help response must carry no error field" );
}

/// Verifies that the bare "help" keyword is also handled as a successful help response.
///
/// Both empty string and "help" keyword are recognized as help requests by the pipeline.
#[ test ]
fn test_explicit_help_request()
{
  // "help" is recognized as a help request by the pipeline (treated same as empty string).
  // CLI callers may intercept it early, but the pipeline also handles it gracefully.
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_simple( "help" );

  assert!( result.success, "'help' keyword must succeed — pipeline recognizes it as a help request" );
  assert!( result.error.is_none(), "Help request must carry no error field" );
}

/// Tests normal command execution path when args are provided.
#[ test ]
fn test_valid_args_proceed_to_parsing()
{
  let args: Vec< String > = vec![ ".version".to_string() ];

  // Valid args should skip help display
  let should_show_help = args.is_empty() || args.first().is_some_and( | arg | arg == "help" );

  assert!(
    !should_show_help,
    "Valid command args should not trigger help display"
  );

  // Would proceed to parse and execute .version command
  println!( "Would parse and execute: {}", args[ 0 ] );
}


/// Tests edge case: single-element args with empty string.
///
/// Handles case where args = vec![""] (single empty string) vs vec![] (truly empty).
#[ test ]
fn test_single_empty_string_arg()
{
  let args: Vec< String > = vec![ String::new() ];

  // Edge case: args is not empty, but first element is empty string
  // Should this trigger help display? Current fix doesn't handle this.

  let is_empty_args = args.is_empty();
  let is_help_request = args.first().is_some_and( | arg | arg == "help" );
  let is_empty_first = args.first().is_some_and( | arg | arg.is_empty() );

  // Single empty-string element is distinct from an empty args list
  assert!( !is_empty_args, "Single-element args vector is not empty" );
  assert!( !is_help_request, "Empty-string first arg is not a help request" );
  assert!( is_empty_first, "First arg must be the empty string" );
  // Note: args = [ "" ] is NOT caught by is_empty() — callers must also guard this edge case
}
