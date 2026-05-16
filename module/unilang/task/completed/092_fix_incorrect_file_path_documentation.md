# Task 092: Fix Incorrect File Path Parameter Documentation

## Execution State
- **Status:** ✅ (Completed)
- **Executor Type:** AI
- **Actor:** N/A (pre-template)
- **Claimed At:** N/A (pre-template)
- **Priority:** 0
- **Validated By:** N/A (pre-template)
- **Validation Date:** N/A (pre-template)

## Goal
Multiple projects using unilang incorrectly document that "unilang cannot parse file paths" due to `/` being a "token separator". This is **FALSE** - the parser handles file paths perfectly when using correct syntax (`::` double colon operator instead of single `:`).

The root cause is **documentation error**, not parser limitation. Projects are using wrong parameter syntax (`path:value` with single colon) and concluding the parser is broken, when they should be using `path::value` (double colon).

**Impact**: Downstream projects (like `planer`) have disabled CLI functionality and created workarounds because they believe unilang cannot handle file paths. This limits adoption and creates unnecessary technical debt.

## Problem Analysis

### What Was Documented (WRONG)

Projects claim:
> "Unilang DSL parser treats `/` as token separator, making file paths unparsable"

Example test from `planer` project (`tests/cli_parser_limitations.rs:64`):
```rust
// Attempt to pass file path - will fail with parse error
let result = pipeline.process_command_simple( ".plan.phases path:tests/file.md" );
//                                                         ^ SINGLE COLON - WRONG SYNTAX!

assert!( !result.success, "Command should fail due to parser limitation" );
```

### Actual Root Cause (DOCUMENTATION BUG)

The parser **does NOT** use `/` as a token separator in parameter values. The actual behavior:

1. **Parameter syntax is `name::value`** (double colon `::`, not single `:`)
2. **After `::` operator, parser enters "value context"** where special characters are protected
3. **Characters like `/`, `.`, `#`, `?` are preserved** in value context
4. **Only whitespace terminates values** (or can be quoted)

### Confirmed Working Syntax

```bash
# ✅ CORRECT - Double colon with file path
planer .plan.phases plan_path::tests/asset/plan.md

# ✅ CORRECT - Complex path with dots and slashes
planer .plan.phases plan_path::../other/dir/file.md

# ✅ CORRECT - Quoted path (for paths with spaces)
planer .plan.phases plan_path::"path with spaces/file.md"

# ❌ WRONG - Single colon (not valid unilang syntax)
planer .plan.phases path:tests/file.md
```

## Evidence

### MRE Test Results

Created comprehensive MRE at `/home/user1/pro/lib/wip_core/willbe/kbase3/module/planer/tests/-mre_file_paths.rs` with results:

```
Test 1 (single `:`):  Parse error - "Unexpected token 'path:tests/file'" ❌
Test 2 (double `::`): Execution error - "No such file or directory" ✅ PARSER ACCEPTED!
Test 3 (quoted `::`):" Execution error - "No such file or directory" ✅ PARSER ACCEPTED!
Test 4 (alias `::` ): Execution error - "No such file or directory" ✅ PARSER ACCEPTED!
Test 5 (complex `::` ): Execution error - "No such file or directory" ✅ PARSER ACCEPTED!
```

**Key Finding**: Tests 2-5 fail with "No such file or directory" (file system error), NOT "Parse error". This proves the parser successfully extracted the path value!

### Real File Test

```bash
$ cd planer && cargo run -- .plan.phases plan_path::tests/asset/plan_chunking/001/plan.md output_mode::dry-run

🔍 Dry-run mode:
  Would generate 1 phase(s): [0]
```

**SUCCESS!** File path parsed correctly and command executed.

### Parser Source Code Confirmation

Analyzed unilang parser implementation at `/home/user1/pro/lib/wip_core/unilang/dev/module/unilang_parser/src/`:

**Value Context Protection** (`parser_engine/mod.rs:177-306`):
- After detecting `::` operator, enter "value context"
- Accumulate all tokens until whitespace delimiter
- Merge accumulated tokens into single value
- Characters like `/`, `.`, `#`, `?` preserved

**Test Coverage** (`tests/value_context_tests.rs`):
- 19 comprehensive tests covering special characters
- All tests passing, confirming protection works
- Includes path-like values: `dir/file#123`

## Requirements

1. **Audit documentation** across all unilang-related projects for incorrect file path claims
2. **Fix incorrect documentation** stating "/` is a token separator" or "file paths cannot be parsed"
3. **Add correct syntax examples** to unilang documentation showing `param::value` with file paths
4. **Update downstream projects** (planer, etc.) to remove "known limitations" about file paths
5. **Add unilang syntax guide** section explaining `::` operator and value context protection
6. **Create migration guide** for projects using wrong syntax (single `:` → double `::`)

## Acceptance Criteria

- [x] Unilang main documentation includes "Parameter Syntax" section explaining `::` operator
- [x] Documentation explicitly shows file path examples: `path::dir/file.md`
- [x] Value context protection behavior documented (what characters are preserved)
- [x] Downstream project `planer` updates:
  - [x] Remove "KNOWN LIMITATION" comment from code
  - [x] Delete/update `tests/cli_parser_limitations.rs` (it documents wrong syntax)
  - [x] Add positive test showing correct `::` syntax works
  - [x] Update CLI documentation with correct syntax examples
- [x] Migration guide created for users of wrong syntax
- [x] Tests added to unilang demonstrating file path parsing works

## Rulebook References

- `code_design.rulebook.md` - For proper test organization and documentation
- `codebase_hygiene.rulebook.md` - For avoiding duplicate/outdated documentation
- `test_organization.rulebook.md` - For MRE test structure and documentation
- `code_style.rulebook.md` - For code example formatting

## Notes

### Why This Bug Persisted

1. **Single colon looks intuitive**: Developers naturally tried `param:value` (common in many CLIs)
2. **Error message unhelpful**: "Unexpected token" doesn't explain correct syntax should use `::`
3. **No syntax guide**: Unilang lacks prominent documentation of `::` operator requirement
4. **Confirmation bias**: Once "parser can't handle paths" belief formed, it wasn't questioned

### Impact on Adoption

This documentation bug has real costs:
- Projects avoid using unilang for CLI tools (believe it can't handle basic use cases)
- Workarounds created (environment variables, config files) adding complexity
- Users conclude parser is "broken" without realizing syntax issue

### Parser Actually Supports

The value context protection system handles:
- File paths: `dir/subdir/file.ext`
- URLs: `https://example.com/path?query=value`
- Special characters: `#`, `?`, `/`, `.`, `&`, `=`
- Complex patterns: `regex::"[a-zA-Z0-9]+"`
- Quoted values: `msg::"Hello World"`

All of these work correctly when using `::` operator!

## Related Files

**Evidence**:
- `/home/user1/pro/lib/wip_core/willbe/kbase3/module/planer/tests/-mre_file_paths.rs` - MRE tests
- `/home/user1/pro/lib/wip_core/willbe/kbase3/module/planer/tests/cli_parser_limitations.rs` - Wrong docs

**Parser Source**:
- `/home/user1/pro/lib/wip_core/unilang/dev/module/unilang_parser/src/parser_engine/mod.rs` - Value context logic
- `/home/user1/pro/lib/wip_core/unilang/dev/module/unilang_parser/tests/value_context_tests.rs` - Test coverage

**Affected Projects**:
- `planer` - Has "known limitation" docs and workaround code
- Any other projects using unilang with file path parameters

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
