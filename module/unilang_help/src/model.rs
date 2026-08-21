//! Renderer-independent help-page domain model.
//!
//! The model carries pre-rendered strings only — no dependency on any command
//! framework's type system. The producing side (e.g. `unilang`'s help adapter)
//! converts its own definitions into these structs; every renderer in this
//! crate consumes them unchanged.

/// Help data for one command parameter.
///
/// `#[ non_exhaustive ]` blocks external struct literals so fields can be added
/// without breaking consumers. Construct via `Default` + field assignment:
///
/// ```
/// use unilang_help::HelpParamData;
/// let mut param = HelpParamData::default();
/// param.name = "scope".into();
/// param.kind = "Enum".into();
/// param.kind_compact = "enum".into();
/// param.choices = vec![ "local".into(), "global".into() ];
/// assert!( !param.optional );
/// ```
///
/// External struct expressions fail to compile (E0639):
///
/// ```compile_fail
/// let _ = unilang_help::HelpParamData
/// {
///   name : String::new(), kind : String::new(), kind_compact : String::new(),
///   description : String::new(), hint : String::new(), optional : false,
///   multiple : false, default : None, choices : vec![], validation_rules : vec![],
///   aliases : vec![], examples : vec![],
/// };
/// ```
#[ non_exhaustive ]
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
pub struct HelpParamData
{
  /// Parameter name as typed by the user (e.g. `"scope"`).
  pub name : String,
  /// Type name in display form for detailed contexts (e.g. `"String"`, `"List(String)"`).
  pub kind : String,
  /// Lowercased short type form used by compact verbosity levels (e.g. `"string"`).
  pub kind_compact : String,
  /// Full description; may be empty.
  pub description : String,
  /// Short hint; compact levels fall back to it when `description` is empty.
  pub hint : String,
  /// Whether the parameter may be omitted.
  pub optional : bool,
  /// Whether the parameter accepts multiple values.
  pub multiple : bool,
  /// Pre-rendered default value, when one exists.
  pub default : Option< String >,
  /// Valid values for enum-like kinds; empty for open-ended kinds.
  pub choices : Vec< String >,
  /// Pre-rendered validation rule descriptions.
  pub validation_rules : Vec< String >,
  /// Alternative names accepted for this parameter.
  pub aliases : Vec< String >,
  /// Usage examples relevant to this specific parameter.
  pub examples : Vec< String >,
}

/// Help data for one command, including its parameters.
///
/// `#[ non_exhaustive ]` blocks external struct literals; construct via
/// `Default` + field assignment:
///
/// ```
/// use unilang_help::{ HelpCommandData, HelpParamData };
/// let mut cmd = HelpCommandData::default();
/// cmd.name = ".file.copy".into();
/// cmd.description = "Copy a file.".into();
/// cmd.params.push( HelpParamData::default() );
/// assert_eq!( cmd.params.len(), 1 );
/// ```
///
/// External struct expressions fail to compile (E0639):
///
/// ```compile_fail
/// let _ = unilang_help::HelpCommandData
/// {
///   name : String::new(), description : String::new(), hint : String::new(),
///   version : String::new(), status : String::new(), show_version : true,
///   aliases : vec![], tags : vec![], examples : vec![], params : vec![],
/// };
/// ```
#[ non_exhaustive ]
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
pub struct HelpCommandData
{
  /// Full command name including namespace (e.g. `".file.copy"`).
  pub name : String,
  /// Full description; may be empty.
  pub description : String,
  /// Short hint shown alongside or instead of the description.
  pub hint : String,
  /// Version string; empty when the command declares none.
  pub version : String,
  /// Pre-rendered status (e.g. `"Active"`, `"Deprecated"`).
  pub status : String,
  /// Per-command flag: whether the version belongs in help output at all.
  /// Renderers additionally honor the global `HelpDisplayOptions::show_version`.
  pub show_version : bool,
  /// Alternative names for the whole command.
  pub aliases : Vec< String >,
  /// Classification tags.
  pub tags : Vec< String >,
  /// Whole-command usage examples.
  pub examples : Vec< String >,
  /// Parameters in declaration order.
  pub params : Vec< HelpParamData >,
}
