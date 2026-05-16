# Semantic Tests

Tests for command validation, argument binding, type checking, and typo suggestions.

## Files

| File | Responsibility |
|------|----------------|
| `command_validation.rs` | Command-level validation: required args, unknown commands |
| `argument_binding.rs` | Named/positional argument binding to definitions |
| `multiple_parameters.rs` | Multiple same-name parameter collection |
| `parameter_storage_validation.rs` | Parameter storage invariants |
| `parameter_typo_suggestion.rs` | Levenshtein-based typo suggestions for parameters |
| `parser_semantic.rs` | Interaction between parser output and semantic layer |
| `unknown_parameters.rs` | Error handling for unrecognized parameters |
| `unknown_parameters_edge_cases.rs` | Edge cases in unknown parameter detection |
| `centralized_validation.rs` | Cross-path validation: all construction paths share validation rules |
| `auto_categorize_decoupling.rs` | Auto-categorize logic decoupled from output formatting |
| `format_category_name_decoupling.rs` | Category name formatting decoupled from semantic model |
