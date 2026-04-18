# Implement `#[unilang::command]` Procedural Attribute Macro

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 📥 (Backlog)
- **Validated By:** null
- **Validation Date:** null

## Metrics

| Value | Easiness | Priority | Safety | Advisability |
|-------|----------|----------|--------|--------------|
| 8 | 4 | 2 | 8 | 512 |

## Goal

Create a procedural attribute macro `#[unilang::command]` in the `unilang_meta` proc-macro crate that eliminates the boilerplate of manual `CommandDefinition` construction. The macro inspects an annotated Rust function, infers argument metadata from parameter names and types, and generates: (1) a `static CommandDefinition` populated from macro attributes and inferred parameters; (2) a wrapper function bridging the interpreter's `fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData>` signature to the user's simpler parameter-by-name signature; (3) a public registration accessor `fn() -> &'static CommandDefinition`. Primary dependency is `macro_tools` — direct use of `syn`, `quote`, or `proc_macro2` is forbidden; all token manipulation must go through `macro_tools` re-exports.

## In Scope

- `unilang_meta/src/lib.rs` — full macro implementation across 5 increments
- `unilang_meta/Cargo.toml` — add `unilang` and `trybuild` dependencies as needed
- `unilang_meta/tests/` — `trybuild` test harness and all UI test fixtures
- Attribute parsing: `name`, `namespace`, `hint`, `description` via `macro_tools::attr_prop::AttributePropertySyn`
- Type-to-Kind mapping: `String` → `Kind::String`, `i64`/`i32`/`usize` → `Kind::Integer`, `bool` → `Kind::Boolean`, `PathBuf` → `Kind::Path`, `Option<T>` → inferred kind + optional=true
- Wrapper function generation via `macro_tools::quote::qt!` / `format_ident!`
- Error reporting via `macro_tools::diag::syn_err!` / `return_syn_err!`
- `trybuild` success tests (compiles correctly) and error tests (`.stderr` fixture files)

## Out of Scope

- `unilang` crate modifications — `CommandDefinition`, `ArgumentDefinition`, `Kind`, `VerifiedCommand`, `ExecutionContext` remain unchanged
- Async function support — synchronous functions only in this task
- Per-parameter override attributes (e.g., `#[unilang::arg(hint="...")]`) — deferred to follow-up
- `status`, `permissions`, `examples` advanced attributes — deferred to increment 5
- Serialization or YAML loading integration — separate concerns
- Any direct dependency on `syn`, `quote`, or `proc_macro2` — all must come through `macro_tools`

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Minimum rulebook references: `code_design.rulebook.md`, `codebase_hygiene.rulebook.md`, `test_organization.rulebook.md`, `code_style.rulebook.md`
- Custom codestyle per `code_style.rulebook.md` — 2-space indents, no `cargo fmt`
- Tests in `unilang_meta/tests/` — no `#[cfg(test)]` in `src/`
- No mocking — test the actual generated code via `trybuild` compilation
- **macro_tools mandatory:** `syn`, `quote`, `proc_macro2` must only be accessed through `macro_tools` re-exports; no direct crate dependencies
- Each increment must leave `cargo test -p unilang_meta` green before starting the next

## Work Procedure

Execute increments in order. Each increment must pass its own verification before proceeding.

### Increment 1 — Project Setup and Basic Attribute Parsing

1. **Read rulebooks** — `kbase .rulebooks`; internalize code style, test organization, and fix documentation constraints
2. **Verify Cargo.toml** — confirm `macro_tools = { workspace = true, features = ["full"] }` is present; add `unilang = { path = "../unilang" }` to `[dependencies]` and `trybuild = "1.0"` to `[dev-dependencies]`
3. **Attribute struct** — in `src/lib.rs`, define `CommandAttributes` to parse `name = "..."`, `namespace = "..."`, `hint = "..."`, `description = "..."` using `macro_tools::attr_prop::AttributePropertySyn<syn::LitStr, Marker>` pattern (see Technical Context)
4. **Macro stub** — implement `command(attr: TokenStream, item: TokenStream) -> TokenStream`; parse attrs into `CommandAttributes`; return item unmodified for now
5. **Trybuild harness** — create `tests/trybuild.rs` with `t.pass(...)` calls
6. **UI test** — create `tests/ui/01_basic_command_compiles.rs`: function annotated with `#[unilang::command(name = ".hello", namespace = ".app")]` that compiles without errors
7. **Verify** — `cargo test -p unilang_meta`; `trybuild` test must pass

