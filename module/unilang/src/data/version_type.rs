//!  — validated non-empty version string.

use crate::error::Error;


  ///
  /// A validated version string.
  ///
  /// # Type Safety Guarantees
  /// - Cannot be empty
  /// - Basic format validation (semver-like)
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate version strings?**
  ///
  /// Command versions track API changes and help users understand stability.
  /// The old API accepted any String including empty strings, leading to problems:
  ///
  /// ```text
  /// // Old API - all compile fine but semantically wrong
  /// let cmd1 = CommandDefinition { version: "".to_string(), ... }; // Empty!
  /// let cmd2 = CommandDefinition { version: "version 1".to_string(), ... }; // Invalid format
  /// let cmd3 = CommandDefinition { version: "latest".to_string(), ... }; // Not a version
  /// ```
  ///
  /// **Validation rules:**
  ///
  /// Current validation is minimal (non-empty only) to remain flexible while preventing
  /// the most obvious errors. Future enhancements could enforce strict semver.
  ///
  /// **Why not full semver parsing?**
  ///
  /// Trade-off decision: Strict semver (`1.2.3` only) would break valid use cases like:
  /// - `"2.1"` (two-part versions)
  /// - `"1.0.0-alpha"` (pre-release versions)
  /// - `"0.1"` (development versions)
  ///
  /// Current approach: Validate non-empty, allow flexible formats. This catches the
  /// most common error (empty string) without being overly restrictive.
  ///
  /// **Design evolution:**
  ///
  /// Current: non-empty validation catches the most common error without over-constraining.
  /// Future: could add optional strict semver mode if needed.
  ///
  /// **Migration impact:**
  ///
  /// Old: `version: "1.0.0".to_string()` or default `"1.0.0"`
  /// New: `version: VersionType::new("1.0.0").unwrap()` or builder handles conversion
  ///
  /// The builder provides `"1.0.0"` default, making most migrations transparent.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::VersionType;
  ///
  /// // Valid versions
  /// let v = VersionType::new("1.0.0").expect("valid");
  /// assert_eq!(v.as_str(), "1.0.0");
  ///
  /// let v2 = VersionType::new("2.1").expect("valid");
  /// assert_eq!(v2.as_str(), "2.1");
  ///
  /// // Invalid - empty version
  /// assert!(VersionType::new("").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct VersionType( String );

  impl VersionType
  {
    ///
    /// Creates a new VersionType with validation.
    ///
    /// # Validation Rules
    /// 1. Version cannot be empty
    ///
    /// # Arguments
    /// * `version` - The version string to validate
    ///
    /// # Returns
    /// * `Ok(VersionType)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let valid = VersionType::new("1.0.0");
    /// assert!(valid.is_ok());
    ///
    /// let empty = VersionType::new("");
    /// assert!(empty.is_err());
    /// ```
    pub fn new( version : impl Into< String > ) -> Result< Self, Error >
    {
      let version = version.into();

      // Validation Rule 1: Version cannot be empty
      if version.is_empty()
      {
        return Err( Error::Registration(
          "Invalid version: version string cannot be empty".to_string()
        ));
      }

      Ok( Self( version ) )
    }

    ///
    /// Returns the version as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let v = VersionType::new("1.0.0").unwrap();
    /// assert_eq!(v.as_str(), "1.0.0");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the VersionType and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::VersionType;
    ///
    /// let v = VersionType::new("1.0.0").unwrap();
    /// let inner : String = v.into_inner();
    /// assert_eq!(inner, "1.0.0");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }
  }

  impl std::fmt::Display for VersionType
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for VersionType
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for VersionType
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      VersionType::new( s ).map_err( serde::de::Error::custom )
    }
  }
