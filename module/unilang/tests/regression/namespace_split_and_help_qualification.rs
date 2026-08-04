//! Regression tests for YAML explicit-empty-namespace override and `.help`
//! name-qualification drop (task 111).
//!
//! ## Root Cause
//!
//! Two independent defects chained together:
//!
//! 1. `serde_impl.rs`'s deserializer collapsed "namespace explicitly declared empty"
//!    and "namespace omitted" into the same `is_empty() == true` state (via
//!    `namespace.unwrap_or_default()` running before the convenience-split check), so
//!    an explicit `namespace: ""` was silently overridden by the compact-form split
//!    meant only for an *omitted* namespace.
//! 2. `command_status.rs`'s `construct_full_command_name` inferred "name is already
//!    fully qualified" from the mere presence of any embedded dot in the stripped name,
//!    rather than checking whether the name actually already incorporates the
//!    namespace. Every auto-generated `.help`/`.h` companion name contains such a dot
//!    (from the `.help` suffix itself), so this heuristic discarded the namespace of
//!    every namespaced command's help companion.
//!
//! ## Why Not Caught
//!
//! No existing test combined "namespaced command" with "check `full_name()` (not just
//! `.name()`) on its generated help command" -- `test_v2_help_command_generation` and
//! `test_v2_generate_help_command` in `tests/data/command_definition.rs` both used
//! unnamespaced parents and never called `.full_name()` on the result. Similarly, no
//! test authored a compound dotted `name` alongside an explicit empty `namespace: ""`
//! to distinguish "explicitly empty" from "omitted".
//!
//! ## Fix Applied
//!
//! `serde_impl.rs`: capture `namespace.is_none()` before defaulting, and gate the
//! compact-form split on that presence flag instead of on `namespace.is_empty()`.
//! `command_status.rs`: an interim fix attempt narrowed the heuristic to "does `name`
//! already begin with `namespace` as a whole path segment" -- but that check itself
//! false-positives whenever a local name textually equals its namespace (e.g. namespace
//! `.enabled` + local name `.enabled` would still collapse to `.enabled` instead of
//! `.enabled.enabled`). The heuristic was dropped entirely instead: `namespace` and
//! `name` are independently-tracked fields with no legitimate call site that sets a
//! non-empty `namespace` alongside an already-namespace-prefixed `name`, so a non-empty
//! `namespace` now always concatenates, unconditionally, regardless of `name`'s shape.
//!
//! ## Prevention
//!
//! These tests cover every authoring shape named in task 111's invariant: omitted
//! namespace (single- and multi-segment compact form), explicit namespace with a bare
//! name, explicit-empty namespace with a compound name, a 3+-level namespace, and a
//! local name that textually equals its own namespace (the interim fix attempt's own
//! false-positive case, named above) -- each verified through the public
//! `full_name()`/`generate_help_command()` API and, for the originally-reported
//! combination, through the full `Pipeline` lookup path.
//!
//! ## Pitfall
//!
//! `construct_full_command_name` cannot distinguish "already fully qualified" from
//! "local name happens to contain a dot" by string shape alone -- any future change to
//! this function must reason about namespace *containment*, not dot *presence*. A local
//! name legitimately contains embedded dots for reasons unrelated to qualification
//! (help/alias suffixes, and potentially other generated companions in the future).

#![ allow( clippy::doc_markdown ) ]

use unilang::data::{ CommandDefinition, CommandName, NamespaceType };
use unilang::registry::CommandRegistryBuilder;
use unilang::pipeline::Pipeline;

// test_kind: bug_reproducer(BUG-103)
/// Reproduces the originally-reported symptom end-to-end: a command authored with a
/// compound dotted name and an *explicit* empty namespace must keep its declared name
/// verbatim, and its auto-generated `.help` companion must resolve through the pipeline
/// instead of failing with "No executable routine found".
#[ test ]
fn test_mre_explicit_empty_namespace_help_resolves_via_pipeline()
{
  let yaml = r#"
- name: ".session.delete"
  namespace: ""
  description: "Delete a saved conversation session"
  arguments: []
"#;

  let registry = CommandRegistryBuilder::new()
    .load_from_yaml_str( yaml )
    .expect( "registry build" )
    .build();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_from_argv_simple( &[ ".session.delete.help".to_string() ] );

  assert!(
    result.success,
    "'.session.delete.help' should resolve to the auto-generated help routine, got error: {:?}",
    result.error
  );
}

