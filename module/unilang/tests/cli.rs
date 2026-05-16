//! CLI Domain Tests
//!
//! All tests related to CLI interfaces: CLI builder APIs, ergonomic interfaces,
//! and shell integration.

mod cli {
  mod cli_builder_api;
  mod cli_integration;
  mod ergonomic_apis;
  mod multiword_params;
  mod shell_argument_parsing;
  mod verbosity_control;
}