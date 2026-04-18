//! Manual Test: Documentation Examples
//!
//! Verifies that all code examples from readme.md actually compile and work.

#[test]
#[cfg(feature = "static_registry")]
fn test_basic_usage_example() {
  // Exact code from readme.md usage section
  use unilang::phf::{self, Map};

  static COMMANDS: Map<&str, u32> = phf::phf_map! {
    "help" => 1,
    "version" => 2,
  };

  assert_eq!(COMMANDS.get("help"), Some(&1));
  assert_eq!(COMMANDS.get("version"), Some(&2));

  println!("✅ Basic usage example works");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_migration_after_example() {
  // Exact code from "After" migration example
  use unilang::phf::{self, Map};

  static MAP: Map<&str, i32> = phf::phf_map! { "key" => 1 };

  assert_eq!(MAP.get("key"), Some(&1));

  println!("✅ Migration 'after' example works");
}

#[test]
#[cfg(feature = "static_registry")]
fn test_troubleshooting_example() {
  // Test the recommended pattern from troubleshooting section
  use unilang::phf::{self, Map};

  static TEST: Map<&str, i32> = phf::phf_map! {
    "test" => 42,
  };

  assert_eq!(TEST.get("test"), Some(&42));

  println!("✅ Troubleshooting example pattern works");
}
