//! `.check` command handler
//!
//! Validates unilang project for common mistakes and anti-patterns.

use crate::checks::{ check_build_rs, check_duplicate_deps, check_deprecated_api };
use std::path::{ Path, PathBuf };

/// Parameters for `.check` command
#[derive( Debug )]
pub struct CheckParams
{
  pub path : PathBuf,
  pub verbosity : u8,
  #[ allow( dead_code ) ] // TODO: Implement auto-fix functionality
  pub fix : bool,
}

impl CheckParams
{
  /// Parse parameters from command arguments
  pub fn parse( args : &[ ( String, String ) ] ) -> Result< Self, String >
  {
    let mut path = PathBuf::from( "." );
    let mut verbosity = 2u8;
    let mut fix = false;

    for ( key, value ) in args
    {
      match key.as_str()
      {
        "path" | "p" => path = PathBuf::from( value ),
        "verbosity" | "v" => verbosity = validate_verbosity( value )?,
        "fix" | "f" => fix = parse_bool( value )?,
        _ => return Err( format!( "Unknown parameter: {}", key ) ),
      }
    }

    // Validate path
    validate_path( &path )?;

    Ok( Self { path, verbosity, fix } )
  }
}

/// Validate verbosity level
pub fn validate_verbosity( level : &str ) -> Result< u8, String >
{
  match level.parse::< u8 >()
  {
    Ok( n ) if n <= 5 => Ok( n ),
    Ok( n ) => Err( format!( "Verbosity must be 0-5, got {}", n ) ),
    Err( _ ) => Err( format!( "Invalid verbosity '{}', must be 0-5", level ) ),
  }
}

/// Parse boolean value
pub fn parse_bool( value : &str ) -> Result< bool, String >
{
  match value.to_lowercase().as_str()
  {
    "true" | "1" | "yes" => Ok( true ),
    "false" | "0" | "no" => Ok( false ),
    _ => Err( format!( "Invalid boolean '{}', must be 'true' or 'false'", value ) ),
  }
}

/// Validate path exists and is accessible
pub fn validate_path( path : &Path ) -> Result< (), String >
{
  // Check if path exists
  if !path.exists()
  {
    return Err( format!( "Path '{}' does not exist", path.display() ) );
  }

  // Check if path is a directory
  if !path.is_dir()
  {
    return Err( format!( "Path '{}' is not a directory", path.display() ) );
  }

  // Check read permissions
  match std::fs::read_dir( path )
  {
    Ok( _ ) => Ok( () ),
    Err( e ) => Err( format!( "Cannot read directory '{}': {}", path.display(), e ) ),
  }
}

/// Execute `.check` command
pub fn execute( params : CheckParams ) -> Result< i32, String >
{
  if params.verbosity >= 3
  {
    eprintln!( "[INFO] Checking unilang project: {}", params.path.display() );
  }

  // Run all checks
  let mut results = Vec::new();

  // Check 1: Custom build.rs
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Checking for custom build.rs..." );
  }
  let build_rs_result = check_build_rs( &params.path );
  if !build_rs_result.passed
  {
    if params.verbosity >= 3
    {
      eprintln!( "[DEBUG] Found custom build.rs - ISSUE DETECTED" );
    }
    results.push( build_rs_result );
  }

  // Check 2: Duplicate dependencies
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Checking Cargo.toml for duplicate dependencies..." );
  }
  let deps_result = check_duplicate_deps( &params.path );
  if !deps_result.passed
  {
    if params.verbosity >= 3
    {
      eprintln!( "[DEBUG] Found duplicate dependencies - ISSUE DETECTED" );
    }
    results.push( deps_result );
  }

  // Check 3: Deprecated API
  if params.verbosity >= 3
  {
    eprintln!( "[DEBUG] Checking for deprecated API usage..." );
  }
  let api_result = check_deprecated_api( &params.path );
  if !api_result.passed
  {
    if params.verbosity >= 3
    {
      eprintln!( "[DEBUG] Found deprecated API - ISSUE DETECTED" );
    }
    results.push( api_result );
  }

  // Determine exit code
  let exit_code = if results.is_empty()
  {
    0 // All checks passed
  }
  else
  {
    1 // Issues found
  };

  // Output based on verbosity
  match params.verbosity
  {
    0 =>
    {
      // Silent - no output, only exit code
    }
    1 =>
    {
      // Single line
      if results.is_empty()
      {
        println!( "✅ All checks passed" );
      }
      else
      {
        println!( "❌ {} issue(s) found", results.len() );
      }
    }
    2 =>
    {
      // Concise (default)
      println!( "Checking unilang project: {}", params.path.display() );

      if results.is_empty()
      {
        println!( "✅ All checks passed" );
      }
      else
      {
        println!();
        println!( "❌ PROBLEMS DETECTED:" );
        println!();

        for ( i, result ) in results.iter().enumerate()
        {
          println!( "  {}. {}", i + 1, result.issue_type );
          println!( "     Location: {}", result.location );
          println!( "     Issue: {}", result.issue );
          println!( "     Fix: {}", result.fix );
          println!( "     Docs: {}", result.docs_url );
          println!();
        }

        println!( "Summary: {} issue(s) found", results.len() );
      }
    }
    _ =>
    {
      // Debug (3+)
      if params.verbosity >= 3
      {
        eprintln!( "[INFO] Check complete: {} issue(s) found", results.len() );
      }

      println!( "Checking unilang project: {}", params.path.display() );

      if results.is_empty()
      {
        println!( "✅ All checks passed" );
      }
      else
      {
        println!();
        println!( "❌ PROBLEMS DETECTED:" );
        println!();

        for ( i, result ) in results.iter().enumerate()
        {
          println!( "  {}. {}", i + 1, result.issue_type );
          println!( "     Location: {}", result.location );
          println!( "     Issue: {}", result.issue );
          println!( "     Fix: {}", result.fix );
          println!( "     Docs: {}", result.docs_url );
          println!();
        }

        println!( "Summary: {} issue(s) found", results.len() );
      }
    }
  }

  Ok( exit_code )
}

