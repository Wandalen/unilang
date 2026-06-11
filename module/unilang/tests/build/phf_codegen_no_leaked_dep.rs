//! Tests that `generate_static_registry_source()` emits no `phf_map!` invocations.
//!
//! ## Root Cause
//!
//! `generate_static_registry_source()` (codegen.rs) emitted `phf_map! { ... }` (bare),
//! then `phf::phf_map! { ... }` (qualified). Both forms embed `phf_map!` as a substring.
//! When downstream crates compile the generated code, the `phf_map!` proc-macro expands
//! to `::phf::Map` absolute paths — forcing every consumer to add `phf` as a direct
//! `Cargo.toml` dependency even though `unilang` already re-exports `phf` via `pub use phf`.
//!
//! Qualifying the call as `phf::phf_map!` (Fix A) does not solve the root cause:
//! the proc-macro's internal expansion hardcodes `::phf::` regardless of call-site
//! qualification. The complete fix uses `phf_codegen` struct-literal generation — no
//! macro invocation in the generated source at all.
//!
//! ## Why Not Caught
//!
//! All prior tests called `generate_static_registry_source()` from within `unilang`'s
//! own test suite — where `phf` is a direct dependency and any `phf_map!` call resolves.
//! No test checked the generated source string for ALL `phf_map!` occurrences (bare or
//! qualified), and no test compiled the output in a crate without phf as a direct dep.
//!
//! ## Fix Applied
//!
//! `codegen.rs`: replaced `phf::phf_map! { ... }` emission with `phf_codegen::Map`
//! struct-literal generation via `.phf_path("phf")`. The generated source now contains
//! `phf::Map { key: ..., disps: ..., entries: ... }` where `phf` is the `unilang::phf`
//! re-export alias. No macro invocation — downstream crates need no direct `phf` dep.
//!
//! ## Prevention
//!
//! Assert the generated source string: (1) does NOT contain any `phf_map!` invocation
//! (bare or qualified — both are broken), (2) DOES contain `phf::Map {` struct literal.
//! Any future edit reintroducing `phf_map!` will fail this test immediately.
//!
//! ## Pitfall
//!
//! `phf_codegen::Map.phf_path("phf")` generates `phf::Map { ... }` where `phf` is the
//! module alias from `use unilang::phf::{self, Map}` in the generated source header.
//! The struct literal contains pre-computed hash data (key, disps, entries) — no macro
//! expansion occurs at downstream compile time, so no `::phf::` absolute path is needed.

use unilang::multi_yaml::{ MultiYamlAggregator, AggregationConfig };

/// bug_reproducer(BUG-090)
///
/// Verifies that `generate_static_registry_source()` emits NO `phf_map!` invocations.
///
/// RED state (pre-fix): source contained `phf::phf_map! { ... }` — `phf_map!` as a
/// substring causes the proc-macro to expand with `::phf::` absolute paths at downstream
/// compile time, requiring `phf` as a direct Cargo.toml dep in any consuming crate.
/// GREEN state (post-fix): source uses `phf::Map { key: ..., disps: ..., entries: ... }`
/// struct literal (phf_codegen output) — no macro, no `::phf::` expansion, no forced dep.
#[ test ]
fn phf_codegen_no_bare_phf_map_in_generated_source()
{
  let config = AggregationConfig::default();
  let aggregator = MultiYamlAggregator::new( config );
  let source = aggregator.generate_static_registry_source();

  // Must not contain any phf_map! invocation — bare or qualified both cause the
  // proc-macro to expand with ::phf:: absolute paths at downstream compile time
  assert!(
    !source.contains( "phf_map!" ),
    "generated source must not contain any 'phf_map!' invocation (bare or qualified);\n\
     use phf_codegen struct-literal generation instead\nsource:\n{}",
    source
  );

  // Must use phf_codegen struct literal — no macro invocation at all
  assert!(
    source.contains( "phf::Map {" ),
    "generated source must contain 'phf::Map {{' struct literal (phf_codegen output)\nsource:\n{}",
    source
  );
}
