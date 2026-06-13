#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc
(
  html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
) ]
#![ doc( html_root_url = "https://docs.rs/unilang/latest/unilang/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Universal language processing" ) ]

//!
//! ## Design Rules Compliance Notice
//!
//! **CRITICAL: This codebase must follow strict design rules. Before making changes, review:**
//! - `$PRO/genai/code/rules/code_design.rulebook.md` - Core design patterns and architecture rules
//! - `$PRO/genai/code/rules/code_style.rulebook.md` - Code formatting and style requirements
//!
//! **Key Rules Summary:**
//! - **Testing:** All tests MUST be in `tests/` directory, NOT in `src/` as `mod tests`
//! - **Benchmarking:** Use `benchkit` framework ONLY - no custom timing code in tests
//! - **Performance Tests:** NEVER mix benchmarks with unit tests - separate concerns
//! - **Test Documentation:** Every test file MUST have Test Matrix documentation
//! - **Directory Structure:** `tests/` for tests, `benches/` for benchmarks (if using benchkit)
//!
//! **Common Violations to Avoid:**
//! ❌ Custom `std::time::Instant` timing code in test files
//! ❌ Performance/benchmark tests in `tests/` directory
//! ❌ Missing file-level documentation with Test Matrix in test files
//! ❌ Using anything other than `benchkit` for performance measurement
//!
//! ## Feature Flags
//!
//! Unilang supports multiple feature flags to customize functionality and dependencies:
//!
//! ### Core Features
//! - `enabled` - Core functionality (included in `default`)
//! - `full` - All features enabled for maximum functionality
//!
//! ### REPL Features  
//! - **`repl`** - Basic REPL functionality with standard I/O
//!   - Provides interactive command execution
//!   - Basic command history tracking
//!   - Cross-platform compatibility
//!   - No additional dependencies
//!
//! - **`enhanced_repl`** ⭐ **Enabled by Default** - Advanced REPL with rustyline integration
//!   - **Enables**: All features from `repl` plus:
//!   - **Arrow Key Navigation**: ↑/↓ for command history browsing
//!   - **Tab Auto-completion**: Command and argument completion
//!   - **Interactive Prompts**: Secure password input with masking
//!   - **Session Persistence**: History saved across sessions
//!   - **Terminal Detection**: Auto-fallback to basic REPL in non-interactive environments
//!   - **Dependencies**: `rustyline`, `std::io::IsTerminal`
//!
//! ### Performance Features
//! - **`simd`** - SIMD optimizations for parsing and JSON processing
//!   - **Enables**: `simd-json` (4-25x faster JSON), SIMD string operations
//!   - **Automatic**: Included in `default` for maximum performance
//!   - **Disable with**: `cargo build --no-default-features --features enabled`
//!
//! ### Optional Features
//! - `on_unknown_suggest` - Fuzzy command suggestions (requires `textdistance`)
//!
//! **Note**: Benchmarking tools are available in the separate `unilang_benchmarks` workspace crate
//!
//! ### Usage Examples
//!
//! **Basic REPL (minimal dependencies):**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", features = ["repl"] }
//! ```
//!
//! **Default (Enhanced REPL included):**
//! ```toml
//! [dependencies]
//! unilang = "0.10"  # Enhanced REPL enabled by default
//! ```
//!
//! **Performance-optimized CLI:**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", features = ["enhanced_repl", "simd", "on_unknown_suggest"] }
//! ```
//!
//! **Embedded/minimal:**
//! ```toml
//! [dependencies]
//! unilang = { version = "0.10", default-features = false, features = ["enabled"] }
//! ```
//!
//! ### Feature Compatibility
//!
//! - `enhanced_repl` automatically includes `repl`
//! - `full` includes all features except development-only ones
//! - All features work together without conflicts
//! - Enhanced REPL gracefully falls back to basic REPL when needed

/// Internal namespace.
mod private
{
}

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{
  /// Core data structures and types.
  layer data;

  /// Static data structures for compile-time commands.
  /// **Requires feature**: `static_registry`
  #[ cfg( feature = "static_registry" ) ]
  layer static_data;

  /// Error handling utilities.
  layer error;

  /// Configuration loading from YAML/JSON.
  /// Functions gated by `yaml_parser` and `json_parser` features.
  layer loader;

  /// Value types and type system.
  layer types;

  /// Help generation system.
  layer help;

  /// Command execution interpreter.
  layer interpreter;

  /// Command registry management.
  /// Some functions gated by approach features.
  layer registry;

  /// Command validation utilities.
  layer command_validation;

  /// Core validation logic shared between runtime and build.rs.
  /// This module can be included in build.rs via include!() since it has no dependencies.
  layer validation_core;

  /// Semantic analysis and validation.
  layer semantic;

  /// High-level pipeline API.
  layer pipeline;

  /// Multi-YAML build system for compile-time aggregation.
  /// **Requires feature**: `multi_file`
  #[ cfg( feature = "multi_file" ) ]
  layer multi_yaml;

  /// String interning system for performance optimization.
  layer interner;

  /// SIMD-optimized JSON parsing for 4-25x performance improvements.
  /// **Requires features**: `simd-json` AND `json_parser`
  #[ cfg( all( feature = "simd-json", feature = "json_parser" ) ) ]
  layer simd_json_parser;

  /// SIMD-optimized tokenization for 3-6x performance improvements.
  layer simd_tokenizer;

  /// Build-time helper utilities for type analysis and hint generation.
  /// Provides tools for detecting type issues in YAML command definitions during build.
  /// **Requires feature**: `yaml_parser`
  #[ cfg( feature = "yaml_parser" ) ]
  layer build_helpers;

  /// Config value extraction utilities.
  /// Generic extractors for `HashMap<String, (JsonValue, S)>` config maps.
  /// **Requires feature**: `json_parser`
  #[ cfg( feature = "json_parser" ) ]
  layer config_extraction;


  // NOTE: Benchmark modules have been moved to unilang_benchmarks workspace crate
  // to avoid polluting production dependencies. Use unilang_benchmarks for all
  // benchmarking needs.
}