### Increment 2 — Infer `ArgumentDefinition`s from Function Parameters

1. **Parse function** — `syn::parse2::<syn::ItemFn>(item_stream)` to get `sig.inputs`
2. **Type mapper** — implement `fn map_type_to_kind(ty: &syn::Type) -> Result<(TokenStream, bool), syn::Error>` returning `(Kind variant tokens, is_optional)`: handle `String`, `i64`, `i32`, `usize`, `bool`, `PathBuf`, `Option<T>` (recursive call on inner T)
3. **ArgumentDefinition generation** — generate `qt! { vec![ #( unilang::data::ArgumentDefinition { name: #name, kind: #kind, ... } ),* ] }` for all non-special params (skip `VerifiedCommand`/`ExecutionContext` if user passes them)
4. **Compile test** — create `tests/ui/02_argument_inference_compiles.rs`: function with `String`, `bool`, `Option<i64>` params annotated with macro
5. **Verify** — `cargo test -p unilang_meta`

### Increment 3 — Generate the Routine Wrapper Function

1. **Wrapper name** — `format_ident!("__unilang_wrapper_{}", fn_ident)` from `macro_tools::quote::format_ident`
2. **Wrapper signature** — `fn #wrapper_ident(command: unilang::semantic::VerifiedCommand, context: unilang::interpreter::ExecutionContext) -> Result<unilang::data::OutputData, unilang::data::ErrorData>`
3. **Argument marshalling** — for each param: generate `let #param_name = command.arguments.get(#param_name_str).and_then(|v| match v { ... }).ok_or_else(|| ErrorData { ... })?;`; handle `Option<T>` as non-failing `.and_then(...).and_then(...)` with no `?`
4. **Return wrapping** — call user function with bound vars; wrap `String` result in `OutputData { content: result, format: "text".to_string(), execution_time_ms: None }`
5. **Compile test** — create `tests/ui/03_wrapper_generation_compiles.rs`: function with mixed params, assert it compiles and return type is correct
6. **Verify** — `cargo test -p unilang_meta`

### Increment 4 — Generate Static `CommandDefinition`

1. **Static name** — `format_ident!("__UNILANG_DEF_{}", fn_ident_upper)` as `static` item
2. **Static body** — `qt! { static #static_ident: unilang::data::CommandDefinition = unilang::data::CommandDefinition { name: #name, namespace: #namespace, hint: #hint, description: #description, arguments: #args_tokens, ... }; }` — populate all fields from parsed attributes and inferred args
3. **Registration fn** — `pub fn #register_ident() -> &'static unilang::data::CommandDefinition { &#static_ident }` where `register_ident = format_ident!("__unilang_register_{}", fn_ident)`
4. **Full output** — macro emits: original user function + wrapper function + static definition + registration function
5. **Integration test** — create `tests/ui/04_generates_full_definition.rs`: call `__unilang_register_*()` and assert returned `CommandDefinition` fields match macro attributes
6. **Verify** — `cargo test -p unilang_meta`

### Increment 5 — Error Handling, Advanced Attributes, and Finalization

1. **Error cases** — use `macro_tools::diag::syn_err!(span, "message")` / `return_syn_err!` for: missing required `name` attribute; unsupported Rust type in parameter; non-function item (struct, enum, etc.) passed to macro
2. **Advanced attrs** — extend `CommandAttributes` to parse optional `status = "..."` and `permissions = [...]` fields
3. **Error test fixtures** — create `tests/ui/05_missing_name_fails.rs` + `tests/ui/05_missing_name_fails.stderr`, `tests/ui/06_unsupported_type_fails.rs` + `.stderr` — verify exact compiler error messages
4. **Documentation** — add doc comment to `command` proc-macro function in `src/lib.rs` with usage example
5. **Conformance check** — `cargo clippy -p unilang_meta --all-targets -- -D warnings` must be clean; `cargo test -p unilang_meta --all-targets` must pass
6. **Verify** — full Level 3: `w3 .test level::3` scoped to `unilang_meta`

## Test Matrix

