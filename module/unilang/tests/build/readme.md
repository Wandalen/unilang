# Build Domain

Build-time code generation tests: YAML/JSON extraction, PHF generation, hint generation, type analysis, and compile-time validation.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `helpers_hint_generator.rs` | `build_helpers::HintGenerator` — hint message formatting for type mismatches |
| `helpers_type_analyzer.rs` | `build_helpers::TypeAnalyzer` — type mismatch detection during build |
| `validation.rs` | Build-time validation of command definitions from YAML/JSON sources |
| `phf_codegen_no_leaked_dep.rs` | Codegen: `generate_static_registry_source()` emits no bare `phf_map!` |
