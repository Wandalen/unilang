//! Shared support for downstream-fixture manifests used by validation tests.
//!
//! Validation tests generate throwaway consumer crates in temp directories and
//! build them against this crate by path. Those builds run OUTSIDE this
//! workspace, so the workspace root's `[patch.crates-io]` entries do not apply
//! to them — any internal dependency that is declared by version and patched
//! to a local path (currently `cli_fmt`, consumed by `unilang_help`) would
//! resolve to crates.io instead and miss unpublished API. Every generated
//! fixture manifest must therefore append [`local_dep_patch`] so the fixture
//! build graph resolves the same local sources the workspace does.

/// Returns a `[patch.crates-io]` manifest section mirroring the workspace's
/// local-path patches, for appending to a generated fixture `Cargo.toml`.
pub fn local_dep_patch() -> String
{
  let cli_fmt_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) )
    .join( "../../../../wtools/dev/module/core/cli_fmt" );
  let cli_fmt_path = cli_fmt_path.canonicalize().unwrap_or( cli_fmt_path );
  format!( "\n[patch.crates-io]\ncli_fmt = {{ path = \"{}\" }}\n", cli_fmt_path.display() )
}