/// Isolates defect 2: a *cleanly* namespaced command (built via the well-trodden
/// `namespace` + bare-`name` authoring shape, not the empty-override edge case) must
/// still have its help companion resolve through the pipeline.
// test_kind: bug_reproducer(BUG-103)
#[ test ]
fn test_namespaced_command_help_resolves_via_pipeline()
{
  let yaml = r#"
- name: "delete"
  namespace: ".session"
  description: "Delete a saved conversation session"
  arguments: []
"#;

  let registry = CommandRegistryBuilder::new()
    .load_from_yaml_str( yaml )
    .expect( "registry build" )
    .build();
  let pipeline = Pipeline::new( registry );

  let result = pipeline.process_command_from_argv_simple( &[ ".session.delete.help".to_string() ] );

  assert!(
    result.success,
    "'.session.delete.help' should resolve for a cleanly-namespaced command, got error: {:?}",
    result.error
  );
}

/// Isolates defect 2 at the unit level (no YAML/pipeline involved): `full_name()` on a
/// help command generated from a namespaced parent must include the namespace.
#[ test ]
fn test_help_command_full_name_includes_namespace()
{
  let name = CommandName::new( ".delete" ).unwrap();
  let ns = NamespaceType::new( ".session" ).unwrap();
  let cmd = CommandDefinition::new( name, "Delete a session".to_string() )
    .with_namespace( ns.to_string() );

  let help_cmd = cmd.generate_help_command();

  assert_eq!( help_cmd.full_name(), ".session.delete.help" );
}

// test_kind: bug_reproducer(BUG-103)
/// Guards the interim fix attempt's own false-positive case (see this file's "Fix
/// Applied" doc section): a local name that textually equals its namespace must still
/// be concatenated, not mistaken for an already-qualified name by a prefix-shape check.
#[ test ]
fn test_namespace_equal_to_name_still_concatenates()
{
  let name = CommandName::new( ".enabled" ).unwrap();
  let ns = NamespaceType::new( ".enabled" ).unwrap();
  let cmd = CommandDefinition::new( name, "Toggle enabled state".to_string() )
    .with_namespace( ns.to_string() );

  assert_eq!(
    cmd.full_name(), ".enabled.enabled",
    "a local name equal to its namespace must still be concatenated, not returned as-is"
  );
}

/// Isolates defect 1 at the unit level: an explicit empty namespace on a compound
/// dotted name must be honored verbatim, not silently re-split.
#[ test ]
fn test_explicit_empty_namespace_not_split()
{
  let yaml = r#"
name: ".session.delete"
namespace: ""
description: "Delete a saved conversation session"
"#;

  let cmd : CommandDefinition = serde_yaml_ng::from_str( yaml ).unwrap();

  assert_eq!( cmd.name().as_str(), ".session.delete", "name must be kept verbatim, not re-split" );
  assert_eq!( cmd.namespace(), "", "explicit empty namespace must be honored, not overridden" );
  assert_eq!( cmd.full_name(), ".session.delete" );
}

/// Regression guard: the *omitted*-namespace compact-form split (task 005's original
/// convenience feature) must remain unchanged by the defect-1 fix -- only an
/// explicitly-empty namespace skips the split, not an absent one.
#[ test ]
fn test_omitted_namespace_compound_name_still_splits()
{
  let yaml = r#"
name: ".session.list"
description: "List saved conversation sessions"
"#;

  let cmd : CommandDefinition = serde_yaml_ng::from_str( yaml ).unwrap();

  assert_eq!( cmd.name().as_str(), ".list", "omitted namespace must still trigger the compact-form split" );
  assert_eq!( cmd.namespace(), ".session" );
  assert_eq!( cmd.full_name(), ".session.list" );
}

/// Prevention's explicit ask: a 3+-level namespace's help companion must resolve, not
/// just a single-level one. The fixed `construct_full_command_name` no longer branches
/// on namespace depth at all (it unconditionally concatenates a non-empty namespace),
/// but this integration-level check still matters: it confirms the full deserialize ->
/// register -> dispatch path agrees at a depth beyond the single-segment common case.
#[ test ]
fn test_three_level_namespace_help_resolves_via_pipeline()
{
  let yaml = r#"
- name: "service"
  namespace: ".cloud.deploy"
  description: "Manage the deploy service"
  arguments: []
"#;

  let registry = CommandRegistryBuilder::new()
    .load_from_yaml_str( yaml )
    .expect( "registry build" )
    .build();
  let pipeline = Pipeline::new( registry );

  let base = pipeline.process_command_from_argv_simple( &[ ".cloud.deploy.service".to_string(), "?".to_string() ] );
  assert!(
    base.success,
    "base 3-level-namespaced command should resolve, got error: {:?}",
    base.error
  );

  let help = pipeline.process_command_from_argv_simple( &[ ".cloud.deploy.service.help".to_string() ] );
  assert!(
    help.success,
    "'.cloud.deploy.service.help' should resolve at 3-level namespace depth, got error: {:?}",
    help.error
  );
}
