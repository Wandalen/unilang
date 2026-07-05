//! Workspace dependency format standards tests.
//!
//! Implements IN-1..3, IN-5 specification cases from `tests/docs/invariant/04_workspace_dependency_standards.md`.
//!
//! Tests verify that all four dependency format rules (R1–R3) defined in
//! `docs/invariant/004_workspace_dependency_standards.md` hold in the workspace Cargo files.
//!
//! ## Coverage
//!
//! - IN-1: External deps (no `path` field) use `^X.Y` caret format
//! - IN-2: Workspace-internal path deps use `=X.Y.Z` exact pin format
//! - IN-3: Individual crate `Cargo.toml` files reference workspace via `{ workspace = true }`
//! - IN-4: `--no-default-features` build compiles without errors or warnings
//! - IN-5: Library crate dependencies are all marked `optional = true`
//! - IN-6: `--no-default-features` build compiles zero external dependency crates
//! - IN-7: Workspace manifest declares no `features = [...]` lists
//! - IN-8: The `enabled` feature activates dependencies via `dep:name` syntax
//!
//! ## Approach
//!
//! Tests read Cargo.toml files as plain text and inspect version string formats. No TOML
//! parsing library is used — regex-free string inspection suffices for format verification.
//! IN-6 is the exception: it shells out to `cargo build` and `cargo tree` as subprocesses
//! since the invariant concerns actual compiled/linked crates, not manifest text.

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
// test_kind: in_spec(IN-1)  [invariant/04_workspace_dependency_standards]
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
// test_kind: in_spec(IN-2)  [invariant/04_workspace_dependency_standards]
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
// test_kind: in_spec(IN-3)  [invariant/04_workspace_dependency_standards]
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

/// IN-4: `--no-default-features` build compiles without errors or warnings.
///
/// Runs `cargo check --no-default-features` on the unilang crate and verifies
/// it exits with code 0. This confirms all optional dependencies are properly gated
/// and the crate compiles to a no-op with no features enabled.
// test_kind: in_spec(IN-4)  [invariant/04_workspace_dependency_standards]
#[ test ]
fn test_in4_no_default_features_builds_cleanly()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let result = Command::new( "cargo" )
    .args([ "check", "--no-default-features" ])
    .current_dir( manifest_dir )
    .env( "RUSTFLAGS", "-D warnings" )
    .output()
    .expect( "Failed to execute cargo check" );

  let stderr = String::from_utf8_lossy( &result.stderr );

  assert!(
    result.status.success(),
    "IN-4 violation: `cargo check --no-default-features` must succeed with zero warnings; \
     exit code: {:?}\nstderr:\n{}",
    result.status.code(),
    stderr
  );
}

/// IN-5: All library crate dependencies are marked `optional = true`.
///
/// Reads the `unilang` crate's `Cargo.toml` and verifies that every entry under
/// `[dependencies]` includes `optional = true`. Binary crate `cargo_unilang` is exempt.
/// `[build-dependencies]` and `[dev-dependencies]` are also exempt since they are not
/// part of the library's public dependency surface.
// test_kind: in_spec(IN-5)  [invariant/04_workspace_dependency_standards]
#[ test ]
fn test_in5_library_deps_all_optional()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let crate_cargo = format!( "{}/Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &crate_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read crate Cargo.toml at {crate_cargo}: {e}" ) );

  let mut in_dependencies = false;
  let mut violations : Vec< String > = vec![];

  for line in content.lines()
  {
    let trimmed = line.trim();

    // Only inspect [dependencies] section — not [build-dependencies] or [dev-dependencies]
    if trimmed == "[dependencies]"
    {
      in_dependencies = true;
      continue;
    }
    if trimmed.starts_with( '[' )
    {
      in_dependencies = false;
    }

    if !in_dependencies { continue; }
    if trimmed.is_empty() || trimmed.starts_with( '#' ) { continue; }
    // Skip lines that are sub-table headers like `[dependencies.foo]`
    if trimmed.starts_with( '[' ) { continue; }

    // Each dependency line should contain `optional = true` (or `optional=true`)
    // Only check lines that define a dependency (contain `=` and look like `name = { ... }`)
    if trimmed.contains( '=' ) && !trimmed.starts_with( '#' )
    {
      // Lines like `## internal` or continuation comments are fine
      if trimmed.starts_with( "##" ) { continue; }

      if !trimmed.contains( "optional = true" ) && !trimmed.contains( "optional=true" )
      {
        violations.push( format!( "  {trimmed}" ) );
      }
    }
  }

  assert!(
    violations.is_empty(),
    "IN-5 violation: all library crate [dependencies] must be optional = true. \
     Found non-optional deps:\n{}",
    violations.join( "\n" )
  );
}

