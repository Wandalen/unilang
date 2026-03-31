# CommandDefinition Module

Split from `data/command_definition.rs`. Full definition and construction of commands.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point and public re-exports |
| `core.rs` | `CommandDefinition` struct, accessors, and core methods |
| `builder.rs` | `CommandDefinitionBuilder` — fluent command construction |
| `accessors.rs` | Additional accessor methods for command metadata |
| `serde_impl.rs` | Serde serialization/deserialization for YAML/JSON |
