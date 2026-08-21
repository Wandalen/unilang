//! Tests for `PlainRenderer` — command pages at all five verbosity levels and
//! the parameter detail page.
//!
//! The command-page goldens pin the line-faithful port of the original
//! `unilang` `HelpGenerator` formats: any byte drift from the historical
//! output fails these tests.
//!
//! ## Test Matrix
//!
//! **Test Factors:**
//! - Verbosity Level: Minimal, Basic, Standard, Detailed, Comprehensive
//! - Fixture: full (all metadata populated), minimal (bare command)
//! - Display Options: default, each `hide_*` variant
//! - Page: command page, parameter detail page
//!
//! **Test Combinations:**
//!
//! | ID   | Aspect                      | Fixture / Options            | Expected                                        |
//! |------|-----------------------------|------------------------------|-------------------------------------------------|
//! | P1.1 | Minimal golden              | full, defaults               | one line, no trailing newline                   |
//! | P1.2 | Basic golden                | full, defaults               | name line + PARAMETERS with compact kinds       |
//! | P1.3 | Standard golden             | full, defaults               | Usage/Status/Aliases/Arguments/Examples         |
//! | P1.4 | Detailed golden             | full, defaults               | Tags, Hint, display kinds, hint parentheses     |
//! | P1.5 | Comprehensive golden        | full, defaults               | USAGE/DESCRIPTION/PARAMETERS/EXAMPLES/TAGS      |
//! | P1.6 | Standard minimal fixture    | minimal, defaults            | no version, no arguments, no examples           |
//! | P2.1 | hide_version                | full, hide_version           | no `(v...)` suffix                              |
//! | P2.2 | hide_status                 | full, hide_status            | no Status line                                  |
//! | P2.3 | hide_aliases                | full, hide_aliases           | no Aliases line                                 |
//! | P2.4 | hide_tags                   | full, hide_tags, Detailed    | no Tags line                                    |
//! | P3.1 | param page, everything      | full param `src`             | usage, desc+hint, facts, validation, examples   |
//! | P3.2 | param page, hint fallback   | param `force`                | hint as description, default shown              |
//! | P3.3 | param page, choices+multiple| enum param                   | Choices row and Multiple row                    |

#![ cfg( feature = "enabled" ) ]

use unilang_help::{ HelpCommandData, HelpParamData, HelpDisplayOptions, HelpVerbosity, PlainRenderer };

mod inc;
use inc::{ full_fixture, minimal_fixture };

/// P1.1: Minimal is a single line with no trailing newline.
#[ test ]
fn minimal_golden()
{
  let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Minimal );
  assert_eq!
  (
    renderer.render( &full_fixture() ),
    ".file.copy - Copy a file from source to destination."
  );
}

/// P1.2: Basic adds the compact-kind parameter list.
#[ test ]
fn basic_golden()
{
  let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Basic );
  let expected = concat!
  (
    ".file.copy - Copy a file from source to destination.\n",
    "\n",
    "PARAMETERS:\n",
    "  src::path\n",
    "  force::boolean\n",
    "  tags::list\n",
  );
  assert_eq!( renderer.render( &full_fixture() ), expected );
}

/// P1.3: Standard shows usage, metadata, arguments with rules, and numbered examples.
#[ test ]
fn standard_golden()
{
  let renderer = PlainRenderer::default();
  let expected = concat!
  (
    "Usage: .file.copy (v1.2.0)\n",
    "Copy a file from source to destination.\n",
    "\n",
    "Status: Active\n",
    "Aliases: fc, copy\n",
    "\n",
    "Arguments:\n",
    "src (Type: path)\n",
    "  Source file path.\n",
    "  Rules: [min_length(1)]\n",
    "\n",
    "force (Type: boolean) - Optional\n",
    "  Overwrite without asking.\n",
    "\n",
    "tags (Type: list) - Optional, Multiple\n",
    "  Tag list.\n",
    "\n",
    "Examples:\n",
    "  1. .file.copy src::a.txt dst::b.txt\n",
    "  2. .file.copy src::a.txt force::true\n",
    "\n",
  );
  assert_eq!( renderer.render( &full_fixture() ), expected );
}

/// P1.4: Detailed uses display kinds, shows tags, hint header, and parenthesized param hints.
#[ test ]
fn detailed_golden()
{
  let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Detailed );
  let expected = concat!
  (
    "Usage: .file.copy (v1.2.0)\n",
    "Aliases: fc, copy\n",
    "Tags: fs, io\n",
    "\n",
    "  Hint: Copy files.\n",
    "  Copy a file from source to destination.\n",
    "\n",
    "Status: Active\n",
    "\n",
    "Arguments:\n",
    "src (Type: Path)\n",
    "  Source file path.\n",
    "  (The file to copy.)\n",
    "  Rules: [min_length(1)]\n",
    "\n",
    "force (Type: Boolean) - Optional\n",
    "  Overwrite without asking.\n",
    "\n",
    "tags (Type: List(String)) - Optional, Multiple\n",
    "  Tag list.\n",
    "\n",
  );
  assert_eq!( renderer.render( &full_fixture() ), expected );
}

