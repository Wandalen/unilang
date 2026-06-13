//! Domain types for the CliBuilder API.

use crate::data::CommandDefinition;
use std::collections::HashMap;
use std::path::PathBuf;

/// Ergonomic CLI aggregation modes
#[ derive( Debug, Clone, PartialEq ) ]
pub enum AggregationMode
{
  /// Pure static aggregation (compile-time only)
  Static,
  /// Pure dynamic aggregation (runtime loading)
  Dynamic,
  /// Hybrid mode (static + dynamic optimizations)
  Hybrid,
  /// Automatic mode selection based on environment
  Auto,
}

/// Static module configuration for ergonomic APIs
#[ derive( Debug, Clone ) ]
pub struct StaticModule
{
  /// Module identifier
  pub name : String,
  /// Commands to include
  pub commands : Vec< CommandDefinition >,
  /// Namespace prefix
  pub prefix : Option< String >,
  /// Whether module is enabled
  pub enabled : bool,
}

/// Dynamic YAML module configuration for ergonomic APIs
#[ derive( Debug, Clone ) ]
pub struct DynamicModule
{
  /// Module identifier
  pub name : String,
  /// YAML file path
  pub yaml_path : PathBuf,
  /// Namespace prefix
  pub prefix : Option< String >,
  /// Whether module is enabled
  pub enabled : bool,
}

/// Conditional module based on feature flags
#[ derive( Debug, Clone ) ]
pub struct ConditionalModule
{
  /// Module identifier
  pub name : String,
  /// Feature flag to check
  pub feature : String,
  /// Module configuration when enabled
  pub module : Box< StaticModule >,
}

/// Re-export ModuleConfig from aggregator to avoid duplication
pub use crate::multi_yaml::aggregator::ModuleConfig;

/// Module source type for aggregation
#[ derive( Debug, Clone ) ]
pub enum ModuleSource
{
  /// Static commands compiled into binary
  Static( StaticModule ),
  /// Dynamic YAML file loaded at runtime
  Dynamic( DynamicModule ),
  /// Conditional module based on feature flags
  Conditional( ConditionalModule ),
}

/// Global CLI configuration
#[ derive( Debug, Clone, Default ) ]
pub struct CliConfig
{
  /// Application name
  pub app_name : String,
  /// Global prefix for all commands
  pub global_prefix : Option< String >,
  /// Whether to enable help generation
  pub auto_help : bool,
  /// Whether to detect conflicts
  pub detect_conflicts : bool,
  /// Environment variable overrides
  pub env_overrides : HashMap< String, String >,
  /// Environment variable exclusions
  pub exclude_env_overrides : Vec< String >,
}

/// Re-export ConflictReport and ConflictType from aggregator to avoid duplication
pub use crate::multi_yaml::aggregator::{ ConflictReport, ConflictType };
