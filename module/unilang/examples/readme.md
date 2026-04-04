# examples/

Comprehensive examples demonstrating unilang framework usage patterns and features.

## Quick Start Examples (00_*)

- `00_minimal.rs` - Absolute minimum working example
- `00_pipeline_basics.rs` - Basic pipeline usage
- `00_quick_start.rs` - Getting started guide

## Feature Examples (01-24)

### Core Features (01-09)
- `01_basic_command_registration.rs` - Command registration patterns
- `02_argument_types.rs` - Argument type system
- `03_collection_types.rs` - List and collection arguments
- `04_validation_rules.rs` - Input validation
- `05_namespaces_and_aliases.rs` - Command organization and aliases
- `06_help_system.rs` - Help generation
- `07_yaml_json_loading.rs` - External config loading
- `08_semantic_analysis_simple.rs` - Semantic analysis
- `09_command_execution.rs` - Command execution pipeline

### Advanced Features (10-19)
- `10_full_pipeline.rs` - Full pipeline demo
- `11_pipeline_api.rs` - Pipeline API patterns
- `12_error_handling.rs` - Error handling and type validation
- `12_repl_loop.rs` - REPL loop
- `13_static_dynamic_registry.rs` - Static and dynamic registry
- `14_advanced_types_validation.rs` - Advanced types and validation
- `15_interactive_repl_mode.rs` - Interactive REPL mode
- `16_comprehensive_loader_demo.rs` - Comprehensive loader demo
- `17_advanced_repl_features.rs` - Advanced REPL features
- `18_help_conventions_demo.rs` - Help conventions demo

### Integration Examples (20-29)
- `20_rust_dsl_inline_closures.rs` - Inline closure DSL
- `21_rust_dsl_static.rs` - Static DSL registration
- `22_minimal_cli_aggregation.rs` - CLI aggregation across modules
- `23_help_verbosity_demo.rs` - Help verbosity levels
- `24_posix_style_commands.rs` - POSIX-compatible command interfaces

## Static Registry Examples (static_*)

Compile-time command registration with zero-overhead lookups using Perfect Hash Functions.

- `static_01_basic_compile_time.rs` - Basic compile-time registry
- `static_02_yaml_build_integration.rs` - YAML build integration
- `static_03_performance_comparison.rs` - Runtime vs compile-time performance comparison
- `static_04_multi_module_aggregation.rs` - Multi-module static aggregation

## Aggregation Examples

- `compile_time_aggregation.rs` - Compile-time CLI aggregation demo
- `ergonomic_cli_aggregation.rs` - Ergonomic CLI export and aggregation
- `full_cli_example.rs` - Comprehensive CLI framework usage
- `practical_cli_aggregation.rs` - Real-world CLI aggregation patterns
- `repl_comparison.rs` - REPL feature comparison
- `yaml_cli_aggregation.rs` - YAML-based CLI aggregation workflow
- `manual_corner_case_test.rs` - Edge cases difficult to cover in automated tests

## Running Examples

```bash
# Run specific example
cargo run --example 00_minimal --all-features

# Run with specific approach
cargo run --example static_01_basic_compile_time --features approach_yaml_multi_build
```

## See Also

- Main documentation: `../readme.md`
- Specification: `../spec/readme.md`
- Tests: `../tests/`
