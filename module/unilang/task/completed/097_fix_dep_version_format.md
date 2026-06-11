# Fix Workspace Dependency Version Format

## Execution State

- **Executor Type:** ai
- **Actor:** null
- **Claimed At:** null
- **Reopen Count:** 0
- **State:** ✅ (Completed)
- **Priority:** 0
- **Closes:** null
- **Blocked Reason:** null
- **Dir:** .
- **Validated By:** w3 .test level::3
- **Validation Date:** 2026-04-19

## Goal

Rewrite every version string in the workspace-root `Cargo.toml` to comply with R1 of invariant `004_workspace_dependency_standards.md`: external deps → `^X.Y` (caret, major.minor only); internal path deps → `=X.Y.Z` (exact pin). Bare strings (`"1"`, `"0.4"`) and tilde strings (`"~0.39.0"`) are FORBIDDEN.

- **Motivated:** Invariant 004 R1 requires explicit caret semantics for external deps and exact pins for internal path deps; current bare and tilde strings leave resolution policy ambiguous and make dep audits unreliable.
- **Observable:** `grep -P 'version = "~|version = "[0-9]' Cargo.toml | grep -v '# pin:' | wc -l` → 0.
- **Scoped:** Single file edit — workspace-root `Cargo.toml` only; no source code changes.
- **Testable:** `cargo check --workspace --all-features` → exit 0 (all crates still compile after reformat).

## In Scope

- Workspace-root `Cargo.toml` `[workspace.dependencies]` section — all 45 version strings
- External deps without `path`: bare string → `^X.Y`; if already `X.Y` add only the caret prefix
- wTools ecosystem deps (`error_tools`, `mod_interface`, `former`, `strs_tools`, `iter_tools`, `macro_tools`, `component_model_types`, `cli_fmt`, `test_tools`): tilde+patch → `^X.Y` (drop patch field, replace `~` with `^`)
- Internal path deps (`unilang`, `unilang_parser`, `unilang_meta`, `cargo_unilang`): tilde → `=X.Y.Z` (exact pin, drop tilde)
- `bincode`: `"1"` → `"^1.0"` (intent to stay on 1.x is preserved by `^1.0 = >=1.0, <2.0`)

## Out of Scope

- Changing actual dep versions (only format changes, not version bumps)
- Source code changes
- Feature flag changes
- Any other Cargo.toml files (member crates remain untouched — they use `workspace = true`)

## Work Procedure

1. Open `Cargo.toml` at workspace root
2. **Internal path deps** — replace 4 tilde-pinned versions with exact pins:
   - `unilang = { version = "~0.53.0"` → `version = "=0.53.0"`
   - `unilang_parser = { version = "~0.35.0"` → `version = "=0.35.0"`
   - `unilang_meta = { version = "~0.1.0"` → `version = "=0.1.0"`
   - `cargo_unilang = { version = "~0.1.0"` → `version = "=0.1.0"`
3. **wTools ecosystem** — replace 9 tilde+patch versions with caret+minor:
   - `error_tools = { version = "~0.39.0"` → `version = "^0.39"`
   - `mod_interface = { version = "~0.61.0"` → `version = "^0.61"`
   - `former = { version = "~2.45.0"` → `version = "^2.45"`
   - `strs_tools = { version = "~0.45.0"` → `version = "^0.45"`
   - `iter_tools = { version = "~0.50.0"` → `version = "^0.50"`
   - `macro_tools = { version = "~0.85.0"` → `version = "^0.85"`
   - `component_model_types = { version = "~0.27.0"` → `version = "^0.27"`
   - `cli_fmt = { version = "~0.3.0"` → `version = "^0.3"`
   - `test_tools = { version = "~0.16.0"` → `version = "^0.16"`
