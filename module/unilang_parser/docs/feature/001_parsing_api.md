# Feature: Parser Public API

### Scope

- **Purpose:** Define the behavioral contract for the public parsing API exposed by unilang_parser
- **Responsibility:** Entry-point method signatures, input/output contracts, parser-ignorance constraint
- **In Scope:** parse_from_argv, parse_repl_input, parse_multiple_instructions contracts; semantic ignorance requirement
- **Out of Scope:** Internal parsing algorithms, tokenization strategy, error recovery implementation

The `unilang_parser` crate must expose `Parser::parse_from_argv(&[String])` for shell argv input, `Parser::parse_repl_input(&str)` for single-instruction string input (REPL/config), and `Parser::parse_multiple_instructions(&str)` for multi-command strings, producing `Vec<GenericInstruction>` in all cases without any knowledge of command definitions or semantics.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [invariant/001_parser_mandate.md](../invariant/001_parser_mandate.md) | Tokenization strategy constraint governing the implementation |
