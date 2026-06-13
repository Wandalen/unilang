//! Configuration types and environment parser for multi-YAML aggregation.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::Error;

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
