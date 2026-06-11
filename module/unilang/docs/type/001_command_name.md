# Type: Command Name

### Scope

- **Purpose:** Guarantee the dot-prefix naming convention for all command identifiers
- **Responsibility:** Documents the CommandName validated newtype, its construction rules, and error behavior
- **In Scope:** Validation rules (non-empty, dot prefix), construction API, error variants, serde behavior
- **Out of Scope:** Command naming policy rationale (see invariant/005), registry behavior (see feature/001)

### Definition

`CommandName` is a newtype wrapper around `String` that guarantees:

1. The inner value is non-empty
2. The inner value starts with `.` (dot prefix)

Invalid states are impossible to represent. If you hold a `CommandName`, it is valid.

### Validation

| Rule | Check | Error |
|------|-------|-------|
| Non-empty | `name.is_empty()` | `Error::EmptyCommandName` |
| Dot prefix | `!name.starts_with('.')` | `Error::MissingDotPrefix(name)` |

### Construction

- `CommandName::new(impl Into<String>) -> Result<Self, Error>` — fallible constructor
- Serde `Deserialize` — validates on deserialization, rejects invalid names

### Accessors

- `as_str() -> &str` — borrow inner string
- `into_inner() -> String` — consume and unwrap

### Invariants

| File | Relationship |
|------|--------------|
| [005_command_naming.md](../invariant/005_command_naming.md) | Formalizes the naming contract this type enforces |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | FR-REG-6 specifies naming rules |

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Lists CommandName in public type surface |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/validated_types.rs` | CommandName struct and impl |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/validated_command_name.rs` | Construction, validation, edge cases |
