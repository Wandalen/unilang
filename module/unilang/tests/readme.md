# Test Organization

This directory follows a **domain-based organization principle** where tests are grouped by functional domain rather than test methodology.

## Organization Principle

**Core Rule**: Tests are organized by **what you're testing** (functional domain), not **how you're testing** (test methodology).

## Core Principles

1. **Feature-Based Organization**: Tests are organized by functionality, not by tasks, issues, or implementation history
2. **Clear Hierarchy**: Tests follow a predictable directory and naming structure
3. **Single Responsibility**: Each test file has a single, well-defined purpose
4. **No Duplication**: Overlapping test coverage is eliminated through clear boundaries
5. **Comprehensive Coverage**: All functionality must be tested at appropriate levels

## Directory Structure

```
tests/
├── readme.md           # This file
├── parser/            # All parser functionality tests
├── semantic/          # All semantic analysis tests
├── interpreter/       # All interpreter execution tests
├── registry/          # All registry management tests
├── help/             # All help system tests
├── cli/              # All CLI integration tests
├── pipeline/         # All pipeline processing tests
├── data/             # All data model tests
├── build/            # All build-time generation tests
├── performance/      # All performance tests
├── system/           # Cross-cutting system tests
├── acceptance/       # User acceptance tests
├── regression/       # Bug prevention tests
└── manual/           # Manual testing procedures
```

## Domain Descriptions

### Core Functionality Domains

- **`parser/`**: Tokenization, argument parsing, SIMD parsing, string interning
- **`semantic/`**: Command validation, argument binding, type checking, multiple parameter collection
- **`interpreter/`**: Command execution, context management, error handling
- **`registry/`**: Static/dynamic registry, command lookup, performance metrics
- **`help/`**: Help generation, formatting, conventions
- **`pipeline/`**: End-to-end command processing workflows

### Supporting Domains

- **`cli/`**: CLI builder APIs, ergonomic interfaces, shell integration
- **`data/`**: Data models, serialization, validation
- **`build/`**: Build-time code generation, static registries, static compilation
- **`performance/`**: Benchmarks, stress tests, performance analysis

### Cross-Cutting Domains

- **`system/`**: End-to-end workflows, API compatibility, external usage patterns
- **`acceptance/`**: User-facing acceptance criteria
- **`regression/`**: Critical bug prevention tests

## Test Methodology

Each domain directory contains **all test types** relevant to that domain:
- Unit tests (component isolation)
- Integration tests (component interaction)
- Performance tests (benchmarks and stress tests)
- Edge case tests
- Error condition tests

## Benefits

✅ **Mental Model Alignment**: Find all parser tests in `parser/`, all semantic tests in `semantic/`
✅ **Locality of Change**: Working on registry? All related tests are in `registry/`
✅ **Easy Discovery**: No hunting across multiple directories for related functionality
✅ **Maintainable**: Changes to a domain only affect one test directory

## Adding New Tests

1. **Identify the domain**: What component/feature are you testing?
2. **Add to appropriate directory**: Put the test in `tests/{domain}/`
3. **Use descriptive naming**: File names should indicate specific functionality tested
4. **Include all test types**: Add unit, integration, and performance tests as needed

## Naming Conventions

### Files
- All test files use `snake_case` naming
- File names should indicate specific functionality: `{feature}.rs` (e.g., `tokenization.rs`, `multiple_parameters.rs`)

### Test Functions
- `test_{specific_behavior}()` for most tests
- `{category}_{specific_scenario}()` when categorization helps (e.g., `regression_{bug_prevention}()`)

### Test Modules
- Each test file should have a module doc comment explaining its scope
- Tests should be grouped logically within files using mod blocks if needed

## Responsibility Table

### Top-Level Test Files

