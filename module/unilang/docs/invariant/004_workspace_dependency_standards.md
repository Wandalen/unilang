# Invariant: Workspace Dependency Standards

### Scope

- **Purpose:** Enforce correct version format, workspace centralization, and no-op-when-disabled pattern across all workspace member crates
- **Responsibility:** Workspace Cargo.toml version format compliance and library crate optional-dependency feature-gating pattern
- **In Scope:** Version string format (^X.Y for external, =X.Y.Z for internal path deps), workspace centralization, optional deps, `enabled` feature gating, no-op behavior under `--no-default-features`
- **Out of Scope:** Dependency selection decisions, binary crate feature flags (`cargo_unilang` is exempt per rulebook binary exception), wasm-repl and dummy_lib (non-workspace-member crates)

### Invariant Statement

The workspace `Cargo.toml` and all library member crates MUST continuously comply with `crate_distribution.rulebook.md` dependency rules. Four classes of requirements are in scope:

**R1 — Version Format:**
- External deps (no `path` field): `version = "^X.Y"` — explicit caret, major.minor only, patch MUST be omitted
- Internal deps (have `path` field): `version = "=X.Y.Z"` — exact pin with full patch
- Bare strings (`"1.0"`, `"0.4"`) and tilde strings (`"~0.39.0"`) are FORBIDDEN

**R2 — Workspace Centralization:**
- All versions and sources live exclusively in `[workspace.dependencies]`
- Member crates MUST use `{ workspace = true }` for every dependency
- Features belong in member crates, never in the workspace manifest

**R3 — Optional Dependencies Only:**
- Every dependency in library member crates MUST be declared `optional = true`
- Exception: binary crates (cargo_unilang) are exempt

**R4 — No-Op When Disabled:**
- `cargo build -p <lib_crate> --no-default-features` MUST produce zero external dep compilations
- The `enabled` feature MUST activate all deps using `dep:name` syntax
- When `enabled` is off, the crate compiles to an empty library

### Enforcement Mechanism

- **R1:** Audit `[workspace.dependencies]` in the workspace `Cargo.toml` for bare version strings (e.g. `"1.0"`) or tilde-prefixed strings (e.g. `"~0.39"`). Zero matches required.
- **R3:** Audit `[dependencies]` in each library crate's `Cargo.toml` to confirm every entry declares `optional = true`. Build and dev dependencies are exempt.
- **R4:** Build each library crate with `--no-default-features` and confirm zero external crate compilations occur (only the target crate itself compiles).

### Violation Consequences

**R1 violation:** Hidden resolution policy makes dependency audits ambiguous; bare `"1"` does not communicate whether compatible-update semantics are intended; tilde on external deps over-constrains the resolver to patch-level range (wrong for external deps).

**R3/R4 violation:** Building a library crate with `--no-default-features` still compiles heavy transitive dependencies even though nothing is enabled. This defeats the `enabled`/`full` feature isolation architecture mandated in [architecture/001_mandates.md](../architecture/001_mandates.md).

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](002_non_functional_requirements.md) | Performance NFRs enabled by no-op compile pattern |
| [003_governing_principles.md](003_governing_principles.md) | Explicit Dependencies principle source of R2 requirement |

### Architectures

| File | Relationship |
|------|--------------|
| [001_mandates.md](../architecture/001_mandates.md) | `enabled` feature gate mandate source of R4 requirement |

### Known Pitfalls

**bincode 3.0.0 — poison-pill release:** bincode published v3.0.0 as an intentionally breaking release that is incompatible with v1.x APIs and has known stability issues. If bincode is ever reintroduced as a dependency, stay on the `^1.0` range until the 3.x line stabilizes. Do not upgrade to `^3.0` without explicit validation.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | Workspace dependency centralization |
| `module/unilang/Cargo.toml` | Feature flag and dependency configuration |

### Tests

| File | Relationship |
|------|--------------|
| `tests/build/dependency_standards.rs` | R1–R4 compliance: version format, centralization, optional deps, no-op feature gate |
