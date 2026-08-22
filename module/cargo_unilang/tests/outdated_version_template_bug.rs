//! Regression test for outdated unilang version in generated Cargo.toml
//!
//! # Root Cause
//!
//! In `src/templates/cargo_toml.rs:21`, the `cargo_toml()` function hardcoded
//! `unilang = "0.33"` in the generated Cargo.toml template. This version was
//! outdated - the actual published version on crates.io is 0.46.0+. When users
//! created new projects with `cargo_unilang .new`, the generated Cargo.toml
//! specified a nonexistent version, causing `cargo build` to fail with error:
//! "failed to select a version for the requirement `unilang = "^0.33"`".
//!
//! The hardcoded version was never updated when unilang was published with
//! newer versions, creating immediate compilation failures for all generated
//! projects.
//!
//! # Why Not Caught
//!
//! No integration test verified that generated projects actually compile.
//! Existing tests in `templates/cargo_toml.rs:45-67` only checked that the
//! template *contained* `unilang = "0.33"` (line 50), but never validated
//! whether that version actually exists on crates.io or whether the generated
//! project can build successfully.
//!
//! The test suite verified string generation but not real-world usability of
//! the generated output.
//!
//! # Fix Applied
//!
//! Updated `src/templates/cargo_toml.rs:21` to use current version "0.46".
//! Changed template from:
//! ```rust
//! unilang = "0.33"
//! ```
//! to:
//! ```rust
//! unilang = "0.46"
//! ```
//!
//! Also updated corresponding assertion in `cargo_toml.rs:50` from checking
//! "0.33" to checking "0.46" to maintain test coverage of template generation.
//!
//! Commit: [TBD], PR: [TBD]
//!
//! # Prevention
//!
//! 1. **Integration testing**: Add tests that verify generated projects compile
//!    successfully, not just that templates contain expected strings
//! 2. **Version synchronization**: Consider reading version from Cargo.toml at
//!    compile time using `env!("CARGO_PKG_VERSION_MAJOR")` or similar mechanism
//! 3. **CI validation**: Run `cargo build` on generated test projects in CI
//! 4. **Version range**: Use `unilang = "0.46"` instead of exact version to
//!    allow future 0.x updates without template changes
//! 5. **Regular audits**: Check template versions match published crate versions
//!    during pre-release checklist
//!
//! # Pitfall
//!
//! Similar pattern may exist in other scaffolding tools in wTools workspace.
//! Audit all template generators for hardcoded version strings. Any tool
//! generating Cargo.toml files should either use workspace-relative versions,
//! read versions from build environment, or have automated tests verifying
//! specified versions exist on crates.io.
//!
//! Watch for this pattern:
//! - Hardcoded version strings in template generation code
//! - Tests that validate string presence without validating correctness
//! - Missing integration tests that verify generated artifacts work

use assert_fs::prelude::*;

/// Verify generated Cargo.toml specifies a version that exists on crates.io
///
/// This test ensures `cargo_unilang .new` creates projects that can actually
/// compile. It checks that the unilang version in the generated Cargo.toml
/// matches a published version (currently 0.46.x).
#[cfg_attr(test, test)]
fn generated_cargo_toml_uses_current_unilang_version()
{
  let temp = assert_fs::TempDir::new().unwrap();

  // Create a new project
  assert_cmd::cargo::cargo_bin_cmd!( "cargo_unilang" )
    .arg( ".new" )
    .arg( "project::version-test" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success()
    .code( 0 );

  // Read generated Cargo.toml
  let cargo_toml_path = temp.child( "version-test/Cargo.toml" );
  cargo_toml_path.assert( predicates::path::exists() );

  let cargo_toml_content = std::fs::read_to_string( cargo_toml_path.path() ).unwrap();

  // Verify unilang version is 0.46 (current published version)
  // NOT 0.33 (nonexistent version that causes build failures)
  assert!(
    cargo_toml_content.contains( "unilang = \"0.46\"" ),
    "Generated Cargo.toml should specify unilang version 0.46 (current published version), \
     but contains: {}",
    cargo_toml_content
      .lines()
      .find( |line| line.contains( "unilang" ) && !line.trim().starts_with( '#' ) )
      .unwrap_or( "[unilang dependency line not found]" )
  );

  // Verify the old broken version is NOT present
  assert!(
    !cargo_toml_content.contains( "unilang = \"0.33\"" ),
    "Generated Cargo.toml should NOT contain outdated version 0.33 (doesn't exist on crates.io)"
  );
}

/// Verify generated projects can at least resolve dependencies
///
/// This test runs `cargo metadata` on the generated project to verify:
/// 1. Cargo.toml is valid TOML
/// 2. Specified dependencies exist and can be resolved
/// 3. Version specifications are valid
///
/// Note: This doesn't do a full `cargo build` (too slow for unit tests),
/// but does verify dependency resolution which catches nonexistent versions.
// DISABLED: predates permission system (discovered 2026-07-16, disabled before that)
// REASON: requires live network access to crates.io to resolve dependencies; unsuitable for default/sandboxed/offline test runs
// RE-ENABLE: N/A — permanent, by-design opt-in via `cargo test -- --ignored` when live network access is available
// APPROVED: n/a (pre-existing; predates the permission workflow)
// TRACKING: unilang task 008 (undocumented_ignore_network_dependency_test)
#[ignore]
#[cfg_attr(test, test)]
fn generated_project_dependencies_resolve()
{
  let temp = assert_fs::TempDir::new().unwrap();

  // Create a new project
  assert_cmd::cargo::cargo_bin_cmd!( "cargo_unilang" )
    .arg( ".new" )
    .arg( "project::dep-test" )
    .arg( "verbosity::0" )
    .current_dir( &temp )
    .assert()
    .success();

  // Run cargo metadata to verify dependencies resolve
  // (This downloads crate metadata but doesn't compile, so it's faster than full build)
  let metadata_result = std::process::Command::new( "cargo" )
    .arg( "metadata" )
    .arg( "--format-version" )
    .arg( "1" )
    .current_dir( temp.child( "dep-test" ).path() )
    .output()
    .expect( "Failed to run cargo metadata" );

  assert!(
    metadata_result.status.success(),
    "cargo metadata should succeed for generated project. \
     This failure indicates dependency resolution failed, likely due to \
     nonexistent version in Cargo.toml. Stderr: {}",
    String::from_utf8_lossy( &metadata_result.stderr )
  );
}
