//! Compile-fail test runner for type-state and private-field enforcement.
//!
//! ## Scope
//! Verifies that the type-state builder and private field constraints are
//! enforced at compile time by the Rust compiler. Each fixture in
//! `tests/compile_fail/` must fail to compile.
//!
//! ## Coverage
//! - T40: Builder without `.name()` does not compile (type-state enforcement) — also satisfies AP-1 and IN-2
//! - T50: Direct `name` field access does not compile (private field) — also satisfies AP-5
//! - T50b: Direct `description` field access does not compile (private field)
//!
//! ## Related
//! - Fixture files: `tests/compile_fail/t40_*.rs`, `t50_*.rs`, `t50b_*.rs`
//! - Builder source: `src/data/command_definition/builder.rs`
//! - Core source: `src/data/command_definition/core.rs`

/// T40/T50: Type-state builder and private field compile-fail verification.
///
/// Runs each fixture through trybuild to confirm it is rejected by rustc.
// test_kind: tc_spec(T40), tc_spec(T50), ap_spec(AP-1), ap_spec(AP-5), in_spec(IN-2)
#[ test ]
fn test_tc_compile_fail_type_state_and_private_fields()
{
  let t = trybuild::TestCases::new();
  t.compile_fail( "tests/compile_fail/t40_builder_missing_name.rs" );
  t.compile_fail( "tests/compile_fail/t50_private_field_name.rs" );
  t.compile_fail( "tests/compile_fail/t50b_private_field_description.rs" );
}
