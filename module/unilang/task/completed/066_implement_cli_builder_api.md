# Implement CliBuilder API

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
- **Validated By:** N/A
- **Validation Date:** N/A

## Goal
Implement the `CliBuilder` fluent API for ergonomic CLI aggregation in the `src/multi_yaml/` module. This builder enables combining multiple CLI tools into unified commands with prefix management, namespace isolation, and conflict detection. The implementation must support both static and dynamic command sources.

Links to related tasks: Depends on task 065 (tests), leads to task 067 (multi-YAML system).

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Must create `src/multi_yaml/builder.rs` with `CliBuilder` struct
-   Must implement fluent API with method chaining
-   Must provide `static_module_with_prefix()`, `detect_conflicts()`, `build_static()` methods
-   Must implement `ModuleConfig` and `ModuleSource` supporting structs
-   Must detect and report command prefix conflicts at build time
-   Must use 2-space indentation following codestyle rules
-   All tests from task 065 must pass after implementation
-   Must integrate with `StaticCommandRegistry` for zero-overhead lookup
-   No clippy warnings when running `cargo clippy --all-targets --all-features -- -D warnings`

## In Scope

_N/A — pre-template task. Scope not formally documented._

## Out of Scope

_N/A — pre-template task._

## Work Procedure

_N/A — pre-template task. See git history for changes made._

## Test Matrix

_N/A — pre-template task. Testing not formally documented._

## Validation

### Checklist

_N/A — pre-template task._

### Measurements

_N/A — pre-template task._

### Invariants

_N/A — pre-template task._

### Anti-faking Checks

_N/A — pre-template task._

## Outcomes

_Pre-template task — outcomes not formally recorded. See task body for implementation details._

## History

- **N/A** `COMPLETED` — Validated by N/A (pre-template). Implement CliBuilder API.
