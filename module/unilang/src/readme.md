# Source Files

Library source code for the `unilang` crate.

## Files

| File / Directory | Responsibility |
|------------------|----------------|
| `lib.rs` | Crate root; re-exports public API |
| `data/` | Core data models (command definitions, arguments, types) |
| `multi_yaml/` | Multi-file YAML aggregation and static code generation |
| `build_helpers/` | Build-time utilities (type hints, code generators) |
| `bin/` | Binary entry points |
| `registry/` | Command registry: static PHF, dynamic LRU, bridge, and traits |
| `pipeline/` | Command pipeline: batch and single-command processing |
| `semantic/` | Semantic analysis: argument binding and validation |
| `interpreter.rs` | Command execution engine |
| `help/` | Help text generation and formatting |
| `loader.rs` | Load command definitions from YAML/JSON strings |
| `interner.rs` | String interning for zero-copy command name lookups |
| `static_data/` | Compile-time static command data structures |
| `error.rs` | Error types for the crate |
| `types.rs` | Shared type aliases |
| `config_extraction.rs` | Extract configuration values from command arguments |
| `command_validation.rs` | Command validation logic |
| `validation_core.rs` | Core validation primitives |
| `simd_json_parser.rs` | SIMD-accelerated JSON parsing |
| `simd_tokenizer.rs` | SIMD-accelerated tokenizer |
| `data.rs` | Module façade for `data/` |
| `build_helpers.rs` | Module façade for `build_helpers/` |
