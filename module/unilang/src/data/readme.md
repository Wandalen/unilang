# Data Models

Core domain types for command definitions, arguments, namespaces, and validated values.

## Files

| File | Responsibility |
|------|----------------|
| `command_definition/` | `CommandDefinition` struct, builder, and serde support |
| `argument_types.rs` | `ArgumentDefinition`, `ArgumentAttributes`, `ValidationRule` |
| `kind.rs` | `Kind` enum and its parse/conversion/format impls |
| `command_status.rs` | `CommandStatus` enum (active, deprecated, etc.) |
| `error_types.rs` | Domain-specific error variants for data operations |
| `namespace.rs` | `Namespace` type for command path organization |
| `command_name.rs` | `CommandName` — validated dot-prefixed command name |
| `namespace_type.rs` | `NamespaceType` — validated empty-or-dot-prefixed namespace |
| `version_type.rs` | `VersionType` — validated non-empty version string |
