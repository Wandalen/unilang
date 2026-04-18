# Invariant: Parser Tokenization Mandate

The `unilang_parser` crate must not implement low-level string tokenization logic from scratch; it must use the `strs_tools` crate as its sole tokenization engine, ensuring syntactic analysis remains focused on instruction-structure recognition rather than raw byte splitting.
