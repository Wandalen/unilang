//! PHF code generation for static command registry.
//!
//! Generates Rust source code with compile-time PHF maps from parsed command definitions.

use std::fs::File;
use std::io::{ BufWriter, Write };
use std::path::Path;

#[cfg(feature = "static_registry")]
pub fn generate_empty_phf( dest_path : &Path )
{
  let mut f = BufWriter::new( File::create( dest_path ).unwrap() );

  writeln!( f, "// Generated static commands (empty)" ).unwrap();
  writeln!( f, "use unilang::phf::phf_map;" ).unwrap();
  writeln!( f, "use ::unilang::static_data::{{StaticCommandDefinition, StaticCommandMap}};" ).unwrap();
  writeln!( f ).unwrap();
  writeln!( f, "/// Static command registry (compile-time generated)." ).unwrap();
  writeln!( f, "/// " ).unwrap();
  writeln!( f, "/// This provides zero-overhead lookup of compile-time registered commands." ).unwrap();
  writeln!( f, "/// No PHF dependency required for crates using this registry." ).unwrap();
  writeln!( f ).unwrap();

  // Generate internal PHF map as const (not pub)
  writeln!( f, "const STATIC_COMMANDS_PHF: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{}};" ).unwrap();
  writeln!( f ).unwrap();

  // Generate public wrapper
  writeln!( f, "pub static STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_PHF);" ).unwrap();
}

