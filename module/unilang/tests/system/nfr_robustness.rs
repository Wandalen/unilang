//! NFR robustness tests: panic safety and zero-feature build.
//!
//! Implements IN-4 and IN-5 from `tests/docs/invariant/02_non_functional_requirements.md`.
//!
//! ## IN-4 implementation note
//!
//! Two layers of `catch_unwind` protect the process from panics:
//! - `SemanticAnalyzer::analyze` wraps `analyze_internal()`, catching analysis-phase panics.
//! - `Interpreter::run` wraps each handler call, catching command-routine panics.
//!
//! Both map caught panics to `ErrorCode::InternalError`.


use unilang::data::{ CommandDefinition, OutputData };
use unilang::registry::CommandRegistry;
use unilang::semantic::VerifiedCommand;
use unilang::interpreter::ExecutionContext;
use unilang::pipeline::Pipeline;

/// IN-4: Panicking command handler is caught and returned as structured error.
///
/// Registers `.panic_cmd` with a handler that calls `panic!("intentional test panic")`.
/// The interpreter's `catch_unwind` catches the panic and maps it to
/// `ErrorCode::InternalError`, so the pipeline returns a failed `CommandResult`
/// instead of unwinding the caller stack.
///
/// Spec: invariant/002_non_functional_requirements.md § IN-4
// test_kind: in_spec(IN-4)  [invariant/02_non_functional_requirements]
#[ test ]
fn test_in4_panicking_handler_does_not_abort_process()
{
  let mut registry = CommandRegistry::new();

  let panic_cmd = CommandDefinition::former()
    .name( ".panic_cmd" )
    .description( "Intentionally panicking command for robustness testing".to_string() )
    .end();

  let panicking_routine = Box::new( | _cmd : VerifiedCommand, _ctx : ExecutionContext |
  {
    panic!( "intentional test panic — NFR-ROBUST-1 test" );
    #[ allow( unreachable_code ) ]
    Ok( OutputData { content : String::new(), format : "text".to_string(), execution_time_ms : None } )
  });

  registry.register_with_routine( &panic_cmd, panicking_routine ).unwrap();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command( ".panic_cmd", ExecutionContext::default() );

  assert!(
    !result.success,
    "IN-4: a panicking handler must not produce a success result"
  );
  let error_msg = result.error.expect( "IN-4: error field must be populated" );
  assert!(
    error_msg.contains( "panicked" ),
    "IN-4: error must mention the panic; got: {:?}",
    error_msg
  );
}

/// IN-5: Zero-feature build compiles without errors.
///
/// Runs `cargo check -p unilang --no-default-features` in a subprocess and asserts exit code 0.
/// Confirms the crate compiles as a no-op stub when all features are disabled, satisfying
/// NFR-MODULARITY-1 (complete feature gating).
///
/// Spec: invariant/002_non_functional_requirements.md § IN-5
// test_kind: in_spec(IN-5)  [invariant/02_non_functional_requirements]
#[ test ]
fn test_in5_zero_feature_build_compiles_clean()
{
  use std::process::Command;

  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  let output = Command::new( "cargo" )
    .args([ "check", "-p", "unilang", "--no-default-features" ])
    .current_dir( manifest_dir )
    .output()
    .expect( "IN-5: failed to spawn cargo check" );

  let stderr = String::from_utf8_lossy( &output.stderr );
  assert!(
    output.status.success(),
    "IN-5 violation: `cargo check -p unilang --no-default-features` must exit 0.\n\
     The crate must compile as a no-op stub with all features disabled.\nstderr:\n{}",
    stderr
  );
}
