//! Help verbosity levels and global display options.

///
/// Help verbosity levels controlling output detail.
///
/// Controls the amount of information displayed in help text, from minimal
/// single-line output to comprehensive documentation. The default level is
/// Standard (Level 2), which provides a good balance of conciseness and completeness.
///
/// # Levels
///
/// - **Level 0 (Minimal)**: Command name and one-line description only
///   - Use case: Quick reference, command discovery
///   - Example: `.config - Display current configuration and sources`
///
/// - **Level 1 (Basic)**: Add parameters list with types
///   - Use case: Syntax lookup, remembering parameter names
///   - Shows: Command description + parameter list
///
/// - **Level 2 (Standard)**: Concise help with usage, parameters, examples (DEFAULT)
///   - Use case: Terminal use, getting started quickly
///   - Shows: USAGE, PARAMETERS with descriptions, EXAMPLES
///   - Inspired by unikit-style concise formatting
///
/// - **Level 3 (Detailed)**: Full metadata including validation rules, aliases, tags
///   - Use case: Comprehensive documentation, understanding constraints
///   - Shows: All command metadata, validation rules, version info
///
/// - **Level 4 (Comprehensive)**: Extensive explanations with rationale and use cases
///   - Use case: Learning, documentation generation, detailed references
///   - Shows: Extended format with rationale and explanations
///   - Inspired by runbox-style comprehensive formatting
///
/// # Environment Variable
///
/// The verbosity level can be controlled via the `UNILANG_HELP_VERBOSITY` environment
/// variable (values 0-4). Values above 4 are capped at Comprehensive.
///
/// # Examples
///
/// ```rust
/// use unilang_help::HelpVerbosity;
///
/// // Parse from integer
/// let level = HelpVerbosity::from_level( 2 );
/// assert_eq!( level, HelpVerbosity::Standard );
///
/// // Read from environment variable
/// let level = HelpVerbosity::from_env();
///
/// // Default is Standard (Level 2)
/// assert_eq!( HelpVerbosity::default(), HelpVerbosity::Standard );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default ) ]
pub enum HelpVerbosity
{
  /// Level 0: Just command name and brief description
  Minimal = 0,
  /// Level 1: Add parameters list with types only
  Basic = 1,
  /// Level 2: Standard concise help (USAGE, PARAMETERS, EXAMPLES) - DEFAULT
  #[ default ]
  Standard = 2,
  /// Level 3: Detailed help with all metadata
  Detailed = 3,
  /// Level 4: Comprehensive with extensive explanations
  Comprehensive = 4,
}

impl HelpVerbosity
{
  /// Parse verbosity level from integer (0-4)
  #[ must_use ]
  pub fn from_level( level : u8 ) -> Self
  {
    match level
    {
      0 => Self::Minimal,
      1 => Self::Basic,
      2 => Self::Standard,
      3 => Self::Detailed,
      4.. => Self::Comprehensive,
    }
  }

  /// Read verbosity level from environment variable UNILANG_HELP_VERBOSITY.
  /// Falls back to default (Level 2: Standard) if not set or invalid.
  #[ must_use ]
  pub fn from_env() -> Self
  {
    std::env::var( "UNILANG_HELP_VERBOSITY" )
      .ok()
      .and_then( |v| v.parse::< u8 >().ok() )
      .map( Self::from_level )
      .unwrap_or_default()
  }
}

/// Global configuration for help output display.
///
/// This struct controls which metadata fields appear in help output
/// across all commands. Per-command settings (like `HelpCommandData::show_version`)
/// can override these defaults.
///
/// # Environment Variable Support
///
/// - `UNILANG_HELP_HIDE_VERSION=1` - Disables version display globally
///
/// # Examples
///
/// ```rust
/// use unilang_help::HelpDisplayOptions;
///
/// // Default: show everything
/// let options = HelpDisplayOptions::default();
/// assert!( options.show_version );
///
/// // Hide version globally
/// let options = HelpDisplayOptions::default().hide_version();
/// assert!( !options.show_version );
///
/// // Read from environment
/// let options = HelpDisplayOptions::default().with_env_overrides();
/// ```
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct HelpDisplayOptions
{
  /// Show version in help output (default: true)
  pub show_version : bool,
  /// Show status in help output (default: true)
  pub show_status : bool,
  /// Show aliases in help output (default: true)
  pub show_aliases : bool,
  /// Show tags in help output at detailed verbosity (default: true)
  pub show_tags : bool,
}

impl Default for HelpDisplayOptions
{
  fn default() -> Self
  {
    Self
    {
      show_version : true,
      show_status : true,
      show_aliases : true,
      show_tags : true,
    }
  }
}

impl HelpDisplayOptions
{
  /// Create options with version display disabled.
  #[ must_use ]
  pub fn hide_version( mut self ) -> Self
  {
    self.show_version = false;
    self
  }

  /// Create options with status display disabled.
  #[ must_use ]
  pub fn hide_status( mut self ) -> Self
  {
    self.show_status = false;
    self
  }

  /// Create options with aliases display disabled.
  #[ must_use ]
  pub fn hide_aliases( mut self ) -> Self
  {
    self.show_aliases = false;
    self
  }

  /// Create options with tags display disabled.
  #[ must_use ]
  pub fn hide_tags( mut self ) -> Self
  {
    self.show_tags = false;
    self
  }

  /// Check environment variable overrides and apply them.
  ///
  /// Supported environment variables:
  /// - `UNILANG_HELP_HIDE_VERSION=1` - Hides version from help output
  #[ must_use ]
  pub fn with_env_overrides( mut self ) -> Self
  {
    if std::env::var( "UNILANG_HELP_HIDE_VERSION" ).is_ok()
    {
      self.show_version = false;
    }
    self
  }
}
