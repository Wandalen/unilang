//! Validation: Direct Import Works
//!
//! This integration test creates a real downstream crate and verifies it can
//! compile and run using only unilang's PHF re-export (no direct PHF dependency).

#[test]
#[cfg(feature = "static_registry")]
fn validation_v1_direct_import_works() {
  use std::process::Command;
  use std::fs;
  use tempfile::TempDir;

  // Create isolated test environment
  let temp = TempDir::new().expect("Failed to create temp directory");
  let crate_path = temp.path();

  println!("Creating test crate at: {:?}", crate_path);

  // Create downstream crate WITHOUT direct PHF dependency
  let cargo_toml = format!(r#"
[package]
name = "test-downstream-v1"
version = "0.1.0"
edition = "2021"

[dependencies]
unilang = {{ path = "{}", features = ["static_registry"] }}
# CRITICAL: No phf dependency - this is the whole point of the test
"#, env!("CARGO_MANIFEST_DIR"));

  fs::write(crate_path.join("Cargo.toml"), cargo_toml)
    .expect("Failed to write Cargo.toml");

  // Create src directory
  fs::create_dir(crate_path.join("src"))
    .expect("Failed to create src directory");

  // Create library code using unilang::phf re-export
  let lib_rs = r#"
// This is the pattern generated code uses
// Import phf module itself (with 'self') so macros work correctly
use unilang::phf::{self, Map};

pub static COMMANDS: Map<&str, u32> = phf::phf_map! {
  "help" => 1,
  "version" => 2,
  "test" => 3,
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_commands_work() {
    assert_eq!(COMMANDS.get("help"), Some(&1));
    assert_eq!(COMMANDS.get("version"), Some(&2));
    assert_eq!(COMMANDS.get("test"), Some(&3));
    assert_eq!(COMMANDS.get("nonexistent"), None);
  }

  #[test]
  fn test_map_properties() {
    assert_eq!(COMMANDS.len(), 3);
    assert!(!COMMANDS.is_empty());
  }
}
"#;

  fs::write(crate_path.join("src/lib.rs"), lib_rs)
    .expect("Failed to write lib.rs");

  // Share target dir across validation tests to cache unilang compilation
  let validation_target = std::env::temp_dir().join("unilang_validation_target");

  // Build the downstream crate
  println!("Building downstream crate...");
  let build_output = Command::new("cargo")
    .args(["build"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo build");

  // Check build succeeded
  if !build_output.status.success() {
    eprintln!("Build stdout:\n{}", String::from_utf8_lossy(&build_output.stdout));
    eprintln!("Build stderr:\n{}", String::from_utf8_lossy(&build_output.stderr));
    panic!("Downstream crate failed to compile without direct PHF dependency!");
  }

  println!("Build succeeded!");

  // Run tests in downstream crate
  println!("Running downstream tests...");
  let test_output = Command::new("cargo")
    .args(["test"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo test");

  // Check tests passed
  if !test_output.status.success() {
    eprintln!("Test stdout:\n{}", String::from_utf8_lossy(&test_output.stdout));
    eprintln!("Test stderr:\n{}", String::from_utf8_lossy(&test_output.stderr));
    panic!("Downstream tests failed!");
  }

  println!("All downstream tests passed!");

  // Verify no phf in dependencies
  let output = Command::new("cargo")
    .args(["tree", "-i", "phf"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo tree");

  let tree_output = String::from_utf8_lossy(&output.stdout);
  println!("Dependency tree for phf:\n{}", tree_output);

  // phf should appear only as transitive via unilang, not as direct dependency
  assert!(tree_output.contains("unilang"), "PHF should come through unilang");

  println!("✅ Validation V1 PASSED: Downstream crate works without direct PHF dependency");
}
