//! Validation: No Direct PHF Dependency Required
//!
//! Verifies that cargo tree shows PHF only as transitive dependency through unilang,
//! never as a direct dependency.

#[test]
#[cfg(feature = "static_registry")]
fn validation_v2_no_direct_phf_dependency() {
  use std::process::Command;
  use std::fs;
  use tempfile::TempDir;

  // Create isolated test environment
  let temp = TempDir::new().expect("Failed to create temp directory");
  let crate_path = temp.path();

  // Create downstream crate WITHOUT direct PHF dependency
  let cargo_toml = format!(r#"
[package]
name = "test-downstream-v2"
version = "0.1.0"
edition = "2021"

[dependencies]
unilang = {{ path = "{}", features = ["static_registry"] }}
# Explicitly NO phf dependency
"#, env!("CARGO_MANIFEST_DIR"));

  fs::write(crate_path.join("Cargo.toml"), cargo_toml)
    .expect("Failed to write Cargo.toml");

  fs::create_dir(crate_path.join("src"))
    .expect("Failed to create src directory");

  let lib_rs = r#"
use unilang::phf::{self, Map};

pub static TEST: Map<&str, i32> = phf::phf_map! {
  "key" => 42,
};
"#;

  fs::write(crate_path.join("src/lib.rs"), lib_rs)
    .expect("Failed to write lib.rs");

  // Share target dir across validation tests to cache unilang compilation
  let validation_target = std::env::temp_dir().join("unilang_validation_target");

  // Build to ensure dependencies are resolved
  let build_result = Command::new("cargo")
    .args(["build"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo build");

  assert!(build_result.status.success(), "Build must succeed");

  // Check dependency tree - phf should NOT be a direct dependency
  let tree_output = Command::new("cargo")
    .args(["tree", "--depth=1"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo tree");

  let tree_str = String::from_utf8_lossy(&tree_output.stdout);

  // Verify phf is NOT listed as direct dependency at depth 1
  let direct_deps: Vec<&str> = tree_str
    .lines()
    .filter(|line| line.starts_with("├──") || line.starts_with("└──"))
    .collect();

  let has_direct_phf = direct_deps.iter().any(|dep| dep.contains("phf "));

  assert!(!has_direct_phf,
    "PHF should NOT be a direct dependency!\nDirect dependencies:\n{}",
    direct_deps.join("\n"));

  // But verify phf IS present in full tree (as transitive)
  let full_tree = Command::new("cargo")
    .args(["tree"])
    .current_dir(crate_path)
    .env("CARGO_TARGET_DIR", &validation_target)
    .output()
    .expect("Failed to execute cargo tree");

  let full_tree_str = String::from_utf8_lossy(&full_tree.stdout);

  assert!(full_tree_str.contains("phf "),
    "PHF should be present as transitive dependency");

  println!("✅ Validation V2 PASSED: PHF is transitive only, not direct dependency");
}
