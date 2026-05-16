# Implement Optional Dep Pattern Across Library Crates

## Execution State

- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** claude-sonnet-4-6
- **Claimed At:** 2026-04-19
- **Priority:** 0
- **Validated By:** w3 .test level::3
- **Validation Date:** 2026-04-19

## Goal

Add `optional = true` to every dependency in `unilang`, `unilang_parser`, and `unilang_meta`, and wire the `enabled` feature to activate all deps via `dep:name` syntax. After this task, `cargo build -p <lib_crate> --no-default-features` produces zero external-dep compilations — the crates compile to empty libraries.

**Motivated:** Invariant 004 R3/R4 mandate optional deps and no-op-when-disabled pattern; currently `cargo build -p unilang --no-default-features` compiles serde, url, chrono, regex, error_tools, mod_interface, former, and 8 other heavy crates even though nothing is enabled, defeating the `enabled`/`full` feature isolation architecture.
**Observable:** `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling" | grep -v " unilang " | wc -l` → 0; same pattern for `unilang_parser` and `unilang_meta`.
**Scoped:** Six files across three library crates — `Cargo.toml` and `src/lib.rs` for each of `unilang_parser`, `unilang`, and `unilang_meta`.
**Testable:** `w3 .test level::3` from workspace root → 0 failures, 0 warnings after all changes.

## In Scope

### `unilang_parser`
- `Cargo.toml`: add `optional = true` to `strs_tools`, `error_tools`, `iter_tools`
- `Cargo.toml`: change `enabled = []` to `enabled = ["dep:strs_tools", "dep:error_tools", "dep:iter_tools"]`
- `src/lib.rs`: gate all public module declarations with `#[cfg(feature = "enabled")]`
- `src/lib.rs`: add `#![cfg_attr(not(feature = "enabled"), allow(unused))]` if needed

### `unilang`
- `Cargo.toml`: add `optional = true` to `serde`, `url`, `chrono`, `regex`, `error_tools`, `mod_interface`, `former`, `unilang_parser`, `log`, `indexmap`, `lru` (11 deps)
- `Cargo.toml`: change `enabled = []` to `enabled = ["dep:serde", "dep:url", "dep:chrono", "dep:regex", "dep:error_tools", "dep:mod_interface", "dep:former", "dep:unilang_parser", "dep:log", "dep:indexmap", "dep:lru"]`
- `Cargo.toml`: add `"enabled"` prerequisite to `simd` feature so `simd-json`, `memchr`, `bytecount` only activate when core deps are present (e.g., `simd = ["enabled", "simd-json", "memchr", "bytecount", "unilang_parser/simd"]`)
- `src/lib.rs`: gate `mod_interface::mod_interface!{...}` block with `#[cfg(feature = "enabled")]`
- `src/lib.rs`: gate `pub use unilang_parser as parser;` and `pub use unilang_parser::{ShellArgv, ReplInput};` with `#[cfg(feature = "enabled")]`
- Build deps `serde` and `serde_yaml` in `[build-dependencies]` may remain non-optional (build-script exception — build.rs only runs when building the crate itself, not as a transitive dep)

### `unilang_meta`
- `Cargo.toml`: add `optional = true` to `macro_tools`, `iter_tools`, `component_model_types`, `unilang`
- `Cargo.toml`: change `enabled = ["macro_tools/enabled", "iter_tools/enabled", "component_model_types/enabled"]` to `enabled = ["dep:macro_tools", "dep:iter_tools", "dep:component_model_types", "dep:unilang", "macro_tools/enabled", "iter_tools/enabled", "component_model_types/enabled"]`
- `src/lib.rs`: gate `use macro_tools::prelude::*;` and all implementation content with `#[cfg(feature = "enabled")]`

## Out of Scope

- Version format fixes (covered by task 097)
- `cargo_unilang` (binary crate — exempt from R3/R4 per rulebook binary exception)
- Changing public API behavior or module structure
- Changing which features are enabled by default

## Work Procedure

### Phase A — `unilang_parser` (lowest dependency, fix first)

1. Edit `module/unilang_parser/Cargo.toml`:
   - Add `optional = true` to `strs_tools`, `error_tools`, `iter_tools`
   - Change `enabled = []` → `enabled = ["dep:strs_tools", "dep:error_tools", "dep:iter_tools"]`
2. Edit `module/unilang_parser/src/lib.rs`:
   - Add `#![cfg_attr(not(feature = "enabled"), allow(unused_imports))]` at crate root if needed
   - Wrap all `pub mod` declarations with `#[cfg(feature = "enabled")]`
3. Verify: `cargo build -p unilang_parser --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_parser" | wc -l` → 0
4. Verify default still works: `cargo build -p unilang_parser 2>&1 | tail -3`

### Phase B — `unilang_meta` (proc-macro crate, fix second)

5. Edit `module/unilang_meta/Cargo.toml`:
   - Add `optional = true` to `macro_tools`, `iter_tools`, `component_model_types`, `unilang`
   - Update `enabled` to include `dep:macro_tools`, `dep:iter_tools`, `dep:component_model_types`, `dep:unilang`
6. Edit `module/unilang_meta/src/lib.rs`:
   - Gate `use macro_tools::prelude::*;` and all struct/impl/fn items with `#[cfg(feature = "enabled")]`
