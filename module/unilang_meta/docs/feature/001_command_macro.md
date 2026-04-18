# Feature: `#[unilang::command]` Attribute Macro

The `unilang_meta` crate must provide a `#[unilang::command]` procedural attribute macro that inspects an annotated Rust function, infers argument metadata from parameter names and types, and generates a `static CommandDefinition` plus a wrapper function bridging the interpreter's `fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData>` signature to the user's simpler parameter-by-name signature.
