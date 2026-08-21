//! Tests for `HelpVerbosity` and `HelpDisplayOptions`.
//!
//! ## Test Matrix
//!
//! **Test Factors:**
//! - Level Source: `from_level` integer, `from_env` variable, `Default`
//! - Level Value: in range (0-4), above range, invalid text, unset
//! - Option Source: `Default`, `hide_*` builders, `with_env_overrides`
//!
//! **Test Combinations:**
//!
//! | ID   | Aspect                  | Input                        | Expected                        |
//! |------|-------------------------|------------------------------|---------------------------------|
//! | V1.1 | from_level in range     | 0, 1, 2, 3, 4                | matching variants               |
//! | V1.2 | from_level above range  | 5, 255                       | Comprehensive                   |
//! | V1.3 | default level           | —                            | Standard                        |
//! | V1.4 | level ordering          | all variants                 | Minimal < ... < Comprehensive   |
//! | V2.1 | from_env valid          | UNILANG_HELP_VERBOSITY=4     | Comprehensive                   |
//! | V2.2 | from_env invalid text   | UNILANG_HELP_VERBOSITY=abc   | Standard (default)              |
//! | V2.3 | from_env unset          | variable removed             | Standard (default)              |
//! | V2.4 | from_env above range    | UNILANG_HELP_VERBOSITY=9     | Comprehensive                   |
//! | V3.1 | options default         | —                            | all four flags true             |
//! | V3.2 | hide builders           | each `hide_*`                | that flag false, others true    |
//! | V3.3 | env override set        | UNILANG_HELP_HIDE_VERSION=1  | show_version false              |
//! | V3.4 | env override unset      | variable removed             | show_version true               |

#![ cfg( feature = "enabled" ) ]

use unilang_help::{ HelpVerbosity, HelpDisplayOptions };

/// V1.1: Every in-range integer maps to its variant.
#[ test ]
fn from_level_in_range()
{
  assert_eq!( HelpVerbosity::from_level( 0 ), HelpVerbosity::Minimal );
  assert_eq!( HelpVerbosity::from_level( 1 ), HelpVerbosity::Basic );
  assert_eq!( HelpVerbosity::from_level( 2 ), HelpVerbosity::Standard );
  assert_eq!( HelpVerbosity::from_level( 3 ), HelpVerbosity::Detailed );
  assert_eq!( HelpVerbosity::from_level( 4 ), HelpVerbosity::Comprehensive );
}

/// V1.2: Values above 4 cap at Comprehensive.
#[ test ]
fn from_level_above_range_caps()
{
  assert_eq!( HelpVerbosity::from_level( 5 ), HelpVerbosity::Comprehensive );
  assert_eq!( HelpVerbosity::from_level( 255 ), HelpVerbosity::Comprehensive );
}

/// V1.3: The default level is Standard.
#[ test ]
fn default_is_standard()
{
  assert_eq!( HelpVerbosity::default(), HelpVerbosity::Standard );
}

/// V1.4: Levels order from Minimal to Comprehensive.
#[ test ]
fn levels_are_ordered()
{
  assert!( HelpVerbosity::Minimal < HelpVerbosity::Basic );
  assert!( HelpVerbosity::Basic < HelpVerbosity::Standard );
  assert!( HelpVerbosity::Standard < HelpVerbosity::Detailed );
  assert!( HelpVerbosity::Detailed < HelpVerbosity::Comprehensive );
}

/// V2.1: A valid environment value selects that level.
#[ test ]
fn from_env_valid_value()
{
  std::env::set_var( "UNILANG_HELP_VERBOSITY", "4" );
  assert_eq!( HelpVerbosity::from_env(), HelpVerbosity::Comprehensive );
  std::env::remove_var( "UNILANG_HELP_VERBOSITY" );
}

/// V2.2: Non-numeric environment text falls back to the default.
#[ test ]
fn from_env_invalid_text_falls_back()
{
  std::env::set_var( "UNILANG_HELP_VERBOSITY", "abc" );
  assert_eq!( HelpVerbosity::from_env(), HelpVerbosity::Standard );
  std::env::remove_var( "UNILANG_HELP_VERBOSITY" );
}

/// V2.3: An unset variable yields the default level.
#[ test ]
fn from_env_unset_falls_back()
{
  std::env::remove_var( "UNILANG_HELP_VERBOSITY" );
  assert_eq!( HelpVerbosity::from_env(), HelpVerbosity::Standard );
}

/// V2.4: An out-of-range environment value caps at Comprehensive.
#[ test ]
fn from_env_above_range_caps()
{
  std::env::set_var( "UNILANG_HELP_VERBOSITY", "9" );
  assert_eq!( HelpVerbosity::from_env(), HelpVerbosity::Comprehensive );
  std::env::remove_var( "UNILANG_HELP_VERBOSITY" );
}

/// V3.1: Default options show every metadata field.
#[ test ]
fn options_default_all_shown()
{
  let options = HelpDisplayOptions::default();
  assert!( options.show_version );
  assert!( options.show_status );
  assert!( options.show_aliases );
  assert!( options.show_tags );
}

/// V3.2: Each hide builder clears exactly its own flag.
#[ test ]
fn hide_builders_clear_single_flags()
{
  let options = HelpDisplayOptions::default().hide_version();
  assert!( !options.show_version );
  assert!( options.show_status && options.show_aliases && options.show_tags );

  let options = HelpDisplayOptions::default().hide_status();
  assert!( !options.show_status );
  assert!( options.show_version && options.show_aliases && options.show_tags );

  let options = HelpDisplayOptions::default().hide_aliases();
  assert!( !options.show_aliases );
  assert!( options.show_version && options.show_status && options.show_tags );

  let options = HelpDisplayOptions::default().hide_tags();
  assert!( !options.show_tags );
  assert!( options.show_version && options.show_status && options.show_aliases );
}

/// V3.3: `UNILANG_HELP_HIDE_VERSION` disables version display.
#[ test ]
fn env_override_hides_version()
{
  std::env::set_var( "UNILANG_HELP_HIDE_VERSION", "1" );
  let options = HelpDisplayOptions::default().with_env_overrides();
  assert!( !options.show_version );
  assert!( options.show_status && options.show_aliases && options.show_tags );
  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
}

/// V3.4: Without the variable, version display stays on.
#[ test ]
fn env_override_absent_keeps_version()
{
  std::env::remove_var( "UNILANG_HELP_HIDE_VERSION" );
  let options = HelpDisplayOptions::default().with_env_overrides();
  assert!( options.show_version );
}
