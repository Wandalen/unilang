//! Type-safe CommandDefinition with validated newtypes and builder pattern.
//!
//! Provides the core CommandDefinition struct with private fields, validated
//! newtype wrappers, and a type-state builder pattern that enforces required
//! fields at compile time.

mod core;
mod accessors;
mod serde_impl;
mod builder;

pub use core::CommandDefinition;
pub use builder::{ CommandDefinitionBuilder, Set, NotSet };
