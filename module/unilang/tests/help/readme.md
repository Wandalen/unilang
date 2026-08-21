# Help System Tests

Tests for help text generation, formatting, conventions, and completeness.

## Files

| File | Responsibility |
|------|----------------|
| `generation.rs` | Help text generation from command definitions |
| `formatting.rs` | Help output formatting and layout |
| `conventions.rs` | Naming and content conventions for help text |
| `detection_matrix.rs` | Routing matrix for the `??` help token |
| `enforcement.rs` | Validation that help standards are enforced |
| `unified_format.rs` | Unified help format across command types |
| `operator.rs` | Positional `??` help routing at the semantic stage |
| `nonexistent_command.rs` | Help behavior for unknown/missing commands |
| `help_completeness_validation.rs` | Verify all commands have help text |
| `help_divergence_prevention.rs` | Prevent help text drifting from implementation |
| `cli_invocation.rs` | CLI binary-level help output: flags, commands, and format |
| `features_comprehensive.rs` | Comprehensive help feature coverage across all scenarios |
| `show_version.rs` | Version string displayed in help output |
| `verbosity.rs` | Help output detail level controlled by verbosity flags |