4. **External bare-string deps** — add `^` prefix and minor if absent (32 entries):
   - Major-only (`"1"`, `"2"`, etc.) → add minor: `"1"` → `"^1.0"`, `"2"` → `"^2.0"`, etc.
   - Major.minor (`"0.4"`, `"0.9"`, etc.) → add only caret: `"0.4"` → `"^0.4"`, `"0.9"` → `"^0.9"`, etc.
   - Full list: `serde "1"→"^1.0"`, `url "2"→"^2.0"`, `chrono "0.4"→"^0.4"`, `regex "1"→"^1.0"`, `log "0.4"→"^0.4"`, `serde_json "1"→"^1.0"`, `serde_yaml "0.9"→"^0.9"`, `toml "1"→"^1.0"`, `ron "0.12"→"^0.12"`, `phf "0.13"→"^0.13"`, `walkdir "2"→"^2.0"`, `indexmap "2"→"^2.0"`, `lru "0.17"→"^0.17"`, `toml_edit "0.25"→"^0.25"`, `simd-json "0.17"→"^0.17"`, `memchr "2"→"^2.0"`, `bytecount "0.6"→"^0.6"`, `rustyline "18.0"→"^18.0"`, `prost "0.14"→"^0.14"`, `async-graphql "7"→"^7.0"`, `utoipa "5"→"^5.0"`, `libloading "0.9"→"^0.9"`, `bincode "1"→"^1.0"`, `phf_codegen "0.13"→"^0.13"`, `assert_cmd "2"→"^2.0"`, `predicates "3"→"^3.0"`, `assert_fs "1"→"^1.0"`, `clap "4"→"^4.0"`, `pico-args "0.5"→"^0.5"`, `tempfile "3"→"^3.0"`, `criterion "0.8"→"^0.8"`, `trybuild "1"→"^1.0"`
5. Run enforcement check: `grep -P 'version = "~|version = "[0-9]' Cargo.toml | grep -v '# pin:' | wc -l` → 0
6. Run compile check: `cargo check --workspace --all-features` → exit 0

## Test Matrix

| Scenario | Expected |
|----------|----------|
| Enforcement grep after edits | 0 matches |
| `cargo check --workspace --all-features` | exit 0 |
| `cargo check -p unilang --no-default-features` | exit 0 |
| Internal path dep version `unilang` | `=0.53.0` (exact) |
| wTools dep version `error_tools` | `^0.39` (no patch) |
| External dep version `serde` | `^1.0` (caret + minor) |
| External dep version `bincode` | `^1.0` (stays 1.x range) |

## Acceptance Criteria

1. `grep -P 'version = "~|version = "[0-9]' Cargo.toml | grep -v '# pin:' | wc -l` → 0
2. `grep 'version = "=0\.53\.0"' Cargo.toml | wc -l` → 1 (internal exact pin)
3. `grep 'version = "\^0\.39"' Cargo.toml | wc -l` → 1 (wTools caret+minor)
4. `grep 'version = "\^1\.0"' Cargo.toml | wc -l` → ≥ 3 (multiple major-only deps reformatted)
5. `cargo check --workspace --all-features` → exit 0

## Validation

### Checklist

- [ ] C1 — All internal path deps use `=X.Y.Z` exact pin format
- [ ] C2 — All wTools ecosystem deps use `^X.Y` format (no patch, no tilde)
- [ ] C3 — All external bare-string deps use `^X.Y` format with minor component
- [ ] C4 — Enforcement grep returns 0 (no remaining bare or tilde strings)
- [ ] C5 — `cargo check --workspace --all-features` passes

### Measurements

- [ ] M1 — `grep -P 'version = "~|version = "[0-9]' Cargo.toml | grep -v '# pin:' | wc -l` → 0 (was: 45)
- [ ] M2 — `grep -c '= "\^' Cargo.toml` → ≥ 41 (all external deps with caret prefix)
- [ ] M3 — `grep -c '= "=[0-9]' Cargo.toml` → 4 (all internal path deps with exact pin)

### Anti-faking Checks

- [ ] AF1 — `grep 'version = "~' Cargo.toml | wc -l` → 0 (no tildes remain)
- [ ] AF2 — `grep -P 'version = "[0-9]' Cargo.toml | wc -l` → 0 (no bare digits remain)
- [ ] AF3 — `grep 'version = "=0\.53\.0"' Cargo.toml` matches `unilang` line exactly

## Requirements

Apply all rulebooks discovered via `kbase .role name::dev`. Key references: `crate_distribution.rulebook.md § R1 Version Format`; `docs/invariant/004_workspace_dependency_standards.md`.

## Outcomes

- All workspace `Cargo.toml` version strings migrated to `^X.Y` / `=X.Y.Z` format per invariant 004 R1
- Internal path deps use exact `=X.Y.Z` pin format; wTools ecosystem deps use `^X.Y` caret-minor format; external deps use `^X.Y` caret-minor format
- Enforcement grep (`grep -P 'version = "~|version = "[0-9]'`) returns 0 — no bare or tilde version strings remain
- `cargo check --workspace --all-features` exits 0

## History

- **2026-04-19** `COMPLETED` — Validated by w3 .test level::3. Fix Workspace Dependency Version Format.
