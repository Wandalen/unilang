//! Build-runtime separation invariant tests.
//!
//! Implements IN-1..IN-5 specification cases from
//! `tests/docs/invariant/06_build_runtime_separation.md`.
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
//! - IN-5: Real `OUT_DIR/static_commands.rs` codegen output (from crate-root `unilang.commands.yaml`)
//!   structurally matches the source YAML manifest
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
//!
//! IN-5 differs from IN-2 in kind: instead of hand-constructing a synthetic `const`, it
//! `include!()`s the ACTUAL file the real build script wrote to `OUT_DIR` for this crate's own
//! `cargo build` (the same mechanism `examples/00_minimal.rs` uses), then asserts the resulting
//! `StaticCommandMap` structurally matches the known contents of the crate-root
//! `unilang.commands.yaml` manifest — proving the real codegen pipeline, not a stand-in, is valid
//! and parsing-free at runtime.

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

/// IN-5: Build-time codegen produces valid static data from a real YAML manifest.
///
/// Unlike IN-2 (which constructs a synthetic `StaticCommandDefinition` by hand), this test
/// `include!()`s the ACTUAL `OUT_DIR/static_commands.rs` file generated by the real
/// `build/main.rs` → `build/codegen.rs` pipeline during this crate's own `cargo build` —
/// the same mechanism used by `examples/00_minimal.rs`. The generator discovers the crate-root
/// `unilang.commands.yaml` manifest (the only top-level YAML file; `build/main.rs` excludes
/// `tests/`, `test_data/`, and `examples/` subdirectories from discovery, but not the crate
/// root) and emits one `StaticCommandDefinition` const per command plus a PHF-backed
/// `pub static STATIC_COMMANDS: StaticCommandMap`.
///
/// The `include!()` is scoped inside a private module so its generated top-level items
/// (`STATIC_COMMANDS`, `CMD_0`..`CMD_N`, `STATIC_COMMANDS_PHF`) cannot collide with the
/// hand-built `CMD` const from `test_in2_static_data_accessible_without_parsing`.
///
/// This confirms the generated static data structurally matches the source manifest
/// (command count, names, namespace, argument count) and is readable via `StaticCommandMap`
/// methods (`len()`, `get()`) with zero runtime parsing calls involved.
///
/// Isolates the real build-time-generated `OUT_DIR/static_commands.rs` `include!()` so its
/// top-level generated consts (`STATIC_COMMANDS`, `CMD_0`..`CMD_N`) do not collide with the
/// hand-constructed `CMD` const used by `test_in2_static_data_accessible_without_parsing`.
///
/// No local `use unilang::static_data::StaticCommandMap;` is added here — the generated
/// file already brings `StaticCommandMap` into scope itself (`use ::unilang::static_data::
/// StaticCommandMap;` at its own top), matching the pattern in `examples/00_minimal.rs`.
/// Duplicating the import here collides with the generated one (E0252).
// Fix(issue-006): item had both inner (//!) and outer (///) doc attributes on the same
// `mod` item, tripping clippy::mixed_attributes_style.
// Root cause: a module-internals note (import-collision rationale) was written as an
// inner //! block split from the outer /// block documenting the same item's purpose.
// Pitfall: clippy treats doc comments as attributes — splitting one item's docs across
// outer (before) and inner (after the opening brace) forms is flagged even when each
// block is individually well-formed; keep one item's documentation in one contiguous block.
// test_kind: in_spec(IN-5)  [invariant/06_build_runtime_separation]
#[ cfg( feature = "static_registry" ) ]
mod in5_real_codegen_fixture
{
  // Real build-time codegen output — NOT a hand-constructed stand-in. Generated by
  // `build/codegen.rs::generate_static_commands()` from the crate-root `unilang.commands.yaml`
  // manifest during this crate's own `cargo build` (same mechanism as `examples/00_minimal.rs`).
  include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

  /// Exposes the `include!()`d `STATIC_COMMANDS` to the enclosing test function.
  pub fn static_commands() -> &'static StaticCommandMap
  {
    &STATIC_COMMANDS
  }
}

#[ cfg( feature = "static_registry" ) ]
#[ test ]
fn test_in5_real_codegen_matches_yaml_manifest()
{
  let commands = in5_real_codegen_fixture::static_commands();

  // `unilang.commands.yaml` (crate root) defines exactly 6 commands: .version, .help,
  // .system.status, .system.info, .performance.stats, .test.search. The build script's
  // multi-file discovery walks the crate root (not excluded — only tests/, test_data/,
  // examples/ subdirectories are excluded) and finds this single manifest.
  assert_eq!(
    commands.len(),
    6,
    "IN-5 violation: real build-time codegen from unilang.commands.yaml must produce exactly \
     6 commands (.version, .help, .system.status, .system.info, .performance.stats, \
     .test.search); got {}",
    commands.len()
  );

  // Structural match: top-level command with no namespace, dot-prefixed name.
  let version_cmd = commands.get( ".version" )
    .expect( "IN-5 violation: '.version' from unilang.commands.yaml must be present in real codegen output" );
  assert_eq!( version_cmd.name, ".version", "IN-5: .version name must match YAML source" );
  assert_eq!( version_cmd.namespace, "", "IN-5: .version namespace must match YAML source (empty)" );
  assert_eq!(
    version_cmd.description, "Show version information",
    "IN-5: .version description must match YAML source verbatim"
  );

  // Structural match: namespaced command (namespace: ".system").
  let status_cmd = commands.get( ".system.status" )
    .expect( "IN-5 violation: '.system.status' from unilang.commands.yaml must be present in real codegen output" );
  assert_eq!( status_cmd.namespace, ".system", "IN-5: .system.status namespace must match YAML source" );
  assert_eq!(
    status_cmd.description, "Show system status",
    "IN-5: .system.status description must match YAML source verbatim"
  );

  // Structural match: command with arguments (namespace: ".test", 2 arguments in YAML).
  let search_cmd = commands.get( ".test.search" )
    .expect( "IN-5 violation: '.test.search' from unilang.commands.yaml must be present in real codegen output" );
  assert_eq!( search_cmd.namespace, ".test", "IN-5: .test.search namespace must match YAML source" );
  assert_eq!(
    search_cmd.arguments.len(), 2,
    "IN-5 violation: .test.search must have exactly 2 arguments (query, title) as defined in \
     unilang.commands.yaml; got {}",
    search_cmd.arguments.len()
  );
  assert_eq!( search_cmd.arguments[ 0 ].name, "query", "IN-5: first argument name must match YAML source" );
  assert!(
    !search_cmd.arguments[ 0 ].attributes.optional,
    "IN-5: 'query' argument must be required (optional: false) per YAML source"
  );
  assert_eq!( search_cmd.arguments[ 1 ].name, "title", "IN-5: second argument name must match YAML source" );
  assert!(
    search_cmd.arguments[ 1 ].attributes.optional,
    "IN-5: 'title' argument must be optional (optional: true) per YAML source"
  );

  // Non-existent command absent — confirms the map is not vacuously accepting everything.
  assert!(
    commands.get( ".nonexistent_command_not_in_yaml" ).is_none(),
    "IN-5 violation: a command absent from unilang.commands.yaml must not appear in real codegen output"
  );
}
