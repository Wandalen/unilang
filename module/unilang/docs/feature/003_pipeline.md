# Feature: Pipeline

### Scope

- **Purpose:** Define behavioral requirements for command execution pipeline orchestration
- **Responsibility:** FR-PIPE-1 through FR-PIPE-4: parse→semantic→execute flow, batch, sequence, argv
- **In Scope:** Pipeline orchestration requirements, batch vs sequence semantics, argv integration
- **Out of Scope:** Parser implementation, interpreter internals, performance characteristics

Functional requirements governing the command processing pipeline, batch execution, and argv integration.

### FR-PIPE-1 (Pipeline Orchestration)

The `Pipeline` API **must** correctly orchestrate the full sequence: Parsing -> Semantic Analysis -> Interpretation.

**Implementation status:** ✅ Implemented as `Pipeline::process_command()` in `src/pipeline.rs`. Orchestrates full Parse → SemanticAnalysis → Interpretation sequence. Comprehensive test coverage in `tests/pipeline/`.

### FR-PIPE-2 (Batch Processing)

The `Pipeline::process_batch` method **must** execute a list of commands independently, collecting results for each and not stopping on individual failures.

**Implementation status:** ✅ Implemented as `Pipeline::process_batch()` in `src/pipeline.rs`. Executes commands independently, collects all results, continues on individual failures. Returns `BatchResult`.

### FR-PIPE-3 (Sequence Processing)

The `Pipeline::process_sequence` method **must** execute a list of commands in order and **must** terminate immediately upon the first command failure.

**Implementation status:** ✅ Implemented as `Pipeline::process_sequence()` in `src/pipeline.rs`. Executes commands in order, terminates immediately on first failure. Returns `BatchResult`.

### FR-PIPE-4 (Argv-Based Command Execution)

The framework **must** provide argv-based parsing and execution APIs (`Pipeline::process_command_from_argv`, `Pipeline::process_command_from_argv_simple`) that accept command-line arguments as `&[String]` arrays. These methods **must** intelligently combine consecutive argv elements to preserve argument boundaries: elements containing `::` start named arguments, following elements without `::` or `.` prefix are combined into parameter values with proper quoting. This eliminates information loss when CLI applications receive OS-provided argv arrays, enabling natural shell syntax without special quoting requirements. The argv API **must** integrate with the full semantic analysis and execution pipeline, providing identical functionality to string-based APIs while preserving type-safe argv boundaries.

**Implementation status:** ✅ Implemented in `unilang_parser/src/parser_engine.rs` (`parse_from_argv`) and `unilang/src/pipeline.rs` (`process_command_from_argv`, `process_command_from_argv_simple`). Comprehensive test coverage in `tests/argv_api.rs` with 9 tests covering all argv scenarios. Resolves Task 080 CLI integration issues.

### Pipeline Methods Reference

The `Pipeline` struct provides the following primary methods:

- `process_command(input: &str, context: ExecutionContext) -> Result<OutputData, Error>` — Full pipeline for single string input.
- `process_command_simple(input: &str) -> Result<OutputData, Error>` — Convenience variant with default context.
- `process_command_from_argv(argv: &[String], context: ExecutionContext) -> Result<OutputData, Error>` — Argv-based variant preserving shell argument boundaries (see FR-PIPE-4).
- `process_command_from_argv_simple(argv: &[String]) -> Result<OutputData, Error>` — Convenience argv variant with default context.
- `process_batch(commands: &[String], context: ExecutionContext) -> BatchResult` — Independent execution, collects all results (see FR-PIPE-2).
- `process_sequence(commands: &[String], context: ExecutionContext) -> BatchResult` — Ordered execution, stops on first failure (see FR-PIPE-3).
- `process_help_request(command_name: &str, context: ExecutionContext) -> Result<OutputData, Error>` — Processes help requests uniformly across the framework.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_command_registry.md](001_command_registry.md) | Command registry queried by pipeline |
| doc | [feature/002_argument_system.md](002_argument_system.md) | Argument binding performed by pipeline |
| doc | [feature/004_help_system.md](004_help_system.md) | Help commands processed by pipeline |
