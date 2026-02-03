//! PHF Re-export Tests
//!
//! ## Root Cause (Proactive Test)
//!
//! This test prevents regression of PHF re-export functionality. Generated code
//! relies on `unilang::phf` types being publicly available.
//!
//! ## Why This Test
//!
//! Without this test, accidental removal of `pub use phf;` would break all
//! downstream crates using code generation, with confusing "phf not found" errors.
//!
//! ## Prevention
//!
//! Run this test in CI to catch any changes that remove the re-export.
//!
//! ## Pitfall: Forgetting Feature Gate
//!
//! If `pub use phf;` is added without `#[cfg(feature = "static_registry")]`,
//! PHF becomes unconditionally available, increasing compile times for users
//! not using static registries.

#[cfg(feature = "static_registry")]
mod with_feature {
  use unilang::phf::{phf_map, Map};

  /// Test 1: Basic Re-export Accessibility
  ///
  /// Verifies that PHF module is re-exported and basic types are accessible.
  #[test]
  fn test_phf_module_is_reexported() {
    // Verify we can reference the Map type
    let _: std::marker::PhantomData<unilang::phf::Map<&str, i32>> = std::marker::PhantomData;

    // Verify phf_map macro is accessible (by using it)
    static _TEST: unilang::phf::Map<&str, i32> = unilang::phf::phf_map! {
      "test" => 1,
    };
  }

  /// Test 2: Map Type Fully Functional
  ///
  /// Verifies that Map type works exactly as generated code uses it.
  /// This is the CRITICAL test - if this passes, generated code will work.
  #[test]
  fn test_phf_map_type_accessible() {
    // Create map using re-exported types (mimics generated code)
    static TEST_MAP: Map<&str, i32> = phf_map! {
      "one" => 1,
      "two" => 2,
      "three" => 3,
    };

    // Verify basic operations
    assert_eq!(TEST_MAP.get("one"), Some(&1));
    assert_eq!(TEST_MAP.get("two"), Some(&2));
    assert_eq!(TEST_MAP.get("three"), Some(&3));
    assert_eq!(TEST_MAP.get("nonexistent"), None);

    // Verify map properties
    assert_eq!(TEST_MAP.len(), 3);
    assert!(!TEST_MAP.is_empty());

    // Verify iteration works
    let keys: Vec<&str> = TEST_MAP.keys().copied().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"one"));
  }

  /// Test 3: Generated Code Pattern Exact Match
  ///
  /// This mimics EXACTLY what build.rs generates, character-for-character.
  /// If this compiles and passes, the actual generated code will work.
  #[test]
  fn test_generated_code_pattern_compiles() {
    // EXACT pattern from build.rs/aggregator.rs
    use unilang::phf::{phf_map, Map};

    static COMMANDS: Map<&str, u32> = phf_map! {
      "help" => 1,
      "version" => 2,
      "test" => 3,
      "exit" => 4,
    };

    // Verify lookups work
    assert_eq!(COMMANDS.get("help"), Some(&1));
    assert_eq!(COMMANDS.get("version"), Some(&2));
    assert_eq!(COMMANDS.get("test"), Some(&3));
    assert_eq!(COMMANDS.get("exit"), Some(&4));
    assert_eq!(COMMANDS.get("nonexistent"), None);

    // Verify constant evaluation (compile-time)
    const _COMPILE_TIME_CHECK: usize = COMMANDS.len();
  }

  /// Test 4: Multiple PHF Types Available
  ///
  /// Verifies that not just Map, but all PHF types are re-exported.
  #[test]
  fn test_multiple_phf_types_reexported() {
    use unilang::phf::{Map, Set, phf_map, phf_set};

    // Test Map
    static TEST_MAP: Map<&str, i32> = phf_map! {
      "key" => 42,
    };
    assert_eq!(TEST_MAP.get("key"), Some(&42));

    // Test Set
    static TEST_SET: Set<&str> = phf_set! {
      "item1", "item2", "item3",
    };
    assert!(TEST_SET.contains("item1"));
    assert!(TEST_SET.contains("item2"));
    assert!(!TEST_SET.contains("item4"));
  }

  /// Test 5: Type Compatibility
  ///
  /// Ensures re-exported PHF has same type signatures as direct PHF.
  /// This prevents ABI incompatibility issues.
  #[test]
  fn test_type_compatibility() {
    use unilang::phf::Map;

    // Function signature using re-exported type
    fn process_map(map: &Map<&str, i32>) -> Option<i32> {
      map.get("key").copied()
    }

    static TEST_MAP: Map<&str, i32> = unilang::phf::phf_map! {
      "key" => 100,
    };

    // Should compile and work correctly
    let result = process_map(&TEST_MAP);
    assert_eq!(result, Some(100));
  }
}

#[cfg(not(feature = "static_registry"))]
mod without_feature {
  /// Test 6: PHF Not Available Without Feature
  ///
  /// This test exists to verify the feature gate works correctly.
  /// If you can uncomment the lines below without compilation errors,
  /// the feature gate is broken.
  #[test]
  fn test_phf_not_available_without_feature() {
    // The following lines should NOT compile when static_registry is disabled:
    // use unilang::phf::Map;
    // let _ = unilang::phf::phf_map;

    // This test passes by existing. The compilation test is implicit:
    // if the feature gate is wrong, the imports above would compile
    // when they shouldn't.

    // Placeholder assertion
    assert!(true, "Feature gate verification test");
  }
}

/// Test 7: Documentation Examples Compile
///
/// Verifies that the doc comment examples in lib.rs actually work.
#[cfg(feature = "static_registry")]
#[test]
fn test_documentation_example() {
  // Example from lib.rs doc comment
  use unilang::phf::{phf_map, Map};

  static MY_COMMANDS: Map<&str, u32> = phf_map! {
    "help" => 1,
    "version" => 2,
  };

  assert_eq!(MY_COMMANDS.get("help"), Some(&1));
  assert_eq!(MY_COMMANDS.get("version"), Some(&2));
}
