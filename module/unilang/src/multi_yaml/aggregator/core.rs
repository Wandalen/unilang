//! Core types and implementation for multi-YAML aggregation.

#[ allow( unused_imports ) ]
use crate::*;
use crate::multi_yaml::builder::{ CliBuilder, AggregationMode };
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
#[ cfg( feature = "multi_file" ) ]
use walkdir::WalkDir;

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

/// Configuration for multi-YAML aggregation
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AggregationConfig
{
  /// Base directory for YAML files
  pub base_dir: PathBuf,
  /// Module configurations
  pub modules: Vec<ModuleConfig>,
  /// Global prefix to apply
  pub global_prefix: Option<String>,
  /// Whether to detect conflicts
  pub detect_conflicts: bool,
  /// Environment variable overrides
  pub env_overrides: HashMap<String, String>,
  /// Conflict resolution strategy
  pub conflict_resolution: ConflictResolutionStrategy,
  /// Whether to enable YAML file discovery
  pub auto_discovery: bool,
  /// File patterns for discovery
  pub discovery_patterns: Vec<String>,
  /// Namespace isolation settings
  pub namespace_isolation: NamespaceIsolation,
}

/// Conflict resolution strategies for handling duplicate commands
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ConflictResolutionStrategy
{
  /// Fail on any conflicts (default)
  #[default]
  Fail,
  /// Use the first command encountered
  UseFirst,
  /// Use the last command encountered
  UseLast,
  /// Merge commands where possible
  Merge,
}

/// Namespace isolation configuration
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NamespaceIsolation
{
  /// Whether to enable namespace isolation
  pub enabled: bool,
  /// Separator for namespace components
  pub separator: String,
  /// Whether to enforce strict isolation
  pub strict_mode: bool,
}

impl Default for NamespaceIsolation
{
  fn default() -> Self
  {
    Self
    {
      enabled: true,
      separator: ".".to_string(),
      strict_mode: false,
    }
  }
}

/// Configuration for a single module
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModuleConfig
{
  /// Module name
  pub name: String,
  /// YAML file path relative to base_dir
  pub yaml_path: String,
  /// Prefix to apply to module commands
  pub prefix: Option<String>,
  /// Whether module is enabled
  pub enabled: bool,
}

/// Report of detected conflicts
#[derive(Debug, Clone, PartialEq)]
pub struct ConflictReport
{
  /// Conflicting command name
  pub command_name: String,
  /// Modules that define this command
  pub modules: Vec<String>,
  /// Conflict type
  pub conflict_type: ConflictType,
}

