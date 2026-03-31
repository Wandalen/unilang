# Data Models

Core domain types for command definitions, arguments, namespaces, and validated values.

## Files

| File | Responsibility |
|------|----------------|
| `command_definition/` | `CommandDefinition` struct, builder, and serde support |
| `argument_types.rs` | `ArgumentDefinition`, `Kind`, `ArgumentAttributes` |
| `command_status.rs` | `CommandStatus` enum (active, deprecated, etc.) |
| `error_types.rs` | Domain-specific error variants for data operations |
| `namespace.rs` | `Namespace` type for command path organization |
| `validated_types.rs` | Newtype wrappers enforcing validation invariants |
