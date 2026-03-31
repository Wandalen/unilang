# Semantic Module

Split from `semantic.rs`. Validates parsed instructions against the command registry.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point and public re-exports |
| `core.rs` | `SemanticAnalyzer`, `VerifiedCommand`, typed argument accessors |
| `argument_binding.rs` | Binding parsed arguments to command definitions |
| `validation.rs` | Type checking, constraint validation, alias resolution |
