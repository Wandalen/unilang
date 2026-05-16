//! Validation: Documentation Examples Compile
//!
//! Verifies that all documentation examples in readme.md and src/lib.rs compile correctly.
//! This is tested via `cargo test --doc` which runs all doc tests.

#[test]
fn validation_v5_doc_tests_pass() {
  use std::process::Command;

  // Run doc tests
  let result = Command::new("cargo")
    .args(["test", "--doc", "--all-features"])
    .current_dir(env!("CARGO_MANIFEST_DIR"))
    .output()
    .expect("Failed to run cargo test --doc");

  let stdout = String::from_utf8_lossy(&result.stdout);
  let stderr = String::from_utf8_lossy(&result.stderr);

  // Check for success
  assert!(result.status.success(),
    "Doc tests should pass\nStdout:\n{}\nStderr:\n{}",
    stdout, stderr);

  // Verify doc tests actually ran (not skipped)
  assert!(stdout.contains("Doc-tests unilang") || stdout.contains("test result: ok"),
    "Doc tests should have run");

  println!("✅ Validation V5 PASSED: All documentation examples compile and pass");
}
