//! Help Domain Tests
//!
//! All tests related to help system: help generation, formatting,
//! and conventions.

mod help {
  mod cli_invocation;
  mod conventions;
  mod enforcement;
  mod features_comprehensive;
  mod formatting;
  mod generation;
  mod help_completeness_validation;
  mod help_divergence_prevention;
  mod nonexistent_command;
  mod operator;
  mod show_version;
  mod unified_format;
  mod verbosity;
}