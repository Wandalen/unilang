# Feature: REPL Interactive

### Scope

- **Purpose:** Define behavioral requirements for REPL and interactive CLI modalities
- **Responsibility:** FR-REPL-1, FR-INTERACTIVE-1, FR-MOD-WASM-REPL: read-eval-print loop, interactive mode, WASM
- **In Scope:** REPL behavior requirements, interactive mode requirements, WASM modality
- **Out of Scope:** REPL implementation details, feature flag configuration (see architecture/006)

### Design

The REPL design rests on the stateless execution loop guarantee: the `Parser`, `SemanticAnalyzer`, and `Interpreter` components are fully reusable across multiple invocations within a single process. No state accumulates between command executions. This means REPL session length has no effect on per-command processing overhead, and memory usage does not grow with session activity.

The interactive argument protocol integrates with the REPL loop at the error-interception layer. When a mandatory argument with `interactive: true` is absent, the semantic analyzer returns a distinct `ArgumentInteractiveRequired` signal — not a validation failure — allowing the REPL layer to intercept it, prompt the user for the missing value (using secure input for sensitive arguments), and re-submit the command with the value supplied. Optional arguments with defaults never trigger this protocol.

The two-tier feature structure (`repl` base feature + `enhanced_repl` extension) allows integrators to choose between a minimal standard-IO implementation and a readline-enhanced implementation without changing application code. The base tier provides all REPL functional behavior; the enhanced tier adds terminal navigation (history, arrow keys, tab completion) via an optional dependency. See `architecture/006_repl_implementation.md` for configuration details.

The WASM modality requires that the core library compiles to `wasm32-unknown-unknown` without platform-specific dependencies, enabling browser-hosted REPL instances.

### FR-REPL-1 (REPL Support)

The framework's core components (`Pipeline`, `Parser`, `SemanticAnalyzer`, `Interpreter`) **must** be structured to support a REPL-style execution loop. They **must** be reusable for multiple, sequential command executions within a single process lifetime.

**Implementation Notes:**
- Pipeline components are fully stateless and reusable
- Each command execution is independent with no state accumulation
- Memory efficient operation verified through performance benchmarks
- Reference implementations available in the examples directory

**Implementation status:** ✅ Implemented with comprehensive examples and verified stateless operation.

### FR-INTERACTIVE-1 (Interactive Argument Prompting)

When a mandatory argument with the `interactive: true` attribute is not provided, the `Semantic Analyzer` **must** return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`). This allows the calling modality to intercept the error and prompt the user for input.

**Implementation status:** ✅ Implemented in the semantic analyzer with comprehensive test coverage and REPL integration.

### FR-MOD-WASM-REPL (WebAssembly REPL Modality)

The framework **must** support a web-based REPL modality that can operate entirely on the client-side without a backend server. This requires the core `unilang` library to be fully compilable to the `wasm32-unknown-unknown` target.

**Implementation status:** ✅ Implemented — see `examples/wasm-repl/` (browser REPL) and `examples/wasm-repl/tests/wasm.rs` (wasm-bindgen-test suite).

This requirement is connected to NFR-PLATFORM-1 (WASM Compatibility), which requires the core logic of the `unilang` and `unilang_parser` crates to be platform-agnostic and fully compatible with the `wasm32-unknown-unknown` target.

### Analyses

| File | Relationship |
|------|--------------|
| [002_usability_improvements.md](../analysis/002_usability_improvements.md) | Usability improvements for REPL and interactive patterns |

### APIs

| File | Relationship |
|------|--------------|
| [002_error_codes.md](../api/002_error_codes.md) | ArgumentInteractiveRequired error code surfaced by REPL |

### Architectures

| File | Relationship |
|------|--------------|
| [006_repl_implementation.md](../architecture/006_repl_implementation.md) | REPL implementation guide and feature flags |

### Features

| File | Relationship |
|------|--------------|
| [003_pipeline.md](003_pipeline.md) | Pipeline used by REPL for command processing |

### Invariants

| File | Relationship |
|------|--------------|
| [002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFR-PLATFORM-1 WASM compatibility requirement |

### Sources

| File | Relationship |
|------|--------------|
| `src/bin/unilang_cli.rs` | REPL loop implementation |
| `src/interpreter.rs` | Command execution used by REPL |

### Tests

| File | Relationship |
|------|--------------|
| `tests/pipeline/pipeline_core.rs` | FT-1 stateless REPL, FT-2 interactive arg absent, FT-3 arg provided, FT-5 empty input |
| `tests/system/nfr_platform.rs` | FT-4 WASM build compiles without std-only APIs |
| `examples/wasm-repl/tests/wasm.rs` | FT-7 WASM REPL glue executes commands and returns formatted output/error string |
| `tests/manual/readme.md` | Manual REPL testing plan |
