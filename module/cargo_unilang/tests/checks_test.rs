//!
//! Tests for check validation functions
//!

use cargo_unilang::checks::{ check_deprecated_api, check_build_rs, check_duplicate_deps };
use assert_fs::prelude::*;

// Tests for API checks
#[test]
fn test_no_deprecated_api_passes()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let src = temp.child( "src" );
  src.create_dir_all().unwrap();
  src.child( "main.rs" ).write_str(
r"use unilang::prelude::*;
fn main() {
  let registry = CommandRegistry::with_static_commands();
}
"
  ).unwrap();

  let result = check_deprecated_api( temp.path() );
  assert!( result.passed );
}

#[test]
fn test_deprecated_new_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let src = temp.child( "src" );
  src.create_dir_all().unwrap();
  src.child( "main.rs" ).write_str(
r"use unilang::prelude::*;
fn main() {
  let registry = CommandRegistry::new();
}
"
  ).unwrap();

  let result = check_deprecated_api( temp.path() );
  assert!( !result.passed );
  assert!( result.issue_type.contains( "Deprecated API" ) );
  assert!( result.fix.contains( "with_static_commands" ) );
}

#[test]
fn test_check_in_subdirectory()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let src = temp.child( "src/commands" );
  src.create_dir_all().unwrap();
  src.child( "greet.rs" ).write_str( "pub fn greet() {}" ).unwrap();

  let result = check_deprecated_api( temp.path() );
  assert!( result.passed );
}

// Tests for build.rs checks
#[test]
fn test_no_build_rs_passes()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let result = check_build_rs( temp.path() );
  assert!( result.passed );
}

#[test]
fn test_build_rs_with_yaml_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { let yaml = serde_yaml_ng::from_str(); }" ).unwrap();

  let result = check_build_rs( temp.path() );
  assert!( !result.passed );
  assert!( result.issue_type.contains( "Custom build.rs" ) );
}

#[test]
fn test_build_rs_with_phf_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { phf_codegen::Map::new(); }" ).unwrap();

  let result = check_build_rs( temp.path() );
  assert!( !result.passed );
}

#[test]
fn test_build_rs_without_unilang_stuff_passes()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "build.rs" ).write_str( "fn main() { println!(\"cargo:rerun-if-changed=build.rs\"); }" ).unwrap();

  let result = check_build_rs( temp.path() );
  assert!( result.passed );
}

// Tests for dependency checks
#[test]
fn test_no_duplicates_passes()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
unilang = "0.33"
"#
  ).unwrap();

  let result = check_duplicate_deps( temp.path() );
  assert!( result.passed );
}

#[test]
fn test_serde_yaml_ng_duplicate_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[package]
name = "test"
version = "0.1.0"

[dependencies]
unilang = "0.33"
serde_yaml_ng = "0.10"
"#
  ).unwrap();

  let result = check_duplicate_deps( temp.path() );
  assert!( !result.passed );
  assert!( result.issue.contains( "serde_yaml_ng" ) );
}

#[test]
fn test_multiple_duplicates_fails()
{
  let temp = assert_fs::TempDir::new().unwrap();
  temp.child( "Cargo.toml" ).write_str(
r#"[package]
name = "test"
version = "0.1.0"

[dependencies]
unilang = "0.33"
serde_yaml_ng = "0.10"
phf = "0.11"
"#
  ).unwrap();

  let result = check_duplicate_deps( temp.path() );
  assert!( !result.passed );
  assert!( result.issue.contains( "serde_yaml_ng" ) );
  assert!( result.issue.contains( "phf" ) );
}