| # | Input Scenario | What Is Tested | Expected Behavior |
|---|---------------|----------------|-------------------|
| T01 | `#[unilang::command(name = ".hello", namespace = ".app")] fn hello() -> String` | Basic attribute parsing | Compiles without errors |
| T02 | Function with `String`, `bool`, `Option<i64>` params | Type-to-Kind inference | `Kind::String`, `Kind::Boolean`, `Kind::Integer` with optional=true generated correctly |
| T03 | Function with `PathBuf` param | PathBuf mapping | `Kind::Path` generated |
| T04 | Macro applied to a `struct` | Non-function item error | `syn_err!` fires; compile error with descriptive message |
| T05 | `#[unilang::command(namespace = ".app")] fn ...` (no name) | Missing required attr | Compile error: "name attribute is required" |
| T06 | Function with `Vec<String>` param (unsupported) | Unknown type error | Compile error: "unsupported parameter type" |
| T07 | Full pipeline: annotated function → `__unilang_register_*()` | End-to-end codegen | Registration fn returns `CommandDefinition` with correct name/namespace/args |
| T08 | Wrapper function called with `VerifiedCommand` having correct args | Wrapper argument extraction | User function receives correct typed values |
| T09 | Wrapper called with missing required arg | Missing arg error path | Returns `Err(ErrorData { ... })` without panicking |
| T10 | `cargo test -p unilang_meta --all-targets` | Full test suite | 0 failures, 0 warnings |

## Acceptance Criteria

**Feature Completeness (all must be satisfied):**

1. `#[unilang::command(name = "...", namespace = "...")]` applied to a Rust function compiles without errors
2. Generated `static CommandDefinition` has `name`, `namespace`, `hint`, `description` matching macro attributes
3. `ArgumentDefinition` entries in the static are inferred from function parameters with correct `name`, `kind`, and `optional` fields
4. Generated wrapper function has exact signature `fn(VerifiedCommand, ExecutionContext) -> Result<OutputData, ErrorData>`
5. Wrapper correctly marshals named arguments from `command.arguments` to typed local variables
6. Missing required argument in wrapper returns `Err(ErrorData { ... })` — no panic
7. `__unilang_register_*()` public function returns a valid `&'static CommandDefinition`

**Quality Requirements (all must be satisfied):**

8. **macro_tools only** — `Cargo.toml` has no direct `syn`, `quote`, or `proc_macro2` dependencies; all token manipulation uses `macro_tools` re-exports
9. **Error messages** — unsupported type, missing name attr, and non-function item each produce a `syn::Error` with the relevant `Span` (compiler points to the problematic token)
10. **trybuild coverage** — at least 4 passing UI tests and 2 failing UI tests with `.stderr` fixtures
11. **No `#[cfg(test)]` in src** — all tests in `unilang_meta/tests/`
12. **Clippy clean** — `cargo clippy -p unilang_meta --all-targets -- -D warnings` passes with 0 warnings
13. **Codestyle** — 2-space indents, custom style per `code_style.rulebook.md`; no `cargo fmt` output

## Validation

**Execution:** Independent validator walks this section after SUBMIT per `validation.rulebook.md`.

### Checklist

Desired answer for every question is YES.

**Attribute Parsing**
- [ ] C1 — Does `CommandAttributes` parse `name`, `namespace`, `hint`, `description` via `AttributePropertySyn`?
- [ ] C2 — Does the macro return a `syn::Error` with appropriate span when `name` attribute is missing?
- [ ] C3 — Does `tests/ui/05_missing_name_fails.stderr` exist and match actual compiler output?

**Type Inference**
- [ ] C4 — Does `map_type_to_kind` correctly map `String`, `bool`, `i64`, `PathBuf` to their `Kind` variants?
- [ ] C5 — Does `Option<T>` produce the same `Kind` as `T` with `optional = true`?
- [ ] C6 — Does an unsupported type (e.g., `Vec<String>`) produce a `syn::Error` pointing to that parameter?

**Code Generation**
- [ ] C7 — Does the generated static have fields matching the macro attributes (verify via `tests/ui/04_generates_full_definition.rs`)?
- [ ] C8 — Does the wrapper function have the exact interpreter-expected signature?
- [ ] C9 — Does the wrapper return `Err(ErrorData { ... })` for a missing required argument (not a panic)?
- [ ] C10 — Is `__unilang_register_*()` a public function returning `&'static CommandDefinition`?

