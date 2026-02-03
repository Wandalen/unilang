//! Manual Test: Wildcard Import Pattern
//!
//! Tests that wildcard imports work correctly with PHF re-export.

#[test]
#[cfg(feature = "static_registry")]
fn test_wildcard_import_pattern() {
  // Test wildcard import
  use unilang::phf::*;

  // Should be able to use types and macros
  static TEST_MAP: Map<&str, i32> = phf_map! {
    "a" => 1,
    "b" => 2,
  };

  static TEST_SET: Set<&str> = phf_set! {
    "x", "y", "z"
  };

  // Verify Map works
  assert_eq!(TEST_MAP.get("a"), Some(&1));
  assert_eq!(TEST_MAP.len(), 2);

  // Verify Set works
  assert!(TEST_SET.contains("x"));
  assert!(!TEST_SET.contains("w"));
  assert_eq!(TEST_SET.len(), 3);

  println!("✅ Wildcard import pattern works correctly");
}
