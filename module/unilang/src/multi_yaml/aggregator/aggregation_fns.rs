//! Convenience aggregation functions for multi-YAML registry construction.
//!
//! Free functions that combine `MultiYamlAggregator`, `CliBuilder`, and
//! environment configuration into ready-to-use registries.

#[ allow( unused_imports ) ]
use crate::*;
use crate::multi_yaml::builder::{ CliBuilder, AggregationMode };
use std::path::PathBuf;
use super::core::MultiYamlAggregator;
use super::core_types::{ AggregationConfig, ModuleConfig, EnvConfigParser };

/// Parse Cargo.toml metadata for build configuration
pub fn parse_cargo_metadata( _cargo_toml_path: &PathBuf ) -> Result< AggregationConfig, crate::Error >
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
pub fn aggregate_cli_simple() -> Result< crate::CommandRegistry, crate::Error >
{
  CliBuilder::new()
    .mode( AggregationMode::Static )
    .static_module( "core", vec![
      crate::data::CommandDefinition::new(
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
pub fn aggregate_cli_complex() -> Result< crate::CommandRegistry, crate::Error >
{
  CliBuilder::new()
    .mode( AggregationMode::Hybrid )
    .app_name( "myapp" )
    .global_prefix( "myapp" )
    .static_module_with_prefix( "core", "core", vec![
      crate::data::CommandDefinition::new(
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
      crate::data::CommandDefinition::new(
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
/// ```text
/// // Generated at compile-time by build system
/// include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));
///
/// // Zero-cost static registry (~80ns lookups)
/// let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
/// ```
///
/// ## Example Usage (Runtime Aggregation)
///
/// ```text
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
