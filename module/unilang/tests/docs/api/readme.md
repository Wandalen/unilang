# API Test Surface

Test spec files for `docs/api/` doc instances.
Case prefix: `AP-`. Minimum 4 cases per spec.

### Scope

- **Purpose:** Enumerate test cases verifying public API contracts and type guarantees
- **Responsibility:** One spec file per API doc instance; each case targets a named public type or compatibility guarantee
- **In Scope:** `CommandDefinition`, `ArgumentDefinition`, `CommandRegistry`, `Pipeline`, `Value`, `Kind`, `StaticCommandDefinition`, `OutputData`, `ErrorData`; semver compatibility guarantees
- **Out of Scope:** Internal implementation details; behavioral feature testing (covered in `feature/`)

### Overview Table

| Name | Purpose | Status |
|------|---------|--------|
| `01_public_types.md` | `api` spec for all public types and compatibility guarantees | ⏳ |
