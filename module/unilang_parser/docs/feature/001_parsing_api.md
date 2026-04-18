# Feature: Parser Public API

The `unilang_parser` crate must expose `Parser::parse_from_argv(&[String])` for shell argv input, `Parser::parse_repl_input(&str)` for single-instruction string input (REPL/config), and `Parser::parse_multiple_instructions(&str)` for multi-command strings, producing `Vec<GenericInstruction>` in all cases without any knowledge of command definitions or semantics.
