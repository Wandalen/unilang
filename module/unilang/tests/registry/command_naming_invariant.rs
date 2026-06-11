//! Invariant tests for the dot-prefix naming contract.
//!
//! Covers spec cases IN-1..IN-3 from `tests/docs/invariant/05_command_naming.md`.
//!
//! ## Spec Coverage
//!
//! | Case | Invariant Verified |
//! |------|--------------------|
//! | IN-1 | Construction of a `CommandDefinition` with no leading dot is rejected |
//! | IN-2 | Construction and registration of a `CommandDefinition` with leading dot succeeds |
//! | IN-3 | `compute_full_name_core(namespace, name)` always produces a dot-prefixed full name |
//!
//! ## Phase 2 Validation Note (IN-1)
//!
//! Validation was moved from registration time to construction time ("fail-fast").
//! `CommandDefinition::former().name("nodot")` panics at `.name()` with a
//! `MissingDotPrefix` error, before `command_add_runtime()` is ever invoked.
//! `std::panic::catch_unwind` captures this rejection so the test can also assert
//! the registry remains unmodified.
//!
//! ## IN-3 Implementation Note
//!
//! The spec describes the build-time `compute_full_name()` function in
//! `build/validation.rs`, which is not importable from integration tests.
//! `compute_full_name_core()` from `unilang::validation_core` is the runtime
//! equivalent: both implement the invariant that namespace + name always yield
//! exactly one leading dot.  The runtime function receives normalized namespace
//! strings (already dot-prefixed) while the build-time function accepts the raw
//! YAML strings (no leading dot) and adds the dot internally — the invariant
//! holds in both cases.

#![ allow( deprecated ) ]

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;
use unilang::validation_core::compute_full_name_core;

fn create_mock_routine() -> Box< dyn Fn( VerifiedCommand, ExecutionContext ) -> Result< OutputData, unilang::data::ErrorData > + Send + Sync + 'static >
{
  Box::new( | _cmd, _ctx | Ok( OutputData::new( "ok", "text" ) ) )
}

/// IN-1: Construction of a `CommandDefinition` without a leading dot is rejected;
/// the registry is not modified.
///
/// Phase 2 behaviour: the panic happens at `CommandDefinition::former().name()`,
/// before any registration call.  `catch_unwind` captures the panic so the test
/// can also verify the post-condition "registry remains unmodified".
// test_kind: in_spec(IN-1)
#[ test ]
fn test_in1_runtime_registration_rejects_name_without_dot()
{
  let registry = CommandRegistry::new();

  // Snapshot the command count before the attempted registration.
  // `CommandRegistry::new()` pre-populates a mandatory `.help` command, so the
  // registry is not empty — we compare counts to verify it stays unmodified.
  let count_before = registry.commands().len();

  // Attempt to construct a CommandDefinition with no leading dot.
  // Phase 2: panics at construction time with "MissingDotPrefix".
  let result = std::panic::catch_unwind( ||
  {
    CommandDefinition::former()
    .name( "nodot" )   // ← no leading dot: panics here
    .description( "Invalid command" )
    .end()
  });

  // Construction must fail (panic ≡ rejection in Phase 2 fail-fast design).
  assert!( result.is_err(), "Construction of a command name without a leading dot must be rejected" );

  // Registry must remain unmodified (registration was never reached).
  assert_eq!(
    registry.commands().len(),
    count_before,
    "Registry command count must be unchanged after a failed construction attempt",
  );
  assert!(
    registry.command( "nodot" ).is_none(),
    "The invalid 'nodot' command must not appear in the registry",
  );
}

/// IN-2: Construction and registration of a `CommandDefinition` with a leading dot
/// succeeds; the command is subsequently retrievable from the registry.
// test_kind: in_spec(IN-2)
#[ test ]
fn test_in2_runtime_registration_accepts_name_with_dot()
{
  let mut registry = CommandRegistry::new();

  let valid_cmd = CommandDefinition::former()
  .name( ".valid" )
  .description( "Valid dotted command" )
  .hint( "A command whose name starts with a dot" )
  .arguments( vec![
    ArgumentDefinition::former()
    .name( "x" )
    .description( "Unused argument" )
    .kind( Kind::String )
    .hint( "x" )
    .attributes( ArgumentAttributes { optional : true, multiple : false, default : None, sensitive : false, interactive : false } )
    .validation_rules( vec![] )
    .aliases( vec![] )
    .tags( vec![] )
    .end()
  ])
  .end();

  let result = registry.command_add_runtime( &valid_cmd, create_mock_routine() );

  assert!( result.is_ok(), "Registration of '.valid' must succeed, got: {result:?}" );
  assert!(
    registry.command( ".valid" ).is_some(),
    "Registry must return Some(_) for '.valid' after successful registration",
  );
}

/// IN-3: `compute_full_name_core(namespace, name)` always produces a dot-prefixed
/// full name regardless of whether the namespace is empty or populated.
///
/// The runtime function receives normalized (dot-prefixed) namespace strings.
/// `compute_full_name_core(".math", "add")` mirrors the YAML entry
/// `namespace: "math", name: "add"` after the build-time dot normalization step,
/// and must produce `".math.add"` — exactly one leading dot.
// test_kind: in_spec(IN-3)
#[ test ]
fn test_in3_namespace_construction_produces_dot_prefixed_full_name()
{
  // Populated namespace + bare name → single leading dot
  let full_name = compute_full_name_core( ".math", "add" );
  assert_eq!(
    full_name, ".math.add",
    "compute_full_name_core must yield exactly one leading dot for namespace + name",
  );

  // Empty namespace + already-dotted name → unchanged single leading dot
  let from_empty_ns = compute_full_name_core( "", ".math" );
  assert_eq!(
    from_empty_ns, ".math",
    "compute_full_name_core must yield the name as-is when namespace is empty",
  );

  // Nested namespace → only one leading dot
  let nested = compute_full_name_core( ".system.session", "list" );
  assert!(
    nested.starts_with( '.' ),
    "Result must start with a dot; got '{nested}'",
  );
  assert_eq!(
    nested, ".system.session.list",
    "Nested namespace must concatenate correctly",
  );
}
