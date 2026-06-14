//! Build-runtime separation invariant tests.
//!
//! Implements IN-1..IN-4 specification cases from
//! `tests/docs/invariant/006_build_runtime_separation.md`.
//!
//! Tests verify that the build-time/runtime separation boundary defined in
//! `docs/invariant/006_build_runtime_separation.md` holds: YAML/JSON parsing
//! crates are absent from the runtime dependency tree, static command data is
//! accessible as compile-time constants, and `validation_core` shared logic
//! is accessible via the runtime module path without linking the build pipeline.
//!
//! ## Coverage
//!
//! - IN-1: `serde_yaml_ng` absent from runtime dep tree (`--no-default-features --features enabled`)
//! - IN-2: `StaticCommandDefinition` fields accessible as compile-time constants (no parsing call)
//! - IN-3: `validate_command_name_core` produces correct results at runtime (shared logic identity)
//! - IN-4: `serde_json` absent from runtime dep tree (`--no-default-features --features enabled`)
//!
//! ## Approach
//!
//! IN-1 and IN-4 run `cargo tree` as a subprocess with `--no-default-features --features enabled
//! --edges=normal` and inspect **stdout content**. Exit code is NOT used — `cargo tree` exits 0
//! regardless of whether a specific package is found. Only stdout absence of the package name
//! is the meaningful signal.
//!
//! IN-2 constructs a `StaticCommandDefinition` as a compile-time `const` and reads its fields.
//! No YAML parsing is performed; the `const` construction would fail at compile time if the type
//! were not eligible, confirming zero-cost runtime-only access.
//!
//! IN-3 calls `validate_command_name_core` via the runtime module path. `src/validation_core.rs`
//! is designed to be dependency-free so it can be `include!()`d by build scripts when needed.
//! Correct runtime results confirm behavioral identity without runtime linkage to the build pipeline.

/// IN-1: `serde_yaml_ng` is absent from the runtime dependency tree.
///
/// Runs `cargo tree -p unilang --no-default-features --features enabled --edges=normal`
/// to inspect only normal (runtime) edges — build-dependencies are excluded by `--edges=normal`.
/// The `enabled` feature activates the runtime-only feature set; `approach_yaml_multi_build`
/// (which pulls `yaml_parser`) is NOT active. Inspects stdout — exit code is irrelevant.
// test_kind: in_spec(IN-1)  [invariant/06_build_runtime_separation]
#[ test ]
fn test_in1_runtime_deps_exclude_serde_yaml_ng()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = Command::new( "cargo" )
    .args([
      "tree",
      "-p", "unilang",
      "--no-default-features",
      "--features", "enabled",
      "--edges=normal",
    ])
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute cargo tree" );

  let stdout = String::from_utf8_lossy( &output.stdout );

  assert!(
    !stdout.contains( "serde_yaml_ng" ),
    "IN-1 violation: serde_yaml_ng must not appear in the runtime dependency tree \
     (cargo tree --no-default-features --features enabled --edges=normal).\n\
     serde_yaml_ng is a build-only dep — it must not leak into the runtime binary.\n\
     cargo tree stdout:\n{}",
    stdout
  );
}

/// IN-2: `StaticCommandDefinition` fields are accessible as compile-time constants.
///
/// Constructs a `StaticCommandDefinition` as a `const` value — only possible because
/// the type is fully `const`-eligible (uses `&'static str` and `bool`, no heap types).
/// Reads fields directly with no YAML parsing call in between. This confirms that
/// runtime code accessing the static registry incurs zero parsing overhead; all data
/// is encoded into the binary by `build/main.rs` during `cargo build`.
// test_kind: in_spec(IN-2)  [invariant/06_build_runtime_separation]
#[ cfg( feature = "static_registry" ) ]
#[ test ]
#[ allow( clippy::assertions_on_constants ) ] // IN-2: asserts const-derived defaults — vacuousness is intentional
fn test_in2_static_data_accessible_without_parsing()
{
  use unilang::static_data::StaticCommandDefinition;

  // `const` construction: would fail at compile time if any parse call were required
  const CMD : StaticCommandDefinition = StaticCommandDefinition::new(
    ".greet",
    "",
    "Greets the user",
  );

  // Copy to runtime binding — `const` above proves const-eligibility; assertions verify defaults.
  let cmd = CMD;

  // Read fields directly — no parsing function is invoked
  assert_eq!( cmd.name,             ".greet",            "name must equal the compile-time literal" );
  assert_eq!( cmd.description,      "Greets the user",   "description must equal the compile-time literal" );
  assert_eq!( cmd.namespace,        "",                  "namespace must equal the compile-time literal" );
  assert_eq!( cmd.version,          "1.0.0",             "version default must be 1.0.0" );
  assert!( cmd.auto_help_enabled,                        "auto_help_enabled default must be true" );
  assert!( cmd.arguments.is_empty(),                     "arguments default must be empty slice" );
}

/// IN-3: `validate_command_name_core` produces correct and identical results at runtime.
///
/// Calls the shared validation function via the runtime module path
/// (`unilang::validation_core::validate_command_name_core`). `src/validation_core.rs` is
/// dependency-free by design so it can be `include!()`d by build scripts when needed;
/// at runtime it is exposed via the normal module path. Correct results here confirm
/// the validation logic is accessible at runtime without linking any build-only dependencies.
// test_kind: in_spec(IN-3)  [invariant/06_build_runtime_separation]
#[ test ]
fn test_in3_validation_core_identity()
{
  use unilang::validation_core::validate_command_name_core;

  // Valid: dot-prefixed name is accepted
  let valid_result = validate_command_name_core( ".cmd" );
  assert!(
    valid_result.is_ok(),
    "IN-3 violation: validate_command_name_core must accept '.cmd'; got: {:?}",
    valid_result
  );

  // Invalid: no dot prefix is rejected
  let invalid_result = validate_command_name_core( "nodot" );
  assert!(
    invalid_result.is_err(),
    "IN-3 violation: validate_command_name_core must reject 'nodot' (missing dot prefix)"
  );

  // Error message must reference the offending name — matches build-time error format
  let err_msg = invalid_result.unwrap_err();
  assert!(
    err_msg.contains( "nodot" ),
    "IN-3 violation: error message must reference the invalid input 'nodot'; got: {}",
    err_msg
  );
}

/// IN-4: `serde_json` is absent from the runtime dependency tree.
///
/// Same approach as IN-1: `cargo tree --no-default-features --features enabled --edges=normal`.
/// JSON parsing (via `serde_json`) is confined to build-time codegen only — loading JSON command
/// manifests during `cargo build`. It must not appear in the runtime binary produced when
/// only the `enabled` feature is active.
// test_kind: in_spec(IN-4)  [invariant/06_build_runtime_separation]
#[ test ]
fn test_in4_runtime_deps_exclude_serde_json()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = Command::new( "cargo" )
    .args([
      "tree",
      "-p", "unilang",
      "--no-default-features",
      "--features", "enabled",
      "--edges=normal",
    ])
    .current_dir( manifest_dir )
    .output()
    .expect( "Failed to execute cargo tree" );

  let stdout = String::from_utf8_lossy( &output.stdout );

  assert!(
    !stdout.contains( "serde_json" ),
    "IN-4 violation: serde_json must not appear in the runtime dependency tree \
     (cargo tree --no-default-features --features enabled --edges=normal).\n\
     JSON parsing is a build-only operation — it must not be linked into the runtime binary.\n\
     cargo tree stdout:\n{}",
    stdout
  );
}
