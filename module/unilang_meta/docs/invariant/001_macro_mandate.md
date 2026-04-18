# Invariant: Macro Tooling Mandate

The `unilang_meta` crate must use `macro_tools` as its sole dependency for all procedural macro development; direct dependencies on `syn`, `quote`, or `proc-macro2` are forbidden — `macro_tools` re-exports all three and provides higher-level abstractions that enforce consistent error reporting, attribute parsing, and token generation across all generated code.
