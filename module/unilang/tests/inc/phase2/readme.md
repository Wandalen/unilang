# Phase 2 Tests

Feature integration tests covering CLI, loaders, registry, validation, and help generation.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module root for phase 2 tests |
| `argument_types_test.rs` | All argument kinds: String, Int, Bool, Path, URL, etc. |
| `cli_integration_test.rs` | CLI builder end-to-end integration |
| `collection_types_test.rs` | Vec/Set/Map collection argument types |
| `command_loader_build_time_test.rs` | Build-time YAML/JSON loader pipeline |
| `command_loader_error_test.rs` | Loader error handling and reporting |
| `command_loader_json_test.rs` | JSON command definition loading |
| `command_loader_yaml_test.rs` | YAML command definition loading |
| `command_validation_test.rs` | Command name, namespace, version validation |
| `complex_types_and_attributes_test.rs` | Nested types, optional/required attributes |
| `help_generation_test.rs` | Help text generation across verbosity levels |
| `runtime_command_registration_test.rs` | Dynamic runtime command registration |
| `rust_dsl_inline_closure_test.rs` | Rust DSL builder API with inline closures |
| `static_const_constructor_test.rs` | Const fn static command constructors |
