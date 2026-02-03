//! Cargo.toml template generator

/// Generate Cargo.toml content for unilang project
pub fn cargo_toml( project_name : &str, author : Option< &str >, license : Option< &str > ) -> String
{
  let author_line = author
    .map( |a| format!( "authors = [ \"{}\" ]\n", a ) )
    .unwrap_or_default();

  let license_name = license.unwrap_or( "MIT" );

  format!(
r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"
{author_line}license = "{license_name}"

[dependencies]
# Unilang with default features (Approach #2: Multi-YAML Build-Time Static)
# Fix(outdated-version): Version updated from 0.33 to 0.46 to match published crates.io version
# Root cause: Hardcoded version string was never updated when unilang published newer versions,
# causing all generated projects to fail compilation with "failed to select a version" error
# Pitfall: Any scaffolding tool with hardcoded versions needs tests verifying generated artifacts
# compile successfully, not just that templates contain expected strings. Consider reading version
# from Cargo.toml at compile time to prevent future desynchronization.
unilang = "0.46"

# ⚠️  IMPORTANT: Do NOT add these - unilang already provides them:
# ❌ serde_yaml (via yaml_parser feature)
# ❌ walkdir (via multi_file feature)
# ❌ phf (via static_registry feature)
#
# ⚠️  IMPORTANT: Do NOT create build.rs
# Unilang already provides build.rs that handles everything.
#
# If you see warnings during cargo build, that's unilang working!
# It's processing your YAML files at compile-time.
"#,
    project_name = project_name,
    author_line = author_line,
    license_name = license_name
  )
}
