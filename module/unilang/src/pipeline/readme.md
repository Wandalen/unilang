# Pipeline Module

Split from `pipeline.rs`. Orchestrates parse → semantic analysis → execution flow.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point and public re-exports |
| `core.rs` | `Pipeline` struct, `CommandResult`, core processing methods |
| `error_parsing.rs` | Error message string parsing helpers |
| `batch.rs` | `BatchResult` and multi-command batch processing |
| `argv.rs` | `process_command_from_argv` — CLI argument vector entry points |
