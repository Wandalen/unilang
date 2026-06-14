#![allow(clippy::all)]
//! WebAssembly tests for UniLang REPL
//!
//! These tests verify that the WebAssembly bridge works correctly and can execute
//! commands in a browser-like environment.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use unilang_wasm_repl::{UniLangWasmRepl, log};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_repl_creation() {
  let repl = UniLangWasmRepl::new();
  drop(repl);
}

#[wasm_bindgen_test]
fn test_help_command() {
  let repl = UniLangWasmRepl::new();
  let result = repl.get_help();
  assert!(!result.is_empty(), "Help should return non-empty content");
  assert!(!result.starts_with("Error:"), "Help should not be an error");
}

#[wasm_bindgen_test]
fn test_command_execution() {
  let repl = UniLangWasmRepl::new();
  let result = repl.execute_command(".demo.echo text::hello");
  assert!(!result.starts_with("Error:"), "Command should execute without errors; got: {result}");
  assert!(!result.is_empty(), "Command should return output");
}

#[wasm_bindgen_test]
fn test_invalid_command() {
  let repl = UniLangWasmRepl::new();
  let result = repl.execute_command(".invalid.command");
  assert!(
    result.starts_with("Error:") || result.starts_with("Unknown"),
    "Invalid command should return error; got: {result}"
  );
}

#[wasm_bindgen_test]
fn test_empty_command() {
  let repl = UniLangWasmRepl::new();
  let result = repl.execute_command("");
  assert!(!result.is_empty(), "Empty command should return some response");
}

#[wasm_bindgen_test]
fn test_calculator_command() {
  let repl = UniLangWasmRepl::new();
  let result = repl.execute_command(".calc.add a::5 b::3");
  assert!(!result.starts_with("Error:"), "Calc command should execute without errors; got: {result}");
  assert!(!result.is_empty(), "Calc command should return output");
}

#[wasm_bindgen_test]
fn test_log_function() {
  log("Test log message");
  log("");
  log("Test with special chars: <>\"'&");
}

#[wasm_bindgen_test]
fn test_multiple_commands() {
  let repl = UniLangWasmRepl::new();
  let commands = vec![
    ".help",
    ".demo.echo text::test1",
    ".calc.add a::1 b::2",
    ".demo.echo text::test2",
  ];
  for command in commands {
    let result = repl.execute_command(command);
    assert!(!result.is_empty(), "Command {} should return non-empty result", command);
  }
}

#[wasm_bindgen_test]
fn test_malformed_commands() {
  let repl = UniLangWasmRepl::new();
  let malformed_commands = vec![
    "no.dot.prefix",
    "..",
    ".",
    ".demo.",
    ".demo.echo.too.many.parts",
  ];
  for command in malformed_commands {
    let result = repl.execute_command(command);
    assert!(!result.is_empty(), "Malformed command {} should return some response", command);
  }
}

#[wasm_bindgen_test]
fn test_performance_rapid_commands() {
  let repl = UniLangWasmRepl::new();
  for i in 0..50 {
    let result = repl.execute_command(".demo.echo text::test");
    assert!(!result.is_empty(), "Rapid command {} should return result", i);
  }
}

#[wasm_bindgen_test]
fn test_wasm_specific_features() {
  let repl = UniLangWasmRepl::new();
  let fs_commands = vec![
    ".file.read ./test.txt",
    ".dir.list /",
  ];
  for command in fs_commands {
    let result = repl.execute_command(command);
    assert!(!result.is_empty(), "FS command {} should be handled", command);
  }
}