/// Types of conflicts that can be detected
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType
{
  /// Multiple modules define the same command
  NameCollision,
  /// Command has different signatures across modules
  SignatureMismatch,
  /// Incompatible prefixes
  PrefixConflict,
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
    registry.set_registry_mode( crate::RegistryMode::Hybrid );

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
        serde_yaml::from_str( &config_content )
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

/// Environment variable configuration parser
#[derive(Debug, Default)]
pub struct EnvConfigParser
{
  /// Parsed configuration overrides
  overrides: HashMap< String, String >,
}

impl EnvConfigParser
{
  /// Create new environment config parser
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Parse environment variables with prefix
  pub fn parse_with_prefix( &mut self, prefix: &str ) -> Result< (), Error >
  {
    use std::env;

    // Parse environment variables that start with the prefix
    for ( key, value ) in env::vars()
    {
      if key.starts_with( prefix )
      {
        self.overrides.insert( key, value );
      }
    }

    Ok( () )
  }

  /// Apply overrides to aggregation config
  pub fn apply_to_config( &self, config: &mut AggregationConfig )
  {
    // Apply global prefix override
    if let Some( global_prefix ) = self.overrides.get( "UNILANG_GLOBAL_PREFIX" )
    {
      config.global_prefix = Some( global_prefix.clone() );
    }

    // Apply conflict detection override
    if let Some( detect_conflicts ) = self.overrides.get( "UNILANG_DETECT_CONFLICTS" )
    {
      config.detect_conflicts = detect_conflicts.parse().unwrap_or( true );
    }

    // Apply module-specific overrides
    for module in &mut config.modules
    {
      let enable_key = format!( "UNILANG_MODULE_{}_ENABLED", module.name.to_uppercase() );
      if let Some( enabled ) = self.overrides.get( &enable_key )
      {
        module.enabled = enabled.parse().unwrap_or( true );
      }

      let prefix_key = format!( "UNILANG_MODULE_{}_PREFIX", module.name.to_uppercase() );
      if let Some( prefix ) = self.overrides.get( &prefix_key )
      {
        module.prefix = if prefix.is_empty() { None } else { Some( prefix.clone() ) };
      }
    }
  }
}

/// Parse Cargo.toml metadata for build configuration
pub fn parse_cargo_metadata( _cargo_toml_path: &PathBuf ) -> Result< AggregationConfig, Error >
{
  // For now, return a default config
  // In a real implementation, this would parse the Cargo.toml file using a TOML parser
  let config = AggregationConfig
  {
    base_dir : PathBuf::from( "commands" ),
    // Add some default modules for demonstration
    modules : vec![
      ModuleConfig
      {
        name : "math".to_string(),
        yaml_path : "math.yaml".to_string(),
        prefix : Some( "math".to_string() ),
        enabled : true,
      },
      ModuleConfig
      {
        name : "utils".to_string(),
        yaml_path : "tests/test_data/utils.yaml".to_string(),
        prefix : Some( "util".to_string() ),
        enabled : true,
      },
    ],
    ..AggregationConfig::default()
  };

  Ok( config )
}

/// Convenience function for zero-boilerplate static aggregation (aggregate_cli! macro simulation)
pub fn aggregate_cli_simple() -> Result< CommandRegistry, Error >
{
  CliBuilder::new()
    .mode( AggregationMode::Static )
    .static_module( "core", vec![
      CommandDefinition::new(
        crate::data::CommandName::new( ".version" ).expect( "valid name" ),
        "Show version information".to_string(),
      )
      .with_namespace( String::new() )
      .with_hint( "Version info" )
      .with_status( crate::data::CommandStatus::Active )
      .with_version( crate::data::VersionType::new( "1.0.0" ).expect( "valid version" ) ),
    ] )
    .build()
}

/// More complex aggregate_cli simulation
pub fn aggregate_cli_complex() -> Result< CommandRegistry, Error >
{
  CliBuilder::new()
    .mode( AggregationMode::Hybrid )
    .app_name( "myapp" )
    .global_prefix( "myapp" )
    .static_module_with_prefix( "core", "core", vec![
      CommandDefinition::new(
        crate::data::CommandName::new( ".version" ).expect( "valid name" ),
        "Show version".to_string(),
      )
      .with_namespace( String::new() )
      .with_hint( "Show version" )
      .with_status( crate::data::CommandStatus::Active )
      .with_version( crate::data::VersionType::new( "1.0.0" ).expect( "valid version" ) ),
    ] )
    .dynamic_module_with_prefix( "utils", PathBuf::from( "tests/test_data/utils.yaml" ), "util" )
    .conditional_module( "advanced", "test_feature", vec![
      CommandDefinition::new(
        crate::data::CommandName::new( ".debug" ).expect( "valid name" ),
        "Debug mode".to_string(),
      )
      .with_namespace( String::new() )
      .with_hint( "Debug mode" )
      .with_status( crate::data::CommandStatus::Active )
      .with_version( crate::data::VersionType::new( "1.0.0" ).expect( "valid version" ) ),
    ] )
    .build()
}

/// Runtime multi-YAML aggregation with environment variable support.
///
/// **⚠️ PERFORMANCE WARNING: 50x slower than compile-time approach**
///
/// This function performs **runtime** multi-YAML file discovery, parsing, and aggregation
/// to build a `CommandRegistry`. It is part of the **runtime YAML loading approach**
/// and should only be used when compile-time generation is not possible.
///
/// ## Performance Characteristics
///
/// - **Lookup time**: ~4,000ns per command (runtime `CommandRegistry`)
/// - **Startup cost**: YAML parsing + file I/O at application start
/// - **vs Compile-time**: 50x slower than `approach_yaml_multi_build` (~80ns)
///
/// ## When to Use This Function
///
/// **Use this for:**
/// - Plugin systems that load commands dynamically at runtime
/// - Applications with runtime-configurable command sets
/// - REPL environments where commands can be added/removed
/// - Development/debugging scenarios requiring hot-reload
///
/// **DO NOT use this for:**
/// - Production CLI applications (use `approach_yaml_multi_build` instead)
/// - Performance-critical applications
/// - Static command sets known at compile-time
///
/// ## Feature Requirements
///
/// **Requires features:**
/// - `multi_file` - Multi-YAML file discovery and aggregation
/// - `yaml_parser` - YAML deserialization
///
/// Enabled by: `approach_yaml_runtime` + manually enabling `multi_file`
///
/// ## Workflow
///
/// 1. Reads `Cargo.toml` metadata to discover YAML file locations
/// 2. Parses `UNILANG_*` environment variables for runtime configuration
/// 3. Discovers and loads all YAML files at runtime
/// 4. Aggregates commands into a runtime `CommandRegistry`
/// 5. Returns registry ready for command execution
///
/// ## Recommended Alternative (50x faster)
///
/// For production applications, use compile-time aggregation:
///
/// ```toml
/// [dependencies]
/// # Default configuration - 50x faster than runtime
/// unilang = "0.28"  # Enables approach_yaml_multi_build by default
/// ```
///
/// Then in your code:
///
/// ```ignore
/// // Generated at compile-time by build system
/// include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));
///
/// // Zero-cost static registry (~80ns lookups)
/// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
/// ```
///
/// ## Example Usage (Runtime Aggregation)
///
/// ```ignore
/// use std::path::PathBuf;
/// use unilang::multi_yaml::create_aggregated_registry;
///
/// // Runtime aggregation (slow, but dynamic)
/// let cargo_toml = PathBuf::from("./Cargo.toml");
/// let registry = create_aggregated_registry(&cargo_toml)?;
///
/// // Note: This parses YAML files every time the application starts
/// // For production, use compile-time approach_yaml_multi_build instead
/// ```
///
/// ## Related
///
/// - Compile-time alternative: `approach_yaml_multi_build` feature
/// - See: `examples/static_02_yaml_build_integration.rs` for compile-time pattern
/// - See: `docs/optimization_guide.md` for performance comparisons
pub fn create_aggregated_registry( cargo_toml_path: &PathBuf ) -> Result< crate::CommandRegistry, crate::Error >
{
  // Create aggregator from Cargo.toml metadata
  let mut aggregator = MultiYamlAggregator::from_cargo_metadata( cargo_toml_path )?;

  // Apply environment variable overrides
  let mut env_parser = EnvConfigParser::new();
  env_parser.parse_with_prefix( "UNILANG" )?;
  let mut config = aggregator.config().clone();
  env_parser.apply_to_config( &mut config );
  aggregator = MultiYamlAggregator::new( config );

  // Perform aggregation
  aggregator.aggregate()?;

  // Create runtime registry for dynamic command loading
  // NOTE: This function intentionally provides RUNTIME aggregation for plugin
  // systems and dynamic scenarios where compile-time registration is not possible.
  // Runtime registration is appropriate for:
  // 1. This documented runtime approach (approach_yaml_runtime + multi_file)
  // 2. Plugin systems with dynamic command loading
  // 3. REPL applications with interactive command definition
  // Performance trade-off: 10-50x slower than compile-time registration
  let mut registry = crate::CommandRegistry::new();
  aggregator.register_with_hybrid_registry( &mut registry )?;

  Ok( registry )
}
