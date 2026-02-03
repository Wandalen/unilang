# docs/

Documentation for the unilang_parser crate.

## Contents

- `cli_integration.md` - Integration guide for CLI applications

## Overview

The unilang_parser crate provides low-level lexical and syntactic analysis for the unilang command language. It transforms raw input strings into structured `GenericInstruction` objects without requiring knowledge of command definitions.

## Key Components

- **Lexer**: Token generation from input strings
- **Parser**: Syntactic analysis and instruction building
- **Error Recovery**: Robust error handling and reporting

## Usage

Most users should use the main `unilang` crate which provides a higher-level API. Direct usage of `unilang_parser` is only needed for:

- Custom parser implementations
- Tooling that analyzes command syntax
- Advanced integration scenarios

## See Also

- Main crate: `../../unilang/readme.md`
- Examples: `../../unilang/examples/`
- Benchmarks: `../benches/`