**Dependency Constraint**
- [ ] C11 — Does `unilang_meta/Cargo.toml` contain NO direct `syn`, `quote`, or `proc_macro2` entries?
- [ ] C12 — Is every use of `syn` / `quote` / `proc_macro2` accessed through `macro_tools::` path?

**Test Coverage**
- [ ] C13 — Do all `trybuild` pass tests succeed (`cargo test -p unilang_meta`)?
- [ ] C14 — Do all `.stderr` fixtures match actual compiler error output exactly?
- [ ] C15 — Is there no `#[cfg(test)]` anywhere in `unilang_meta/src/`?

### Measurements

- [ ] M1 — trybuild pass tests: count of `t.pass(...)` calls ≥ 4
- [ ] M2 — trybuild fail tests: count of `t.compile_fail(...)` calls ≥ 2
- [ ] M3 — test count: `cargo test -p unilang_meta --all-targets 2>&1 | grep -E '^test result'` → all passed
- [ ] M4 — clippy: `cargo clippy -p unilang_meta --all-targets -- -D warnings 2>&1 | grep -c '^error'` → 0
- [ ] M5 — no direct syn/quote/proc_macro2: `grep -E '^\s*(syn|quote|proc_macro2)\s*=' unilang_meta/Cargo.toml | wc -l` → 0

### Invariants

- [ ] I1 — test suite: `cargo test -p unilang_meta --all-targets` → 0 failures
- [ ] I2 — compiler clean: `RUSTFLAGS="-D warnings" cargo check -p unilang_meta --all-features` → 0 warnings
- [ ] I3 — no cfg(test) in src: `grep -r '#\[cfg(test)\]' unilang_meta/src/` → no matches

### Anti-faking checks

- [ ] AF1 — trybuild tests are substantive: each `.rs` file in `tests/ui/` contains an annotated function (not just `fn main() {}`); UI tests for error cases contain the pattern that should trigger the error
- [ ] AF2 — `.stderr` fixtures are accurate: `trybuild` passes (meaning expected output matches actual), not manually crafted to fake passing
- [ ] AF3 — integration test validates content: `tests/ui/04_generates_full_definition.rs` calls `__unilang_register_*()` and accesses `.name` field (not just `let _ = ...`)

## Outcomes

*(Executor fills this section during execution. Required before SUBMIT.)*

**Increment 1 — Attribute Parsing Green:**

```
[Paste cargo test output showing Increment 1 trybuild test passing]
```

**Increment 2 — Type Inference Green:**

```
[Paste cargo test output showing Increment 2 trybuild test passing]
```

**Increment 3 — Wrapper Generation Green:**

```
[Paste cargo test output showing Increment 3 trybuild test passing]
```

**Increment 4 — Static CommandDefinition Green:**

```
[Paste cargo test output showing Increment 4 trybuild test passing]
```

**Increment 5 — Full Test Suite Green:**

```
[Paste w3 .test level::3 output showing 0 failures, 0 warnings]
```

**Key Learnings:**

*(Insights for future proc-macro development in this workspace)*

## Technical Context

### macro_tools API Reference

All token manipulation must use `macro_tools` re-exports. Never add direct `syn`, `quote`, or `proc_macro2` to `Cargo.toml`.

**Attribute property pattern (from `macro_tools::attr_prop`):**

```rust
use macro_tools::attr_prop::{ AttributePropertyComponent, AttributePropertySyn };

// Step 1: Define a marker struct per property
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct NameMarker;
impl AttributePropertyComponent for NameMarker
{
  const KEYWORD : &'static str = "name";
}

// Step 2: Type alias for convenience
pub type NameProperty = AttributePropertySyn< syn::LitStr, NameMarker >;

// Step 3: Parsing struct with all properties
#[ derive( Debug, Default ) ]
pub struct CommandAttributes
{
  pub name : NameProperty,
  pub namespace : AttributePropertySyn< syn::LitStr, NamespaceMarker >,
  pub hint : AttributePropertySyn< syn::LitStr, HintMarker >,
  pub description : AttributePropertySyn< syn::LitStr, DescriptionMarker >,
}

// Step 4: Parse via `from_attrs` idiom
impl CommandAttributes
{
  pub fn from_attrs( attrs : &[ syn::Attribute ] ) -> Result< Self, syn::Error >
  {
    let mut result = Self::default();
    for attr in attrs
    {
      // iterate nested meta pairs: name = "value"
      // set result.name, result.namespace, etc. via AttributePropertySyn::assign
    }
    Ok( result )
  }
}
```

