//! Core types and implementation for multi-YAML aggregation.

#[ allow( unused_imports ) ]
use crate::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
#[ cfg( feature = "multi_file" ) ]
use walkdir::WalkDir;

use super::core_types::*;

/// Multi-YAML aggregation system for compile-time command processing
#[derive(Debug, Clone)]
pub struct MultiYamlAggregator
{
  /// Configuration for aggregation
  pub(in super) config: AggregationConfig,
  /// Loaded YAML files content
  pub(in super) yaml_files: HashMap<String, String>,
  /// Processed command definitions
  pub(in super) commands: HashMap<String, CommandDefinition>,
  /// Detected conflicts
  pub(in super) conflicts: Vec<ConflictReport>,
}


impl MultiYamlAggregator
{
  /// Create a new multi-YAML aggregator
  pub fn new( config: AggregationConfig ) -> Self
  {
    Self
    {
      config,
      yaml_files: HashMap::new(),
      commands: HashMap::new(),
      conflicts: Vec::new(),
    }
  }

  /// Returns a mutable reference to the loaded YAML files map.
  /// Used in tests to inject YAML content directly without filesystem access.
  pub fn yaml_files_mut( &mut self ) -> &mut HashMap< String, String >
  {
    &mut self.yaml_files
  }

  /// Load YAML files from configured modules
  pub fn load_yaml_files( &mut self ) -> Result< (), Error >
  {
    for module in &self.config.modules
    {
      if !module.enabled
      {
        continue;
      }

      let yaml_path = self.config.base_dir.join( &module.yaml_path );

      // Try to read the actual file first, fallback to mock data for testing
      let yaml_content = if yaml_path.exists()
      {
        fs::read_to_string( &yaml_path )
          .map_err( |e| Error::Registration( format!( "Failed to read YAML file: {}", e ) ) )?
      }
      else
      {
        // Generate sample YAML content for development/testing
        self.generate_sample_yaml_content( &module.name )
      };

      self.yaml_files.insert( module.name.clone(), yaml_content );
    }

    Ok( () )
  }

  /// Generate sample YAML content for development/testing
  fn generate_sample_yaml_content( &self, module_name: &str ) -> String
  {
    format!(
      r#"---
- name: "example"
  namespace: ""
  description: "Example command from {}"
  hint: "Example"
  arguments: []
  routine_link: null
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
  auto_help_enabled: true
"#,
      module_name
    )
  }

  /// Process YAML files and apply prefixes
  pub fn process_yaml_files( &mut self ) -> Result< (), Error >
  {
    for module in &self.config.modules
    {
      if !module.enabled
      {
        continue;
      }

      if let Some( yaml_content ) = self.yaml_files.get( &module.name )
      {
        let command_defs = crate::load_command_definitions_from_yaml_str( yaml_content )?;

        for mut cmd in command_defs
        {
          // Apply module prefix
          if let Some( prefix ) = &module.prefix
          {
            let new_namespace_str = if cmd.namespace().is_empty()
            {
              format!( ".{}", prefix )
            }
            else
            {
              format!( ".{}{}", prefix, cmd.namespace() )
            };
            cmd = cmd.with_namespace( new_namespace_str );
          }

          // Apply global prefix if configured
          if let Some( global_prefix ) = &self.config.global_prefix
          {
            let new_namespace_str = if cmd.namespace().is_empty()
            {
              format!( ".{}", global_prefix )
            }
            else
            {
              format!( ".{}{}", global_prefix, cmd.namespace() )
            };
            cmd = cmd.with_namespace( new_namespace_str );
          }

          let full_name = if cmd.namespace().is_empty()
          {
            cmd.name().as_str().to_string()
          }
          else
          {
            format!( "{}.{}", cmd.namespace(), cmd.name().as_str().strip_prefix( '.' ).unwrap_or( cmd.name().as_str() ) )
          };

          self.commands.insert( full_name, cmd );
        }
      }
    }

    Ok( () )
  }

  /// Get detected conflicts
  pub fn conflicts( &self ) -> &[ ConflictReport ]
  {
    &self.conflicts
  }

  /// Get processed commands
  pub fn commands( &self ) -> &HashMap< String, CommandDefinition >
  {
    &self.commands
  }

  /// Get configuration
  pub fn config( &self ) -> &AggregationConfig
  {
    &self.config
  }

  /// Register all aggregated commands with a hybrid registry
  pub fn register_with_hybrid_registry( &self, registry: &mut crate::CommandRegistry ) -> Result< (), Error >
  {
    // Set the registry to hybrid mode for optimal performance
    registry.set_mode( crate::RegistryMode::Hybrid );

    // Register all processed commands
    for cmd in self.commands.values()
    {
      registry.register( cmd.clone() )?;
    }

    Ok( () )
  }

  /// Create a new aggregation workflow from Cargo.toml metadata
  pub fn from_cargo_metadata( cargo_toml_path: &PathBuf ) -> Result< Self, Error >
  {
    let config = parse_cargo_metadata( cargo_toml_path )?;
    Ok( Self::new( config ) )
  }

