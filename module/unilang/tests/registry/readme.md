# Registry Tests

Tests for command registration, lookup, static/dynamic modes, and validation.

## Files

| File | Responsibility |
|------|----------------|
| `registry_basic.rs` | Core registration and lookup operations |
| `static_registry.rs` | Compile-time static registry behavior |
| `phf_map_functionality.rs` | PHF-based map lookup correctness |
| `duplicate_detection.rs` | Duplicate command registration error handling |
| `registration_error_handling.rs` | Error paths in command registration |
| `validation_enforcement.rs` | Registry-level validation rule enforcement |
| `debug.rs` | Debug output and introspection of registry state |
| `command_loader_yaml.rs` | YAML file loading into command registry |
| `command_loader_json.rs` | JSON file loading into command registry |
| `command_loader_build_time.rs` | Build-time static command loading paths |
| `command_loader_error.rs` | Error handling for malformed command loader input |
| `rust_dsl_inline_closure.rs` | Rust DSL inline closure command registration |
| `static_const_constructor.rs` | Static const constructor approach for commands |
| `feature_parity.rs` | Feature parity between dynamic and static registries |
| `static_registry_conversion.rs` | Conversion from dynamic definitions to static registry |
| `phf_reexport.rs` | PHF map accessible via unilang re-export without direct dep |
| `multi_yaml_conflict_detection.rs` | Conflict detection when merging multiple YAML sources |
