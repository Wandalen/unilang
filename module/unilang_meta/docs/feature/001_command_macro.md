# Feature: `#[unilang::command]` Attribute Macro

### Scope

- **Purpose:** Define the behavioral contract for the attribute macro that reduces command registration boilerplate
- **Responsibility:** Macro inference rules, generated code contract, bridging signature specification
- **In Scope:** Parameter name/type inference, CommandDefinition generation, wrapper function signature
- **Out of Scope:** Tokenization internals, macro_tools usage strategy, build-time vs runtime registration

The `unilang_meta` crate must provide a `#[unilang::command]` procedural attribute macro that inspects an annotated Rust function, infers argument metadata from parameter names and types, and generates a `static CommandDefinition` plus a wrapper function bridging the interpreter's `fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData>` signature to the user's simpler parameter-by-name signature.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc  | [invariant/001_macro_mandate.md](../invariant/001_macro_mandate.md) | macro_tools dependency constraint governing macro implementation |
