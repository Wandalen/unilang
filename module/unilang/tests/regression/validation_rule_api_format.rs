//! Regression test for validation rule API format bugs (BUG-095).
//!
//! ## Root Cause
//!
//! Example `07_yaml_json_loading.rs` used incorrect validation rule keywords in embedded
//! YAML/JSON: `min_length:1` instead of `minlength:1`, and `regex:^[0-9]` instead of
//! `pattern:^[0-9]`. `ValidationRule::from_str()` expects specific keywords without
//! underscores (`minlength`, `maxlength`, `pattern`), but examples used intuitive
//! English variants that the parser doesnt recognize.
//!
//! ## Why Not Caught
//!
//! No test parsed the exact YAML strings from the example. Example code was reviewed
//! for structure but not executed end-to-end with the embedded validation rules.
//!
//! ## Fix Applied
//!
//! Corrected all validation rule keywords in `07_yaml_json_loading.rs` to match the
//! parser's expected format: `min_length` → `minlength`, `regex` → `pattern`.
//!
//! ## Prevention
//!
//! These tests validate that the parser rejects intuitive-but-wrong formats and accepts
//! the canonical keywords. Any new validation rule must have both accepted/rejected tests.
//!
//! ## Pitfall
//!
//! Validation rule keywords dont follow Rust snake_case convention (`minlength` not
//! `min_length`). Users naturally write snake_case variants, which silently fail with
//! "Unknown validation rule" — the error message doesnt suggest the correct format.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]

use unilang::ValidationRule;

/// Reproduces validation rule API bug where underscore format is rejected.
///
/// ## Root Cause
///
/// In `examples/07_yaml_json_loading.rs:203`, embedded YAML used:
/// ```yaml
/// validation_rules: ["min_length:1"]
/// ```
///
/// When parsed, `ValidationRule::from_str("min_length:1")` in `src/argument/types.rs`
/// checks keyword against:
/// ```rust,ignore
/// "minlength" => { /* parse */ }
/// "maxlength" => { /* parse */ }
/// "pattern" => { /* parse */ }
/// _ => Err(format!("Unknown validation rule: {}", s))
/// ```
///
/// The keyword `"min_length"` doesn't match `"minlength"`, so parser returns error.
/// This is non-intuitive because:
/// 1. Rust style guide uses snake_case (`min_length`)
/// 2. YAML keys in same file use underscores (`validation_rules`)
/// 3. Other languages use underscores (Python: `min_length`, JSON Schema: `minLength`)
///
/// The parser chose compact format (`minlength`) over conventional format (`min_length`),
/// but didn't document this clearly or provide helpful error messages.
///
/// ## Why Not Caught Initially
///
/// Unit tests for `ValidationRule::from_str()` use correct format strings. Integration
/// tests loading YAML likely use valid format. No test coverage for "reasonable but wrong"
/// formats that users would naturally try.
///
/// Examples were written before validation rule testing was comprehensive. The YAML syntax
/// looked correct visually - only runtime parsing revealed the mismatch.
///
/// ## Fix Applied
///
/// Updated `examples/07_yaml_json_loading.rs`:
/// - Line 203: `"min_length:1"` → `"minlength:1"`
/// - Line 291: `"regex:^[0-9]+\\.[0-9]+\\.[0-9]+.*$"` → `"pattern:^[0-9]+\\.[0-9]+\\.[0-9]+.*$"`
///
/// Both fixes align with actual parser keywords while preserving validation semantics.
///
/// ## Prevention
///
/// 1. **Support aliases:** Accept both `min_length` and `minlength`, map to same rule
/// 2. **Better errors:** Parse keyword variants and suggest correct format in error
/// 3. **API documentation:** Explicitly list accepted keywords with examples
/// 4. **Type-safe API:** Rust builder pattern instead of string parsing (`.min_length(1)`)
///
/// ## Pitfall to Avoid
///
/// When designing string-based APIs, expect users to:
/// 1. Follow conventions from other languages/tools
/// 2. Match naming style of surrounding code (snake_case YAML keys → snake_case rules)
/// 3. Use descriptive terms (`regex` more specific than `pattern`)
///
/// Either accept common variants or provide clear error messages pointing to correct format.
/// Rejecting `min_length` with generic "Unknown validation rule" is poor UX.
// test_kind: bug_reproducer(BUG-095)
#[ test ]
fn test_min_length_underscore_rejected()
{
  // Reproduce the bug: underscore format should be rejected

  let result = "min_length:5".parse::< ValidationRule >();

  assert!(
    result.is_err(),
    "Underscore format min_length should be rejected by parser"
  );

  let error = result.unwrap_err().to_string();
  assert!(
    error.contains( "Unknown validation rule" ),
    "Error should mention unknown rule, got: {}",
    error
  );
}

