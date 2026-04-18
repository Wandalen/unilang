# unilang_parser Source Directory

## Responsibility

Source code for the `unilang_parser` crate — parser engine, public types, and configuration.

## File Responsibility Table

| File / Directory | Responsibility |
|------------------|---------------|
| `lib.rs` | Crate root: module declarations and public API exports |
| `parser_engine/` | Core parsing engine: `Parser` impl, tokenization, command path and argument parsing |
| `argv_types.rs` | Input marker newtypes: `ShellArgv` and `ReplInput` |
| `cli_parser.rs` | CLI parameter parsing convenience API (`CliParams` trait) |
| `config.rs` | Parser configuration options (`UnilangParserOptions`) |
| `error.rs` | Error types: `ParseError`, `ErrorKind`, `SourceLocation` |
| `instruction.rs` | Output types: `GenericInstruction`, `Argument` |
| `item_adapter.rs` | Split item classification and adaptation for the parser engine |
