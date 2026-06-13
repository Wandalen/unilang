//!
//! Pipeline utilities for common Unilang workflows.
//!
//! This module provides convenient helper functions that combine multiple
//! Unilang components to handle common use cases, making it easier to
//! integrate Unilang into applications.
//!
//! # REPL Implementation Insights
//!
//! The Pipeline is specifically designed for REPL (Read-Eval-Print Loop) applications:
//!
//! ## Stateless Operation
//! - **Critical**: All components (Parser, `SemanticAnalyzer`, Interpreter) are completely stateless
//! - Each `process_command` call is independent - no state accumulation between calls
//! - Memory usage remains constant regardless of session length
//! - Safe for long-running REPL sessions without memory leaks
//!
//! ## Command Pipeline Performance Analysis
//! - Component reuse provides 20-50% performance improvement over creating new instances
//! - Static command registry lookups are zero-cost even with millions of commands
//! - Parsing overhead is minimal and constant-time for typical command lengths
//!
//! ## Error Isolation
//! - Command failures are isolated - one failed command doesn't affect subsequent commands
//! - Parse errors, semantic errors, and execution errors are all safely contained
//! - REPL sessions can continue indefinitely even with frequent command failures
//!
//! ## Interactive Argument Handling
//! - The `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error is designed to be caught by REPL loops
//! - Interactive prompts should be handled at the REPL level, not within the pipeline
//! - Secure input (passwords, API keys) should never be logged or stored in pipeline state

mod core;
mod result;
mod batch;
mod argv;
mod error_parsing;

/// Internal namespace (placeholder for mod_interface compatibility).
mod private {}

mod_interface::mod_interface!
{
  exposed use result::UnilangError;
  exposed use result::CommandResult;
  exposed use batch::BatchResult;
  exposed use core::Pipeline;
  exposed use core::process_single_command;
  exposed use core::validate_single_command;

  prelude use result::UnilangError;
  prelude use result::CommandResult;
  prelude use batch::BatchResult;
  prelude use core::Pipeline;
  prelude use core::process_single_command;
}

