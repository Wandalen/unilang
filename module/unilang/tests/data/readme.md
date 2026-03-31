# Data Model Tests

Tests for serialization, validation, and consistency of core data structures.

## Files

| File | Responsibility |
|------|----------------|
| `command_definition.rs` | `CommandDefinition` construction, serialization, and accessors |
| `api_consistency.rs` | Cross-API consistency checks for data model interfaces |
| `error_handling.rs` | Error propagation and error type correctness |
| `loader.rs` | YAML/JSON loading into `CommandDefinition` structures |
| `static_data.rs` | Static command data compilation and lookup |
