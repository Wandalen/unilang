# Type Test Surface

Test spec files for `docs/type/` doc instances.
Case prefix: `TC-`. Minimum 4 cases per spec.

### Scope

- **Purpose:** Enumerate test cases verifying domain type construction-time validation guarantees
- **Responsibility:** One spec file per type doc instance; each case exercises a validation rule boundary
- **In Scope:** CommandName dot-prefix validation, NamespaceType empty-or-dot validation, VersionType non-empty validation, CommandStatus lifecycle variants and serde behavior
- **Out of Scope:** Registry behavior using these types (covered in `feature/`); naming policy rationale (covered in `invariant/`)

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `01_command_name.md` | `type` spec for CommandName validated newtype | ✅ |
| `02_namespace_type.md` | `type` spec for NamespaceType validated newtype | ✅ |
| `03_version_type.md` | `type` spec for VersionType validated newtype | ✅ |
| `04_command_status.md` | `type` spec for CommandStatus lifecycle enum | ✅ |
