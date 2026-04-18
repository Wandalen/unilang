use std::path::{ Path, PathBuf };

/// Parse command definitions from a file based on its extension.
///
/// Supports:
/// - `.yaml`, `.yml` → `serde_yaml` parsing
/// - `.json` → `serde_json` parsing (converted to `serde_yaml::Value` for consistency)
#[cfg(feature = "static_registry")]
pub fn parse_command_file( file_path : &Path ) -> Result< Vec< serde_yaml::Value >, String >
{
  let content = std::fs::read_to_string( file_path )
    .map_err( |e| format!( "Failed to read file {}: {e}", file_path.display() ) )?;

  let extension = file_path
    .extension()
    .and_then( |ext| ext.to_str() )
    .ok_or_else( || format!( "File has no extension: {}", file_path.display() ) )?;

  match extension
  {
    "yaml" | "yml" =>
    {
      serde_yaml::from_str( &content )
        .map_err( |e| format!( "Failed to parse YAML file {}: {e}", file_path.display() ) )
    }
    "json" =>
    {
      #[cfg(feature = "json_parser")]
      {
        // Parse JSON first, then convert to YAML Value for unified processing
        let json_value : serde_json::Value = serde_json::from_str( &content )
          .map_err( |e| format!( "Failed to parse JSON file {}: {e}", file_path.display() ) )?;

        // Convert JSON Value to YAML Value via intermediate JSON string
        // This works because both implement serde Serialize/Deserialize
        let json_str = serde_json::to_string( &json_value )
          .map_err( |e| format!( "Failed to serialize JSON: {e}" ) )?;

        serde_yaml::from_str( &json_str )
          .map_err( |e| format!( "Failed to convert JSON to YAML representation: {e}" ) )
      }
      #[cfg(not(feature = "json_parser"))]
      {
        Err( format!( "JSON support requires the 'json_parser' feature. File: {}", file_path.display() ) )
      }
    }
    other => Err( format!( "Unsupported file extension '{other}' for file: {}", file_path.display() ) )
  }
}

/// Print build summary showing what unilang did automatically.
///
/// Makes the invisible visible — shows developers that unilang processed their YAML files
/// and generated the command registry, so they don't need to write build.rs themselves.
///
/// Suppressible via `UNILANG_QUIET_BUILD` environment variable.
#[cfg(feature = "static_registry")]
pub fn print_build_summary( yaml_files : &[ PathBuf ], command_count : usize )
{
  // Don't print if no files discovered
  if yaml_files.is_empty() { return; }

  // Allow suppression for CI builds or when output is unwanted
  if std::env::var( "UNILANG_QUIET_BUILD" ).is_ok() { return; }

  eprintln!();
  eprintln!( "╔══════════════════════════════════════════════════════════╗" );
  eprintln!( "║  Unilang: Compile-Time Command Registry                 ║" );
  eprintln!( "╟──────────────────────────────────────────────────────────╢" );

  let file_word = if yaml_files.len() == 1 { "file" } else { "files" };
  eprintln!( "║  Found {} YAML {:<46}║", yaml_files.len(), file_word );

  // Show up to 5 files, then "... and N more"
  let files_to_show = yaml_files.iter().take( 5 );
  for file in files_to_show
  {
    let name = file.file_name()
      .and_then( |n| n.to_str() )
      .unwrap_or( "unknown" );

    eprintln!( "║    - {name:<50} ║" );
  }

  if yaml_files.len() > 5
  {
    let remaining = yaml_files.len() - 5;
    eprintln!( "║    ... and {} more {:<38}║", remaining, "" );
  }

  let command_word = if command_count == 1 { "command" } else { "commands" };
  eprintln!( "║  Generated PHF map with {command_count} {command_word:<32}║" );
  eprintln!( "║  Lookup time: ~80ns (zero runtime overhead)             ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  ✅ You did NOT need to write build.rs                  ║" );
  eprintln!( "║  ✅ YAML parsed at compile-time                         ║" );
  eprintln!( "║  ✅ Command registry ready                              ║" );
  eprintln!( "║                                                          ║" );
  eprintln!( "║  Docs: https://docs.rs/unilang                          ║" );
  eprintln!( "╚══════════════════════════════════════════════════════════╝" );
  eprintln!();
}
