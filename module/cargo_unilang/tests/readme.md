# cargo_unilang Test Suite

## Organization

Tests are organized by **functional domain** — what is being tested, not how.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `checks_test.rs` | Validate check functions: deprecated API, build.rs, duplicate deps detection |
| `commands_test.rs` | Validate command implementation: verbosity, bool parsing, path/name validation |
| `exit_code_documentation_bug.rs` | Regression: exit code 1 for all error conditions (bug reproducer) |
| `integration_test.rs` | End-to-end integration: `cargo_unilang new/check` CLI invocations |
| `outdated_version_template_bug.rs` | Regression: generated Cargo.toml uses current unilang version (bug reproducer) |
| `templates_test.rs` | Validate template generation: Cargo.toml, main.rs, build.rs output correctness |

## Domain Map

| Domain | Test Files | What Is Tested |
|--------|------------|----------------|
| **Commands** | `commands_test.rs` | Parameter validation, CLI argument processing |
| **Checks** | `checks_test.rs` | Health check functions for project validation |
| **Templates** | `templates_test.rs` | Code generation output for scaffolded files |
| **Integration** | `integration_test.rs` | Full CLI workflow: new project, check project |
| **Regressions** | `exit_code_documentation_bug.rs`, `outdated_version_template_bug.rs` | Bug prevention guards |

## Running Tests

```bash
# Run all tests
cargo nextest run --all-features

# Run with warnings-as-errors
RUSTFLAGS="-D warnings" cargo nextest run --all-features
```

## Standards

All tests comply with `test_organization.rulebook.md`:
- ✅ Domain-based organization
- ✅ Tests in `tests/` only (never in `src/`)
- ✅ No mocking — real CLI invocations via `assert_cmd`
- ✅ No disabled tests without explicit permission
- ✅ Bug reproducers preserved permanently
