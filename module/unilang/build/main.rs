//! Build script for unilang crate.
//!
//! Generates static command definitions from YAML/JSON manifests using Perfect Hash Functions (PHF)
//! for zero-overhead command lookup at runtime.
//!
//! Supports both YAML and JSON formats with complete parity:
//! - Single-file mode: `.yaml`, `.yml`, or `.json` files
//! - Multi-file mode: Discovers all `.yaml`, `.yml`, and `.json` files
//!
//! ## Design Rules Compliance for PHF Build Process
//!
//! **✅ CORRECT Build-Time Optimization:**
//! - PHF generation during build for zero runtime overhead
//! - Static command definitions compiled into binary
//! - YAML-driven configuration for maintainability
//!
//! **❌ TESTING VIOLATIONS TO AVOID:**
//! - Do NOT create build-time performance tests comparing PHF vs `HashMap`
//! - Do NOT add timing measurements to verify PHF generation speed
//! - Do NOT create benchmark tests for PHF lookup performance in `tests/` directory
//!
//! **Performance Testing Rules:**
//! - PHF vs dynamic lookup comparisons belong in `benchkit` framework
//! - Build script should focus on correctness, not performance measurement
//! - Static command functionality testing goes in `tests/` (correctness only)
//!
//! ## Critical: Three-Layer Data Integrity Chain
//!
//! **Adding a new field to `StaticCommandDefinition` requires updates in THREE locations:**
//!
//! 1. **Struct Definition** (`src/static_data.rs`) - Add field to `StaticCommandDefinition`
//! 2. **Build Script Extraction** (`build/codegen.rs`) - Extract field from YAML in `generate_command_const()`
//! 3. **Conversion** (`src/static_data.rs`) - Map field in `From<&StaticCommandDefinition>`
//!
//! **Missing any location = silent data loss.** YAML values will be read but never reach runtime.
//!
//! **Example (BUG-088)**: The `auto_help_enabled` field was missing from steps 1 and 2,
//! causing all static commands to have `auto_help_enabled: false` regardless of YAML configuration.
//! This broke `.command.help` generation for all users.
//!
//! **Prevention**: When adding fields, update `generate_command_const()` in `build/codegen.rs`
//! to extract from YAML and include in generated const, then add conversion tests in `tests/data/static_data.rs`.

#![allow(clippy::useless_format)]

use std::env;
#[cfg(not(feature = "static_registry"))]
use std::fs::File;
#[cfg(not(feature = "static_registry"))]
use std::io::Write;
use std::path::Path;

#[cfg(feature = "static_registry")]
mod codegen;
#[cfg(feature = "static_registry")]
mod discovery;
#[cfg(feature = "static_registry")]
mod type_hints;
#[cfg(feature = "static_registry")]
mod validation;

#[cfg(feature = "static_registry")]
use type_hints::{ TypeAnalyzer, HintGenerator };

fn main()
{
  println!( "cargo:rerun-if-changed=build/main.rs" );
  println!( "cargo:rerun-if-changed=unilang.commands.yaml" );

  // Only generate static registry if static_registry feature is enabled
  #[cfg(feature = "static_registry")]
  {
    generate_static_registry();
  }

  // If static_registry not enabled, create empty file
  #[cfg(not(feature = "static_registry"))]
  {
    let out_dir = env::var( "OUT_DIR" ).unwrap();
    let dest_path = Path::new( &out_dir ).join( "static_commands.rs" );
    let mut file = File::create( dest_path ).unwrap();
    writeln!( file, "// Static registry not enabled" ).unwrap();
  }
}

