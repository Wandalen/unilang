//! Validation: Clippy Check Passes
//!
//! Verifies that clippy analysis passes with zero warnings on PHF re-export code.

#[test]
fn validation_v6_clippy_passes() {
  use std::process::Command;

  // Run clippy on all targets
  let result = Command::new("cargo")
    .args(["clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])
    .current_dir(env!("CARGO_MANIFEST_DIR"))
    .output()
    .expect("Failed to run cargo clippy");

  let stdout = String::from_utf8_lossy(&result.stdout);
  let stderr = String::from_utf8_lossy(&result.stderr);

  // Check for success (exit code 0 means no warnings/errors with -D warnings)
  assert!(result.status.success(),
    "Clippy should pass with zero warnings\nStdout:\n{}\nStderr:\n{}",
    stdout, stderr);

  println!("✅ Validation V6 PASSED: Clippy analysis passes with zero warnings");
}
