# Invariant: Non-Functional Requirements

### Scope

- **Purpose:** Define measurable system properties that must always hold for correctness and usability
- **Responsibility:** NFR-PERF-*, NFR-SEC-*, NFR-ROBUST-*, NFR-PLATFORM-*, NFR-MODULARITY-*
- **In Scope:** Performance thresholds, security properties, robustness requirements, modularity constraints
- **Out of Scope:** Functional requirements, implementation approaches, architectural decisions

### Invariant Statement

All performance thresholds, security properties, and modularity constraints defined in this document MUST be satisfied by every released version of the framework. Regression against any measurable NFR is a blocking issue.

### Enforcement Mechanism

- CI benchmark suite verifies NFR-PERF-* thresholds on every PR
- Static analysis and test suite verify NFR-SEC-*, NFR-ROBUST-* constraints
- Cargo feature structure enforces NFR-MODULARITY-*

### Violation Consequences

NFR violation = performance regression, security hole, or modularity collapse. All are blocking release criteria.

### Non-Functional Requirements

#### NFR-PERF-1 (Startup Time)

For a utility with 1,000,000+ statically compiled commands, the framework **must** introduce zero runtime overhead for command registration. Application startup time **must not** be proportional to the number of static commands. This **must** be achieved via compile-time generation of optimized static lookup tables (using Perfect Hash Functions).

#### NFR-PERF-2 (Lookup Latency)

The p99 latency for resolving a command `FullName` and its arguments **must** be less than 100 nanoseconds for any registry size.

#### NFR-PERF-3 (Throughput)

The framework **must** be capable of processing over 5,000,000 simple command lookups per second on a standard developer machine.

#### NFR-SEC-1 (Sensitive Data)

Argument values marked as `sensitive: true` **must not** be displayed in logs or user interfaces unless explicitly required by a secure context.

#### NFR-ROBUST-1 (Error Reporting)

All user-facing errors **must** be returned as a structured `ErrorData` object and provide clear, actionable messages. Internal panics **must** be caught and converted to a user-friendly `UNILANG_INTERNAL_ERROR`.

#### NFR-PLATFORM-1 (WASM Compatibility)

The core logic of the `unilang` and `unilang_parser` crates **must** be platform-agnostic and fully compatible with the WebAssembly (`wasm32-unknown-unknown`) target. This implies that the core crates **must not** depend on libraries or functionalities that are tied to a specific native OS (e.g., native threading, direct file system access that cannot be abstracted) unless those features are conditionally compiled and disabled for the WASM target.

#### NFR-MODULARITY-1 (Granular Features)

All non-essential framework functionality **must** be gated behind Cargo features. This includes support for complex types (`Url`, `DateTime`), declarative loading (`serde_yaml_ng`, `serde_json`), and other features that introduce dependencies.

#### NFR-MODULARITY-2 (Lightweight Core)

When compiled with `default-features = false`, the `unilang` framework **must** have a minimal dependency footprint, comparable in lightness (dependencies, compile time) to the `pico-args` crate. The core functionality **must** be contained within the `enabled` feature.

### Cross-Cutting Concerns

#### Error Handling

All recoverable errors **must** be propagated as `unilang::Error`, which wraps an `ErrorData` struct containing a machine-readable `code` (typed `ErrorCode` enum) and a human-readable `message`. The framework defines the following standard error codes via the `ErrorCode` enum:

| ErrorCode Variant | String Representation | Meaning |
| :--- | :--- | :--- |
| `ErrorCode::CommandNotFound` | `UNILANG_COMMAND_NOT_FOUND` | Command does not exist in registry |
| `ErrorCode::ArgumentMissing` | `UNILANG_ARGUMENT_MISSING` | Required argument not provided |
| `ErrorCode::ArgumentTypeMismatch` | `UNILANG_ARGUMENT_TYPE_MISMATCH` | Argument value has wrong type |
| `ErrorCode::TooManyArguments` | `UNILANG_TOO_MANY_ARGUMENTS` | Excess positional arguments provided |
| `ErrorCode::UnknownParameter` | `UNILANG_UNKNOWN_PARAMETER` | Named parameter not defined in command (with typo suggestions) |
| `ErrorCode::ValidationRuleFailed` | `UNILANG_VALIDATION_RULE_FAILED` | Argument validation rule violated |
| `ErrorCode::ArgumentInteractiveRequired` | `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` | Interactive argument requires user input |
| `ErrorCode::CommandAlreadyExists` | `UNILANG_COMMAND_ALREADY_EXISTS` | Duplicate command registration attempt |
| `ErrorCode::CommandNotImplemented` | `UNILANG_COMMAND_NOT_IMPLEMENTED` | Command registered but not implemented |
| `ErrorCode::TypeMismatch` | `UNILANG_TYPE_MISMATCH` | Type conversion or mismatch error |
| `ErrorCode::HelpRequested` | `HELP_REQUESTED` | User requested help via `?` operator or `??` parameter |
| `ErrorCode::InternalError` | `UNILANG_INTERNAL_ERROR` | Unexpected system error |

The `ErrorCode` enum provides compile-time type safety and prevents typos in error code strings. The `ErrorData::new()` method requires an `ErrorCode` enum variant instead of a string.

#### Security

The framework **must** provide a `permissions` field in `CommandDefinition` for integrators to implement role-based access control. The `sensitive` attribute on arguments **must** be respected.

#### Verbosity

The framework **must** support at least three verbosity levels (`quiet`, `normal`, `debug`) configurable via environment variable (`UNILANG_VERBOSITY`) or programmatically.

#### Shell Integration

CLI applications **should** use the argv-based API when receiving command-line arguments from the shell (see FR-PIPE-4). This API preserves argument boundaries from the OS and eliminates information loss, enabling natural shell syntax without special quoting requirements. The string-based API is recommended for REPL/interactive applications where input comes as a single string.

