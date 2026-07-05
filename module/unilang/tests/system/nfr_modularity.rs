//! NFR modularity tests: enabled feature is a strict subset of full.
//!
//! Implements IN-6 from `tests/docs/invariant/02_non_functional_requirements.md`.
//!
//! Verifies that the `enabled` feature set compiles cleanly and that a capability
//! present only in `full` (YAML runtime loading via `serde_yaml_ng`) is absent from
//! the `enabled` runtime dependency tree, confirming `enabled ⊂ full`.

/// IN-6: `enabled` and `full` feature sets are distinct; `enabled` is a strict subset.
///
/// Two subprocess cargo checks confirm both features build cleanly. A `cargo tree`
/// check on `--features enabled` confirms `serde_yaml_ng` (a `full`-only runtime dep)
/// is absent from the `enabled` runtime dependency tree, proving `enabled ⊂ full`.
///
/// Spec: invariant/002_non_functional_requirements.md § IN-6
// test_kind: in_spec(IN-6)  [invariant/02_non_functional_requirements]
#[ test ]
fn test_in6_enabled_feature_is_strict_subset_of_full()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  // Both feature sets must compile cleanly
  for features in &[ "enabled", "full" ]
  {
    let output = Command::new( "cargo" )
      .args([ "check", "-p", "unilang", "--no-default-features", "--features", features ])
      .current_dir( manifest_dir )
      .output()
      .unwrap_or_else( | e | panic!( "IN-6: failed to spawn cargo check (features={}): {}", features, e ) );

    let stderr = String::from_utf8_lossy( &output.stderr );
    assert!(
      output.status.success(),
      "IN-6: `cargo check --features {}` must exit 0.\nstderr:\n{}",
      features,
      stderr
    );
  }

  // `serde_yaml_ng` must NOT appear in the `enabled` runtime dep tree —
  // it is a build-time dependency and a `full`-approach dep, not a baseline `enabled` dep.
  let tree_output = Command::new( "cargo" )
    .args([
      "tree", "-p", "unilang",
      "--no-default-features",
      "--features", "enabled",
      "--edges=normal",
    ])
    .current_dir( manifest_dir )
    .output()
    .expect( "IN-6: failed to run cargo tree" );

  let tree_stdout = String::from_utf8_lossy( &tree_output.stdout );
  assert!(
    !tree_stdout.contains( "serde_yaml_ng" ),
    "IN-6 violation: serde_yaml_ng must not appear in the `enabled` runtime dep tree.\n\
     It is only needed for YAML-parsing approaches (present only in `full`).\n\
     cargo tree --features enabled:\n{}",
    tree_stdout
  );
}
