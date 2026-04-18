# Invariant: Parser Tokenization Mandate

### Scope

- **Purpose:** Prohibit from-scratch tokenization and enforce strs_tools as the sole tokenization engine
- **Responsibility:** Tokenization dependency constraint; string-splitting abstraction enforcement
- **In Scope:** Tokenization implementation approach; strs_tools-only requirement
- **Out of Scope:** API surface definitions, public method signatures, error reporting format

The `unilang_parser` crate must not implement low-level string tokenization logic from scratch; it must use the `strs_tools` crate as its sole tokenization engine, ensuring syntactic analysis remains focused on instruction-structure recognition rather than raw byte splitting.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_parsing_api.md](../feature/001_parsing_api.md) | Public API contract that this tokenization mandate supports |