#### Feature Flags and Modularity

The framework **must** be highly modular, allowing integrators to select only the features they need to minimize binary size and compile times.

#### The `enabled` Feature

Every crate in the `unilang` ecosystem (`unilang`, `unilang_parser`, `unilang_meta`) **must** expose an `enabled` feature. This feature **must** be part of the `default` feature set. Disabling the `enabled` feature (`--no-default-features`) **must** effectively remove all of the crate's code and dependencies from the compilation, allowing it to be "turned off" even when included as a non-optional dependency in a workspace.

#### Opinionated Defaults Strategy

The framework implements an **opinionated defaults strategy** where only **Approach #2** (Multi-YAML Build-Time Static) is enabled by default. This design choice:

1. **Guides users to the recommended approach** with best performance and developer experience
2. **Minimizes binary size** by excluding unused parsers and dependencies
3. **Forces conscious opt-in** for alternative approaches, ensuring developers understand trade-offs
4. **Reduces compilation time** by not building unused infrastructure

To use any approach other than #2, integrators **must** explicitly enable the corresponding feature flag.

#### Feature Architecture

The framework uses a two-tier feature architecture:

**Tier 1: Approach Features (User-Facing)**

Each CLI definition approach has its own feature flag that automatically enables required infrastructure:

| Approach Feature | Enables | Default | Purpose |
| :--- | :--- | :--- | :--- |
| `approach_yaml_single_build` | `static_registry`, `yaml_parser` | No | Single YAML → Build-time static |
| `approach_yaml_multi_build` | `static_registry`, `yaml_parser`, `multi_file` | **Yes** | Multi-YAML → Build-time static (**DEFAULT**) |
| `approach_yaml_runtime` | `yaml_parser` | No | YAML → Runtime registry |
| `approach_json_single_build` | `static_registry`, `json_parser` | No | Single JSON → Build-time static |
| `approach_json_multi_build` | `static_registry`, `json_parser`, `multi_file` | No | Multi-JSON → Build-time static |
| `approach_json_runtime` | `json_parser` | No | JSON → Runtime registry |
| *(Approach #7)* | *(always available)* | Yes | Rust DSL builder (core API) |
| `approach_rust_dsl_const` | `static_registry` | No | Rust DSL → Build-time const |
| `approach_hybrid` | `static_registry` | No | Mixed static + dynamic registry |

**Tier 2: Infrastructure Features (Building Blocks)**

These are enabled automatically by approach features and should not be used directly:

| Infrastructure Feature | Dependencies | Purpose |
| :--- | :--- | :--- |
| `static_registry` | `phf` (Perfect Hash Functions) | Zero-overhead static command lookup |
| `yaml_parser` | `serde_yaml_ng` | YAML deserialization |
| `json_parser` | `serde_json` | JSON deserialization |
| `multi_file` | `walkdir` | Auto-discovery of command files |
| `simd` | `simd-json`, `bytecount` | SIMD-optimized parsing (4-25x faster) |
| `repl` | - | Basic REPL functionality |
| `enhanced_repl` | `rustyline` | Advanced REPL with history/completion |
| `on_unknown_suggest` | `textdistance` | Fuzzy command suggestions |

**Core Features:**

| Feature | Purpose | Default |
| :--- | :--- | :--- |
| `enabled` | Master switch — disables entire crate when off | Yes |
| `default` | Default features: `enabled`, `simd`, `repl`, `enhanced_repl`, `approach_yaml_multi_build` | Yes |
| `full` | All features except dev-only | No |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](../feature/001_command_registry.md) | FR-REG-9 build-time validation satisfies NFR-PERF-1 |
| [005_repl_interactive.md](../feature/005_repl_interactive.md) | REPL uses shell integration guidance from this invariant |

### Invariants

| File | Relationship |
|------|--------------|
| [003_governing_principles.md](003_governing_principles.md) | Principles that these NFRs embody |
| [004_workspace_dependency_standards.md](004_workspace_dependency_standards.md) | Dependency standards enable no-op compile pattern |
| [006_build_runtime_separation.md](006_build_runtime_separation.md) | NFR-PERF-1 depends on build-runtime separation boundary |

### Architectures

| File | Relationship |
|------|--------------|
| [001_mandates.md](../architecture/001_mandates.md) | Architectural mandates that enforce NFRs |
| [002_benchmark_separation.md](../architecture/002_benchmark_separation.md) | Benchmark isolation satisfying NFR-MODULARITY-1 |
| [004_implementation_details.md](../architecture/004_implementation_details.md) | PHF implementation enabling NFR-PERF-1 |

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | ErrorCode type definition |
| [002_error_codes.md](../api/002_error_codes.md) | Error codes stable API contract |

### Sources

| File | Relationship |
|------|--------------|
| `build/codegen.rs` | PHF codegen enabling NFR-PERF-1 |
| `src/simd_tokenizer.rs` | SIMD tokenizer for NFR-PERF-2 |
| `src/interner.rs` | String interning for NFR-PERF-3 |

### Tests

| File | Relationship |
|------|--------------|
| `tests/system/nfr_performance.rs` | IN-1 startup zero-cost, IN-2 throughput ≥5M/sec |
| `tests/system/nfr_sensitive_data.rs` | IN-3 sensitive value absent from error output (coercion + validation paths) |
| `tests/system/nfr_robustness.rs` | IN-4 handler panic caught as InternalError, IN-5 zero-feature build |
| `tests/system/nfr_platform.rs` | FT-4 WASM build compiles without std-only APIs |
| `tests/system/nfr_modularity.rs` | IN-6 enabled is strict subset of full feature set |
