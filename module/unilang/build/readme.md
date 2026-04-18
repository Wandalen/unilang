# Build Script

Build-time code generation for the `unilang` crate. Generates PHF-based static command registries from YAML/JSON manifests.

## Files

| File | Responsibility |
|------|----------------|
| `main.rs` | Entry point: orchestrates discovery, type analysis, and code generation |
| `type_hints.rs` | Type hint analysis: detects String args that should be Boolean/Integer |
| `validation.rs` | Build-time command validation: name, version, dot-prefix, duplicates |
| `codegen.rs` | PHF code generation: command/argument consts, PHF map, string escaping |
| `discovery.rs` | File discovery: YAML/JSON parsing and build summary output |
