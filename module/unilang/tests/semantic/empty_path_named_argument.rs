//! Minimal reproduction: empty `command_path_slices` with an attached unknown named argument
//! bypasses argument validation and returns the help listing instead of an error.
//!
//! ## Root Cause
//!
//! `SemanticAnalyzer::analyze_internal` (`src/semantic/core.rs`) used to return
//! `self.generate_help_listing()` unconditionally whenever `instruction.command_path_slices`
//! was empty, without checking whether any named or positional arguments were attached —
//! so an unknown named argument silently fell through to help output instead of an error.
//!
//! ## Why Not Caught
//!
//! Originally a 2-stage defect spanning the parser and the semantic analyzer. It only became
//! reproducible after the parser-side fix (`unilang_parser/src/parser_engine/mod.rs`,
//! `Fix(issue-cmd-path)`) started correctly excluding a `name::value` identifier from
//! `command_path_slices` — which is what first produces the empty-path-plus-non-empty-
//! named-arguments combination this test exercises. No existing test covered that specific
//! combination before this fix landed.
//!
//! ## Fix Applied
//!
//! `analyze_internal` now checks `named_arguments.is_empty() && positional_arguments.is_empty()`
//! before falling back to the help listing; if either is non-empty, it returns
//! `Error::Execution(Self::unknown_parameter_error_for_empty_path(instruction))` instead
//! (`src/semantic/core.rs:324-331`).
//!
//! ## Prevention
//!
//! This test locks in the empty-path-with-arguments branch as a permanent regression guard.
//! Also guards downstream consumer coverage: `assistant::commands
//! help_unknown_named_parameter_rejected` (separate repo) depends on this exact behavior.
//!
//! ## Pitfall
//!
//! Easy to reintroduce by "simplifying" the empty-path branch back to an unconditional help
//! listing — the bare `.` → help case is the common/expected path and looks like the whole
//! story unless the parser-side history above is known.

use unilang::
{
  registry::CommandRegistry,
  semantic::SemanticAnalyzer,
};
use unilang_parser::{ Parser, UnilangParserOptions };

/// bug_reproducer(issue-003)
///
/// MRE: an instruction with empty command path but a non-empty (and invalid) named argument
/// must NOT silently succeed via the help-listing fallback — it must surface a validation error.
///
/// Registers a registry with commands present (so `analyze()` genuinely has something to
/// validate against and isn't just an empty-registry corner case), parses the bare-dot pattern
/// `. some_unknown_param::xyz`, and asserts semantic analysis rejects it rather than returning
/// `Ok` or a generic help listing.
#[test]
fn test_empty_command_path_with_unknown_named_argument_should_error()
{
  // Registry has at least one real command registered, proving the help-listing fallback
  // is not simply "no commands exist" — the bug fires even when commands ARE registered.
  let cmd = unilang::data::CommandDefinition::former()
    .name( ".test" )
    .description( "Test command used to prove registry is non-empty" )
    .end();

  let mut registry = CommandRegistry::new();
  registry.register( cmd ).expect( "Registration should succeed" );

  let parser = Parser::new( UnilangParserOptions::default() );

  // ". some_unknown_param::xyz" -> empty command_path_slices (the parser fix correctly
  // excludes "some_unknown_param" from the path because it's the NAME in a name::value
  // pattern), non-empty named_arguments containing "some_unknown_param" -> "xyz".
  let instruction_text = ". some_unknown_param::xyz";
  let instruction = parser.parse_repl_input( instruction_text )
    .expect( "Parser should succeed (parser-side defect is already fixed)" );

  // Empirically confirm the parser-side precondition this MRE depends on.
  assert!(
    instruction.command_path_slices.is_empty(),
    "Precondition failed: expected empty command_path_slices, got {:?}",
    instruction.command_path_slices
  );
  assert!(
    instruction.named_arguments.contains_key( "some_unknown_param" ),
    "Precondition failed: expected 'some_unknown_param' in named_arguments, got {:?}",
    instruction.named_arguments.keys().collect::< Vec< _ > >()
  );

  let instructions = vec![ instruction ];
  let analyzer = SemanticAnalyzer::new( &instructions, &registry );

  let result = analyzer.analyze();

  // THE BUG: this currently returns Ok(_) via the help-listing fallback (Err with
  // ErrorCode::HelpRequested is the *pre-existing intentional* help-listing signal for a bare
  // "."; but here we attached an unknown named argument, which should be rejected with
  // ErrorCode::UnknownParameter / "Unknown parameter" text, NOT silently routed to help).
  assert!(
    result.is_err(),
    "Empty command path with an unknown named argument attached must be rejected, not silently \
     accepted or routed to help listing. Got: {result:?}"
  );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "Unknown parameter" ) || error_msg.contains( "some_unknown_param" ),
    "Error should identify 'some_unknown_param' as an unknown/unvalidated parameter \
     (validation should have run before any help-listing fallback), got: {error_msg}"
  );
}
