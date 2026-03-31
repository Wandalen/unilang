# Regression Tests

Bug-prevention tests ensuring previously fixed issues do not recur.

## Files

| File | Responsibility |
|------|----------------|
| `command_registration.rs` | Regression: command registration edge cases |
| `command_namespace_format_validation.rs` | Regression: namespace format validation |
| `dot_command_panic.rs` | Regression: panic on dot-only command names |
| `duplicate_command_registration.rs` | Regression: silent duplicate command behavior |
| `empty_args_handling.rs` | Regression: empty argument list handling |
| `example_yaml_discovery_bug.rs` | Regression: YAML example discovery failure |
| `parameter_collection.rs` | Regression: multiple parameter collection ordering |
| `repeated_parameter_handling.rs` | Regression: repeated parameter accumulation |
| `validation_rule_api_format.rs` | Regression: validation rule API format stability |
