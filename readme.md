# Unilang

Universal command language parser and framework for building CLI applications.

## Overview

Unilang provides a unified syntax for command-line interfaces across multiple languages and platforms. It includes a parser, command registry, and tools for building type-safe CLI applications.

## Crates

- **unilang** - Core library with command registry and execution framework
- **unilang_parser** - Parser for Unilang command syntax
- **unilang_help** - Reusable help-page domain model, verbosity levels, and renderers
- **unilang_meta** - Procedural macros for deriving Unilang traits
- **cargo_unilang** - Cargo integration tool

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
unilang = "0.60.0"
unilang_parser = "0.40.0"
```

## Quick Start

```rust
use unilang::{ Registry, Command };
use unilang_parser::{ Parser, UnilangParserOptions };

// Create a parser
let parser = Parser::new( UnilangParserOptions::default() );

// Parse a command
let instruction = parser.parse_single_instruction( ".greet name::Alice" )?;

// Access command and arguments
println!( "Command: {}", instruction.command_path );
```

## Building

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace --all-targets --all-features
```

## License

Licensed under MIT license.

## Links

- Repository: https://github.com/Wandalen/unilang
- Discord: https://discord.gg/m3yKnHRAGr