/// IN-6: `--no-default-features` build compiles zero external dependency crates.
///
/// Runs `cargo build -p unilang --no-default-features` to satisfy the Given precondition
/// (must exit 0), then runs `cargo tree --edges=normal -p unilang --no-default-features` and
/// verifies the normal-dependency-edge tree contains only the `unilang` crate itself.
///
/// ## Why `cargo tree --edges=normal` and not raw build output
///
/// Raw `cargo build` output also reports compilation of `[build-dependencies]` (e.g.
/// `serde_yaml_ng`), which are required unconditionally by `build/main.rs` regardless of
/// feature flags and are NOT part of the `enabled`-gated optional functionality this
/// invariant targets. `--edges=normal` restricts the tree to normal (runtime) dependency
/// edges only, which is exactly what the spec's parenthetical alternative calls for.
// test_kind: in_spec(IN-6)  [invariant/04_workspace_dependency_standards]
#[ test ]
fn test_in6_no_default_features_build_links_zero_external_crates()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  // Given: the unilang crate built with --no-default-features
  let build_result = Command::new( "cargo" )
    .args([ "build", "-p", "unilang", "--no-default-features" ])
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute cargo build" );

  assert!(
    build_result.status.success(),
    "IN-6 precondition failed: `cargo build -p unilang --no-default-features` must succeed; \
     exit code: {:?}\nstderr:\n{}",
    build_result.status.code(),
    String::from_utf8_lossy( &build_result.stderr )
  );

  // When: cargo tree --edges=normal is inspected for the no-default-features configuration
  let tree_result = Command::new( "cargo" )
    .args([ "tree", "--edges=normal", "-p", "unilang", "--no-default-features" ])
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute cargo tree" );

  assert!(
    tree_result.status.success(),
    "IN-6 check failed: `cargo tree --edges=normal -p unilang --no-default-features` must succeed; \
     exit code: {:?}\nstderr:\n{}",
    tree_result.status.code(),
    String::from_utf8_lossy( &tree_result.stderr )
  );

  let stdout = String::from_utf8_lossy( &tree_result.stdout );
  let crate_lines : Vec< &str > = stdout.lines().filter( | line | !line.trim().is_empty() ).collect();

  // Then: zero external dependency crates are compiled or linked; only unilang itself is present
  assert!(
    crate_lines.len() == 1 && crate_lines[ 0 ].trim_start().starts_with( "unilang " ),
    "IN-6 violation: `--no-default-features` build must link zero external dependency crates; \
     the normal-dependency tree must contain only the `unilang` crate itself. \
     Found tree:\n{stdout}"
  );
}

