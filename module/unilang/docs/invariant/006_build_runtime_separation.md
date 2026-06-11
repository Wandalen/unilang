# Invariant: Build-Runtime Separation

### Scope

- **Purpose:** Guarantee that all YAML/JSON command definition processing occurs at compile time only
- **Responsibility:** Documents the build-time/runtime separation boundary, the validation_core dual-use module, and violation consequences
- **In Scope:** Compile-time-only parsing invariant, OUT_DIR codegen boundary, validation_core include!() pattern
- **Out of Scope:** PHF implementation details (see architecture/004), specific build pipeline steps (see build/ source)

### Invariant Statement

For all command definitions loaded via build-time approaches (YAML/JSON single/multi build): all parsing, validation, and code generation occurs during `cargo build` via `build/main.rs`. No YAML or JSON parsing crate is linked into the runtime binary. The runtime binary accesses only the generated static data in `OUT_DIR`.

### Enforcement Mechanism

The build script (`build/main.rs`) orchestrates: discovery (`build/discovery.rs`) finds YAML/JSON files, parsing uses `serde_yaml`/`serde_json` (build-only deps), validation runs via `include!("../../src/validation_core.rs")` (shared source, not shared linkage), and codegen (`build/codegen.rs`) writes Rust source to `OUT_DIR`. The generated file is `include!()`d by `src/static_data.rs` at compile time. Runtime code never imports `serde_yaml` or `serde_json`.

The `validation_core` module uses `include!()` in both `build/main.rs` and `src/validation_core.rs` to share validation logic without creating a runtime dependency on the build pipeline.

### Violation Consequences

If YAML/JSON parsing crates leak into runtime: binary size increases (~500KB+ for serde_yaml), startup latency increases, and the zero-overhead static registry guarantee (NFR-PERF-1) is compromised. Users who only need build-time command definitions would pay for unused runtime parsing capability.

### Architectures

| File | Relationship |
|------|--------------|
| [004_implementation_details.md](../architecture/004_implementation_details.md) | PHF codegen that implements the build-time side |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | Static registry that consumes build-time output |

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](002_non_functional_requirements.md) | NFR-PERF-1 zero-overhead guarantee depends on this separation |

### Sources

| File | Relationship |
|------|--------------|
| `build/main.rs` | Build-time orchestration |
| `src/static_data.rs` | Runtime include!() of generated code |
| `src/validation_core.rs` | Shared validation logic (include!() pattern) |

### Tests

| File | Relationship |
|------|--------------|
| `tests/build/` | Build pipeline validation |