**Boolean property (flag = true/false):**

```rust
use macro_tools::attr_prop::AttributePropertyBoolean;

#[ derive( Debug, Default, Clone, Copy ) ]
pub struct OptionalMarker;
impl AttributePropertyComponent for OptionalMarker
{
  const KEYWORD : &'static str = "optional";
}
pub type OptionalProperty = AttributePropertyBoolean< OptionalMarker >;
```

**Token generation:**

```rust
use macro_tools::quote::{ qt, format_ident };

// qt! is alias for quote!
let name_lit = &attrs.name.0; // syn::LitStr
let tokens = qt!
{
  static MY_CMD : unilang::data::CommandDefinition = unilang::data::CommandDefinition
  {
    name : #name_lit.to_string(),
    // ...
  };
};

// Identifier creation
let wrapper_ident = format_ident!( "__unilang_wrapper_{}", fn_ident );
```

**Error diagnostics:**

```rust
use macro_tools::diag::{ syn_err, return_syn_err };

// Create error (returns syn::Error)
let err = syn_err!( name_token.span(), "name attribute is required" );

// Create and immediately return from the macro function
return_syn_err!( ty.span(), "unsupported parameter type: {}", ty_str );
```

### unilang Data Types Used in Generated Code

The macro generates code that references these `unilang` types (available after adding `unilang` dep):

```rust
// Wrapper function signature — exact types required
fn wrapper(
  command : unilang::semantic::VerifiedCommand,
  context : unilang::interpreter::ExecutionContext,
) -> Result< unilang::data::OutputData, unilang::data::ErrorData >

// Argument extraction pattern from VerifiedCommand
let value = command.arguments.get( "arg_name" )
  .and_then( | v | if let unilang::data::Value::String( s ) = v { Some( s.clone() ) } else { None } )
  .ok_or_else( || unilang::data::ErrorData { message : "missing arg".to_string(), code : unilang::data::ErrorCode::MissingRequiredArgument } )?;

// Kind variants
unilang::data::Kind::String
unilang::data::Kind::Integer
unilang::data::Kind::Boolean
unilang::data::Kind::Path

// Value variants (for matching in wrapper)
unilang::data::Value::String( s )
unilang::data::Value::Integer( i )
unilang::data::Value::Boolean( b )
```

### Trybuild Test Structure

```
unilang_meta/
  tests/
    trybuild.rs           ← test harness (runs all UI tests)
    ui/
      01_basic_command_compiles.rs        ← t.pass(...)
      02_argument_inference_compiles.rs   ← t.pass(...)
      03_wrapper_generation_compiles.rs   ← t.pass(...)
      04_generates_full_definition.rs     ← t.pass(...)
      05_missing_name_fails.rs            ← t.compile_fail(...)
      05_missing_name_fails.stderr        ← expected error output
      06_unsupported_type_fails.rs        ← t.compile_fail(...)
      06_unsupported_type_fails.stderr    ← expected error output
```

`tests/trybuild.rs` pattern:

```rust
#[ test ]
fn ui_tests()
{
  let t = trybuild::TestCases::new();
  t.pass( "tests/ui/01_basic_command_compiles.rs" );
  t.pass( "tests/ui/02_argument_inference_compiles.rs" );
  t.compile_fail( "tests/ui/05_missing_name_fails.rs" );
  // ...
}
```

## Cross-References

- **Primary crate under implementation:** `module/unilang_meta/src/lib.rs`
- **macro_tools API examples:** `~/.cargo/registry/src/.../macro_tools-*/examples/macro_tools_attr_prop.rs`
- **unilang data structures:** `module/unilang/src/data/` (CommandDefinition, ArgumentDefinition, Kind, Value)
- **unilang interpreter types:** `module/unilang/src/interpreter/`, `module/unilang/src/semantic/`
- **Roadmap milestone:** M4.2 — implement_extension_module_macros
