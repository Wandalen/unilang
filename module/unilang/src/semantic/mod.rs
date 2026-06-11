//!
//! The semantic analyzer for the Unilang framework.
//!
//! # Interactive Argument Handling Implementation
//!
//! This module implements the critical `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error
//! signaling system for REPL applications:
//!
//! ## Key Implementation Details (lines 196-203)
//! - Interactive arguments are detected during semantic analysis, NOT during execution
//! - The specific error code `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` is returned
//! - This allows REPL loops to catch the error and prompt for secure input
//! - Optional interactive arguments with defaults do NOT trigger the error
//!
//! ## Security Considerations
//! - Interactive validation occurs before any command execution
//! - Sensitive arguments should be marked with both `interactive: true` and `sensitive: true`
//! - The semantic analyzer never logs or stores interactive argument values
//! - Error messages for interactive arguments are deliberately generic to avoid information leakage
//!
//! ## REPL Integration Pattern
//! ```text
//! match semantic_analyzer.analyze() {
//!     Err(Error::Execution(error_data))
//!         if error_data.code == "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => {
//!         // Handle secure input prompting at REPL level
//!         prompt_for_secure_input(&error_data.message);
//!     },
//!     // ... other error handling
//! }
//! ```
//!

mod core;
mod argument_binding;
mod validation;

/// Internal namespace (placeholder for mod_interface compatibility).
mod private {}

mod_interface::mod_interface!
{
  exposed use core::VerifiedCommand;
  exposed use core::SemanticAnalyzer;

  prelude use core::VerifiedCommand;
  prelude use core::SemanticAnalyzer;
}
