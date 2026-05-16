# Update SIMD performance validation test to use benchkit

## Execution State
- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** N/A (pre-template)
- **Claimed At:** N/A (pre-template)
- **Priority:** 0
- **Validated By:** N/A (pre-template)
- **Validation Date:** N/A (pre-template)

## Goal
The SIMD performance validation test in `tests/simd_json_integration_test.rs` (function `test_simd_performance_validation`) currently uses manual timing measurements instead of leveraging benchkit's professional benchmarking infrastructure. This test is critical for validating that SIMD optimizations provide the expected performance improvements.

The test needs to be modernized to use benchkit's `ComparativeAnalysis` framework to provide statistically rigorous validation of SIMD performance characteristics and clear pass/fail criteria based on benchkit's measurement capabilities.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`

## Acceptance Criteria

-   Convert manual timing measurements to use benchkit's `ComparativeAnalysis` API
-   Compare standard JSON parsing vs SIMD JSON parsing using benchkit algorithms
-   Implement proper feature gating with `#[cfg(feature = "benchmarks")]`
-   Provide clear validation logic that determines if SIMD performance meets expectations
-   Display comprehensive benchmark results using `ComparisonReport` methods
-   Maintain existing JSON test data generation for realistic performance testing
-   Include SIMD capability detection and reporting in the output
-   Test compiles and runs successfully with proper feature flags
-   Provide actionable PASS/FAIL validation results for SIMD performance
-   Update ignore attribute to reference correct benchmark feature requirements

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
