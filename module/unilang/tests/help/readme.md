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
