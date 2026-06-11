//! Regression tests for namespace format validation.
//!
//! ## Root Cause
//!
//! Example code used plain string namespaces (e.g., `"collections"`) without the
//! required dot prefix. The registry validator rejects non-empty namespaces that
//! dont start with `'.'`, but examples bypassed the builder and set the field directly.
//!
//! ## Why Not Caught
//!
//! No test validated namespace format on manually-constructed `CommandDefinition` structs.
//! Builder-path tests passed because the builder doesnt enforce namespace format; only
//! `register()` does. Direct field assignment skipped the builder entirely.
//!
//! ## Fix Applied
//!
//! Corrected all example namespaces to use dot prefix (e.g., `".collections"`).
//! Added these regression tests to validate both valid and invalid namespace formats.
//!
//! ## Prevention
//!
//! These tests ensure the dot-prefix requirement is enforced on registration. Any
//! future namespace format changes must update these tests.
//!
//! ## Pitfall
//!
//! `CommandDefinition::namespace` is a public `String` field — direct assignment bypasses
//! all validation. The only enforcement point is `CommandRegistry::register()`.

#![ allow( clippy::unnecessary_wraps ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::doc_markdown ) ]

use unilang::{ CommandDefinition, CommandRegistry };

// test_kind: bug_reproducer(BUG-092)
#[ test ]
fn test_namespace_requires_dot_prefix()
{
  let mut registry = CommandRegistry::new();

  let mut cmd = CommandDefinition::former()
    .name( ".test_command" )
    .description( "Test command" )
    .end();

  // Invalid: namespace without dot prefix
  cmd.namespace = "collections".to_string();

  let result = registry.register( cmd );

  assert!(
    result.is_err(),
    "Namespace without dot prefix should be rejected"
  );

  let error = result.unwrap_err().to_string();
  assert!(
    error.contains( "namespace" ) && error.contains( "dot prefix" ),
    "Error should mention namespace format requirement, got: {}",
    error
  );
}

#[ test ]
fn test_valid_namespace_accepted()
{
  let mut registry = CommandRegistry::new();

  // Valid: namespace with dot prefix
  let mut cmd = CommandDefinition::former()
    .name( ".test_command" )
    .description( "Test command" )
    .end();

  cmd.namespace = ".collections".to_string();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Valid namespace with dot prefix should be accepted, got error: {:?}",
    result.err()
  );
}

#[ test ]
fn test_empty_namespace_accepted()
{
  let mut registry = CommandRegistry::new();

  // Valid: empty namespace (root-level command)
  let cmd = CommandDefinition::former()
    .name( ".root_command" )
    .description( "Root command" )
    .namespace( String::new() )
    .end();

  let result = registry.register( cmd );

  assert!(
    result.is_ok(),
    "Empty namespace should be valid, got error: {:?}",
    result.err()
  );
}
