//! Static argument type definitions.

use super::*;

  ///
  /// Static, const-compatible version of `ArgumentDefinition`.
  ///
  #[ derive( Debug, Clone, Copy ) ]
  pub struct StaticArgumentDefinition
  {
    /// The name of the argument, used to reference it in commands and validation.
    pub name : &'static str,
    /// The data type and structure expected for this argument.
    pub kind : StaticKind,
    /// Attributes that control the behavior of this argument.
    pub attributes : StaticArgumentAttributes,
    /// A brief, one-line hint about the argument's purpose.
    pub hint : &'static str,
    /// A more detailed description of the argument.
    pub description : &'static str,
    /// Validation rules that apply to this argument.
    pub validation_rules : &'static [ StaticValidationRule ],
    /// Alternative names for this argument.
    pub aliases : &'static [ &'static str ],
    /// Tags associated with this argument.
    pub tags : &'static [ &'static str ],
  }

  impl StaticArgumentDefinition
  {
    /// Creates a new `StaticArgumentDefinition` with required fields and sensible defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::static_data::{ StaticArgumentDefinition, StaticKind };
    ///
    /// const ARG: StaticArgumentDefinition = StaticArgumentDefinition::new(
    ///   "name",
    ///   StaticKind::String,
    ///   "User name",
    /// );
    /// assert_eq!( ARG.name, "name" );
    /// ```
    #[ must_use ]
    pub const fn new(
      name : &'static str,
      kind : StaticKind,
      description : &'static str,
    ) -> Self
    {
      Self
      {
        name,
        kind,
        attributes : StaticArgumentAttributes::new(),
        hint : "",
        description,
        validation_rules : &[],
        aliases : &[],
        tags : &[],
      }
    }

    /// Sets the attributes for the argument.
    #[ must_use ]
    pub const fn with_attributes( mut self, attributes : StaticArgumentAttributes ) -> Self
    {
      self.attributes = attributes;
      self
    }

    /// Sets the hint for the argument.
    #[ must_use ]
    pub const fn with_hint( mut self, hint : &'static str ) -> Self
    {
      self.hint = hint;
      self
    }

    /// Sets the validation rules for the argument.
    #[ must_use ]
    pub const fn with_validation_rules( mut self, validation_rules : &'static [ StaticValidationRule ] ) -> Self
    {
      self.validation_rules = validation_rules;
      self
    }

    /// Sets the aliases for the argument.
    #[ must_use ]
    pub const fn with_aliases( mut self, aliases : &'static [ &'static str ] ) -> Self
    {
      self.aliases = aliases;
      self
    }

    /// Sets the tags for the argument.
    #[ must_use ]
    pub const fn with_tags( mut self, tags : &'static [ &'static str ] ) -> Self
    {
      self.tags = tags;
      self
    }
  }

  ///
  /// Static, const-compatible version of `ArgumentAttributes`.
  ///
  #[allow(clippy::struct_excessive_bools)]
  #[ derive( Debug, Clone, Copy ) ]
  pub struct StaticArgumentAttributes
  {
    /// Indicates if the argument is optional.
    pub optional : bool,
    /// Indicates if the argument can accept multiple values.
    pub multiple : bool,
    /// The default value for the argument if not provided.
    pub default : Option< &'static str >,
    /// Indicates if the argument contains sensitive data.
    pub sensitive : bool,
    /// Indicates if the argument might require user interaction.
    pub interactive : bool,
  }

  impl Default for StaticArgumentAttributes
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl StaticArgumentAttributes
  {
    /// Creates a new `StaticArgumentAttributes` with sensible defaults.
    ///
    /// Defaults: required (not optional), single value, no default, not sensitive, not interactive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unilang::static_data::StaticArgumentAttributes;
    ///
    /// const ATTRS: StaticArgumentAttributes = StaticArgumentAttributes::new();
    /// const OPTIONAL_ATTRS: StaticArgumentAttributes = StaticArgumentAttributes::new().with_optional( true );
    /// assert!( !ATTRS.optional );
    /// assert!( OPTIONAL_ATTRS.optional );
    /// ```
    #[ must_use ]
    pub const fn new() -> Self
    {
      Self
      {
        optional : false,
        multiple : false,
        default : None,
        sensitive : false,
        interactive : false,
      }
    }

    /// Sets whether the argument is optional.
    #[ must_use ]
    pub const fn with_optional( mut self, optional : bool ) -> Self
    {
      self.optional = optional;
      self
    }

    /// Sets whether the argument can accept multiple values.
    #[ must_use ]
    pub const fn with_multiple( mut self, multiple : bool ) -> Self
    {
      self.multiple = multiple;
      self
    }

    /// Sets the default value for the argument.
    #[ must_use ]
    pub const fn with_default( mut self, default : &'static str ) -> Self
    {
      self.default = Some( default );
      self
    }

    /// Sets whether the argument contains sensitive data.
    #[ must_use ]
    pub const fn with_sensitive( mut self, sensitive : bool ) -> Self
    {
      self.sensitive = sensitive;
      self
    }

    /// Sets whether the argument might require user interaction.
    #[ must_use ]
    pub const fn with_interactive( mut self, interactive : bool ) -> Self
    {
      self.interactive = interactive;
      self
    }
  }
