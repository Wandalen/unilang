# CLI Tests

Tests for the CLI builder API, ergonomic interfaces, and shell argument handling.

## Files

| File | Responsibility |
|------|----------------|
| `cli_builder_api.rs` | Builder pattern for constructing CLI command sets |
| `ergonomic_apis.rs` | Higher-level ergonomic API surface tests |
| `shell_argument_parsing.rs` | Shell-level argument tokenization and quoting |
| `verbosity_control.rs` | Verbosity flag behavior and output suppression |
| `cli_integration.rs` | CLI binary invocation: command execution and output |
| `multiword_params.rs` | Multi-word parameter parsing via CLI invocation |
