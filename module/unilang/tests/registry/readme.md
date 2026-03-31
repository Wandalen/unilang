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
