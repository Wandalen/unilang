//!  — validated dot-prefixed command name.

use crate::error::Error;


  ///
  /// A validated command name that guarantees the dot prefix convention.
  ///
  /// # Type Safety Guarantees
  /// - Cannot be empty
  /// - Always starts with '.' prefix
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate at construction?**
  /// - Fail-fast principle: invalid names panic immediately
  /// - Type system guarantees all CommandName instances are valid
  /// - Eliminates need for runtime validation checks
  /// - Better error messages (MissingDotPrefix vs generic "invalid name")
  ///
  /// **Migration impact:**
  /// Tests expecting invalid names now panic at construction, not registration.
  /// This is **intended behavior** - invalid states are impossible to represent.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandName;
  ///
  /// // Valid construction
  /// let name = CommandName::new(".build").expect("valid name");
  /// assert_eq!(name.as_str(), ".build");
  ///
  /// // Invalid - empty name
  /// assert!(CommandName::new("").is_err());
  ///
  /// // Invalid - missing dot prefix
  /// assert!(CommandName::new("build").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct CommandName( String );

  impl CommandName
  {
    ///
    /// Creates a new CommandName with validation.
    ///
    /// # Validation Rules
    /// 1. Name cannot be empty
    /// 2. Name must start with '.' prefix
    ///
    /// # Arguments
    /// * `name` - The command name to validate
    ///
    /// # Returns
    /// * `Ok(CommandName)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let valid = CommandName::new(".test");
    /// assert!(valid.is_ok());
    ///
    /// let empty = CommandName::new("");
    /// assert!(empty.is_err());
    ///
    /// let no_prefix = CommandName::new("test");
    /// assert!(no_prefix.is_err());
    /// ```
    pub fn new( name : impl Into< String > ) -> Result< Self, Error >
    {
      let name = name.into();

      // Validation Rule 1: Name cannot be empty
      if name.is_empty()
      {
        return Err( Error::EmptyCommandName );
      }

      // Validation Rule 2: Name must start with '.'
      if !name.starts_with( '.' )
      {
        return Err( Error::MissingDotPrefix( name ) );
      }

      Ok( Self( name ) )
    }

    ///
    /// Returns the command name as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let name = CommandName::new(".build").unwrap();
    /// assert_eq!(name.as_str(), ".build");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the CommandName and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::CommandName;
    ///
    /// let name = CommandName::new(".build").unwrap();
    /// let inner : String = name.into_inner();
    /// assert_eq!(inner, ".build");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }
  }

  impl std::fmt::Display for CommandName
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for CommandName
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for CommandName
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      CommandName::new( s ).map_err( serde::de::Error::custom )
    }
  }

