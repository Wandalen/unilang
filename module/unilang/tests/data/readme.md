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
| `data_model_features.rs` | CLI-level data model field and attribute rendering |
| `get_string_normalized.rs` | String normalization across data model values |
| `types.rs` | Type system correctness and conversions |
| `validated_newtypes.rs` | Newtype validation wrapper invariants |
| `category_field_backward_compat.rs` | Category field backward compatibility guarantees |
| `category_field_codegen.rs` | Category field code generation correctness |
| `category_field_conversion.rs` | Category field type conversion and coercion |
| `category_field_edge_cases.rs` | Edge cases in category field handling |
| `category_field_unit.rs` | Category field unit-level behavior |
| `config_extraction.rs` | Configuration value extraction from command definitions |
