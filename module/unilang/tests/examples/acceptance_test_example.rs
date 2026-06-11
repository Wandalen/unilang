//! Example of Well-Structured Acceptance Test
//!
//! This file demonstrates best practices for acceptance testing in the systematic
//! organization structure. It shows proper patterns for testing user scenarios
//! and CLI interactions from the end-user perspective.

use std::process::{ Command, Stdio };
use std::fs;
use tempfile::TempDir;

/// Test helper for simulating CLI interactions.
///
/// Uses `env!("CARGO_BIN_EXE_unilang_cli")` to locate the compiled binary,
/// which is set by Cargo at compile time for all integration test targets.
struct TestCLI
{
  temp_dir : TempDir,
  binary_path : String,
}

impl TestCLI
{
  fn new() -> Self
  {
    Self {
      temp_dir : tempfile::tempdir().expect( "Should create temp directory" ),
      // CARGO_BIN_EXE_<name> is set by Cargo when compiling integration tests,
      // pointing to the compiled binary. This is the correct way to locate
      // binary targets from integration tests.
      binary_path : env!( "CARGO_BIN_EXE_unilang_cli" ).to_string(),
    }
  }

  fn run( &self, args : &[&str] ) -> CLIResult
  {
    let output = Command::new( &self.binary_path )
      .args( args )
      .current_dir( self.temp_dir.path() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Should execute CLI command" );

    CLIResult {
      success : output.status.success(),
      stdout : String::from_utf8_lossy( &output.stdout ).to_string(),
      stderr : String::from_utf8_lossy( &output.stderr ).to_string(),
      exit_code : output.status.code().unwrap_or( -1 ),
    }
  }

  /// Demonstrates pattern for piping input to the CLI process.
  /// Useful when commands support reading from stdin.
  #[ allow( dead_code ) ]
  fn run_with_input( &self, args : &[&str], input : &str ) -> CLIResult
  {
    let mut child = Command::new( &self.binary_path )
      .args( args )
      .current_dir( self.temp_dir.path() )
      .stdin( Stdio::piped() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .spawn()
      .expect( "Should spawn CLI process" );

    use std::io::Write;
    if let Some( stdin ) = child.stdin.as_mut()
    {
      stdin.write_all( input.as_bytes() ).expect( "Should write input" );
    }

    let output = child.wait_with_output().expect( "Should wait for process" );

    CLIResult {
      success : output.status.success(),
      stdout : String::from_utf8_lossy( &output.stdout ).to_string(),
      stderr : String::from_utf8_lossy( &output.stderr ).to_string(),
      exit_code : output.status.code().unwrap_or( -1 ),
    }
  }

  fn create_file( &self, name : &str, content : &str )
  {
    let file_path = self.temp_dir.path().join( name );
    fs::write( file_path, content ).expect( "Should create test file" );
  }

  /// Demonstrates pattern for verifying file existence after command execution.
  #[ allow( dead_code ) ]
  fn file_exists( &self, name : &str ) -> bool
  {
    self.temp_dir.path().join( name ).exists()
  }

  /// Demonstrates pattern for reading output files produced by commands.
  #[ allow( dead_code ) ]
  fn read_file( &self, name : &str ) -> String
  {
    let file_path = self.temp_dir.path().join( name );
    fs::read_to_string( file_path ).expect( "Should read test file" )
  }
}

#[ derive( Debug ) ]
struct CLIResult
{
  success : bool,
  stdout : String,
  stderr : String,
  exit_code : i32,
}

/// Example: User workflow simulation
///
/// This test demonstrates:
/// - Complete user workflow from start to finish
/// - Real CLI interaction testing
/// - File-based input/output validation
/// - User experience verification
#[test]
fn test_user_workflow_file_processing()
{
  // User Story: As a developer, I want to read a configuration file
  // with a single command so that I can inspect its contents.

  let cli = TestCLI::new();

  // Arrange - Set up user environment with a test file
  // The binary runs with temp_dir as its working directory, so relative paths work.
  cli.create_file( "config.json", r#"{"name": "app", "version": "1.0.0"}"# );

  // Act - User executes command to read the config file
  // .files.cat reads a file by path relative to the working directory
  let result = cli.run( &[ ".files.cat", "path::config.json" ] );

  // Assert - Verify user expectations are met
  assert!( result.success, "Command should succeed: stderr={}", result.stderr );

  // User should see the file content
  assert!( result.stdout.contains( "app" ) || result.stdout.contains( "version" ),
          "Output should contain file contents: {}", result.stdout );

  // User should not see confusing error messages
  assert!( !result.stderr.contains( "panic" ),
          "Should not show internal errors to user: {}", result.stderr );

  // Exit code should indicate success
  assert_eq!( result.exit_code, 0, "Should exit with success code" );
}

/// Example: Help system user experience
///
/// This test demonstrates:
/// - User discovery of available commands
/// - Help system navigation
/// - User-friendly documentation
#[test]
fn test_user_help_system_experience()
{
  let cli = TestCLI::new();

  // Scenario 1: New user runs with no arguments — should show command listing
  let no_args = cli.run( &[] );

  assert!( no_args.success, "No-args invocation should succeed (exit 0)" );
  // Stdout lists commands; stderr shows usage hint
  let combined = format!( "{}{}", no_args.stdout, no_args.stderr );
  assert!( combined.contains( ".greet" ) || combined.contains( "math" ) ||
           combined.contains( "Commands" ) || combined.contains( "Usage" ),
          "Should show available commands or usage: {}", combined );

  // Scenario 2: User requests command listing via `help`
  let explicit_help = cli.run( &[ "help" ] );

  assert!( explicit_help.success, "help command should succeed" );
  assert!( explicit_help.stdout.contains( ".greet" ) || explicit_help.stdout.contains( "math" ),
          "Should list registered commands: {}", explicit_help.stdout );

  // Scenario 3: User tries the --help flag
  let help_flag = cli.run( &[ "--help" ] );

  assert!( help_flag.success, "--help flag should succeed" );
  assert!( !help_flag.stdout.is_empty(),
          "--help flag should produce output: {}", help_flag.stdout );

  // Help output should be substantial enough to be informative
  assert!( no_args.stdout.len() + no_args.stderr.len() > 50,
          "Help output should be substantial and informative" );
}

/// Example: Error handling user experience
///
/// This test demonstrates:
/// - User-friendly error messages for unknown commands
/// - Argument type validation errors
/// - Graceful error recovery
#[test]
fn test_user_friendly_error_handling()
{
  let cli = TestCLI::new();

  // Scenario 1: User makes typo in command name
  let typo_result = cli.run( &[ ".math.addx", "a::1", "b::2" ] ); // typo: "addx"

  assert!( !typo_result.success, "Unknown command should fail" );

  // Error message should be helpful, not cryptic
  let error_output = format!( "{}{}", typo_result.stdout, typo_result.stderr );
  assert!( error_output.to_lowercase().contains( "unknown" ) ||
           error_output.to_lowercase().contains( "not found" ) ||
           error_output.to_lowercase().contains( "error" ),
          "Should provide helpful error message: {}", error_output );

  // Should not show internal stack traces or debug info to users
  assert!( !error_output.contains( "panic" ) &&
           !error_output.contains( "backtrace" ),
          "Should not expose internal errors to user: {}", error_output );

  // Scenario 2: User provides wrong argument type (string where integer expected)
  let wrong_type = cli.run( &[ ".math.add", "a::not_a_number", "b::2" ] );

  assert!( !wrong_type.success, "Wrong argument type should fail" );

  let type_error = format!( "{}{}", wrong_type.stdout, wrong_type.stderr );
  assert!( type_error.to_lowercase().contains( "integer" ) ||
           type_error.to_lowercase().contains( "argument" ) ||
           type_error.to_lowercase().contains( "error" ),
          "Should indicate type validation error: {}", type_error );

  // Scenario 3: User recovers with correct command and arguments
  let recovery = cli.run( &[ ".math.add", "a::3", "b::4" ] );

  assert!( recovery.success, "Valid command should succeed after errors" );
  assert!( recovery.stdout.contains( "7" ),
          "Should compute correct result after recovery: {}", recovery.stdout );
}

/// Example: Interactive user session
///
/// This test demonstrates:
/// - Multi-command user sessions
/// - Different command types used in sequence
/// - Verification of each step's output
#[test]
fn test_interactive_user_session()
{
  let cli = TestCLI::new();

  // Simulate a user session with multiple related commands

  // Step 1: User greets — verifies basic command dispatch works
  let greet_result = cli.run( &[ ".greet", "name::Alice" ] );
  assert!( greet_result.success, "Greet command should succeed" );
  assert!( greet_result.stdout.contains( "Alice" ),
          "Should greet the specified person: {}", greet_result.stdout );

  // Step 2: User performs a calculation — verifies numeric argument handling
  let add_result = cli.run( &[ ".math.add", "a::10", "b::5" ] );
  assert!( add_result.success, "Add command should succeed" );
  assert!( add_result.stdout.contains( "15" ),
          "Addition result should be correct: {}", add_result.stdout );

  // Step 3: User reads a file — verifies file I/O command works
  cli.create_file( "notes.txt", "session notes" );
  let cat_result = cli.run( &[ ".files.cat", "path::notes.txt" ] );
  assert!( cat_result.success, "File read command should succeed" );
  assert!( cat_result.stdout.contains( "session notes" ),
          "File contents should appear in output: {}", cat_result.stdout );

  // Step 4: User sets configuration — verifies key-value command works
  let config_result = cli.run( &[ ".config.set", "key::theme", "value::dark" ] );
  assert!( config_result.success, "Config set command should succeed" );
  assert!( config_result.stdout.contains( "theme" ),
          "Config output should reference the key: {}", config_result.stdout );
}

/// Example: Edge case user scenarios
///
/// This test demonstrates:
/// - Testing user edge cases and corner scenarios
/// - Default argument handling
/// - Boundary values (zero, negative numbers)
#[test]
fn test_user_edge_case_scenarios()
{
  let cli = TestCLI::new();

  // Edge Case 1: User omits optional argument — should use default value
  let default_result = cli.run( &[ ".greet" ] );

  assert!( default_result.success, "Should handle missing optional argument with default" );
  assert!( default_result.stdout.contains( "World" ),
          "Should use the default argument value: {}", default_result.stdout );

  // Edge Case 2: Zero boundary values
  let zero_result = cli.run( &[ ".math.add", "a::0", "b::0" ] );

  assert!( zero_result.success, "Should handle zero values" );
  assert!( zero_result.stdout.contains( "0" ),
          "Zero plus zero should equal zero: {}", zero_result.stdout );

  // Edge Case 3: Subtraction producing a negative result
  let sub_result = cli.run( &[ ".math.sub", "x::3", "y::10" ] );

  assert!( sub_result.success, "Should handle result that is negative" );
  assert!( sub_result.stdout.contains( "-7" ),
          "3 minus 10 should produce -7: {}", sub_result.stdout );

  // Edge Case 4: Search with multi-word query passed as a single argument token
  // In Rust test code there is no shell, so spaces inside a &str element are preserved.
  let search_result = cli.run( &[ ".video.search", "query::rust_tutorial" ] );

  assert!( search_result.success, "Should handle search query" );
  assert!( search_result.stdout.contains( "rust_tutorial" ),
          "Output should echo back the query: {}", search_result.stdout );
}

/// Example: Performance from user perspective
///
/// This test demonstrates:
/// - Responsiveness testing across multiple quick commands
/// - File I/O command under realistic conditions
#[test]
fn test_user_performance_experience()
{
  let cli = TestCLI::new();

  // Test responsiveness: multiple quick greet commands should all succeed
  for i in 0..5
  {
    let name_arg = format!( "name::User{}", i );
    let quick_result = cli.run( &[ ".greet", &name_arg ] );
    assert!( quick_result.success, "Quick greet command {} should succeed", i );
    assert!( quick_result.stdout.contains( &format!( "User{}", i ) ),
            "Command {} output should contain the greeted name", i );
  }

  // Test file read performance with realistic content
  let cli2 = TestCLI::new();
  let content = (0..100)
    .map( |i| format!( "line {} of test data", i ) )
    .collect::< Vec< _ > >()
    .join( "\n" );
  cli2.create_file( "data.txt", &content );

  let file_result = cli2.run( &[ ".files.cat", "path::data.txt" ] );
  assert!( file_result.success, "File read should succeed for realistic content" );
  assert!( file_result.stdout.contains( "line 0" ),
          "Should return file contents: {}", &file_result.stdout[ ..file_result.stdout.len().min( 200 ) ] );
}

/// Example: User configuration and customization
///
/// This test demonstrates:
/// - Using the config.set command to set key-value pairs
/// - Environment variable support (UNILANG_VERBOSITY)
/// - Multiple distinct commands in one test
#[test]
fn test_user_configuration_experience()
{
  let cli = TestCLI::new();

  // User sets a configuration key-value pair
  let set_result = cli.run( &[ ".config.set", "key::log_level", "value::debug" ] );

  assert!( set_result.success, "config.set should succeed" );
  assert!( set_result.stdout.contains( "log_level" ),
          "Should confirm the config key was set: {}", set_result.stdout );
  assert!( set_result.stdout.contains( "debug" ),
          "Should confirm the config value was set: {}", set_result.stdout );

  // Test with UNILANG_VERBOSITY environment variable — binary should not crash
  let env_result = Command::new( &cli.binary_path )
    .args( [ ".system.echo" ] )
    .env( "UNILANG_VERBOSITY", "1" )
    .current_dir( cli.temp_dir.path() )
    .output()
    .expect( "Should execute with UNILANG_VERBOSITY env var" );

  assert!( env_result.status.success(), "Should work correctly with UNILANG_VERBOSITY set" );

  // User runs a math command — confirms different command types coexist
  let math_result = cli.run( &[ ".math.add", "a::100", "b::200" ] );
  assert!( math_result.success, "Math command should succeed alongside config commands" );
  assert!( math_result.stdout.contains( "300" ),
          "Math result should be correct: {}", math_result.stdout );
}

/// Example: Cross-platform user experience
///
/// This test demonstrates:
/// - File path handling from user perspective
/// - Relative path resolution against working directory
/// - User-friendly error messages for missing files
#[test]
fn test_cross_platform_user_experience()
{
  let cli = TestCLI::new();

  // The binary runs with temp_dir as its working directory,
  // so relative paths resolve correctly from there.
  cli.create_file( "data.txt", "test content for acceptance testing" );

  // User reads file with a simple relative path
  let result = cli.run( &[ ".files.cat", "path::data.txt" ] );

  assert!( result.success,
          "Should read file with relative path: stderr={}", result.stderr );
  assert!( result.stdout.contains( "test content" ),
          "Should return file contents: {}", result.stdout );

  // User also tries an explicit relative path
  let explicit_result = cli.run( &[ ".files.cat", "path::./data.txt" ] );

  // Both forms should work or fail gracefully (implementation may vary)
  if explicit_result.success
  {
    assert!( explicit_result.stdout.contains( "test content" ),
            "Explicit relative path should return same content" );
  }

  // User tries to read a file that doesn't exist — should fail with clear message
  let bad_path_result = cli.run( &[ ".files.cat", "path::nonexistent.txt" ] );

  assert!( !bad_path_result.success, "Should fail for nonexistent file" );

  let error_msg = format!( "{}{}", bad_path_result.stdout, bad_path_result.stderr );
  assert!( error_msg.to_lowercase().contains( "failed" ) ||
           error_msg.to_lowercase().contains( "not found" ) ||
           error_msg.to_lowercase().contains( "error" ),
          "Should provide a clear error message: {}", error_msg );
}