#[cfg(feature = "static_registry")]
#[allow(clippy::too_many_lines)]
fn generate_static_registry()
{
  use std::path::PathBuf;

  let out_dir = env::var( "OUT_DIR" ).unwrap();
  let dest_path = Path::new( &out_dir ).join( "static_commands.rs" );

  // Support both single file and multi-file discovery modes
  let yaml_discovery_paths = env::var( "UNILANG_YAML_DISCOVERY_PATHS" )
    .map_or_else( | _ | vec![ "./".to_string() ], | paths | paths.split( ':' ).map( String::from ).collect::< Vec< _ > >() );

  // Track discovered files for build summary
  let mut discovered_files : Vec< PathBuf > = Vec::new();

  // Check if we have a custom manifest path from environment variable (single file mode)
  if let Ok( manifest_path ) = env::var( "UNILANG_STATIC_COMMANDS_PATH" )
  {
    // Single file mode - supports both YAML and JSON
    let manifest_path_buf = Path::new( &manifest_path );

    let command_definitions = match discovery::parse_command_file( manifest_path_buf )
    {
      Ok( definitions ) =>
      {
        discovered_files.push( manifest_path_buf.to_path_buf() );
        definitions
      },
      Err( e ) =>
      {
        eprintln!( "Warning: {e}" );
        codegen::generate_empty_phf( &dest_path );
        return;
      }
    };

    // Analyze command definitions for type hints
    analyze_command_types( &command_definitions );

    codegen::generate_static_commands( &dest_path, &command_definitions );
    discovery::print_build_summary( &discovered_files, command_definitions.len() );
  }
  else
  {
    // Multi-file discovery mode using walkdir
    let mut all_command_definitions = Vec::new();

    // Multi-file discovery using walkdir
    {
      use walkdir::WalkDir;

      for discovery_path in &yaml_discovery_paths
      {
        // Add discovery path to rerun conditions
        println!( "cargo:rerun-if-changed={discovery_path}" );

        if Path::new( discovery_path ).exists()
        {
          for entry in WalkDir::new( discovery_path )
            .into_iter()
            .filter_map( core::result::Result::ok )
            .filter( | e | e.file_type().is_file() )
            .filter( | e |
            {
              // Exclude test and example directories from static command discovery using proper path handling
              let path = e.path();

              // Convert to canonical form and check path components
              let should_exclude = path.components().any( | component |
              {
                if let std::path::Component::Normal( os_str ) = component
                {
                  let name = os_str.to_string_lossy();
                  name == "tests" || name == "test_data" || name == "examples"
                }
                else
                {
                  false
                }
              } );

              !should_exclude
            } )
            .filter( | e |
            {
              if let Some( extension ) = e.path().extension()
              {
                extension == "yaml" || extension == "yml" || extension == "json"
              }
              else
              {
                false
              }
            } )
          {
            match discovery::parse_command_file( entry.path() )
            {
              Ok( mut definitions ) =>
              {
                discovered_files.push( entry.path().to_path_buf() );
                all_command_definitions.append( &mut definitions );
              },
              Err( e ) =>
              {
                eprintln!( "Warning: {e}" );
              }
            }
          }
        }
      }
    }

    // If no YAML files found, try the default single file
    if all_command_definitions.is_empty()
    {
      let default_manifest = "unilang.commands.yaml";
      if let Ok( yaml_content ) = std::fs::read_to_string( default_manifest )
      {
        match serde_yaml_ng::from_str( &yaml_content )
        {
          Ok( definitions ) =>
          {
            discovered_files.push( PathBuf::from( default_manifest ) );
            all_command_definitions = definitions;
          },
          Err( e ) =>
          {
            eprintln!( "Warning: Failed to parse default YAML manifest: {e}" );
          }
        }
      }
    }

    // Analyze command definitions for type hints before generation
    analyze_command_types( &all_command_definitions );

    codegen::generate_static_commands( &dest_path, &all_command_definitions );
    discovery::print_build_summary( &discovered_files, all_command_definitions.len() );
  }
}

#[cfg(feature = "static_registry")]
fn analyze_command_types( command_definitions : &[ serde_yaml_ng::Value ] )
{
  let analyzer = TypeAnalyzer::new();
  let mut all_hints = Vec::new();

  for cmd_def in command_definitions
  {
    // Analyze arguments if present
    if let Some( args ) = cmd_def.get( "arguments" ).and_then( | a | a.as_sequence() )
    {
      for arg in args
      {
        let hints = analyzer.analyze_argument( arg );
        all_hints.extend( hints );
      }
    }
  }

  // Emit all hints at end of build (after success message from cargo)
  HintGenerator::emit_hints( all_hints );
}
