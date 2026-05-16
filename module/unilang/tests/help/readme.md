# Help System Tests

Tests for help text generation, formatting, conventions, and completeness.

## Files

| File | Responsibility |
|------|----------------|
| `generation.rs` | Help text generation from command definitions |
| `formatting.rs` | Help output formatting and layout |
| `conventions.rs` | Naming and content conventions for help text |
| `enforcement.rs` | Validation that help standards are enforced |
| `unified_format.rs` | Unified help format across command types |
| `operator.rs` | Help generation for operator-style commands |
| `nonexistent_command.rs` | Help behavior for unknown/missing commands |
| `help_completeness_validation.rs` | Verify all commands have help text |
| `help_divergence_prevention.rs` | Prevent help text drifting from implementation |
| `cli_invocation.rs` | CLI binary-level help output: flags, commands, and format |
| `features_comprehensive.rs` | Comprehensive help feature coverage across all scenarios |
| `show_version.rs` | Version string displayed in help output |
| `verbosity.rs` | Help output detail level controlled by verbosity flags |

## Excluded from Compilation

The following files exist in this directory but are **not** referenced in `help.rs` because they use private `CommandDefinition` fields or non-existent API methods not yet aligned with the current builder API:

| File | Blocker |
|------|---------|
| `conventions.rs` | Direct field writes (`cmd.auto_help_enabled = true`), non-existent `Pipeline::process_command`, non-existent `CommandDefinition::generate_help_command` |
| `enforcement.rs` | Direct field access on private `CommandDefinition` fields |
| `operator.rs` | `error_data.code` compared against `&str` but typed `ErrorCode`; private field access |

Fix: update each file to use `former()`-based builder and getter methods, then re-enable in `help.rs`.