  /// Create aggregator from configuration file
  #[ cfg( feature = "multi_file" ) ]
  pub fn from_config_file( config_path: &PathBuf ) -> Result< Self, Error >
  {
    let config_content = fs::read_to_string( config_path )
      .map_err( |e| Error::Registration( format!( "Failed to read config file: {}", e ) ) )?;

    // Try to parse as JSON first (if json_parser enabled), fallback to YAML
    let config: AggregationConfig = if config_path.extension()
      .and_then( |ext| ext.to_str() )
      .map( |ext| ext.to_lowercase() == "json" )
      .unwrap_or( false )
    {
      #[ cfg( feature = "json_parser" ) ]
      {
        serde_json::from_str( &config_content )
          .map_err( |e| Error::Registration( format!( "Failed to parse JSON config: {}", e ) ) )?
      }
      #[ cfg( not( feature = "json_parser" ) ) ]
      {
        return Err( Error::Registration( "JSON config parsing requires the 'json_parser' feature".to_string() ) );
      }
    }
    else
    {
      #[ cfg( feature = "yaml_parser" ) ]
      {
        serde_yaml_ng::from_str( &config_content )
          .map_err( |e| Error::Registration( format!( "Failed to parse YAML config: {}", e ) ) )?
      }
      #[ cfg( not( feature = "yaml_parser" ) ) ]
      {
        return Err( Error::Registration( "YAML config parsing requires the 'yaml_parser' feature".to_string() ) );
      }
    };

    let mut aggregator = Self::new( config );

    // Perform auto-discovery if enabled
    if aggregator.config.auto_discovery
    {
      aggregator.discover_yaml_files()?;
    }

    Ok( aggregator )
  }

  /// Discover YAML files automatically using walkdir
  #[ cfg( feature = "multi_file" ) ]
  pub fn discover_yaml_files( &mut self ) -> Result< (), Error >
  {
    let base_dir = &self.config.base_dir;

    if !base_dir.exists()
    {
      return Ok( () ); // Skip discovery if base directory doesn't exist
    }

    let patterns = if self.config.discovery_patterns.is_empty()
    {
      vec![ "*.yaml".to_string(), "*.yml".to_string() ]
    }
    else
    {
      self.config.discovery_patterns.clone()
    };

    for entry in WalkDir::new( base_dir )
      .follow_links( false )
      .into_iter()
      .filter_map( |e| e.ok() )
    {
      if !entry.file_type().is_file()
      {
        continue;
      }

      let path = entry.path();
      let file_name = path.file_name()
        .and_then( |name| name.to_str() )
        .unwrap_or( "" );

      // Check if file matches any discovery pattern
      let matches_pattern = patterns.iter().any( |pattern| {
        if pattern.contains( '*' )
        {
          // Simple glob matching
          let pattern_regex = pattern.replace( '*', ".*" );
          regex::Regex::new( &pattern_regex )
            .map( |re| re.is_match( file_name ) )
            .unwrap_or( false )
        }
        else
        {
          file_name == pattern
        }
      } );

      if matches_pattern
      {
        let relative_path = path.strip_prefix( base_dir )
          .map_err( |e| Error::Registration( format!( "Failed to get relative path: {}", e ) ) )?;

        let module_name = relative_path.file_stem()
          .and_then( |stem| stem.to_str() )
          .unwrap_or( "unknown" )
          .to_string();

        // Add discovered module to configuration
        let module_config = ModuleConfig
        {
          name: module_name,
          yaml_path: relative_path.to_string_lossy().to_string(),
          prefix: None, // No automatic prefix for discovered files
          enabled: true,
        };

        self.config.modules.push( module_config );
      }
    }

    Ok( () )
  }

  /// Full aggregation workflow: load, process, detect conflicts
  pub fn aggregate( &mut self ) -> Result< (), Error >
  {
    self.load_yaml_files()?;
    self.process_yaml_files()?;
    self.detect_conflicts();
    self.resolve_conflicts()?;

    // Analyze command types and emit hints (non-blocking)
    #[ cfg( feature = "yaml_parser" ) ]
    self.analyze_command_types();

    Ok( () )
  }

  /// Analyze all aggregated commands for type issues and emit hints
  ///
  /// This method analyzes all commands for potential type mismatches
  /// (e.g., Boolean-as-String, Integer-as-String) and emits helpful
  /// warnings to stderr. Build continues normally.
  #[ cfg( feature = "yaml_parser" ) ]
  pub fn analyze_command_types( &self )
  {
    use crate::build_helpers::{ TypeAnalyzer, HintGenerator };

    let analyzer = TypeAnalyzer::new();
    let mut all_hints = Vec::new();

    for cmd in self.commands.values()
    {
      for arg in cmd.arguments()
      {
        let hints = analyzer.analyze_argument_definition( arg );
        all_hints.extend( hints );
      }
    }

    // Emit all hints to stderr
    HintGenerator::emit_hints( all_hints );
  }
}
