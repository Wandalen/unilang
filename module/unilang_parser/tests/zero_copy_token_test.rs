//! Tests for zero-copy token implementation.
//!
//! ## Root Cause
//!
//! Token creation at initial tokenization called `classify_split()` which internally
//! called `.to_owned()` on every token, allocating a new `String` per Identifier/Number/Unrecognized
//! token. With 5-15 tokens per command, this was 40-60% of parsing hot path time.
//!
//! ## Why Not Caught
//!
//! All token-related tests checked BEHAVIOR (parse results) not HOT-PATH ALLOCATION PATTERNS.
//! No test verified that the token classification path avoided `String` allocations.
//!
//! ## Fix Applied
//!
//! Changed `ZeroCopyTokenKind<'a>` to use `Cow<'a, str>` (was `&'a str`) to handle
//! both borrowed tokens (from input) and synthetic tokens (merge_value_context).
//! Changed `RichItem<'a>.kind` from `UnilangTokenKind` to `ZeroCopyTokenKind<'a>`.
//! The initial tokenization now stores `Cow::Borrowed` (zero-copy) for input tokens;
//! only synthetic tokens use `Cow::Owned`.
//!
//! ## Prevention
//!
//! Assert that parser_engine/mod.rs uses `ZeroCopyTokenKind` (not `UnilangTokenKind`)
//! for its internal token matching — any revert to owned types will fail this test.
//!
//! ## Pitfall
//!
//! `Cow<'a, str>` in match arms requires `.as_ref()` to get `&str`. Pattern matching
//! on `Cow` requires `ref` binding: `ZeroCopyTokenKind::Identifier(ref s)` gives `s: &Cow<'a, str>`.
//! Use `s.as_ref()` or `&**s` to get `&str`. Do not use `s.clone()` for owned value extraction;
//! use `s.into_owned()` instead.

/// bug_reproducer(parser-001)
///
/// Verifies that the parser engine hot path uses `ZeroCopyTokenKind` (no `String` allocation
/// per token) rather than `UnilangTokenKind` (allocates `String` per token).
///
/// RED state (pre-fix): parser_engine imports and matches on `UnilangTokenKind` (owned String).
/// GREEN state (post-fix): parser_engine imports and matches on `ZeroCopyTokenKind` (Cow<str>).
#[ test ]
fn parser_engine_hot_path_uses_zero_copy_token_kind()
{
  let source = include_str!("../src/parser_engine/mod.rs");

  // Must NOT use the allocating owned type in hot path
  assert!(
    !source.contains("UnilangTokenKind"),
    "parser_engine/mod.rs must not use UnilangTokenKind (allocates String per token);\
     use ZeroCopyTokenKind (Cow<str>) instead"
  );

  // Must use the zero-copy type
  assert!(
    source.contains("ZeroCopyTokenKind"),
    "parser_engine/mod.rs must use ZeroCopyTokenKind for token matching"
  );
}
