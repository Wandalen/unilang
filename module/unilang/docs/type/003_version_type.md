# Type: Version Type

### Scope

- **Purpose:** Guarantee non-empty version strings for command metadata
- **Responsibility:** Documents the VersionType validated newtype and its construction rules
- **In Scope:** Validation rules (non-empty), construction API, error variant
- **Out of Scope:** Version formatting conventions beyond non-empty, semantic versioning policy

### Definition

`VersionType` is a newtype wrapper around `String` that guarantees:

1. The inner value is non-empty

### Validation

| Rule | Check | Error |
|------|-------|-------|
| Non-empty | `version.is_empty()` | `Error::EmptyVersion` |

### Construction

- `VersionType::new(impl Into<String>) -> Result<Self, Error>` — fallible constructor
- Serde `Deserialize` — validates on deserialization

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Lists VersionType in public type surface |

### Sources

| File | Relationship |
|------|--------------|
| `src/data/validated_types.rs` | VersionType struct and impl |

### Tests

| File | Relationship |
|------|--------------|
| `tests/data/validated_version_status.rs` | Version construction and validation |
