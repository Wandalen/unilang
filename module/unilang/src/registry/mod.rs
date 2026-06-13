//!
//! The command registry for the Unilang framework.
//!
//! ## Performance Optimization Design Notes
//!
//! This module implements performance optimizations following design rules:
//!
//! **✅ CORRECT Performance Implementation:**
//! - LRU caching for hot commands (production optimization)
//! - Compile-time optimized static commands (zero-overhead lookups)
//! - Hybrid registry modes for different workload patterns
//! - Memory-efficient IndexMap storage for cache locality
//!
//! **❌ TESTING VIOLATIONS TO AVOID:**
//! - Do NOT add custom timing code (`std::time::Instant`) in tests
//! - Do NOT create performance assertions in unit tests
//! - Do NOT mix benchmarks with functional tests
//! - Use `benchkit` framework for performance measurement
//!
//! **Rule Compliance:**
//! - Performance optimizations: ✅ Implemented in production code
//! - Performance testing: ❌ Must use `benchkit`, not custom test files
//! - Test separation: ✅ `tests/` for correctness, `benchkit` for performance
//!

// NOTE: The generated static_commands.rs file is NOT included here.
// It's meant for external users (examples, applications using unilang).
// External users should include it in their own code:
//   include!(concat!(env!("OUT_DIR"), "/static_commands.rs"));

mod metrics;
mod traits;
mod map;
mod builder;
mod dynamic;
mod help;
mod trait_impl;
#[ cfg( feature = "static_registry" ) ]
mod static_reg;
#[ cfg( feature = "static_registry" ) ]
mod bridge;

/// Internal namespace.
mod private {}

mod_interface::mod_interface!
{
  exposed use traits::CommandRoutine;
  exposed use traits::CommandRegistryTrait;
  exposed use traits::RegistryMode;
  exposed use map::DynamicCommandMap;
  exposed use builder::CommandRegistryBuilder;
  exposed use dynamic::CommandRegistry;
  exposed use metrics::PerformanceMetrics;
  #[ cfg( feature = "static_registry" ) ]
  exposed use static_reg::StaticCommandRegistry;

  prelude use traits::RegistryMode;
  prelude use metrics::PerformanceMetrics;
  prelude use traits::CommandRoutine;
  #[ cfg( feature = "static_registry" ) ]
  #[ doc = "High-performance static command registry with zero-cost compile-time lookup." ]
  prelude use static_reg::StaticCommandRegistry;
  #[ doc = "Runtime command registration. Consider compile-time alternatives for better performance." ]
  prelude use dynamic::CommandRegistry;
  prelude use builder::CommandRegistryBuilder;
}
