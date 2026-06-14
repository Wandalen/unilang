//! NFR platform compatibility tests.
//!
//! Implements FT-4 specification case from `tests/docs/feature/005_repl_interactive.md`.
//!
//! Tests verify that the `unilang` crate compiles for `wasm32-unknown-unknown` without
//! referencing std-only APIs (threads, filesystem, process exit), satisfying FR-MOD-WASM-REPL.
//! If the WASM target is not installed the test skips gracefully with a diagnostic message.

/// FT-4: WASM build compiles without std-only features.
///
/// Runs `cargo check --target wasm32-unknown-unknown --no-default-features --features enabled`
/// in a subprocess. A zero exit code confirms the crate is WASM-compatible: no thread APIs,
/// no filesystem calls, and no process-exit paths are reachable in the `enabled` feature set.
///
/// Skips with an informational message when `wasm32-unknown-unknown` is not installed.
///
/// Spec: feature/005_repl_interactive.md § FT-4
// test_kind: ft_spec(FT-4)  [feature/05_repl_interactive]
#[ test ]
fn test_ft4_wasm_build_compiles_without_std_only_features()
{
  use std::process::Command;

  // Check if WASM target is installed — skip gracefully if not
  let rustup_check = Command::new( "rustup" )
    .args([ "target", "list", "--installed" ])
    .output();

  match rustup_check
  {
    Ok( output ) =>
    {
      let installed = String::from_utf8_lossy( &output.stdout );
      if !installed.contains( "wasm32-unknown-unknown" )
      {
        eprintln!(
          "FT-4 skipped: wasm32-unknown-unknown target not installed.\n\
           Install with: rustup target add wasm32-unknown-unknown"
        );
        return;
      }
    }
    Err( _ ) =>
    {
      eprintln!( "FT-4 skipped: rustup not found on PATH" );
      return;
    }
  }

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = Command::new( "cargo" )
    .args([
      "check",
      "--target", "wasm32-unknown-unknown",
      "--no-default-features",
      "--features", "enabled",
      "-p", "unilang",
    ])
    .current_dir( manifest_dir )
    .output()
    .expect( "FT-4: failed to spawn cargo check" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!(
    output.status.success(),
    "FT-4 violation: cargo check --target wasm32-unknown-unknown must exit 0.\n\
     This indicates a std-only API is reachable in the `enabled` feature path.\nstderr:\n{}",
    stderr
  );
}
