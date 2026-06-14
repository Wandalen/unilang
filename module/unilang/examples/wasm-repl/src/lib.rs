#![allow(clippy::all)]
//! # WebAssembly REPL for Unilang
//!
//! This example demonstrates how to use the unilang command framework in a WebAssembly environment.
//! It provides a web-based REPL (Read-Eval-Print Loop) interface for interacting with unilang commands.

use wasm_bindgen::prelude::*;
use web_sys::console;

use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::pipeline::Pipeline;
use unilang::registry::CommandRegistry;

// Set up panic hook for better error messages in development
#[wasm_bindgen(start)]
pub fn main() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

/// WebAssembly REPL interface for unilang commands
#[wasm_bindgen]
pub struct UniLangWasmRepl {
  pipeline: Pipeline,
}

#[wasm_bindgen]
impl UniLangWasmRepl {
  /// Create a new WebAssembly REPL instance
  #[wasm_bindgen(constructor)]
  pub fn new() -> UniLangWasmRepl {
    let mut registry = CommandRegistry::new();
    Self::register_basic_commands(&mut registry);
    let pipeline = Pipeline::new(registry);
    UniLangWasmRepl { pipeline }
  }

  /// Process a command input and return the result
  #[wasm_bindgen]
  pub fn execute_command(&self, input: &str) -> String {
    let result = self.pipeline.process_command_simple(input);
    if result.success {
      if result.outputs.is_empty() {
        "Command executed successfully".to_string()
      } else {
        result.outputs.iter()
          .map(|output| output.content.clone())
          .collect::<Vec<_>>()
          .join("\n")
      }
    } else {
      match &result.error {
        Some(error) => format!("Error: {}", error),
        None => "Unknown error occurred".to_string(),
      }
    }
  }

  /// Get help information for available commands
  #[wasm_bindgen]
  pub fn get_help(&self) -> String {
    let result = self.pipeline.process_command_simple(".help");
    if result.success {
      result.outputs.iter()
        .map(|output| output.content.clone())
        .collect::<Vec<_>>()
        .join("\n")
    } else {
      "Help not available".to_string()
    }
  }

  /// Register basic commands for demonstration
  fn register_basic_commands(registry: &mut CommandRegistry) {
    let echo_cmd = CommandDefinition::former()
      .name(".echo")
      .description("Simple echo command for WebAssembly demo")
      .namespace("demo")
      .hint("Echo the input text")
      .status("stable")
      .version("1.0.0")
      .arguments(vec![
        ArgumentDefinition
        {
          name: "text".to_string(),
          kind: Kind::String,
          attributes: unilang::data::ArgumentAttributes::default(),
          hint: "Text to echo".to_string(),
          description: "The text that will be echoed back".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        }
      ])
      .end();

    let echo_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
      let text = match cmd.arguments.get("text") {
        Some(unilang::types::Value::String(s)) => s.clone(),
        _ => "(empty)".to_string(),
      };
      Ok(OutputData { content: text, format: "text".to_string(), execution_time_ms: None })
    });

    if let Err(e) = registry.register_with_routine(&echo_cmd, echo_routine) {
      console::error_1(&format!("Failed to register echo command: {}", e).into());
    }

    let add_cmd = CommandDefinition::former()
      .name(".add")
      .description("Simple addition calculator for WebAssembly demo")
      .namespace("calc")
      .hint("Add two numbers")
      .status("stable")
      .version("1.0.0")
      .arguments(vec![
        ArgumentDefinition
        {
          name: "a".to_string(),
          kind: Kind::Integer,
          attributes: unilang::data::ArgumentAttributes::default(),
          hint: "First number".to_string(),
          description: "The first number to add".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        },
        ArgumentDefinition
        {
          name: "b".to_string(),
          kind: Kind::Integer,
          attributes: unilang::data::ArgumentAttributes::default(),
          hint: "Second number".to_string(),
          description: "The second number to add".to_string(),
          validation_rules: Vec::new(),
          aliases: Vec::new(),
          tags: Vec::new(),
        },
      ])
      .end();

    let add_routine = Box::new(|cmd: unilang::semantic::VerifiedCommand, _ctx| {
      let a = match cmd.arguments.get("a") {
        Some(unilang::types::Value::Integer(n)) => *n,
        _ => 0,
      };
      let b = match cmd.arguments.get("b") {
        Some(unilang::types::Value::Integer(n)) => *n,
        _ => 0,
      };
      let result = format!("{} + {} = {}", a, b, a + b);
      Ok(OutputData { content: result, format: "text".to_string(), execution_time_ms: None })
    });

    if let Err(e) = registry.register_with_routine(&add_cmd, add_routine) {
      console::error_1(&format!("Failed to register add command: {}", e).into());
    }
  }
}

/// Utility function to log messages from WASM to browser console
#[wasm_bindgen]
pub fn log(s: &str) {
  console::log_1(&s.into());
}
