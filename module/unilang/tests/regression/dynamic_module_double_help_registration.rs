//! Regression test for `CliBuilder`'s dynamic-module registration double-counting
//! an auto-generated `.help` companion (BUG-104).
//!
//! ## Root Cause
//!
//! `CliBuilder::register_dynamic_modules` loads a YAML module into a temporary
//! `CommandRegistry` (`temp_registry`) purely to parse it, then re-registers every
//! entry from `temp_registry.commands()` into the real registry with the module's
//! prefix applied. But `temp_registry.build()` already runs full auto-registration
//! -- including auto-help generation -- before this loop ever inspects it, so
//! `temp_registry.commands()` holds *both* the authored command (e.g. `.example`)
//! *and* its already-generated `.help` companion (`.example.help`). The loop
//! processed both: registering the real command a second time (correctly, via the
//! destination registry's own auto-help step), then separately re-registering the
//! temp registry's help entry, whose recomputed `full_name()` collides with the
//! key the first registration's auto-help step already produced.
//!
//! ## Why Not Caught
//!
//! Prior to fixing BUG-103's `construct_full_command_name` heuristic, this
//! collision was silently masked: the buggy heuristic produced two *different*
//! (both wrong) keys for the help entry depending on whether it was reached via
//! auto-generation (correctly prefixed) or via this double-registration path
//! (returned as-is, missing its prefix) -- so the two never collided, and the
//! wrong, unprefixed entry sat in the registry undetected. Correcting that
//! heuristic made both computations agree, turning a silent correctness bug into
//! a loud "already registered" registration error. No existing test asserted the
//! *absence* of the spurious unprefixed entry, only the presence of the correctly
//! prefixed one.
//!
//! ## Fix Applied
//!
//! `register_dynamic_modules` now skips any `temp_registry` entry whose name is
//! already a help command (`command_validation::is_help_command`) before
//! re-registering it -- the destination registry regenerates that `.help`
//! companion itself, correctly prefixed, as part of registering the real command.
//!
//! ## Prevention
//!
//! This test asserts both sides: the correctly prefixed command and its help
//! companion are present, *and* the spurious unprefixed help entry that the old
//! code silently produced is absent.
//!
//! ## Pitfall
//!
//! Any registration path that copies entries out of one `CommandRegistry` into
//! another must not assume the source's `.commands()` snapshot contains only
//! user-authored commands -- auto-generated companions (currently just `.help`)
//! are stored alongside them and must be filtered before re-registration, or the
//! destination's own auto-generation step will collide with them.

use unilang::multi_yaml::CliBuilder;
use std::path::PathBuf;

// test_kind: bug_reproducer(BUG-104)
/// Reproduces the originally-reported symptom: building a `CliBuilder` with a
/// single dynamic YAML module must not fail with "already registered", and must
/// not leave behind the spurious unprefixed `.help` entry the bug used to produce.
#[ test ]
fn test_dynamic_module_registration_does_not_duplicate_help_command()
{
  let registry = CliBuilder::new()
    .dynamic_module_with_prefix( "utils", PathBuf::from( "tests/test_data/utils.yaml" ), "util" )
    .build()
    .expect( "CliBuilder::build() must not fail with a duplicate-registration error" );

  assert!(
    registry.command( ".util.example" ).is_some(),
    "'.util.example' must be present, correctly prefixed"
  );
  assert!(
    registry.command( ".util.example.help" ).is_some(),
    "'.util.example.help' must be present, correctly prefixed"
  );
  assert!(
    registry.command( ".example.help" ).is_none(),
    "the spurious unprefixed '.example.help' entry from the temp-registry \
    double-registration bug must not be present"
  );
}