/// IN-7: Workspace manifest declares no `features = [...]` lists.
///
/// Reads the workspace `Cargo.toml` and verifies that no entry under
/// `[workspace.dependencies]` declares a `features = [...]` list. Feature selection must
/// live exclusively in member crate `Cargo.toml` files. `default-features = false` on path
/// deps is explicitly permitted since it is not a `features` list.
// test_kind: in_spec(IN-7)  [invariant/04_workspace_dependency_standards]
#[ test ]
fn test_in7_workspace_manifest_declares_no_features_lists()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let workspace_cargo = format!( "{}/../../Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &workspace_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read workspace Cargo.toml at {workspace_cargo}: {e}" ) );

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
    if trimmed.is_empty() || trimmed.starts_with( '#' ) { continue; }

    // A `features = [...]` list is a violation; `default-features = false` is permitted
    // since it is a boolean flag, not a features list.
    if trimmed.contains( "features =" ) || trimmed.contains( "features=" )
    {
      let is_default_features = trimmed.contains( "default-features" );
      if !is_default_features
      {
        violations.push( format!( "  {trimmed}" ) );
      }
    }
  }

  assert!(
    violations.is_empty(),
    "IN-7 violation: [workspace.dependencies] must not declare 'features = [...]' lists; \
     feature selection belongs exclusively in member crate Cargo.toml files. \
     Found violations:\n{}",
    violations.join( "\n" )
  );
}

/// IN-8: The `enabled` feature activates dependencies via `dep:name` syntax.
///
/// Reads the `unilang` crate's `Cargo.toml` `[features]` section and verifies that every
/// entry in the `enabled` feature's activation list uses either the `dep:name` syntax (to
/// gate an optional dependency) or the `crate/feature` slash syntax (to activate a feature
/// on an already-`dep:`-activated dependency, e.g. `unilang_parser/enabled`). A bare crate
/// name with neither form would implicitly enable a same-named feature rather than gating
/// an optional dependency, which is the violation this invariant guards against.
// test_kind: in_spec(IN-8)  [invariant/04_workspace_dependency_standards]
#[ test ]
fn test_in8_enabled_feature_activates_deps_via_dep_colon_syntax()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let crate_cargo = format!( "{}/Cargo.toml", manifest_dir );

  let content = std::fs::read_to_string( &crate_cargo )
    .unwrap_or_else( | e | panic!( "Cannot read crate Cargo.toml at {crate_cargo}: {e}" ) );

  // Locate the `enabled = [ ... ]` list, which may span multiple lines.
  let start_marker = "enabled = [";
  let start_idx = content.find( start_marker )
    .unwrap_or_else( || panic!( "IN-8 invariant check: 'enabled = [' feature list not found in {crate_cargo}" ) );

  let after_start = &content[ start_idx + start_marker.len().. ];
  let end_idx = after_start.find( ']' )
    .unwrap_or_else( || panic!( "IN-8 invariant check: unterminated 'enabled' feature list in {crate_cargo}" ) );

  let list_body = &after_start[ ..end_idx ];

  // Extract each quoted entry in the list.
  let mut entries : Vec< &str > = vec![];
  let mut rest = list_body;
  while let Some( quote_start ) = rest.find( '"' )
  {
    let after_quote = &rest[ quote_start + 1.. ];
    let quote_end = after_quote.find( '"' )
      .unwrap_or_else( || panic!( "IN-8 invariant check: unterminated string literal in 'enabled' list" ) );
    entries.push( &after_quote[ ..quote_end ] );
    rest = &after_quote[ quote_end + 1.. ];
  }

  assert!( !entries.is_empty(), "IN-8 invariant check: 'enabled' feature list has no entries in {crate_cargo}" );

  let mut violations : Vec< String > = vec![];
  for entry in &entries
  {
    // Valid forms: `dep:name` (gates an optional dependency) or `crate/feature` (slash
    // syntax activating a feature on an already-dep:-activated dependency).
    let is_dep_colon = entry.starts_with( "dep:" );
    let is_slash_feature = entry.contains( '/' );
    if !is_dep_colon && !is_slash_feature
    {
      violations.push( format!( "  \"{entry}\"" ) );
    }
  }

  assert!(
    violations.is_empty(),
    "IN-8 violation: every entry in the 'enabled' feature list must use 'dep:name' syntax \
     (or 'crate/feature' slash syntax); bare crate names implicitly enable a same-named \
     feature instead of gating an optional dependency. Found violations:\n{}",
    violations.join( "\n" )
  );
}
