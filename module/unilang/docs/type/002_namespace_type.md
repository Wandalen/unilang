# Type: Namespace Type

### Scope

- **Purpose:** Guarantee valid namespace formatting for command grouping
- **Responsibility:** Documents the NamespaceType validated newtype, its construction rules, and the empty-namespace special case
- **In Scope:** Validation rules (empty or dot-prefixed), construction API, error variants, the root-level command case
- **Out of Scope:** Command naming policy rationale (see invariant/005), namespace isolation in aggregation (see feature/001)

### Definition

`NamespaceType` is a newtype wrapper around `String` that guarantees:

1. Either the inner value is empty (root-level commands like `.help`)
2. Or the inner value starts with `.` (e.g., `.video`, `.session`)

An empty value can arise from two distinct authoring shapes at the `CommandDefinition` level — an *omitted* `namespace` field, or an *explicit* `namespace: ""` — which `NamespaceType` itself does not distinguish (both validate identically here). The two shapes are NOT equivalent one level up at the deserializer: an omitted namespace on a compound dotted `name` triggers a convenience compact-form split, while an explicit empty namespace does not. See `invariant/005_command_naming.md` for the full algorithm.

### Validation

| Rule | Check | Error |
|------|-------|-------|
| Empty allowed | `namespace.is_empty()` | (valid — root level) |
| Non-empty must have dot | `!namespace.starts_with('.')` | `Error::InvalidNamespace(namespace)` |

### Construction

- `NamespaceType::new(impl Into<String>) -> Result<Self, Error>` — fallible constructor
- Serde `Deserialize` — validates on deserialization

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Lists NamespaceType in public type surface |

### Invariants

| File | Relationship |
|------|--------------|
| [005_command_naming.md](../invariant/005_command_naming.md) | Namespace naming convention this type enforces |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/namespace_type.rs` | NamespaceType struct and impl |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/validated_namespace.rs` | Construction, validation, empty namespace |
