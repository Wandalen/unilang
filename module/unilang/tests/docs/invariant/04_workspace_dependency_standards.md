# Invariant Spec: Workspace Dependency Standards

### Scope

- **Purpose:** Verify that the four dependency format rules R1–R4 defined in `docs/invariant/004_workspace_dependency_standards.md` hold in the workspace Cargo files
- **Responsibility:** Test cases confirming version format correctness, workspace centralization, optional-flag coverage, and no-default-features build
- **In Scope:** R1 (`^X.Y` for external deps without a `path` field, `=X.Y.Z` for workspace-internal path deps), R2 (all versions declared in workspace `Cargo.toml`), R3 (all library deps marked `optional = true`), R4 (`--no-default-features` produces no-op build)
- **Out of Scope:** Runtime behavioral features; NFR performance thresholds

### IN-1: External dependencies use `^X.Y` caret version format

- **Given:** The workspace `Cargo.toml` at the repository root
- **When:** All `[workspace.dependencies]` version strings are inspected
- **Then:** Every external dependency (non-wTools) uses the form `^X.Y` or `^X.Y.Z` with a leading caret; no external dependency uses `=` pinning or bare version numbers

### IN-2: Workspace-internal path deps use `=X.Y.Z` exact version format

- **Given:** The workspace `Cargo.toml`
- **When:** The version strings for `unilang`, `unilang_parser`, `unilang_meta`, and `cargo_unilang` (the entries that include a `path = "module/..."` field) are inspected
- **Then:** Each uses the form `=X.Y.Z` (exact pin with leading `=`); no caret or tilde is used; wTools crates (`error_tools`, `mod_interface`, `former`, `macro_tools`, `strs_tools`) are NOT path deps and correctly use `^X.Y`

### IN-3: Individual crate Cargo.toml files contain no standalone version literals

- **Given:** The `Cargo.toml` files for `unilang`, `unilang_parser`, `unilang_meta`, and `cargo_unilang`
- **When:** Each file's `[dependencies]` section is inspected for version strings
- **Then:** No version literal (`"X.Y.Z"`, `"^X.Y"`, `"=X.Y.Z"`) appears in individual crate files; all dependencies reference workspace via `{ workspace = true }`

### IN-4: `--no-default-features` build compiles without errors or warnings

- **Given:** The `unilang` crate
- **When:** `RUSTFLAGS="-D warnings" cargo check --no-default-features` is run
- **Then:** Exits with code 0; zero errors and zero warnings; the `enabled` feature gate correctly disables all optional functionality

### IN-5: All library crate dependencies marked optional

- **Given:** The `Cargo.toml` files for library crates `unilang` and `unilang_parser`
- **When:** Each file's `[dependencies]` section is inspected for `optional` flags
- **Then:** Every dependency entry includes `optional = true`; no non-optional dependency exists in library crates (binary crate `cargo_unilang` is exempt from this rule)

### IN-6: `--no-default-features` build compiles zero external dependencies

- **Given:** The `unilang` crate built with `cargo build -p unilang --no-default-features`
- **When:** The build output (or `cargo tree --edges=normal` for the no-default-features configuration) is inspected for compiled/linked external crates
- **Then:** Zero external dependency crates are compiled or linked; only the `unilang` crate itself is present, confirming the `enabled` feature gate fully isolates all optional functionality rather than merely suppressing warnings

### IN-7: Workspace manifest declares no `features` lists

- **Given:** The workspace `Cargo.toml` `[workspace.dependencies]` section
- **When:** Each dependency entry is inspected for a `features = [...]` list
- **Then:** No entry in `[workspace.dependencies]` declares a `features = [...]` list; feature selection lives exclusively in member crate `Cargo.toml` files (`default-features = false` on path deps is permitted since it is not a `features` list)

### IN-8: The `enabled` feature activates dependencies via `dep:name` syntax

- **Given:** The `unilang` crate `Cargo.toml` `[features]` section
- **When:** The `enabled` feature's activation list is inspected
- **Then:** Every dependency activated by `enabled` uses the `dep:name` syntax (e.g., `dep:serde`, `dep:regex`); no bare crate name (which would implicitly enable a same-named feature rather than gating an optional dependency) appears in the list
