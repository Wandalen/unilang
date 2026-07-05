# Type: Command Status

### Scope

- **Purpose:** Provide type-safe lifecycle stage classification for commands
- **Responsibility:** Documents the CommandStatus enum, its variants, and structured deprecation metadata
- **In Scope:** Four lifecycle variants (Active, Deprecated, Experimental, Internal), deprecation metadata, serde behavior
- **Out of Scope:** How status affects registry behavior (see feature/001), help output formatting (see feature/004)

### Definition

`CommandStatus` is an enum with four variants representing command lifecycle stages:

| Variant | Meaning | Data |
|---------|---------|------|
| `Active` | Stable for production use | none (default) |
| `Experimental` | API may change | none |
| `Internal` | Internal use only | none |
| `Deprecated` | May be removed in future | `reason`, `since`, `replacement` |

### Query Methods

- `is_active() -> bool`
- `is_deprecated() -> bool`
- `is_experimental() -> bool`
- `is_internal() -> bool`
- `deprecation_info() -> Option<(&str, &Option<String>, &Option<String>)>`

### Validation

| Rule | Check | Error |
|------|-------|-------|
| Known variant | Unknown string during deserialization | Serde rejection with unknown variant message |
| Case-insensitive | Accepts `"active"`, `"Active"`, `"ACTIVE"` | (valid — all casings accepted) |
| Map form | `Deprecated` requires `reason` field | Serde rejection if required fields missing |

### Serde Behavior

Simple variants serialize as lowercase strings (`"active"`, `"experimental"`, `"internal"`). The `Deprecated` variant serializes as a map with `status`, `reason`, `since`, `replacement` fields. Deserialization accepts both forms and is case-insensitive.

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Lists CommandStatus in public type surface |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | Registry uses status for command lifecycle |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/command_status.rs` | CommandStatus enum and impls |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/validated_version_status.rs` | Status construction and serde roundtrip |