| File | Responsibility |
|------|----------------|
| `acceptance.rs` | Acceptance domain entry point: user-facing CLI acceptance criteria |
| `argv_api.rs` | Argv-based API: `parse_cli`/`parse_repl` entry points via ShellArgv/ReplInput |
| `auto_categorize_decoupling_test.rs` | Decoupling: `auto_categorize` independence from internal modules |
| `build.rs` | Build domain entry point: build-time code generation and static registry tests |
| `build_helpers_hint_generator.rs` | `build_helpers::hint_generator` — PHF compile-error hint generation |
| `build_helpers_type_analyzer.rs` | `build_helpers::type_analyzer` — Rust type analysis for codegen |
| `build_validation_test.rs` | Build-time validation of command definitions in YAML/JSON sources |
| `category_field_backward_compat.rs` | Category field: backward compatibility with pre-category data |
| `category_field_codegen.rs` | Category field: build-time code generation output correctness |
| `category_field_conversion.rs` | Category field: type conversion between representations |
| `category_field_edge_cases.rs` | Category field: boundary conditions and edge case handling |
| `category_field_unit.rs` | Category field: unit-level behavior of the category data type |
| `cli.rs` | CLI domain entry point: builder APIs and ergonomic interface tests |
| `cli_multiword_params_test.rs` | CLI binary: multi-word parameter round-trip via `unilang_cli` binary |
| `config_extraction.rs` | Config extraction: retrieving configuration from command definitions |
| `data.rs` | Data domain entry point: data model, serialization, and validation tests |
| `feature_parity_test.rs` | Feature parity: `StaticCommandRegistry` vs `CommandRegistry` equivalence |
| `format_category_name_decoupling_test.rs` | Decoupling: `format_category_name` independence from internal modules |
| `help.rs` | Help domain entry point: help generation and formatting tests |
| `help_verbosity.rs` | Help verbosity: detail level control in generated help output |
| `interpreter.rs` | Interpreter domain entry point: command execution and context tests |
| `multi_yaml_conflict_detection.rs` | Multi-YAML: conflict detection when merging multiple YAML command sources |
| `output_truncation.rs` | Output truncation: long output trimming behavior |
| `parser.rs` | Parser domain entry point: tokenization, SIMD, and string interning tests |
| `parser_reexport_test.rs` | Re-export: `unilang::parser` public API surface accessibility |
| `phf_codegen_no_leaked_dep_test.rs` | Codegen: `generate_static_registry_source()` emits no bare `phf_map!` |
| `phf_reexport_test.rs` | Re-export: `unilang::phf` types available without direct phf dependency |
| `registry.rs` | Registry domain entry point: static/dynamic registry and lookup tests |
| `regression.rs` | Regression domain entry point: critical bug prevention tests |
| `semantic.rs` | Semantic domain entry point: validation, argument binding, type checking |
| `show_version_in_help.rs` | Version display: `--version`/`-V` flag presence in generated help |
| `static_registry_conversion_test.rs` | Static registry: `StaticCommandRegistry` → `CommandRegistry` conversion bridge |
| `system.rs` | System domain entry point: cross-cutting end-to-end workflow tests |
| `task084_verification_test.rs` | Task 084: command macro implementation verification |
| `validation_abi_compatibility.rs` | Validation: cross-crate type ABI compatibility (external build) |
| `validation_clippy.rs` | Validation: crate passes clippy with `-D warnings` (external build) |
| `validation_core_test.rs` | Validation core module: internal validation logic unit tests |
| `validation_direct_import.rs` | Validation: direct import works without feature-gate ceremony |
| `validation_doc_examples.rs` | Validation: documentation examples compile and produce expected output |
| `validation_feature_gate.rs` | Validation: `enabled`/`full` feature gates enable/disable correctly |
| `validation_phf_indirect.rs` | Validation: PHF usable via unilang re-export, no direct dep required |

### Support Files

| File | Responsibility |
|------|----------------|
| `doc_test_status.md` | Doc test enablement tracking: current pass/fail/ignore state |

## Manual Testing

