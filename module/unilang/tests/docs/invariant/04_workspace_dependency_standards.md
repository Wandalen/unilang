# Invariant Spec: Workspace Dependency Standards

### Scope

- **Purpose:** Verify that the four dependency format rules R1–R4 defined in `docs/invariant/004_workspace_dependency_standards.md` hold in the workspace Cargo files
- **Responsibility:** Test cases confirming version format correctness, workspace centralization, optional-flag coverage, and no-default-features build
- **In Scope:** R1 (`^X.Y` for external deps, `=X.Y.Z` for internal wTools deps), R2 (all versions declared in workspace `Cargo.toml`), R3 (all library deps marked `optional = true`), R4 (`--no-default-features` produces no-op build)
- **Out of Scope:** Runtime behavioral features; NFR performance thresholds

### IN-1: External dependencies use `^X.Y` caret version format

- **Given:** The workspace `Cargo.toml` at the repository root
- **When:** All `[workspace.dependencies]` version strings are inspected
- **Then:** Every external dependency (non-wTools) uses the form `^X.Y` or `^X.Y.Z` with a leading caret; no external dependency uses `=` pinning or bare version numbers

### IN-2: Internal wTools dependencies use `=X.Y.Z` exact version format

- **Given:** The workspace `Cargo.toml`
- **When:** The version strings for `error_tools`, `mod_interface`, `former`, and `macro_tools` are inspected
- **Then:** Each uses the form `=X.Y.Z` (exact pin with leading `=`); no caret or tilde is used

### IN-3: Individual crate Cargo.toml files contain no standalone version literals

- **Given:** The `Cargo.toml` files for `unilang`, `unilang_parser`, `unilang_meta`, and `cargo_unilang`
- **When:** Each file's `[dependencies]` section is inspected for version strings
- **Then:** No version literal (`"X.Y.Z"`, `"^X.Y"`, `"=X.Y.Z"`) appears in individual crate files; all dependencies reference workspace via `{ workspace = true }`

### IN-4: `--no-default-features` build compiles without errors or warnings

- **Given:** The `unilang` crate
- **When:** `RUSTFLAGS="-D warnings" cargo check --no-default-features` is run
- **Then:** Exits with code 0; zero errors and zero warnings; the `enabled` feature gate correctly disables all optional functionality
