//! Shared help-data fixtures for renderer tests.

use unilang_help::{ HelpCommandData, HelpParamData };

/// Fixture with every model field populated: three parameters covering
/// required/optional, hint fallback, multiple values, rules, and aliases.
pub fn full_fixture() -> HelpCommandData
{
  let mut src = HelpParamData::default();
  src.name = "src".into();
  src.kind = "Path".into();
  src.kind_compact = "path".into();
  src.description = "Source file path.".into();
  src.hint = "The file to copy.".into();
  src.validation_rules = vec![ "min_length(1)".into() ];
  src.aliases = vec![ "s".into() ];
  src.examples = vec![ ".file.copy src::a.txt".into() ];

  let mut force = HelpParamData::default();
  force.name = "force".into();
  force.kind = "Boolean".into();
  force.kind_compact = "boolean".into();
  force.hint = "Overwrite without asking.".into();
  force.optional = true;
  force.default = Some( "false".into() );

  let mut tags = HelpParamData::default();
  tags.name = "tags".into();
  tags.kind = "List(String)".into();
  tags.kind_compact = "list".into();
  tags.description = "Tag list.".into();
  tags.hint = "Tag list.".into();
  tags.optional = true;
  tags.multiple = true;

  let mut cmd = HelpCommandData::default();
  cmd.name = ".file.copy".into();
  cmd.description = "Copy a file from source to destination.".into();
  cmd.hint = "Copy files.".into();
  cmd.version = "1.2.0".into();
  cmd.status = "Active".into();
  cmd.show_version = true;
  cmd.aliases = vec![ "fc".into(), "copy".into() ];
  cmd.tags = vec![ "fs".into(), "io".into() ];
  cmd.examples = vec!
  [
    ".file.copy src::a.txt dst::b.txt".into(),
    ".file.copy src::a.txt force::true".into(),
  ];
  cmd.params = vec![ src, force, tags ];
  cmd
}

/// Bare fixture: description and status only, `show_version` off.
pub fn minimal_fixture() -> HelpCommandData
{
  let mut cmd = HelpCommandData::default();
  cmd.name = ".ping".into();
  cmd.description = "Check liveness.".into();
  cmd.status = "Active".into();
  cmd
}
