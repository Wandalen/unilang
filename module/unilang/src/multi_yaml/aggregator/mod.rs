//! Multi-YAML Build System and Ergonomic Aggregation APIs
//!
//! This module implements the enhanced build system that processes multiple YAML files
//! and combines them at compile-time with zero runtime overhead. It also provides
//! ergonomic aggregation APIs for simple and complex use cases:
//!
//! - MultiYamlAggregator for processing multiple YAML files
//! - CliBuilder for ergonomic API aggregation
//! - aggregate_cli! macro for zero-boilerplate static aggregation
//! - Prefix application during compilation
//! - Conflict detection across modules
//! - Conditional module loading with feature flags
//! - Intelligent mode selection and auto-detection
//! - Cargo.toml metadata support
//! - Environment variable configuration
//! - Static registry generation with aggregated commands
//! - Integration with hybrid registry system

mod core;
mod core_types;
mod conflict;
mod codegen;
mod aggregation_fns;

pub use core::MultiYamlAggregator;

pub use core_types::{
  AggregationConfig,
  ConflictResolutionStrategy,
  NamespaceIsolation,
  ModuleConfig,
  ConflictReport,
  ConflictType,
  EnvConfigParser,
};

pub use aggregation_fns::{
  parse_cargo_metadata,
  aggregate_cli_simple,
  aggregate_cli_complex,
  create_aggregated_registry,
};
