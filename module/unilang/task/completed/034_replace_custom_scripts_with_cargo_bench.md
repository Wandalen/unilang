# Replace Custom Scripts with Cargo Bench Workflow

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
**HIGH PRIORITY VIOLATION**: Usage.md **Recommendation** - Always use `cargo bench` as primary interface. Don't rely on custom scripts or runners.

**Current Violations**:
- `run_all_benchmarks.sh`
- `run_comprehensive_benchmark.sh`
- `test_benchmark_system.sh`

**Required Workflow** (from usage.md):
```bash
# This should be your standard workflow
cargo bench

# Not this
cargo run --bin my-benchmark-runner
```

**Why This Matters**: Keeps aligned with Rust ecosystem conventions and ensures benchmarks work in CI/CD.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Use cargo bench from Day One" section
-   Related to Task 028 (directory structure) and Task 035 (CI/CD integration)
-   Must maintain all benchmark functionality while using standard workflow

## Acceptance Criteria

-   [ ] All custom benchmark shell scripts removed or deprecated
-   [ ] All benchmark functionality accessible via `cargo bench`
-   [ ] Cargo.toml [[bench]] sections properly configured
-   [ ] Feature flags properly configured for benchmark execution
-   [ ] Documentation updated to show `cargo bench` commands instead of custom scripts
-   [ ] CI/CD integration uses `cargo bench` workflow
-   [ ] Quick vs comprehensive benchmark modes available through cargo bench options
-   [ ] All benchmark features work correctly with standard Rust tooling

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

- **N/A** `COMPLETED` — Validated by N/A (pre-template). Replace Custom Scripts with Cargo Bench Workflow.