#[ test ]
fn test_minlength_no_underscore_accepted()
{
  // Verify correct format is accepted

  let result = "minlength:5".parse::< ValidationRule >();

  assert!(
    result.is_ok(),
    "Correct format minlength (no underscore) should be accepted, got error: {:?}",
    result.err()
  );

  // Verify the parsed rule has correct value
  if let Ok( ValidationRule::MinLength( n ) ) = result
  {
    assert_eq!( n, 5, "MinLength should parse value correctly" );
  }
  else
  {
    panic!( "Expected ValidationRule::MinLength, got: {:?}", result );
  }
}

/// Reproduces validation rule API bug where 'regex' keyword is rejected in favor of 'pattern'.
///
/// ## Root Cause
///
/// In `examples/07_yaml_json_loading.rs:291`, embedded JSON used:
/// ```json
/// "validation_rules": ["regex:^[0-9]+\\.[0-9]+\\.[0-9]+.*$"]
/// ```
///
/// Parser expects `pattern:` keyword, not `regex:`. The choice of `pattern` over `regex`:
/// - Pros: More general (can be glob, SQL LIKE, etc.)
/// - Cons: Less specific (users expect regex specifically for regex patterns)
///
/// Most users coming from regex backgrounds will try `regex:` first, then get confusing error.
///
/// ## Fix Applied
///
/// Updated example line 291: `"regex:..."` → `"pattern:..."`
///
/// ## Prevention
///
/// 1. **Alias support:** Accept both `regex:` and `pattern:` as synonyms
/// 2. **Clear errors:** "Unknown rule 'regex:'. Did you mean 'pattern:'?"
/// 3. **API naming:** Consider renaming to `regex:` for clarity if only regex supported
///
/// ## Pitfall to Avoid
///
/// Generic names (`pattern`) are flexible but ambiguous. Specific names (`regex`) are clear
/// but limiting. If implementation only supports regex, using `regex` is more honest. If
/// future expansion planned, `pattern` is better but needs good documentation.
// test_kind: bug_reproducer(BUG-095)
#[ test ]
fn test_regex_keyword_rejected()
{
  // Reproduce the bug: 'regex:' keyword should be rejected

  let result = "regex:^[0-9]+$".parse::< ValidationRule >();

  assert!(
    result.is_err(),
    "Keyword 'regex' should be rejected by parser (expects 'pattern')"
  );

  let error = result.unwrap_err().to_string();
  assert!(
    error.contains( "Unknown validation rule" ),
    "Error should mention unknown rule, got: {}",
    error
  );
}

#[ test ]
fn test_pattern_keyword_accepted()
{
  // Verify correct keyword is accepted

  let result = "pattern:^[0-9]+$".parse::< ValidationRule >();

  assert!(
    result.is_ok(),
    "Correct keyword 'pattern' should be accepted, got error: {:?}",
    result.err()
  );

  // Verify the parsed rule has correct pattern
  if let Ok( ValidationRule::Pattern( regex ) ) = result
  {
    assert_eq!( regex.as_str(), "^[0-9]+$", "Pattern should parse regex correctly" );
  }
  else
  {
    panic!( "Expected ValidationRule::Pattern, got: {:?}", result );
  }
}

/// Verifies that multiple validation rules can be parsed correctly.
#[ test ]
fn test_multiple_validation_rules()
{
  // Test that correctly formatted validation rules parse successfully

  let min_rule: Result< ValidationRule, _ > = "minlength:1".parse();
  assert!( min_rule.is_ok(), "minlength rule should parse" );

  let pattern_rule: Result< ValidationRule, _ > = "pattern:^[0-9]+\\.[0-9]+\\.[0-9]+.*$".parse();
  assert!( pattern_rule.is_ok(), "pattern rule should parse" );
}

/// Tests that maxlength follows same format (no underscore).
#[ test ]
fn test_maxlength_format()
{
  // Verify maxlength uses same compact format

  let result_wrong = "max_length:10".parse::< ValidationRule >();
  assert!(
    result_wrong.is_err(),
    "Underscore format max_length should be rejected"
  );

  let result_correct = "maxlength:10".parse::< ValidationRule >();
  assert!(
    result_correct.is_ok(),
    "Correct format maxlength (no underscore) should be accepted"
  );

  if let Ok( ValidationRule::MaxLength( n ) ) = result_correct
  {
    assert_eq!( n, 10 );
  }
}
