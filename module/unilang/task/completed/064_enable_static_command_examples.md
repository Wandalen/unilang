# Enable Static Command Examples

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
Enable the static command examples that were disabled during the test-clean process. This includes `static_01_basic_compile_time.rs`, `static_02_yaml_build_integration.rs`, and `static_03_performance_comparison.rs`. These examples demonstrate the zero-overhead PHF-based static command system and validate the performance requirements.

Links to related tasks: Depends on task 063 (registry integration), leads to CLI builder tasks.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   All static command examples must compile without errors or warnings
-   Examples must demonstrate actual PHF-based zero-overhead lookup
-   Performance examples must validate <1ms p99 latency requirement
-   Examples must use 2-space indentation following codestyle rules
-   Must rename `.disabled` files back to `.rs` extension
-   All examples must run successfully with `cargo run --example <name>`
-   Examples must demonstrate compile-time command registration workflow
-   No clippy warnings when running `cargo clippy --examples --all-features -- -D warnings`

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

- **N/A** `COMPLETED` — Validated by N/A (pre-template). Enable Static Command Examples.