#[cfg(feature = "static_registry")]
#[allow(clippy::too_many_lines)]
pub fn generate_static_commands( dest_path : &Path, command_definitions : &[ serde_yaml::Value ] )
{
  let mut f = BufWriter::new( File::create( dest_path ).unwrap() );

  // Write header and imports
  writeln!( f, "// Generated static commands" ).unwrap();
  writeln!( f, "use unilang::phf::phf_map;" ).unwrap();

  // Import StaticCommandMap wrapper (absolute path works in both library and examples)
  writeln!( f, "use ::unilang::static_data::StaticCommandMap;" ).unwrap();

  // Only import types we'll actually use (absolute path works in both library and examples)
  if command_definitions.is_empty()
  {
    writeln!( f, "use ::unilang::static_data::StaticCommandDefinition;" ).unwrap();
  }
  else
  {
    // Check if we have any commands with arguments
    let has_arguments = command_definitions.iter()
      .any( | cmd | cmd[ "arguments" ].as_sequence().is_some_and( | args | !args.is_empty() ) );

    if has_arguments
    {
      writeln!( f, "use ::unilang::static_data::{{StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind}};" ).unwrap();
    }
    else
    {
      writeln!( f, "use ::unilang::static_data::StaticCommandDefinition;" ).unwrap();
    }
  }
  writeln!( f ).unwrap();

  // Task 085 Item #3: Detect duplicate command names at build time
  // Track seen command names to prevent duplicates
  let mut seen_command_names : std::collections::HashMap< String, usize > = std::collections::HashMap::new();

  // Validate and generate const data for each command
  // Fix(H27, H37): Validate commands at build time to prevent runtime panics
  for ( i, cmd_value ) in command_definitions.iter().enumerate()
  {
    let name = cmd_value[ "name" ].as_str().unwrap_or( "" );
    let namespace = cmd_value[ "namespace" ].as_str().unwrap_or( "" );
    let version = cmd_value[ "version" ].as_str().unwrap_or( "" );

    // Validate command definition before generating code
    // This ensures From<StaticCommandDefinition> cannot panic at runtime
    if let Err( e ) = crate::validation::validate_command( name, namespace, version, "unilang.commands.yaml" )
    {
      panic!(
        "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
         ║ BUILD ERROR: Invalid command definition                                       ║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ {e:<76} ║\n\
         ╟──────────────────────────────────────────────────────────────────════════────╢\n\
         ║ Fix: Ensure command names start with '.' (e.g., '.help', '.chat')             ║\n\
         ║      Ensure non-empty namespaces start with '.' (e.g., '.session')            ║\n\
         ║      Ensure version is not empty (e.g., '1.0.0')                              ║\n\
         ╚══════════════════════════════════════════════════════════════════════════════╝\n"
      );
    }

    // Task 085 Item #3: Check for duplicate command names
    // Compute full name using same logic as PHF generation
    use crate::validation::compute_full_name;
    let full_name = compute_full_name( namespace, name );

    if let Some( first_index ) = seen_command_names.get( &full_name )
    {
      panic!(
        "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
         ║ BUILD ERROR: Duplicate command name detected                                  ║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ Command '{}' is defined multiple times in YAML manifest{}║\n\
         ║                                                                                ║\n\
         ║ First occurrence: command index {}{}║\n\
         ║ Duplicate found:  command index {}{}║\n\
         ╟──────────────────────────────────────────────────────────────────────────────╢\n\
         ║ Fix: Rename one of the commands or remove the duplicate entry.                ║\n\
         ║      All command names must be unique across the entire manifest.             ║\n\
         ║                                                                                ║\n\
         ║ Task 085 Item #3: Prevents silent overwrites and confusing behavior           ║\n\
         ╚══════════════════════════════════════════════════════════════════════════════╝\n",
        full_name,
        " ".repeat( 51 - full_name.len().min( 51 ) ),
        first_index,
        " ".repeat( 67 - first_index.to_string().len() ),
        i,
        " ".repeat( 67 - i.to_string().len() )
      );
    }

    seen_command_names.insert( full_name.clone(), i );

    // Task 085 Item #5: Validate parameter storage types (prevent wplan bug)
    // Check that multiple:true parameters use List storage type
    if let Some( arguments ) = cmd_value[ "arguments" ].as_sequence()
    {
      for arg in arguments
      {
        let arg_name = arg[ "name" ].as_str().unwrap_or( "" );
        let multiple = arg[ "attributes" ][ "multiple" ].as_bool().unwrap_or( false );

        if multiple
        {
          // Check if kind is a List
          let is_list = if let Some( kind_str ) = arg[ "kind" ].as_str()
          {
            // Simple string kind - check if it contains "List"
            kind_str.contains( "List" )
          }
          else if let Some( _kind_map ) = arg[ "kind" ].as_mapping()
          {
            // Complex kind structure like {List: ["String", null]}
            // If it has a "List" key or contains List, it's valid
            arg[ "kind" ].as_mapping()
              .and_then( | m | m.keys().next() )
              .and_then( | k | k.as_str() )
              .is_some_and( | k | k == "List" )
          }
          else
          {
            false
          };

          if !is_list
          {
            let kind_debug = format!( "{:?}", arg[ "kind" ] );
            panic!(
              "\n╔══════════════════════════════════════════════════════════════════════════════╗\n\
               ║ BUILD ERROR: Invalid parameter definition (wplan bug pattern)                 ║\n\
               ╟──────────────────────────────────────────────────────────────────────────────╢\n\
               ║ Command:   {}{}║\n\
               ║ Parameter: {}{}║\n\
               ║ Problem:   multiple:true but storage type is NOT List                         ║\n\
               ║                                                                                ║\n\
               ║ Current kind: {}{}║\n\
               ║                                                                                ║\n\
               ║ This causes silent data loss when multiple values overwrite each other.       ║\n\
               ╟──────────────────────────────────────────────────────────────────────────────╢\n\
               ║ Fix: Change parameter kind to List storage:                                   ║\n\
               ║                                                                                ║\n\
               ║   kind: {{List: [\"String\", null]}}  # For string values                        ║\n\
               ║   kind: {{List: [\"Integer\", null]}} # For integer values                       ║\n\
               ║                                                                                ║\n\
               ║ Task 085 Item #5: Prevents the wplan bug pattern                              ║\n\
               ╚══════════════════════════════════════════════════════════════════════════════╝\n",
              full_name,
              " ".repeat( 68 - full_name.len().min( 68 ) ),
              arg_name,
              " ".repeat( 68 - arg_name.len().min( 68 ) ),
              kind_debug,
              " ".repeat( 63 - kind_debug.len().min( 63 ) )
            );
          }
        }
      }
    }

    generate_command_const( &mut f, i, cmd_value );
  }

  // Generate internal PHF map as const (not pub)
  writeln!( f, "const STATIC_COMMANDS_PHF: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{" ).unwrap();

  for ( i, cmd_value ) in command_definitions.iter().enumerate()
  {
    let name = cmd_value[ "name" ].as_str().unwrap_or( "" );
    let namespace = cmd_value[ "namespace" ].as_str().unwrap_or( "" );

    let full_name = if namespace.is_empty()
    {
      // Command name may already have a leading dot, don't duplicate it
      if name.starts_with( '.' )
      {
        name.to_string()
      }
      else
      {
        format!( ".{name}" )
      }
    }
    else
    {
      // Strip leading dot from name to avoid double dots like ".system..status"
      format!( "{namespace}.{}", name.trim_start_matches( '.' ) )
    };

    writeln!( f, "  \"{full_name}\" => &CMD_{i}," ).unwrap();
  }

  writeln!( f, "}};" ).unwrap();
  writeln!( f ).unwrap();

  // Generate public wrapper
  writeln!( f, "/// Static command registry (compile-time generated)." ).unwrap();
  writeln!( f, "/// " ).unwrap();
  writeln!( f, "/// This map provides zero-overhead lookup of compile-time registered commands." ).unwrap();
  writeln!( f, "/// Commands are keyed by their full name (e.g., \".help\" or \"namespace.command\")." ).unwrap();
  writeln!( f, "/// " ).unwrap();
  writeln!( f, "/// No PHF dependency required for crates using this registry." ).unwrap();
  writeln!( f, "pub static STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_PHF);" ).unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_command_const( f : &mut BufWriter< File >, index : usize, cmd_value : &serde_yaml::Value )
{
  let name = cmd_value[ "name" ].as_str().unwrap_or( "" );
  let namespace = cmd_value[ "namespace" ].as_str().unwrap_or( "" );
  let description = cmd_value[ "description" ].as_str().unwrap_or( "" );
  let hint = cmd_value[ "hint" ].as_str().unwrap_or( "" );
  let status = cmd_value[ "status" ].as_str().unwrap_or( "stable" );
  let version = cmd_value[ "version" ].as_str().unwrap_or( "1.0.0" );
  let idempotent = cmd_value[ "idempotent" ].as_bool().unwrap_or( false );
  let deprecation_message = cmd_value[ "deprecation_message" ].as_str().unwrap_or( "" );
  let http_method_hint = cmd_value[ "http_method_hint" ].as_str().unwrap_or( "" );
  // Fix(issue-088): Extract auto_help_enabled from YAML (defaults to true)
  let auto_help_enabled = cmd_value[ "auto_help_enabled" ].as_bool().unwrap_or( true );
  // Fix(issue-089): Extract category from YAML (defaults to empty string)
  let category = cmd_value[ "category" ].as_str().unwrap_or( "" );
  // Extract show_version_in_help from YAML (defaults to true)
  let show_version_in_help = cmd_value[ "show_version_in_help" ].as_bool().unwrap_or( true );

  // Generate arguments array
  if let Some( arguments ) = cmd_value[ "arguments" ].as_sequence()
  {
    if !arguments.is_empty()
    {
      for ( arg_i, arg_value ) in arguments.iter().enumerate()
      {
        generate_argument_const( f, index, arg_i, arg_value );
      }

      writeln!( f, "const CMD_{index}_ARGS: &[StaticArgumentDefinition] = &[" ).unwrap();
      for arg_i in 0..arguments.len()
      {
        writeln!( f, "  CMD_{index}_ARG_{arg_i}," ).unwrap();
      }
      writeln!( f, "];" ).unwrap();
      writeln!( f ).unwrap();
    }
  }

  // Generate arrays for aliases, tags, permissions, examples
  generate_string_array( f, &format!( "CMD_{index}_ALIASES" ), &cmd_value[ "aliases" ] );
  generate_string_array( f, &format!( "CMD_{index}_TAGS" ), &cmd_value[ "tags" ] );
  generate_string_array( f, &format!( "CMD_{index}_PERMISSIONS" ), &cmd_value[ "permissions" ] );
  generate_string_array( f, &format!( "CMD_{index}_EXAMPLES" ), &cmd_value[ "examples" ] );

  // Generate the main command const
  writeln!( f, "const CMD_{index}: StaticCommandDefinition = StaticCommandDefinition {{" ).unwrap();
  writeln!( f, "  name: \"{}\",", escape_string( name ) ).unwrap();
  writeln!( f, "  namespace: \"{}\",", escape_string( namespace ) ).unwrap();
  writeln!( f, "  description: \"{}\",", escape_string( description ) ).unwrap();
  writeln!( f, "  hint: \"{}\",", escape_string( hint ) ).unwrap();

  // Arguments
  if let Some( arguments ) = cmd_value[ "arguments" ].as_sequence()
  {
    if arguments.is_empty()
    {
      writeln!( f, "  arguments: &[]," ).unwrap();
    }
    else
    {
      writeln!( f, "  arguments: CMD_{index}_ARGS," ).unwrap();
    }
  }
  else
  {
    writeln!( f, "  arguments: &[]," ).unwrap();
  }

  writeln!( f, "  routine_link: None," ).unwrap();
  writeln!( f, "  status: \"{}\",", escape_string( status ) ).unwrap();
  writeln!( f, "  version: \"{}\",", escape_string( version ) ).unwrap();
  writeln!( f, "  tags: CMD_{index}_TAGS," ).unwrap();
  writeln!( f, "  aliases: CMD_{index}_ALIASES," ).unwrap();
  writeln!( f, "  permissions: CMD_{index}_PERMISSIONS," ).unwrap();
  writeln!( f, "  idempotent: {idempotent}," ).unwrap();
  writeln!( f, "  deprecation_message: \"{}\",", escape_string( deprecation_message ) ).unwrap();
  writeln!( f, "  http_method_hint: \"{}\",", escape_string( http_method_hint ) ).unwrap();
  writeln!( f, "  examples: CMD_{index}_EXAMPLES," ).unwrap();
  // Fix(issue-088): Include auto_help_enabled field in generated PHF const
  writeln!( f, "  auto_help_enabled: {auto_help_enabled}," ).unwrap();
  // Fix(issue-089): Include category field in generated PHF const
  writeln!( f, "  category: \"{}\",", escape_string( category ) ).unwrap();
  // Include show_version_in_help field in generated PHF const
  writeln!( f, "  show_version_in_help: {show_version_in_help}," ).unwrap();
  writeln!( f, "}};" ).unwrap();
  writeln!( f ).unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_argument_const( f : &mut BufWriter< File >, cmd_index : usize, arg_index : usize, arg_value : &serde_yaml::Value )
{
  let name = arg_value[ "name" ].as_str().unwrap_or( "" );
  let description = arg_value[ "description" ].as_str().unwrap_or( "" );
  let hint = arg_value[ "hint" ].as_str().unwrap_or( "" );
  let kind_str = arg_value[ "kind" ].as_str().unwrap_or( "String" );

  // Generate validation rules array
  if let Some( validation_rules ) = arg_value[ "validation_rules" ].as_sequence()
  {
    if !validation_rules.is_empty()
    {
      writeln!( f, "const CMD_{cmd_index}_ARG_{arg_index}_VALIDATION: &[StaticValidationRule] = &[" ).unwrap();
      for _rule in validation_rules
      {
        // For now, we'll keep validation rules empty since they're complex to parse
        // This can be expanded later if needed
      }
      writeln!( f, "];" ).unwrap();
    }
  }

  // Generate aliases and tags arrays
  generate_string_array( f, &format!( "CMD_{cmd_index}_ARG_{arg_index}_ALIASES" ), &arg_value[ "aliases" ] );
  generate_string_array( f, &format!( "CMD_{cmd_index}_ARG_{arg_index}_TAGS" ), &arg_value[ "tags" ] );

  // Generate attributes
  let attributes = &arg_value[ "attributes" ];
  let optional = attributes[ "optional" ].as_bool().unwrap_or( false );
  let multiple = attributes[ "multiple" ].as_bool().unwrap_or( false );
  let default_value = attributes[ "default" ].as_str();
  let sensitive = attributes[ "sensitive" ].as_bool().unwrap_or( false );
  let interactive = attributes[ "interactive" ].as_bool().unwrap_or( false );

  writeln!( f, "const CMD_{cmd_index}_ARG_{arg_index}_ATTRS: StaticArgumentAttributes = StaticArgumentAttributes {{" ).unwrap();
  writeln!( f, "  optional: {optional}," ).unwrap();
  writeln!( f, "  multiple: {multiple}," ).unwrap();
  if let Some( default ) = default_value
  {
    writeln!( f, "  default: Some(\"{}\"),", escape_string( default ) ).unwrap();
  }
  else
  {
    writeln!( f, "  default: None," ).unwrap();
  }
  writeln!( f, "  sensitive: {sensitive}," ).unwrap();
  writeln!( f, "  interactive: {interactive}," ).unwrap();
  writeln!( f, "}};" ).unwrap();

  // Generate kind
  let static_kind = match kind_str
  {
    "Integer" => "StaticKind::Integer",
    "Float" => "StaticKind::Float",
    "Boolean" => "StaticKind::Boolean",
    "Path" => "StaticKind::Path",
    "File" => "StaticKind::File",
    "Directory" => "StaticKind::Directory",
    "Url" => "StaticKind::Url",
    "DateTime" => "StaticKind::DateTime",
    "Pattern" => "StaticKind::Pattern",
    "JsonString" => "StaticKind::JsonString",
    "Object" => "StaticKind::Object",
    _ => "StaticKind::String", // Default fallback, includes "String"
  };

  // Generate the argument const
  writeln!( f, "const CMD_{cmd_index}_ARG_{arg_index}: StaticArgumentDefinition = StaticArgumentDefinition {{" ).unwrap();
  writeln!( f, "  name: \"{}\",", escape_string( name ) ).unwrap();
  writeln!( f, "  kind: {static_kind}," ).unwrap();
  writeln!( f, "  attributes: CMD_{cmd_index}_ARG_{arg_index}_ATTRS," ).unwrap();
  writeln!( f, "  hint: \"{}\",", escape_string( hint ) ).unwrap();
  writeln!( f, "  description: \"{}\",", escape_string( description ) ).unwrap();
  writeln!( f, "  validation_rules: &[]," ).unwrap(); // Keep empty for now
  writeln!( f, "  aliases: CMD_{cmd_index}_ARG_{arg_index}_ALIASES," ).unwrap();
  writeln!( f, "  tags: CMD_{cmd_index}_ARG_{arg_index}_TAGS," ).unwrap();
  writeln!( f, "}};" ).unwrap();
  writeln!( f ).unwrap();
}

#[cfg(feature = "static_registry")]
fn generate_string_array( f : &mut BufWriter< File >, const_name : &str, yaml_value : &serde_yaml::Value )
{
  if let Some( array ) = yaml_value.as_sequence()
  {
    if array.is_empty()
    {
      writeln!( f, "const {const_name}: &[&str] = &[];" ).unwrap();
    }
    else
    {
      writeln!( f, "const {const_name}: &[&str] = &[" ).unwrap();
      for item in array
      {
        if let Some( s ) = item.as_str()
        {
          writeln!( f, "  \"{}\",", escape_string( s ) ).unwrap();
        }
      }
      writeln!( f, "];" ).unwrap();
    }
  }
  else
  {
    writeln!( f, "const {const_name}: &[&str] = &[];" ).unwrap();
  }
}

#[cfg(feature = "static_registry")]
fn escape_string( s : &str ) -> String
{
  s.replace( '\\', "\\\\" ).replace( '"', "\\\"" )
}
