# Write tests for ergonomic aggregation APIs

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
Write comprehensive tests for the new ergonomic aggregation APIs that provide simple interfaces for common use cases while preserving complex APIs for advanced scenarios. This includes testing the aggregate_cli! macro for zero-boilerplate static aggregation, CliBuilder for complex scenarios, mode selection APIs, and conditional module loading. The tests should validate both compile-time and runtime aggregation paths while ensuring backward compatibility. Links to tasks 048-051 for foundation components.

## Requirements

- All work must strictly adhere to the rules defined in the following rulebooks:
  - `$PRO/genai/code/rules/code_design.rulebook.md`
  - `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

- Tests for aggregate_cli! macro with various syntax forms
- Tests for CliBuilder with static, dynamic, and conditional modules
- Tests for mode selection and intelligent defaults
- Tests for backward compatibility with existing CliAggregator
- Tests for conditional module loading with feature flags
- Tests for error handling and validation
- Integration tests with hybrid registry and multi-YAML build system
- All tests must pass with `ctest1` verification

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

- **N/A** `COMPLETED` — Validated by N/A (pre-template). Write tests for ergonomic aggregation APIs.
