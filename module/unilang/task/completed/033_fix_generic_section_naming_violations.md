# Fix Generic Section Naming Violations

## Execution State
- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** N/A (pre-template)
- **Claimed At:** N/A (pre-template)
- **Priority:** 0
- **Validated By:** N/A (pre-template)
- **Validation Date:** N/A (pre-template)

## Goal
**HIGH PRIORITY VIOLATION**: Usage.md **AVOID** states generic section names cause conflicts and should be avoided. Current documentation uses prohibited generic naming.

**Prohibited Section Names** (from usage.md):
- "Performance" (too generic, causes conflicts)
- "Results" (unclear what kind of results)  
- "Benchmarks" (doesn't specify what's benchmarked)

**Required Section Names**:
- "Algorithm Performance Analysis"
- "String Processing Results"
- "Memory Usage Benchmarks"
- "API Response Times"
- "Core Algorithm Performance"

**Current State**: Multiple references to generic "Performance" sections found in documentation.

## Requirements

-   All work must strictly adhere to the rules defined in the following rulebooks:
    -   `$PRO/genai/code/rules/code_design.rulebook.md`
    -   `$PRO/genai/code/rules/code_style.rulebook.md`
-   Must follow benchkit usage.md "Prohibited Practices and Violations" section
-   Related to Task 032 (automatic documentation) - must use specific section names
-   Must prevent section name conflicts and duplication

## Acceptance Criteria

-   [ ] All generic "Performance" section names replaced with specific descriptive names
-   [ ] All generic "Results" section names replaced with context-specific names
-   [ ] All generic "Benchmarks" section names replaced with algorithm/feature-specific names
-   [ ] Section names clearly indicate what is being measured
-   [ ] No section name conflicts exist in documentation
-   [ ] MarkdownUpdater calls use specific section names
-   [ ] Documentation navigation improved with descriptive section names
-   [ ] All benchmark reports use consistent specific naming convention

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
