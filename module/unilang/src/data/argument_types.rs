//! Argument type definitions and validation
//!
//! Provides type-safe argument definition structures including ArgumentAttributes,
//! ArgumentDefinition, Kind enum for type specification, and ValidationRule for
//! runtime validation constraints.

use crate::error::Error;
use super::kind::Kind;

  ///
  /// Holds attributes and configuration for a specific argument within a command.
  ///
  /// This struct enables fine-grained control over how arguments behave,
  /// such as whether they are required, accept multiple values, or have
  /// default values.
  #[allow(clippy::struct_excessive_bools)]
  #[ derive( Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize ) ]
  pub struct ArgumentAttributes
  {
    /// Indicates if the argument is optional.
    /// If true, the argument can be omitted without causing validation errors.
    pub optional : bool,
    /// Indicates if the argument can accept multiple values.
    /// If true, the argument can be provided multiple times in a single command invocation.
    pub multiple : bool,
    /// The default value for the argument if not provided.
    /// Only applicable when the argument is optional.
    pub default : Option< String >,
    /// Indicates if the argument contains sensitive data (e.g., passwords).
    /// If true, the argument might be masked or logged differently.
    pub sensitive : bool,
    /// Indicates if the argument might require user interaction (e.g., prompts).
    /// If true, the system may need to handle interactive input.
    ///
    /// # REPL Implementation Notes
    /// 
    /// **Critical Behavior**: When `interactive: true` and the argument is required but not provided:
    /// - Semantic analysis returns `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED` error
    /// - REPL loops should catch this error and prompt for secure input
    /// - **Important**: Optional interactive arguments with defaults do NOT trigger the error
    /// 
    /// **Security Best Practices**:
    /// - Always combine with `sensitive: true` for passwords/API keys
    /// - Never log or store interactive argument values
    /// - Use secure input methods (masked input) in REPL implementations
    /// 
    /// **Common Pitfalls**:
    /// - ❌ Don't handle interactive prompts during command execution
    /// - ❌ Don't store interactive values in command history  
    /// - ✅ Handle interactive prompts at the REPL level before re-execution
    /// - ✅ Clear sensitive values from memory after use
    pub interactive : bool,
  }

  ///
  /// Defines an argument within a command, including its name, type, and constraints.
  ///
  /// This struct provides all the necessary information to parse, validate,
  /// and process a single argument within a command.
  ///
  /// # Design Note: Public Fields
  ///
  /// Unlike `CommandDefinition` (which uses private fields for invariant enforcement), `ArgumentDefinition`
  /// retains public fields for pragmatic reasons:
  ///
  /// **Why not private fields here?**
  ///
  /// 1. **Complexity vs benefit**: Arguments are components of commands, not top-level
  ///    domain objects. The validation burden is lower since invalid arguments are caught
  ///    during command validation anyway.
  ///
  /// 2. **Ergonomics**: Arguments are frequently constructed inline in tests and examples.
  ///    Public fields with `Default` trait provide good ergonomics for simple cases.
  ///
  /// 3. **Former integration**: Using `former::Former` derive provides type-safe building
  ///    without custom implementation complexity. The macro generates a full-featured
  ///    builder automatically.
  ///
  /// 4. **Scope**: The type-safe redesign focused on `CommandDefinition` as the
  ///    primary domain object. Arguments could be redesigned further if needed.
  ///
  /// **Trade-off:**
  ///
  /// - Public fields allow mutation after construction (less safe)
  /// - But validation happens at command registration (catches errors before use)
  /// - Simpler implementation (uses derive macro vs custom builder)
  ///
  /// **Future evolution:**
  ///
  /// If mutation bugs appear or validation gaps are discovered, `ArgumentDefinition`
  /// could undergo similar redesign to `CommandDefinition` (private fields, validated
  /// newtypes, custom builder). Current design is pragmatic given actual usage patterns.
  ///
  #[ derive( Debug, Clone, serde::Serialize, serde::Deserialize, former::Former ) ]
  pub struct ArgumentDefinition
  {
    /// The name of the argument, used to reference it in commands and validation.
    pub name : String,
    /// The data type and structure expected for this argument.
    pub kind : Kind,
    /// Attributes that control the behavior of this argument.
    pub attributes : ArgumentAttributes,
    /// A brief, one-line hint about the argument's purpose.
    pub hint : String,
    /// A more detailed description of the argument.
    pub description : String,
    /// Validation rules that apply to this argument.
    pub validation_rules : Vec< ValidationRule >,
    /// Alternative names for this argument.
    pub aliases : Vec< String >,
    /// Tags associated with this argument.
    pub tags : Vec< String >,
  }

  impl ArgumentDefinition
  {
    /// Creates a new argument with sensible defaults (simplified constructor).
    ///
    /// This is the recommended way to create arguments for most use cases.
    /// It requires only the essential fields (name and type) and provides
    /// reasonable defaults for all optional fields.
    ///
    /// # Arguments
    /// * `name` - Argument name
    /// * `kind` - Argument type (String, Integer, etc.)
    ///
    /// # Returns
    /// * `Self` - A new ArgumentDefinition with defaults applied
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// // Simple required string argument
    /// let arg = ArgumentDefinition::new("name", Kind::String);
    /// assert_eq!(arg.name, "name");
    /// assert!(!arg.attributes.optional);
    ///
    /// // Optional integer with default
    /// let count_arg = ArgumentDefinition::new("count", Kind::Integer)
    ///   .with_optional(Some("10"));
    /// assert!(count_arg.attributes.optional);
    /// ```
    #[ must_use ]
    pub fn new( name : impl Into< String >, kind : Kind ) -> Self
    {
      Self
      {
        name : name.into(),
        kind,
        attributes : ArgumentAttributes::default(),
        hint : String::new(),
        description : String::new(),
        validation_rules : Vec::new(),
        aliases : Vec::new(),
        tags : Vec::new(),
      }
    }

    /// Makes the argument optional with an optional default value (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("count", Kind::Integer)
    ///   .with_optional(Some("10"));
    /// assert!(arg.attributes.optional);
    /// assert_eq!(arg.attributes.default, Some("10".to_string()));
    /// ```
    #[ must_use ]
    pub fn with_optional( mut self, default : Option< impl Into< String > > ) -> Self
    {
      self.attributes.optional = true;
      self.attributes.default = default.map( | v | v.into() );
      self
    }

    /// Sets the description for the argument (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("name", Kind::String)
    ///   .with_description("User's name");
    /// assert_eq!(arg.description, "User's name");
    /// ```
    #[ must_use ]
    pub fn with_description( mut self, description : impl Into< String > ) -> Self
    {
      self.description = description.into();
      self
    }

    /// Adds validation rules to the argument (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    /// use unilang::ValidationRule;
    ///
    /// let arg = ArgumentDefinition::new("age", Kind::Integer)
    ///   .with_validation_rules(vec![
    ///     ValidationRule::Min(0.0),
    ///     ValidationRule::Max(150.0),
    ///   ]);
    /// assert_eq!(arg.validation_rules.len(), 2);
    /// ```
    #[ must_use ]
    pub fn with_validation_rules( mut self, rules : Vec< ValidationRule > ) -> Self
    {
      self.validation_rules = rules;
      self
    }

    /// Marks the argument as sensitive (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("password", Kind::String)
    ///   .with_sensitive(true);
    /// assert!(arg.attributes.sensitive);
    /// ```
    #[ must_use ]
    pub fn with_sensitive( mut self, sensitive : bool ) -> Self
    {
      self.attributes.sensitive = sensitive;
      self
    }

    /// Marks the argument as interactive (fluent API).
    ///
    /// # Examples
    /// ```rust
    /// use unilang::prelude::*;
    ///
    /// let arg = ArgumentDefinition::new("password", Kind::String)
    ///   .with_interactive(true)
    ///   .with_sensitive(true);
    /// assert!(arg.attributes.interactive);
    /// assert!(arg.attributes.sensitive);
    /// ```
    #[ must_use ]
    pub fn with_interactive( mut self, interactive : bool ) -> Self
    {
      self.attributes.interactive = interactive;
      self
    }
  }

  /// Validation rule for argument values.
  #[ derive( Debug, Clone, PartialEq, serde::Serialize ) ]
  pub enum ValidationRule
  {
    /// Minimum value for numeric types.
    Min( f64 ),
    /// Maximum value for numeric types.
    Max( f64 ),
    /// Minimum length for string types.
    MinLength( usize ),
    /// Maximum length for string types.
    MaxLength( usize ),
    /// Pattern that string values must match.
    Pattern( String ),
    /// Minimum number of items for collection types.
    MinItems( usize ),
  }

  impl core::str::FromStr for ValidationRule
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let s = s.trim();
      if s.starts_with( "min:" )
      {
        let value_str = s.strip_prefix( "min:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid min value: {e}" ) ) )?;
        Ok( ValidationRule::Min( value ) )
      }
      else if s.starts_with( "max:" )
      {
        let value_str = s.strip_prefix( "max:" ).unwrap();
        let value : f64 = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid max value: {e}" ) ) )?;
        Ok( ValidationRule::Max( value ) )
      }
      else if s.starts_with( "minlength:" )
      {
        let value_str = s.strip_prefix( "minlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minlength value: {e}" ) ) )?;
        Ok( ValidationRule::MinLength( value ) )
      }
      else if s.starts_with( "maxlength:" )
      {
        let value_str = s.strip_prefix( "maxlength:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid maxlength value: {e}" ) ) )?;
        Ok( ValidationRule::MaxLength( value ) )
      }
      else if s.starts_with( "pattern:" )
      {
        let pattern = s.strip_prefix( "pattern:" ).unwrap();
        Ok( ValidationRule::Pattern( pattern.to_string() ) )
      }
      else if s.starts_with( "minitems:" )
      {
        let value_str = s.strip_prefix( "minitems:" ).unwrap();
        let value : usize = value_str.parse().map_err( | e | Error::Registration( format!( "Invalid minitems value: {e}" ) ) )?;
        Ok( ValidationRule::MinItems( value ) )
      }
      else
      {
        Err( Error::Registration( format!( "Unknown validation rule: {s}" ) ) )
      }
    }
  }

  impl< 'de > serde::Deserialize< 'de > for ValidationRule
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      s.parse().map_err( serde::de::Error::custom )
    }
  }
