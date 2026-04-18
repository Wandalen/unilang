//! Validation: ABI Compatibility
//!
//! Verifies that types from unilang::phf are ABI-compatible with direct phf types.
//! This ensures no type incompatibilities when mixing code using different import paths.

#[test]
#[cfg(feature = "static_registry")]
fn validation_v4_abi_compatibility() {
  use unilang::phf::{self, Map};

  // Create a Map using re-exported types
  static TEST_MAP: Map<&str, i32> = phf::phf_map! {
    "key1" => 100,
    "key2" => 200,
  };

  // Function that accepts Map reference
  fn process_map(map: &Map<&str, i32>) -> Option<i32> {
    map.get("key1").copied()
  }

  // Function that returns Map reference
  fn get_map() -> &'static Map<&'static str, i32> {
    &TEST_MAP
  }

  // Test: Pass re-exported Map to function
  let result1 = process_map(&TEST_MAP);
  assert_eq!(result1, Some(100), "Function parameter should work");

  // Test: Return re-exported Map from function
  let result2 = get_map();
  assert_eq!(result2.get("key2"), Some(&200), "Function return should work");

  // Test: Type compatibility - verify we can use reference types normally
  let map_ref: &Map<&str, i32> = &TEST_MAP;
  assert_eq!(map_ref.len(), 2, "Map reference should work correctly");

  // Test: Iterator compatibility
  let keys: Vec<&&str> = TEST_MAP.keys().collect();
  assert_eq!(keys.len(), 2, "Iterator should work correctly");

  // Test: Map is Send + Sync (compile-time check)
  fn assert_send_sync<T: Send + Sync>() {}
  assert_send_sync::<Map<&str, i32>>();

  println!("✅ Validation V4 PASSED: ABI compatibility verified");
}

#[test]
#[cfg(feature = "static_registry")]
fn validation_v4_cross_crate_types() {
  use std::process::Command;
  use std::fs;
  use tempfile::TempDir;

  // Create downstream crate that defines a Map
  let temp = TempDir::new().expect("Failed to create temp directory");
  let crate_path = temp.path();

  let cargo_toml = format!(r#"
[package]
name = "test-downstream-v4"
version = "0.1.0"
edition = "2021"

[dependencies]
unilang = {{ path = "{}", features = ["static_registry"] }}

[lib]
crate-type = ["lib"]
"#, env!("CARGO_MANIFEST_DIR"));

  fs::write(crate_path.join("Cargo.toml"), cargo_toml)
    .expect("Failed to write Cargo.toml");

  fs::create_dir(crate_path.join("src"))
    .expect("Failed to create src directory");

  // Code that tests cross-function type compatibility
  let lib_rs = r#"
use unilang::phf::{self, Map};

pub static GLOBAL_MAP: Map<&str, u32> = phf::phf_map! {
  "test" => 42,
};

pub fn get_value(map: &Map<&str, u32>, key: &str) -> Option<u32> {
  map.get(key).copied()
}

pub fn use_global() -> Option<u32> {
  get_value(&GLOBAL_MAP, "test")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cross_function_abi() {
    // Pass static map to function
    let result = get_value(&GLOBAL_MAP, "test");
    assert_eq!(result, Some(42));

    // Use through wrapper
    let result2 = use_global();
    assert_eq!(result2, Some(42));
  }
}
"#;

  fs::write(crate_path.join("src/lib.rs"), lib_rs)
    .expect("Failed to write lib.rs");

  // Build and test
  let build_result = Command::new("cargo")
    .args(["test"])
    .current_dir(crate_path)
    .output()
    .expect("Failed to execute cargo test");

  assert!(build_result.status.success(),
    "Cross-crate ABI compatibility test should pass");

  println!("✅ Validation V4 PASSED: Cross-crate type compatibility verified");
}
