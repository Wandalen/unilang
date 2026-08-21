# Feature: Pipeline

### Scope

- **Purpose:** Define behavioral requirements for command execution pipeline orchestration
- **Responsibility:** FR-PIPE-1 through FR-PIPE-4: parse→semantic→execute flow, batch, sequence, argv
- **In Scope:** Pipeline orchestration requirements, batch vs sequence semantics, argv integration
- **Out of Scope:** Parser implementation, interpreter internals, performance characteristics

### Design

The pipeline is a stateless orchestrator that composes three independent stages in a fixed order: Parse → Semantic Analyze → Execute. Each stage is a separate actor with a single responsibility. The parser converts raw string input into a structured `GenericInstruction` with no knowledge of command definitions. The semantic analyzer validates the instruction against the registry and produces a `VerifiedCommand` with fully typed, bound argument values. The interpreter dispatches the `VerifiedCommand` to its registered routine.

Error propagation is short-circuit: each stage produces either a success value passed to the next stage or an error returned immediately to the caller, with no partial execution. This guarantees that routines only run when the input has been fully validated.

The pipeline exposes two execution modes beyond single-command invocation. Batch mode executes all commands in a list independently, collecting all results regardless of individual failures. Sequence mode executes commands in order and stops immediately on the first failure. Both modes reuse the same stateless pipeline components, making REPL-style repeated invocation efficient with no state accumulation between calls.

Help request interception is transparent: when the semantic analyzer detects an unquoted `??` help token (bare, positional, or `name::??`), it returns a `HelpRequested` signal that the pipeline converts to a successful `OutputData` containing formatted help text before returning to the caller. Integrators do not need to handle this case separately; those who need `??` as data disable interception with `with_help_detection( false )`.

The argv-based API accepts a `&[String]` array directly from the operating system, intelligently recombining consecutive elements to preserve argument boundaries before passing to the parser. This eliminates the information loss that occurs when shell-provided argument arrays are naively joined into a single string.

### FR-PIPE-1 (Pipeline Orchestration)

The `Pipeline` API **must** correctly orchestrate the full sequence: Parsing → Semantic Analysis → Interpretation.

**Implementation status:** ✅ Implemented as `Pipeline::process_command()`. Orchestrates full Parse → SemanticAnalysis → Interpretation sequence. Comprehensive test coverage in the pipeline test suite.

### FR-PIPE-2 (Batch Processing)

The `Pipeline::process_batch` method **must** execute a list of commands independently, collecting results for each and not stopping on individual failures.

**Implementation status:** ✅ Implemented as `Pipeline::process_batch()`. Executes commands independently, collects all results, continues on individual failures. Returns `BatchResult`.

### FR-PIPE-3 (Sequence Processing)

The `Pipeline::process_sequence` method **must** execute a list of commands in order and **must** terminate immediately upon the first command failure.

**Implementation status:** ✅ Implemented as `Pipeline::process_sequence()`. Executes commands in order, terminates immediately on first failure. Returns `BatchResult`.

### FR-PIPE-4 (Argv-Based Command Execution)

The framework **must** provide argv-based parsing and execution APIs (`Pipeline::process_command_from_argv`, `Pipeline::process_command_from_argv_simple`) that accept command-line arguments as `&[String]` arrays. These methods **must** intelligently combine consecutive argv elements to preserve argument boundaries: elements containing `::` start named arguments, following elements without `::` or `.` prefix are combined into parameter values with proper quoting. This eliminates information loss when CLI applications receive OS-provided argv arrays, enabling natural shell syntax without special quoting requirements. The argv API **must** integrate with the full semantic analysis and execution pipeline, providing identical functionality to string-based APIs while preserving type-safe argv boundaries.

**Implementation status:** ✅ Implemented in the parser engine (`parse_from_argv`) and pipeline (`process_command_from_argv`, `process_command_from_argv_simple`). Comprehensive test coverage in the argv API test suite. Resolves Task 080 CLI integration issues.

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | Types used in pipeline operations |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](001_command_registry.md) | Command registry queried by pipeline |
| [002_argument_system.md](002_argument_system.md) | Argument binding performed by pipeline |
| [004_help_system.md](004_help_system.md) | Help commands processed by pipeline |
| [005_repl_interactive.md](005_repl_interactive.md) | REPL that reuses this pipeline |

### Sources

| File | Relationship |
|------|--------------|
| `src/pipeline/` | Pipeline module: core, batch, argv |
| `src/interpreter.rs` | Command execution interpreter |

### Tests

| File | Relationship |
|------|--------------|
| `tests/pipeline/pipeline_core.rs` | FR-PIPE-1..4: process, batch, sequence, argv spec cases |
| `tests/cli/` | CLI integration tests |
| `tests/interpreter/` | Interpreter execution tests |
