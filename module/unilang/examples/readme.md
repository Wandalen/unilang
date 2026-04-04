# examples/

Comprehensive examples demonstrating unilang framework usage patterns and features.

## Quick Start Examples (00_*)

- `00_minimal.rs` - Absolute minimum working example
- `00_pipeline_basics.rs` - Basic pipeline usage
- `00_quick_start.rs` - Getting started guide

## Feature Examples (01-19)

### Core Features (01-09)
- `01_basic_command_registration.rs` - Command registration patterns
- `02_argument_types.rs` - Argument type system
- `03_collection_types.rs` - List and collection arguments
- `04_validation_rules.rs` - Input validation
- `05_namespaces_and_aliases.rs` - Command organization
- `06_help_system.rs` - Help generation
- `07_yaml_json_loading.rs` - External config loading
- `08_semantic_validation.rs` - Semantic analysis
- `09_error_handling.rs` - Error handling patterns

### Advanced Features (10-19)
- `10_custom_types.rs` - Custom argument types
- `11_middleware_pipeline.rs` - Pipeline middleware
- `12_dynamic_commands.rs` - Runtime command registration
- `13_repl_mode.rs` - Interactive REPL
- `14_web_api.rs` - Web API integration
- `15_gui_integration.rs` - GUI integration
- `16_cli_parsing.rs` - CLI argument parsing
- `17_performance.rs` - Performance optimization
- `18_security.rs` - Security features
- `19_advanced_patterns.rs` - Advanced patterns

### Integration Examples (20-29)
- `20_rust_dsl_inline_closures.rs` - Inline closure DSL
- `21_rust_dsl_static.rs` - Static DSL registration
- `22_minimal_cli_aggregation.rs` - CLI aggregation across modules
- `23_help_verbosity_demo.rs` - Help verbosity levels
- `24_posix_style_commands.rs` - POSIX-compatible command interfaces

## Static Registry Examples (static_*)

Compile-time command registration with zero-overhead lookups using Perfect Hash Functions.

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
