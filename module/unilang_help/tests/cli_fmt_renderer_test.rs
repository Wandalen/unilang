//! Tests for `CliFmtRenderer` — command and parameter pages rendered through
//! `cli_fmt`'s detail-page template, pinned against `tty_detect: false`
//! goldens (no ANSI codes, deterministic column alignment).
//!
//! ## Test Matrix
//!
//! **Test Factors:**
//! - Page: parameter detail page, command page
//! - Fixture: full (all metadata), minimal (bare), local enum param
//! - Display Options: default, all hidden, single hide
//!
//! **Test Combinations:**
//!
//! | ID   | Aspect                     | Fixture / Options       | Expected                                          |
//! |------|----------------------------|-------------------------|---------------------------------------------------|
//! | C1.1 | param page, everything     | full param `src`        | usage, both text lines, aligned facts, examples   |
//! | C1.2 | param page, choices        | enum param with default | Possible values section, Default fact             |
//! | C2.1 | command page, everything   | full, defaults          | usage synth, facts block, Parameters, Examples    |
//! | C2.2 | command page, bare         | minimal, defaults       | Status fact only; no Parameters/Examples sections |
//! | C2.3 | command page, all hidden   | minimal, all `hide_*`   | facts block skipped entirely                      |
//! | C2.4 | command page, hide_version | full, hide_version      | no Version row, Status row kept                   |

#![ cfg( feature = "enabled" ) ]

use cli_fmt::help::CliHelpStyle;
use unilang_help::{ CliFmtRenderer, HelpCommandData, HelpParamData, HelpDisplayOptions };

mod inc;
use inc::{ full_fixture, minimal_fixture };

/// Renderer with colour suppressed for byte-stable goldens.
fn renderer() -> CliFmtRenderer
{
  CliFmtRenderer::default().with_style( CliHelpStyle { tty_detect : false, ..Default::default() } )
}

/// C1.1: Fully-populated parameter page — aligned fact block, description plus
/// hint, validation fact, and examples.
#[ test ]
fn param_page_full_golden()
{
  let cmd = full_fixture();
  let expected = concat!
  (
    "Parameter: src\n",
    "  .file.copy src::<path>\n",
    "\n",
    "Source file path.\n",
    "The file to copy.\n",
    "\n",
    "  Kind        Path\n",
    "  Required    yes\n",
    "  Aliases     s\n",
    "  Validation  min_length(1)\n",
    "\n",
    "Examples:\n",
    "  .file.copy src::a.txt\n",
  );
  assert_eq!( renderer().render_param( &cmd, &cmd.params[ 0 ] ), expected );
}

/// C1.2: Enum parameter — Possible values section, Default fact, hint fallback absent.
#[ test ]
fn param_page_choices_golden()
{
  let mut param = HelpParamData::default();
  param.name = "scope".into();
  param.kind = "Enum".into();
  param.kind_compact = "enum".into();
  param.description = "Discovery strategy selector.".into();
  param.optional = true;
  param.default = Some( "local".into() );
  param.choices = vec![ "local".into(), "global".into() ];
  param.examples = vec![ ".rollup scope::global".into() ];

  let mut cmd = HelpCommandData::default();
  cmd.name = ".rollup".into();

  let expected = concat!
  (
    "Parameter: scope\n",
    "  .rollup scope::<enum>\n",
    "\n",
    "Discovery strategy selector.\n",
    "\n",
    "  Kind      Enum\n",
    "  Required  no\n",
    "  Default   local\n",
    "\n",
    "Possible values:\n",
    "  local\n",
    "  global\n",
    "\n",
    "Examples:\n",
    "  .rollup scope::global\n",
  );
  assert_eq!( renderer().render_param( &cmd, &param ), expected );
}

/// C2.1: Full command page — synthesized usage with optional brackets, aligned
/// facts, Parameters section with attribute markers, and examples.
#[ test ]
fn command_page_full_golden()
{
  let expected = concat!
  (
    "Command: .file.copy\n",
    "  .file.copy src::<path> [force::<boolean>] [tags::<list>]\n",
    "\n",
    "Copy a file from source to destination.\n",
    "Copy files.\n",
    "\n",
    "  Status   Active\n",
    "  Version  1.2.0\n",
    "  Aliases  fc, copy\n",
    "  Tags     fs, io\n",
    "\n",
    "Parameters:\n",
    "  src::<path>       Source file path.\n",
    "  force::<boolean>  Overwrite without asking. (optional)\n",
    "  tags::<list>      Tag list. (optional) (multiple)\n",
    "\n",
    "Examples:\n",
    "  .file.copy src::a.txt dst::b.txt\n",
    "  .file.copy src::a.txt force::true\n",
  );
  assert_eq!( renderer().render_command( &full_fixture() ), expected );
}

/// C2.2: Bare command — Status fact only (empty version and `show_version: false`
/// suppress the Version row); parameterless usage; no Parameters or Examples.
#[ test ]
fn command_page_minimal_golden()
{
  let expected = concat!
  (
    "Command: .ping\n",
    "  .ping\n",
    "\n",
    "Check liveness.\n",
    "\n",
    "  Status  Active\n",
  );
  assert_eq!( renderer().render_command( &minimal_fixture() ), expected );
}

/// C2.3: With every display option hidden the facts block is skipped entirely —
/// no stray blank line where the section would have been.
#[ test ]
fn command_page_all_hidden_golden()
{
  let options = HelpDisplayOptions::default().hide_version().hide_status().hide_aliases().hide_tags();
  let expected = concat!
  (
    "Command: .ping\n",
    "  .ping\n",
    "\n",
    "Check liveness.\n",
  );
  assert_eq!( renderer().with_options( options ).render_command( &minimal_fixture() ), expected );
}

/// C2.4: `hide_version` removes only the Version row; other facts stay.
#[ test ]
fn command_page_hide_version()
{
  let options = HelpDisplayOptions::default().hide_version();
  let page = renderer().with_options( options ).render_command( &full_fixture() );
  assert!( !page.contains( "Version" ) );
  assert!( page.contains( "Status   Active" ) );
  assert!( page.contains( "Aliases  fc, copy" ) );
}
