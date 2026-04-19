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

```bash
# R1: Zero bare or tilde version strings in workspace manifest
# Note: pattern scoped to "version = " to avoid false positives from non-dep fields (e.g. resolver = "2")
grep -Pn 'version\s*=\s*"(~|[0-9])' Cargo.toml | grep -v '# pin:' | wc -l  # → 0

# R3: No non-optional normal deps in library crates (build-deps and dev-deps are exempt)
# Check each crate's [dependencies] section manually or use:
grep 'optional = true' module/unilang_parser/Cargo.toml | wc -l  # → 3 (all deps optional)
grep 'optional = true' module/unilang_meta/Cargo.toml | wc -l    # → 4 (all deps optional)
grep 'optional = true' module/unilang/Cargo.toml | wc -l         # → ≥ 11 (all normal deps optional)

# R4: No-op verification (zero external crate compilations under --no-default-features)
cargo build -p unilang_parser --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_parser" | wc -l  # → 0
cargo build -p unilang_meta --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_meta" | wc -l      # → 0
cargo build -p unilang --no-default-features 2>&1 | grep "Compiling" | grep -v " unilang " | wc -l              # → 0
```

### Violation Consequences

**R1 violation:** Hidden resolution policy makes dependency audits ambiguous; bare `"1"` does not communicate whether compatible-update semantics are intended; tilde on external deps over-constrains the resolver to patch-level range (wrong for external deps).

**R3/R4 violation:** `cargo build -p unilang --no-default-features` still compiles serde, url, chrono, regex, and 7 other heavy crates even though nothing is enabled. This defeats the `enabled`/`full` feature isolation architecture mandated in [architecture/001_mandates.md](../architecture/001_mandates.md) and described as a RIGID AND NON-NEGOTIABLE RULE in `crate_distribution.rulebook.md § Cargo Features Management : Mandatory Enabled and Full Features`.

### Known Violations

No active violations — all previously known violations resolved.

### Resolved Violations (resolved 2026-04-19)

**V1–V3 — Version Format — `Cargo.toml` (workspace root)** — ✅ Fixed by task [097](../../task/completed/097_fix_dep_version_format.md)

43 deps had bare strings or tilde prefix; all rewritten to `^X.Y` (external) or `=X.Y.Z` (internal path deps).

**V4 — Non-Optional Core Deps — 3 library crates** — ✅ Fixed by task [098](../../task/completed/098_implement_optional_dep_pattern.md)

18 deps across `unilang`, `unilang_parser`, `unilang_meta` were non-optional; `enabled` feature activated nothing. All deps are now `optional = true` and `enabled` activates them via `dep:name` syntax. Crate-level code gated under `#[cfg(feature = "enabled")]`.

**Verification (post-fix):**
```bash
# R1: Zero bare/tilde versions
grep -Pn 'version\s*=\s*"(~|[0-9])' Cargo.toml  # → 0 matches

# R4: No runtime deps under --no-default-features
cargo tree -p unilang --no-default-features  # → [build-dependencies] only; zero runtime deps
cargo build -p unilang_parser --no-default-features  # → 0 external compilations
cargo build -p unilang_meta --no-default-features    # → 0 external compilations
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [architecture/001_mandates.md](../architecture/001_mandates.md) | `enabled` feature gate mandate (source of R4) |
| doc | [invariant/002_non_functional_requirements.md](002_non_functional_requirements.md) | Performance NFRs enabled by no-op compile pattern |
| doc | [invariant/003_governing_principles.md](003_governing_principles.md) | Explicit Dependencies principle (source of R2) |
| rulebook | `crate_distribution.rulebook.md` | Complete dependency rules R1–R8, anti-patterns A1–A4 |