7. Verify: `cargo build -p unilang_meta --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_meta" | wc -l` → 0

### Phase C — `unilang` (main crate, fix last — depends on parser)

8. Edit `module/unilang/Cargo.toml`:
   - Add `optional = true` to `serde`, `url`, `chrono`, `regex`, `error_tools`, `mod_interface`, `former`, `unilang_parser`, `log`, `indexmap`, `lru`
   - Update `enabled = []` with all 11 `dep:` activations
   - Update `simd` feature: add `"enabled"` as prerequisite
9. Edit `module/unilang/src/lib.rs`:
   - Gate `mod_interface::mod_interface!{...}` block with `#[cfg(feature = "enabled")]`
   - Gate `pub use unilang_parser as parser;` with `#[cfg(feature = "enabled")]`
   - Gate `pub use unilang_parser::{ShellArgv, ReplInput};` with `#[cfg(feature = "enabled")]`
10. Verify: `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling" | grep -v " unilang " | wc -l` → 0
11. Verify full build: `w3 .test level::3` → 0 failures, 0 warnings

## Test Matrix

| Scenario | Expected |
|----------|----------|
| `cargo build -p unilang_parser --no-default-features` — external Compiling lines | 0 |
| `cargo build -p unilang_meta --no-default-features` — external Compiling lines | 0 |
| `cargo build -p unilang --no-default-features` — external Compiling lines | 0 |
| `cargo build -p unilang --features enabled` | Builds successfully |
| `cargo build -p unilang --features full` | Builds successfully |
| `cargo build -p cargo_unilang` (binary, full features) | Builds successfully |
| `w3 .test level::3` | 0 failures, 0 warnings |
| `RUSTFLAGS="-D warnings" cargo check -p unilang --no-default-features` | exit 0 |

## Acceptance Criteria

1. `cargo build -p unilang_parser --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_parser" | wc -l` → 0
2. `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling" | grep -v " unilang " | wc -l` → 0
3. `cargo build -p unilang_meta --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_meta" | wc -l` → 0
4. `grep 'optional = true' module/unilang_parser/Cargo.toml | wc -l` → 3
5. `grep 'optional = true' module/unilang/Cargo.toml | wc -l` → ≥ 11 (excluding optional deps already marked)
6. `grep 'optional = true' module/unilang_meta/Cargo.toml | wc -l` → 4
7. `grep 'dep:strs_tools\|dep:error_tools\|dep:iter_tools' module/unilang_parser/Cargo.toml | wc -l` → 3
8. `grep 'dep:serde\b' module/unilang/Cargo.toml | wc -l` → 1
9. `w3 .test level::3` → 0 failures, 0 warnings
10. `cargo build -p cargo_unilang` → exit 0 (binary unaffected)

## Validation

### Checklist

- [ ] C1 — `unilang_parser` no-op check: zero external compilations under `--no-default-features`
- [ ] C2 — `unilang` no-op check: zero external compilations under `--no-default-features`
- [ ] C3 — `unilang_meta` no-op check: zero external compilations under `--no-default-features`
- [ ] C4 — All 3 library crates: `enabled` feature uses `dep:name` syntax for all deps
- [ ] C5 — `unilang` `simd` feature has `"enabled"` as prerequisite (or is itself optional-only)
- [ ] C6 — `cargo_unilang` binary still builds without changes
- [ ] C7 — `w3 .test level::3` passes with 0 failures and 0 warnings
- [ ] C8 — No `cfg_attr` workarounds hiding real compile errors (anti-faking)

### Measurements

- [ ] M1 — `cargo build -p unilang_parser --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_parser" | wc -l` → 0 (was: 3+)
- [ ] M2 — `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling" | grep -v " unilang " | wc -l` → 0 (was: 11+)
- [ ] M3 — `cargo build -p unilang_meta --no-default-features 2>&1 | grep "Compiling" | grep -v "unilang_meta" | wc -l` → 0 (was: 4+)
- [ ] M4 — `grep -c 'optional = true' module/unilang/Cargo.toml` → ≥ 11 (was: present on optional-only deps, missing on 11 core deps)

### Invariants

- [ ] I1 — `RUSTFLAGS="-D warnings" cargo check --workspace --all-features` → exit 0
- [ ] I2 — `RUSTFLAGS="-D warnings" cargo check -p unilang --no-default-features` → exit 0

### Anti-faking Checks

- [ ] AF1 — No `#[allow(unused_imports)]` on non-cfg-gated import (all imports that compile unconditionally must not need unused suppression)
- [ ] AF2 — Verify `enabled` feature does not activate deps through sub-feature tricks (must use `dep:name` directly)
- [ ] AF3 — `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling serde"` → empty (serde not compiled)
- [ ] AF4 — `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling regex"` → empty (regex not compiled)
- [ ] AF5 — `cargo build -p unilang --no-default-features 2>&1 | grep "Compiling url "` → empty (url not compiled)

## Requirements

Apply all rulebooks discovered via `kbase .role name::dev`. Key references: `crate_distribution.rulebook.md § R3 Optional Dependencies`, `§ R4 No-Op When Disabled`, `§ A2 enabled Feature Anti-Pattern`; `docs/invariant/004_workspace_dependency_standards.md`; `docs/architecture/001_mandates.md § enabled feature gate mandate`.