See `manual/readme.md` for manual testing procedures and organization.

## Quality Standards

### Coverage Requirements
- **Domain Coverage**: All core domains must have comprehensive test coverage
- **Functionality Coverage**: All public APIs, error conditions, edge cases tested
- **Performance Coverage**: Critical paths must have performance validation
- **Regression Coverage**: All critical bugs must have prevention tests

### Test Quality
- **Fast Execution**: Individual tests should complete quickly (< 100ms typically)
- **Deterministic**: Tests must be reliable and repeatable
- **Isolated**: Tests should not depend on external state or other tests
- **Clear**: Each test should have a single, obvious purpose

## File Organization Rules

### Prohibited Patterns
❌ **Task-based naming**: `task_024_*.rs`, `issue_017_*.rs`
❌ **Type-based directories**: `unit/`, `integration/`, `acceptance/` as primary organization
❌ **Generic naming**: `tests.rs`, `integration_tests.rs`
❌ **Implementation details**: `*_debug_test.rs`, `*_failure_test.rs`

### Required Patterns
✅ **Domain-based organization**: `parser/`, `semantic/`, `registry/`
✅ **Feature-based naming**: `multiple_parameters.rs`, `command_validation.rs`
✅ **Clear scope**: `tokenization.rs`, `cli_integration.rs`
✅ **Descriptive names**: `quoted_values.rs`, `error_scenarios.rs`

## Test Content Standards

### Test Documentation
Each test file must include:
```rust
//! Brief description of the test file's purpose
//!
//! ## Scope
//! What this file tests and doesn't test
//!
//! ## Coverage
//! List of key behaviors validated
//!
//! ## Related
//! Links to related test files or documentation
```

### Test Structure
```rust
#[test]
fn test_specific_behavior() {
    // Arrange: Set up test conditions
    let input = create_test_input();

    // Act: Execute the behavior under test
    let result = system_under_test(input);

    // Assert: Verify expected outcomes
    assert_eq!(result.status, Expected::Success);
    assert!(result.output.contains("expected content"));
}
```

### Error Testing
Every module must include negative tests:
```rust
#[test]
fn test_error_condition_handling() {
    let invalid_input = create_invalid_input();

    let result = system_under_test(invalid_input);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "EXPECTED_ERROR_CODE");
}
```

## Maintenance Guidelines

### Adding New Tests
1. Determine appropriate category (unit/integration/acceptance/regression)
2. Choose correct directory based on component being tested
3. Use clear, descriptive naming following conventions
4. Include comprehensive documentation
5. Ensure no overlap with existing tests

### Removing Tests
1. Verify test is truly redundant (not just similar)
2. Ensure coverage is maintained by other tests
3. Document removal reason in commit message
4. Update related documentation

### Refactoring Tests
1. Maintain existing test names unless fundamentally changing scope
2. Preserve test intent and coverage
3. Update documentation to reflect changes
4. Verify no coverage gaps are introduced

## Enforcement

This organization system is mandatory for all test code. Pull requests that violate these standards will be rejected. Use the following checklist:

- [ ] Test is in correct category directory
- [ ] File name follows naming conventions
- [ ] No overlap with existing test coverage
- [ ] Includes proper documentation
- [ ] Test functions follow naming standards
- [ ] Appropriate level of mocking for category
- [ ] Error conditions are tested
- [ ] Performance considerations addressed

## Migration Plan

Existing tests will be gradually migrated to this system:

### Phase 1: Critical Reorganization
- Remove task-based and issue-based test files
- Consolidate overlapping coverage
- Establish directory structure

### Phase 2: Content Migration
- Move tests to appropriate categories
- Rename files to follow conventions
- Add missing documentation

### Phase 3: Coverage Analysis
- Identify and fill gaps
- Remove remaining redundancy
- Optimize test performance

### Phase 4: Validation
- Verify complete coverage
- Performance benchmarking
- Documentation review