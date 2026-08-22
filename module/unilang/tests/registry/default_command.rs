//! FR-REG-10 (Default Command): registry-level getter/setter/builder validation for the
//! opt-in default-command fallback.
//!
//! ## Scope
//! Tests `CommandRegistry::default_command()` / `set_default_command()` and the equivalent
//! `CommandRegistryBuilder::default_command()` fluent method — configuration and validation
//! only. Routing behavior at analysis time is covered by
//! `tests/semantic/default_command_routing.rs`.
//!
//! ## FR Coverage
//! - FR-REG-10 (FT-26): default-command configuration is opt-in, validated, and does not
//!   require prior registration of the named command


use unilang::error::Error;
use unilang::registry::CommandRegistry;

/// FR-REG-10: A fresh registry has no default command configured.
#[ test ]
fn test_default_command_none_by_default()
{
  let registry = CommandRegistry::new();
  assert_eq!( registry.default_command(), None, "Fresh registry must not have a default command configured" );
}

/// FR-REG-10: Setting a valid, dot-prefixed name succeeds and is reported back verbatim.
#[ test ]
fn test_set_default_command_valid_name_round_trips()
{
  let mut registry = CommandRegistry::new();
  registry.set_default_command( ".report" ).expect( "Valid dot-prefixed name must be accepted" );
  assert_eq!( registry.default_command(), Some( ".report" ) );
}

/// FR-REG-10: A name missing the dot prefix is rejected with the same error `CommandName::new`
/// produces, and the registry's configuration remains unchanged.
#[ test ]
fn test_set_default_command_rejects_missing_dot_prefix()
{
  let mut registry = CommandRegistry::new();
  let result = registry.set_default_command( "report" );

  assert!( result.is_err(), "Name without dot prefix must be rejected" );
  match result.unwrap_err()
  {
    Error::MissingDotPrefix( original ) => assert_eq!( original, "report" ),
    other => panic!( "Expected Error::MissingDotPrefix, got: {other:?}" ),
  }

  assert_eq!( registry.default_command(), None, "A rejected configuration attempt must not change registry state" );
}

/// FR-REG-10: An empty name is rejected.
#[ test ]
fn test_set_default_command_rejects_empty_name()
{
  let mut registry = CommandRegistry::new();
  let result = registry.set_default_command( "" );

  assert!( result.is_err(), "Empty name must be rejected" );
  match result.unwrap_err()
  {
    Error::EmptyCommandName => {},
    other => panic!( "Expected Error::EmptyCommandName, got: {other:?}" ),
  }
}

/// FR-REG-10: A later call to `set_default_command` replaces the previous configuration —
/// single source of truth, no accumulation of multiple defaults.
#[ test ]
fn test_set_default_command_replaces_previous_value()
{
  let mut registry = CommandRegistry::new();
  registry.set_default_command( ".first" ).expect( "First call must succeed" );
  registry.set_default_command( ".second" ).expect( "Second call must succeed" );

  assert_eq!( registry.default_command(), Some( ".second" ), "Second call must replace the first" );
}

/// FR-REG-10: `set_default_command` does not require `name` to already be registered —
/// existence is only checked later, at analysis time.
#[ test ]
fn test_set_default_command_does_not_require_prior_registration()
{
  let mut registry = CommandRegistry::new();
  let result = registry.set_default_command( ".never_registered" );

  assert!( result.is_ok(), "Configuring an unregistered command name must succeed at configuration time" );
  assert_eq!( registry.default_command(), Some( ".never_registered" ) );
}

/// FR-REG-10: The builder's fluent `.default_command()` configures the same field, readable
/// after `.build()`.
#[ test ]
fn test_builder_default_command_fluent_method()
{
  let registry = CommandRegistry::builder()
    .default_command( ".report" )
    .expect( "Valid dot-prefixed name must be accepted by the builder" )
    .build();

  assert_eq!( registry.default_command(), Some( ".report" ) );
}

/// FR-REG-10: The builder propagates the same validation error as the direct setter.
#[ test ]
fn test_builder_default_command_rejects_invalid_name()
{
  let result = CommandRegistry::builder().default_command( "no_dot" );

  assert!( result.is_err(), "Builder must reject a name without dot prefix" );
  match result.unwrap_err()
  {
    Error::MissingDotPrefix( original ) => assert_eq!( original, "no_dot" ),
    other => panic!( "Expected Error::MissingDotPrefix, got: {other:?}" ),
  }
}
