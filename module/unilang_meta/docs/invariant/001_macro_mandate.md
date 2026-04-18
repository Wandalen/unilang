# Invariant: Macro Tooling Mandate

### Scope

- **Purpose:** Prohibit direct syn/quote/proc-macro2 dependencies and enforce macro_tools as the sole macro toolkit
- **Responsibility:** Dependency constraint for procedural macro development; tooling abstraction enforcement
- **In Scope:** macro_tools-only dependency mandate; forbidden direct syn/quote/proc-macro2 usage
- **Out of Scope:** Macro behavior specification, generated code contracts, feature requirements

The `unilang_meta` crate must use `macro_tools` as its sole dependency for all procedural macro development; direct dependencies on `syn`, `quote`, or `proc-macro2` are forbidden — `macro_tools` re-exports all three and provides higher-level abstractions that enforce consistent error reporting, attribute parsing, and token generation across all generated code.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [feature/001_command_macro.md](../feature/001_command_macro.md) | Behavioral contract for the macro this mandate governs |
