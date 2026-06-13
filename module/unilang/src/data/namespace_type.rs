//!  — validated empty-or-dot-prefixed namespace.

use crate::error::Error;

  ///
  /// A validated namespace that guarantees correct naming conventions.
  ///
  /// # Type Safety Guarantees
  /// - Empty namespace allowed (for root-level commands)
  /// - Non-empty namespaces must start with '.' prefix
  /// - Cannot be constructed with invalid values
  /// - Validation happens at construction time
  ///
  /// # Design Rationale
  ///
  /// **Why validate namespaces?**
  ///
  /// Namespaces organize commands into logical groups (e.g., `.video`, `.session`).
  /// The old API allowed invalid namespaces like `"video"` or `"session"` without dots,
  /// breaking the command naming convention and causing runtime errors.
  ///
  /// **Special case: Empty namespace**
  ///
  /// Unlike `CommandName`, empty namespace is valid and represents root-level commands.
  /// A command with `namespace=""` and `name=".help"` has `full_name=".help"`.
  /// This design allows both:
  /// - Root commands: `.help`, `.version`
  /// - Namespaced commands: `.video.search`, `.session.list`
  ///
  /// **Why not just use String?**
  ///
  /// ```text
  /// // Old API - compiles but breaks at runtime
  /// let mut cmd = CommandDefinition { namespace: "video".to_string(), ... };
  /// registry.register(cmd); // Runtime error: "Invalid namespace"
  /// ```
  ///
  /// With `NamespaceType`, invalid namespaces are caught at construction:
  ///
  /// ```
  /// use unilang::data::NamespaceType;
  /// let ns = NamespaceType::new("video");
  /// assert!(ns.is_err()); // Caught immediately, no runtime surprise
  /// ```
  ///
  /// **Migration impact:**
  ///
  /// Old: `namespace: "".to_string()` or `namespace: ".video".to_string()`
  /// New: `namespace: NamespaceType::new("").unwrap()` or builder with String conversion
  ///
  /// The builder API accepts `String` and validates internally, making migration smooth
  /// for most code while maintaining type safety at the boundary.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::NamespaceType;
  ///
  /// // Valid - empty namespace (root level)
  /// let root = NamespaceType::new("").expect("valid");
  /// assert_eq!(root.as_str(), "");
  ///
  /// // Valid - namespace with dot prefix
  /// let ns = NamespaceType::new(".video").expect("valid");
  /// assert_eq!(ns.as_str(), ".video");
  ///
  /// // Invalid - non-empty without dot prefix
  /// assert!(NamespaceType::new("video").is_err());
  /// ```
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct NamespaceType( String );

  impl NamespaceType
  {
    ///
    /// Creates a new NamespaceType with validation.
    ///
    /// # Validation Rules
    /// 1. Empty namespace is allowed (root-level commands)
    /// 2. Non-empty namespace must start with '.' prefix
    ///
    /// # Arguments
    /// * `namespace` - The namespace to validate
    ///
    /// # Returns
    /// * `Ok(NamespaceType)` - If validation passes
    /// * `Err(Error)` - If validation fails
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let empty = NamespaceType::new("");
    /// assert!(empty.is_ok());
    ///
    /// let valid = NamespaceType::new(".video");
    /// assert!(valid.is_ok());
    ///
    /// let invalid = NamespaceType::new("video");
    /// assert!(invalid.is_err());
    /// ```
    pub fn new( namespace : impl Into< String > ) -> Result< Self, Error >
    {
      let namespace = namespace.into();

      // Validation Rule 1: Empty namespace is allowed
      if namespace.is_empty()
      {
        return Ok( Self( namespace ) );
      }

      // Validation Rule 2: Non-empty namespace must start with '.'
      if !namespace.starts_with( '.' )
      {
        return Err( Error::Registration( format!(
          "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.video'). \
          Empty namespace is allowed for root-level commands.",
          namespace
        )));
      }

      Ok( Self( namespace ) )
    }

    ///
    /// Returns the namespace as a string slice.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// assert_eq!(ns.as_str(), ".video");
    /// ```
    pub fn as_str( &self ) -> &str
    {
      &self.0
    }

    ///
    /// Consumes the NamespaceType and returns the inner String.
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// let inner : String = ns.into_inner();
    /// assert_eq!(inner, ".video");
    /// ```
    pub fn into_inner( self ) -> String
    {
      self.0
    }

    ///
    /// Returns true if this is the root namespace (empty).
    ///
    /// # Examples
    /// ```
    /// use unilang::data::NamespaceType;
    ///
    /// let root = NamespaceType::new("").unwrap();
    /// assert!(root.is_root());
    ///
    /// let ns = NamespaceType::new(".video").unwrap();
    /// assert!(!ns.is_root());
    /// ```
    pub fn is_root( &self ) -> bool
    {
      self.0.is_empty()
    }
  }

  impl std::fmt::Display for NamespaceType
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.0 )
    }
  }

  impl serde::Serialize for NamespaceType
  {
    fn serialize< S >( &self, serializer : S ) -> Result< S::Ok, S::Error >
    where
      S : serde::Serializer,
    {
      serializer.serialize_str( &self.0 )
    }
  }

  impl< 'de > serde::Deserialize< 'de > for NamespaceType
  {
    fn deserialize< D >( deserializer : D ) -> Result< Self, D::Error >
    where
      D : serde::Deserializer< 'de >,
    {
      let s = String::deserialize( deserializer )?;
      NamespaceType::new( s ).map_err( serde::de::Error::custom )
    }
  }
