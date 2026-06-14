# Invariant Spec: Build-Runtime Separation

### Scope

- **Purpose:** Verify the compile-time-only processing invariant defined in `docs/invariant/006_build_runtime_separation.md`
- **Responsibility:** Test cases confirming YAML/JSON parsing occurs only at build time; runtime binary has no parsing dependency
- **In Scope:** Build-time codegen produces valid static data, runtime code never imports parsing crates, validation_core include!() pattern works in both contexts
- **Out of Scope:** PHF implementation details (see `architecture/`); specific YAML schema rules

### IN-1: Runtime binary does not link serde_yaml_ng

- **Given:** A built `unilang` library crate with static command definitions loaded from YAML
- **When:** The dependency tree of the compiled output is inspected (e.g., `cargo tree --edges=normal` filtered for runtime deps)
- **Then:** `serde_yaml_ng` does not appear as a runtime dependency (only as a build dependency)

### IN-2: Generated static data is accessible at runtime without parsing

- **Given:** A YAML command definition processed by `build/main.rs` during `cargo build`
- **When:** Runtime code accesses the static registry via `include!()` in `src/static_data/`
- **Then:** Command definitions are available as compile-time constants; no parsing function is called at runtime

### IN-3: validation_core shared logic produces identical results in both contexts

- **Given:** A command definition with a known validation error (e.g., empty command name)
- **When:** The validation is run via `include!()` in build.rs AND via `include!()` in src/validation_core.rs
- **Then:** Both produce the same error result; the shared source guarantees behavioral identity without runtime linking

### IN-4: Runtime binary does not link serde_json

- **Given:** A built `unilang` library crate with the `enabled` feature active
- **When:** The dependency tree is inspected (e.g., `cargo tree --edges=normal` filtered for runtime deps)
- **Then:** `serde_json` does not appear as a runtime dependency; JSON parsing is confined to build-time codegen only
