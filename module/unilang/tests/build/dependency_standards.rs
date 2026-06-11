//! Workspace dependency format standards tests.
//!
//! Implements IN-1..3 specification cases from `tests/docs/invariant/04_workspace_dependency_standards.md`.
//!
//! Tests verify that all four dependency format rules (R1–R3) defined in
//! `docs/invariant/004_workspace_dependency_standards.md` hold in the workspace Cargo files.
//!
//! ## Coverage
//!
//! - IN-1: External deps (no `path` field) use `^X.Y` caret format
//! - IN-2: Workspace-internal path deps use `=X.Y.Z` exact pin format
//! - IN-3: Individual crate `Cargo.toml` files reference workspace via `{ workspace = true }`
//!
//! ## Approach
//!
//! Tests read Cargo.toml files as plain text and inspect version string formats. No TOML
//! parsing library is used — regex-free string inspection suffices for format verification.

/// IN-1: External dependency entries in the workspace Cargo.toml use `^X.Y` caret format.
///
/// Reads the workspace `Cargo.toml` and verifies that every entry under
/// `[workspace.dependencies]` that does NOT have a `path =` field (i.e., external deps)
/// uses a version string beginning with `^`.
///
/// ## Invariant
///
/// External deps must use `^X.Y` — they must not use `=` (exact pin) or bare version
/// numbers without a leading operator.
// test_kind: in_spec(IN-1)
#[ test ]
fn test_in1_external_deps_use_caret_version_format()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  // Navigate from module/unilang/ up to the workspace root (dev/)
  let workspace_cargo = format!( "{}/../../Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &workspace_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read workspace Cargo.toml at {workspace_cargo}: {e}" ) );

  // Parse lines within [workspace.dependencies] section only
  let mut in_workspace_deps = false;
  let mut violations : Vec< String > = vec![];

  for line in content.lines()
  {
    let trimmed = line.trim();

    // Detect section boundaries
    if trimmed == "[workspace.dependencies]"
    {
      in_workspace_deps = true;
      continue;
    }
    if trimmed.starts_with( '[' ) && trimmed != "[workspace.dependencies]"
    {
      in_workspace_deps = false;
    }

    if !in_workspace_deps { continue; }
    // Skip blank lines and comments
    if trimmed.is_empty() || trimmed.starts_with( '#' ) { continue; }
    // Skip lines that are part of a path dep (path deps have their own IN-2 rule)
    if trimmed.contains( "path =" ) || trimmed.contains( "path=" ) { continue; }

    // Check if this line has a version string not using ^
    if trimmed.contains( "version =" ) || trimmed.contains( "version=" )
    {
      // Extract the version value
      if let Some( ver_start ) = trimmed.find( '"' )
      {
        let after_first_quote = &trimmed[ ver_start + 1.. ];
        if let Some( ver_end ) = after_first_quote.find( '"' )
        {
          let version_str = &after_first_quote[ ..ver_end ];
          // Must start with ^ for external deps (caret format)
          if !version_str.starts_with( '^' )
          {
            // Exact pin (=) is reserved for internal path deps
            violations.push( format!( "  line: {trimmed}" ) );
          }
        }
      }
    }
  }

  assert!(
    violations.is_empty(),
    "IN-1 violation: external deps must use '^X.Y' format; found non-caret versions:\n{}",
    violations.join( "\n" )
  );
}

/// IN-2: Workspace-internal path deps use `=X.Y.Z` exact version format.
///
/// Reads the workspace `Cargo.toml` and verifies that entries with `path =` use
/// the exact-pin `=X.Y.Z` format (with leading `=`). The four workspace members are
/// `unilang`, `unilang_parser`, `unilang_meta`, and `cargo_unilang`.
// test_kind: in_spec(IN-2)
#[ test ]
fn test_in2_workspace_internal_path_deps_use_exact_pin_format()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let workspace_cargo = format!( "{}/../../Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &workspace_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read workspace Cargo.toml at {workspace_cargo}: {e}" ) );

  let path_dep_names = [ "unilang", "unilang_parser", "unilang_meta", "cargo_unilang" ];

  for dep_name in &path_dep_names
  {
    // Find the entry for this dep in the file
    let mut found = false;
    for line in content.lines()
    {
      let trimmed = line.trim();
      // Look for lines starting with the dep name and containing path =
      if trimmed.starts_with( dep_name )
        && ( trimmed.contains( "path =" ) || trimmed.contains( "path=" ) )
      {
        found = true;
        // Extract the version string
        if let Some( ver_start ) = trimmed.find( '"' )
        {
          let after_first_quote = &trimmed[ ver_start + 1.. ];
          if let Some( ver_end ) = after_first_quote.find( '"' )
          {
            let version_str = &after_first_quote[ ..ver_end ];
            assert!(
              version_str.starts_with( '=' ) && !version_str.starts_with( "==" ),
              "IN-2 violation: path dep '{dep_name}' must use '=X.Y.Z' format, \
               got: '{version_str}' on line: {trimmed}"
            );
          }
        }
      }
    }
    assert!(
      found,
      "IN-2 invariant check: path dep '{dep_name}' not found in workspace Cargo.toml"
    );
  }
}

/// IN-3: Individual crate `Cargo.toml` files use `{ workspace = true }` for all dependencies.
///
/// Verifies that the `unilang` crate's own `Cargo.toml` does not declare standalone
/// version literals in `[dependencies]` — all dependency versions must be inherited from
/// the workspace via `{ workspace = true }`.
///
/// ## Scope
///
/// Checks the `unilang` crate only (the crate under test). Inspects `[dependencies]` and
/// `[dev-dependencies]` sections.
// test_kind: in_spec(IN-3)
#[ test ]
fn test_in3_crate_cargo_toml_uses_workspace_inheritance()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let crate_cargo = format!( "{}/Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &crate_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read crate Cargo.toml at {crate_cargo}: {e}" ) );

  let mut in_dep_section = false;
  let mut violations : Vec< String > = vec![];

  for line in content.lines()
  {
    let trimmed = line.trim();

    // Detect [dependencies] and [dev-dependencies] sections
    if trimmed == "[dependencies]"
      || trimmed == "[dev-dependencies]"
      || trimmed == "[build-dependencies]"
    {
      in_dep_section = true;
      continue;
    }
    // Any new section ends the dep section
    if trimmed.starts_with( '[' )
    {
      in_dep_section = false;
    }

    if !in_dep_section { continue; }
    if trimmed.is_empty() || trimmed.starts_with( '#' ) { continue; }

    // If this line has a version string but does NOT mention workspace = true, it's a violation
    if ( trimmed.contains( "version =" ) || trimmed.contains( "version=" ) )
      && !trimmed.contains( "workspace = true" )
      && !trimmed.contains( "workspace=true" )
    {
      violations.push( format!( "  {trimmed}" ) );
    }
  }

  assert!(
    violations.is_empty(),
    "IN-3 violation: crate Cargo.toml must not declare standalone version literals; \
     all deps must use workspace inheritance. Found violations:\n{}",
    violations.join( "\n" )
  );
}