/// P1.5: Comprehensive shows the sectioned long format.
#[ test ]
fn comprehensive_golden()
{
  let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Comprehensive );
  let expected = concat!
  (
    ".file.copy - Copy a file from source to destination.\n",
    "\n",
    "USAGE:\n",
    "  .file.copy src::<value> [force::<value>] [tags::<value>]\n",
    "\n",
    "DESCRIPTION:\n",
    "  Copy a file from source to destination.\n",
    "  Copy files.\n",
    "\n",
    "  Status: Active (v1.2.0)\n",
    "  Aliases: fc, copy\n",
    "\n",
    "PARAMETERS:\n",
    "\n",
    "  src::<value>\n",
    "    Source file path.\n",
    "    The file to copy.\n",
    "    Type: Path\n",
    "    Validation:\n",
    "      - min_length(1)\n",
    "\n",
    "  force::<value>\n",
    "    Overwrite without asking.\n",
    "    Type: Boolean\n",
    "    Optional: yes\n",
    "\n",
    "  tags::<value>\n",
    "    Tag list.\n",
    "    Type: List(String)\n",
    "    Optional: yes\n",
    "    Multiple values: yes\n",
    "\n",
    "EXAMPLES:\n",
    "  .file.copy src::a.txt dst::b.txt\n",
    "  .file.copy src::a.txt force::true\n",
    "\n",
    "TAGS: fs, io\n",
  );
  assert_eq!( renderer.render( &full_fixture() ), expected );
}

/// P1.6: A bare command with `show_version: false` renders no version, arguments, or examples.
#[ test ]
fn standard_minimal_fixture_golden()
{
  let renderer = PlainRenderer::default();
  let expected = concat!
  (
    "Usage: .ping\n",
    "Check liveness.\n",
    "\n",
    "Status: Active\n",
  );
  assert_eq!( renderer.render( &minimal_fixture() ), expected );
}

/// P2.1: `hide_version` removes the version suffix even when the command opts in.
#[ test ]
fn hide_version_option()
{
  let renderer = PlainRenderer::default().with_options( HelpDisplayOptions::default().hide_version() );
  let page = renderer.render( &full_fixture() );
  assert!( page.starts_with( "Usage: .file.copy\n" ) );
  assert!( !page.contains( "(v1.2.0)" ) );
}

/// P2.2: `hide_status` removes the Status line.
#[ test ]
fn hide_status_option()
{
  let renderer = PlainRenderer::default().with_options( HelpDisplayOptions::default().hide_status() );
  let page = renderer.render( &full_fixture() );
  assert!( !page.contains( "Status:" ) );
}

/// P2.3: `hide_aliases` removes the Aliases line.
#[ test ]
fn hide_aliases_option()
{
  let renderer = PlainRenderer::default().with_options( HelpDisplayOptions::default().hide_aliases() );
  let page = renderer.render( &full_fixture() );
  assert!( !page.contains( "Aliases:" ) );
}

/// P2.4: `hide_tags` removes the Tags line at Detailed verbosity.
#[ test ]
fn hide_tags_option()
{
  let renderer = PlainRenderer::default()
    .with_verbosity( HelpVerbosity::Detailed )
    .with_options( HelpDisplayOptions::default().hide_tags() );
  let page = renderer.render( &full_fixture() );
  assert!( !page.contains( "Tags:" ) );
}

/// P3.1: A fully-populated parameter page shows usage, both text lines, facts,
/// validation, and examples.
#[ test ]
fn param_page_full_golden()
{
  let cmd = full_fixture();
  let renderer = PlainRenderer::default();
  let expected = concat!
  (
    "Parameter: src\n",
    "  .file.copy src::<path>\n",
    "\n",
    "Source file path.\n",
    "The file to copy.\n",
    "\n",
    "Kind: Path\n",
    "Required: yes\n",
    "Aliases: s\n",
    "Validation:\n",
    "  - min_length(1)\n",
    "\n",
    "Examples:\n",
    "  .file.copy src::a.txt\n",
  );
  assert_eq!( renderer.render_param( &cmd, &cmd.params[ 0 ] ), expected );
}

/// P3.2: An empty description falls back to the hint; the default value is shown.
#[ test ]
fn param_page_hint_fallback_golden()
{
  let cmd = full_fixture();
  let renderer = PlainRenderer::default();
  let expected = concat!
  (
    "Parameter: force\n",
    "  .file.copy force::<boolean>\n",
    "\n",
    "Overwrite without asking.\n",
    "\n",
    "Kind: Boolean\n",
    "Required: no\n",
    "Default: false\n",
  );
  assert_eq!( renderer.render_param( &cmd, &cmd.params[ 1 ] ), expected );
}

/// P3.3: Enum choices render as a Choices row; multiple-value params get a Multiple row.
#[ test ]
fn param_page_choices_and_multiple()
{
  let mut param = HelpParamData::default();
  param.name = "scope".into();
  param.kind = "Enum".into();
  param.kind_compact = "enum".into();
  param.description = "Discovery strategy selector.".into();
  param.optional = true;
  param.multiple = true;
  param.choices = vec![ "local".into(), "global".into() ];

  let mut cmd = HelpCommandData::default();
  cmd.name = ".rollup".into();

  let renderer = PlainRenderer::default();
  let expected = concat!
  (
    "Parameter: scope\n",
    "  .rollup scope::<enum>\n",
    "\n",
    "Discovery strategy selector.\n",
    "\n",
    "Kind: Enum\n",
    "Required: no\n",
    "Multiple: yes\n",
    "Choices: local, global\n",
  );
  assert_eq!( renderer.render_param( &cmd, &param ), expected );
}
