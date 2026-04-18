# docs/

Documentation for the unilang_parser crate.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `cli_integration.md` | Explain correct argv vs string parsing usage and pitfalls |
| `feature/` | Behavioral requirements for parser features |
| `invariant/` | Non-negotiable architectural and correctness constraints |

## Overview

The unilang_parser crate provides low-level lexical and syntactic analysis for the unilang command language. It transforms raw input strings into structured `GenericInstruction` objects without requiring knowledge of command definitions.

## Key Components

- **Lexer**: Token generation from input strings
- **Parser**: Syntactic analysis and instruction building
- **Error Recovery**: Robust error handling and reporting

## Public API Summary

### Primary Parsing Methods

| Method | Input | Use Case |
|--------|-------|----------|
| `Parser::parse_from_argv(&[String])` | Shell argv (pre-tokenized) | CLI applications receiving `std::env::args()` |
| `Parser::parse_single_instruction(&str)` | Raw string | REPL, config files, embedded commands |
| `Parser::parse_multiple_instructions(&str)` | Multi-command string | Batch processing |

> **Note:** `parse_repl_input` is the canonical name; `parse_single_instruction` is `#[deprecated]` forwarding shim. See `feature/001_parsing_api.md`.

## Usage

Most users should use the main `unilang` crate which provides a higher-level API. Direct usage of `unilang_parser` is only needed for:

- Custom parser implementations
- Tooling that analyzes command syntax
- Advanced integration scenarios

## See Also

- Main crate: `../../unilang/readme.md`
- Examples: `../../unilang/examples/`
- Benchmarks: `../benches/`