/// Re-export unilang_parser crate as parser module.
///
/// Provides full access to the parser infrastructure including:
/// - `Parser` and `UnilangParserOptions` for parsing
/// - `GenericInstruction` and `Argument` for results
/// - `cli_parser` module for CLI parameter parsing
/// - `prelude` for convenient imports
///
/// # Example
///
/// ```rust,no_run
/// use unilang::parser::{Parser, UnilangParserOptions};
/// use unilang::parser::cli_parser::{parse_cli_args, CliParams};
/// ```
#[ cfg( feature = "enabled" ) ]
pub use unilang_parser as parser;

/// Re-export of input marker newtypes from `unilang_parser`.
///
/// Provides direct access to [`ShellArgv`] and [`ReplInput`] without requiring
/// users to import from `unilang::parser::argv_types` directly.
#[ cfg( feature = "enabled" ) ]
pub use unilang_parser ::{ ShellArgv, ReplInput };

/// Re-export of PHF (Perfect Hash Function) types for generated code.
///
/// # Purpose
///
/// When unilang's build script generates static command registries, the generated
/// code uses PHF maps for compile-time perfect hashing. To prevent forcing downstream
/// crates to add `phf` as a direct dependency, unilang re-exports these types.
///
/// # Generated Code Pattern
///
/// The build script generates code like:
/// ```text
/// use unilang::phf::{self, Map};
///
/// static COMMANDS: Map<&'static str, CommandDefinition> = phf::phf_map! {
///   ".help" => help_cmd,
///   ".version" => version_cmd,
/// };
/// ```
///
/// # Usage Example
///
/// ```text
/// // In your crate's Cargo.toml:
/// [dependencies]
/// unilang = { version = "0.46", features = ["static_registry"] }
///
/// // In your crate's code:
/// use unilang::phf::{self, Map};  // Import module + types
///
/// static MY_COMMANDS: Map<&str, u32> = phf::phf_map! {  // Qualified call
///   "help" => 1,
///   "version" => 2,
/// };
/// ```
///
/// **Important**: Always import the `phf` module itself with `self` and use
/// qualified macro calls (`phf::phf_map!`), not `phf_map!` directly. This ensures
/// the macro's internal `$crate` references resolve correctly.
///
/// # Important: Do NOT Add PHF as Direct Dependency
///
/// ❌ **WRONG**:
/// ```toml,ignore
/// [dependencies]
/// unilang = { version = "0.46", features = ["static_registry"] }
/// phf = "0.11"  # ← DO NOT ADD THIS
/// ```
///
/// ✅ **CORRECT**:
/// ```toml,ignore
/// [dependencies]
/// unilang = { version = "0.46", features = ["static_registry"] }
/// # PHF types available via unilang::phf - no need to add phf
/// ```
///
/// # Feature Gate
///
/// This re-export is only available when the `static_registry` feature is enabled.
/// This matches the feature that enables build script code generation.
///
/// # Version Control
///
/// Unilang controls the PHF version to ensure compatibility. The re-exported PHF
/// matches the version used internally by unilang (currently phf ^0.11).
///
/// # Migration Guide
///
/// If you previously added `phf` as a direct dependency, see the readme.md
/// "Migration from Direct PHF Dependency" section for upgrade instructions.
#[cfg(feature = "static_registry")]
pub use phf;

// Improved error message when feature not enabled
// This provides helpful guidance instead of generic "unresolved import" error
// NOTE: Requires manual addition of 'phf_error_hint' feature to trigger
#[cfg(all(not(feature = "static_registry"), feature = "phf_error_hint"))]
compile_error!(
  "PHF re-export requires the 'static_registry' feature.\n\
   \n\
   Add this to your Cargo.toml:\n\
   \n\
   [dependencies]\n\
   unilang = { version = \"0.46\", features = [\"static_registry\"] }\n\
   \n\
   Or enable all features:\n\
   unilang = { version = \"0.46\", features = [\"all\"] }"
);