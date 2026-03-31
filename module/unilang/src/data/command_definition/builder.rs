//! Type-state builder for CommandDefinition with compile-time required field enforcement.

use std::marker::PhantomData;
use super::super::validated_types::{ CommandName, VersionType };
use super::super::command_status::CommandStatus;
use super::super::argument_types::ArgumentDefinition;
use super::core::CommandDefinition;

// Type-state markers for compile-time enforcement

/// Marker type indicating a required field has been set.
///
/// This zero-sized type is used in the type-state pattern to track
/// which required fields have been set in the `CommandDefinitionBuilder`.
#[ derive( Debug ) ]
pub struct Set;

/// Marker type indicating a required field has not been set.
///
/// This zero-sized type is used in the type-state pattern to track
/// which required fields have not yet been set in the `CommandDefinitionBuilder`.
#[ derive( Debug ) ]
pub struct NotSet;

/// Type-state builder for `CommandDefinition` that enforces required fields at compile time.
///
/// This builder uses the type-state pattern to ensure all 6 required fields
/// (name, description, namespace, hint, status, version) are set before building.
///
/// # Design Rationale
///
/// **Why type-state builder pattern?**
///
/// The old API used public fields with `Default` trait, causing runtime errors:
///
/// ```ignore
/// // Old API - compiles but panics at runtime
/// let cmd = CommandDefinition::default(); // name is empty string!
/// registry.register(cmd); // Runtime panic: "Invalid command name"
/// ```
///
/// **Problems with old approach:**
///
/// 1. **Invalid states representable:** Empty names, invalid versions compile fine
/// 2. **Runtime failures:** Validation only at registration time, not construction
/// 3. **Unclear requirements:** No indication which fields are truly required
/// 4. **No IDE help:** Autocomplete doesn't guide you through required fields
///
/// **Benefits of type-state builder:**
///
/// - **Compile-time enforcement:** Incomplete builders don't compile
/// - **Progressive disclosure:** Type signature shows what's left to set
/// - **IDE guidance:** Autocomplete only shows methods for unset fields
/// - **Impossible states impossible:** Can't construct invalid CommandDefinition
/// - **Self-documenting:** Type signature IS the documentation
///
/// **How it works:**
///
/// Each type parameter (`Name`, `Description`, etc.) is either `Set` or `NotSet`:
///
/// ```ignore
/// CommandDefinitionBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet> // Initial
///   .name(".test") → CommandDefinitionBuilder<Set, NotSet, NotSet, NotSet, NotSet, NotSet>
///   .description("Test") → CommandDefinitionBuilder<Set, Set, NotSet, NotSet, NotSet, NotSet>
///   .end() // OK - only requires Name and Description to be Set
/// ```
///
/// The `build()` method is only available when ALL type parameters are `Set`:
///
/// ```ignore
/// impl CommandDefinitionBuilder<Set, Set, Set, Set, Set, Set> {
///   pub fn build(self) -> CommandDefinition { ... }
/// }
/// ```
///
/// **Trade-off:** More complex type signatures, but catches errors at compile time instead
/// of runtime. This is a **good trade-off** for domain objects where invalid states
/// should be impossible.
///
#[ derive( Debug ) ]
pub struct CommandDefinitionBuilder< Name, Description, Namespace, Hint, Status, Version >
{
  name : Option< String >,
  description : Option< String >,
  namespace : Option< String >,
  hint : Option< String >,
  status : Option< String >,
  version : Option< String >,
  arguments : Vec< ArgumentDefinition >,
  routine_link : Option< String >,
  tags : Vec< String >,
  aliases : Vec< String >,
  permissions : Vec< String >,
  idempotent : bool,
  deprecation_message : String,
  http_method_hint : String,
  examples : Vec< String >,
  auto_help_enabled : bool,
  category : String,
  short_desc : String,
  hidden_from_list : bool,
  priority : i32,
  group : String,
  show_version_in_help : bool,
  _marker : PhantomData< ( Name, Description, Namespace, Hint, Status, Version ) >,
}

impl Default for CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
{
  fn default() -> Self
  {
    Self::new()
  }
}

// Start with all required fields NotSet
impl CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
{
  /// Create a new builder with all required fields unset
  pub fn new() -> Self
  {
    Self
    {
      name : None,
      description : None,
      namespace : None,
      hint : None,
      status : None,
      version : None,
      arguments : vec![],
      routine_link : None,
      tags : vec![],
      aliases : vec![],
      permissions : vec![],
      idempotent : false,
      deprecation_message : String::new(),
      http_method_hint : String::new(),
      examples : vec![],
      auto_help_enabled : true, // Default to true - help is mandatory
      category : String::new(),
      short_desc : String::new(),
      hidden_from_list : false,
      priority : 0,
      group : String::new(),
      show_version_in_help : true,
      _marker : PhantomData,
    }
  }
}

// Method to set name (transitions Name from NotSet to Set)
impl< Desc, Ns, Hint, Status, Version >
  CommandDefinitionBuilder< NotSet, Desc, Ns, Hint, Status, Version >
{
  /// Sets the command name (required field).
  ///
  /// This method transitions the `Name` type parameter from `NotSet` to `Set`,
  /// ensuring compile-time tracking of this required field.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandDefinition;
  ///
  /// let builder = CommandDefinition::former()
  ///     .name(".my_command");
  /// ```
  pub fn name( self, name : impl Into< String > )
    -> CommandDefinitionBuilder< Set, Desc, Ns, Hint, Status, Version >
  {
    CommandDefinitionBuilder
    {
      name : Some( name.into() ),
      description : self.description,
      namespace : self.namespace,
      hint : self.hint,
      status : self.status,
      version : self.version,
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Method to set description (transitions Description from NotSet to Set)
impl< Name, Ns, Hint, Status, Version >
  CommandDefinitionBuilder< Name, NotSet, Ns, Hint, Status, Version >
{
  /// Sets the command description (required field).
  ///
  /// This method transitions the `Description` type parameter from `NotSet` to `Set`.
  pub fn description( self, description : impl Into< String > )
    -> CommandDefinitionBuilder< Name, Set, Ns, Hint, Status, Version >
  {
    CommandDefinitionBuilder
    {
      name : self.name,
      description : Some( description.into() ),
      namespace : self.namespace,
      hint : self.hint,
      status : self.status,
      version : self.version,
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Method to set namespace (transitions Namespace from NotSet to Set)
impl< Name, Desc, Hint, Status, Version >
  CommandDefinitionBuilder< Name, Desc, NotSet, Hint, Status, Version >
{
  /// Sets the command namespace (required field).
  ///
  /// Use empty string `""` for global namespace commands.
  pub fn namespace( self, namespace : impl Into< String > )
    -> CommandDefinitionBuilder< Name, Desc, Set, Hint, Status, Version >
  {
    CommandDefinitionBuilder
    {
      name : self.name,
      description : self.description,
      namespace : Some( namespace.into() ),
      hint : self.hint,
      status : self.status,
      version : self.version,
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Method to set hint (transitions Hint from NotSet to Set)
impl< Name, Desc, Ns, Status, Version >
  CommandDefinitionBuilder< Name, Desc, Ns, NotSet, Status, Version >
{
  /// Sets the command hint (required field).
  ///
  /// A short hint shown in help text.
  pub fn hint( self, hint : impl Into< String > )
    -> CommandDefinitionBuilder< Name, Desc, Ns, Set, Status, Version >
  {
    CommandDefinitionBuilder
    {
      name : self.name,
      description : self.description,
      namespace : self.namespace,
      hint : Some( hint.into() ),
      status : self.status,
      version : self.version,
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Method to set status (transitions Status from NotSet to Set)
impl< Name, Desc, Ns, Hint, Version >
  CommandDefinitionBuilder< Name, Desc, Ns, Hint, NotSet, Version >
{
  /// Sets the command status (required field).
  ///
  /// Common values: `"stable"`, `"beta"`, `"experimental"`, `"deprecated"`.
  pub fn status( self, status : impl Into< String > )
    -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Set, Version >
  {
    CommandDefinitionBuilder
    {
      name : self.name,
      description : self.description,
      namespace : self.namespace,
      hint : self.hint,
      status : Some( status.into() ),
      version : self.version,
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Method to set version (transitions Version from NotSet to Set)
impl< Name, Desc, Ns, Hint, Status >
  CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, NotSet >
{
  /// Sets the command version (required field).
  ///
  /// Typically follows semantic versioning (e.g., `"1.0.0"`).
  pub fn version( self, version : impl Into< String > )
    -> CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Set >
  {
    CommandDefinitionBuilder
    {
      name : self.name,
      description : self.description,
      namespace : self.namespace,
      hint : self.hint,
      status : self.status,
      version : Some( version.into() ),
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
      _marker : PhantomData,
    }
  }
}

// Methods for optional fields - available on all states
impl< Name, Desc, Ns, Hint, Status, Version >
  CommandDefinitionBuilder< Name, Desc, Ns, Hint, Status, Version >
{
  /// Sets the command arguments (optional field, defaults to empty vec).
  pub fn arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
  {
    self.arguments = arguments;
    self
  }

  /// Sets the routine link (optional field, defaults to None).
  pub fn routine_link( mut self, routine_link : Option< String > ) -> Self
  {
    self.routine_link = routine_link;
    self
  }

  /// Sets the command tags (optional field, defaults to empty vec).
  pub fn tags( mut self, tags : Vec< String > ) -> Self
  {
    self.tags = tags;
    self
  }

  /// Sets the command aliases (optional field, defaults to empty vec).
  pub fn aliases( mut self, aliases : Vec< String > ) -> Self
  {
    self.aliases = aliases;
    self
  }

  /// Sets the command permissions (optional field, defaults to empty vec).
  pub fn permissions( mut self, permissions : Vec< String > ) -> Self
  {
    self.permissions = permissions;
    self
  }

  /// Sets whether the command is idempotent (optional field, defaults to false).
  pub fn idempotent( mut self, idempotent : bool ) -> Self
  {
    self.idempotent = idempotent;
    self
  }

  /// Sets the deprecation message (optional field, defaults to empty string).
  pub fn deprecation_message( mut self, deprecation_message : impl Into< String > ) -> Self
  {
    self.deprecation_message = deprecation_message.into();
    self
  }

  /// Sets the HTTP method hint (optional field, defaults to empty string).
  pub fn http_method_hint( mut self, http_method_hint : impl Into< String > ) -> Self
  {
    self.http_method_hint = http_method_hint.into();
    self
  }

  /// Sets the command examples (optional field, defaults to empty vec).
  pub fn examples( mut self, examples : Vec< String > ) -> Self
  {
    self.examples = examples;
    self
  }

  /// Sets whether auto-help is enabled (optional field, defaults to true).
  ///
  /// When true (default), registering this command will automatically generate a `.command.help` variant.
  /// Set to false ONLY for help commands themselves to prevent recursion.
  pub fn auto_help_enabled( mut self, auto_help_enabled : bool ) -> Self
  {
    self.auto_help_enabled = auto_help_enabled;
    self
  }

  /// Sets the command category (optional field, defaults to empty string).
  pub fn category( mut self, category : impl Into< String > ) -> Self
  {
    self.category = category.into();
    self
  }

  /// Sets the short description (optional field, defaults to empty string).
  pub fn short_desc( mut self, short_desc : impl Into< String > ) -> Self
  {
    self.short_desc = short_desc.into();
    self
  }

  /// Sets whether the command is hidden from list (optional field, defaults to false).
  pub fn hidden_from_list( mut self, hidden : bool ) -> Self
  {
    self.hidden_from_list = hidden;
    self
  }

  /// Sets the command priority (optional field, defaults to 0).
  pub fn priority( mut self, priority : i32 ) -> Self
  {
    self.priority = priority;
    self
  }

  /// Sets the command group (optional field, defaults to empty string).
  pub fn group( mut self, group : impl Into< String > ) -> Self
  {
    self.group = group.into();
    self
  }

  /// Sets whether version is displayed in help output (optional field, defaults to true).
  pub fn show_version_in_help( mut self, show : bool ) -> Self
  {
    self.show_version_in_help = show;
    self
  }
}

// Generic end() method for partial builder - allows building with just name and description
impl< Namespace, Hint, Status, Version > CommandDefinitionBuilder< Set, Set, Namespace, Hint, Status, Version >
{
  /// Builds the `CommandDefinition` with sensible defaults for unset fields.
  ///
  /// This method allows building a command with only name and description set,
  /// providing defaults for namespace (""), hint (""), status ("active"), and version ("1.0.0").
  ///
  /// # Default Values
  ///
  /// - `namespace`: `""` (root-level command)
  /// - `hint`: `""` (no hint)
  /// - `status`: `CommandStatus::Active`
  /// - `version`: `1.0.0`
  ///
  pub fn end( self ) -> CommandDefinition
  {
    let name_str = self.name.unwrap();
    let description_str = self.description.unwrap();
    let namespace_str = self.namespace.unwrap_or_else( || String::new() );
    let hint_str = self.hint.unwrap_or_else( || String::new() );
    let status_str = self.status.unwrap_or_else( || "active".to_string() );
    let version_str = self.version.unwrap_or_else( || "1.0.0".to_string() );

    CommandDefinition
    {
      name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
      description : description_str,
      namespace : namespace_str,
      hint : hint_str,
      status : match status_str.to_lowercase().as_str()
      {
        "experimental" => CommandStatus::Experimental,
        "internal" => CommandStatus::Internal,
        "deprecated" => CommandStatus::Deprecated
        {
          reason : self.deprecation_message.clone(),
          since : None,
          replacement : None,
        },
        _ => CommandStatus::Active,
      },
      version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
    }
  }
}

// .build() ONLY available when ALL required fields are Set
impl CommandDefinitionBuilder< Set, Set, Set, Set, Set, Set >
{
  /// Builds the `CommandDefinition` from the fully-populated builder.
  ///
  /// This method is only available when all 6 required fields have been set,
  /// providing compile-time safety against missing required fields.
  ///
  /// # Examples
  /// ```
  /// use unilang::data::CommandDefinition;
  ///
  /// let cmd = CommandDefinition::former()
  ///     .name(".my_command")
  ///     .description("Does something useful")
  ///     .namespace("".to_string())
  ///     .hint("Brief hint")
  ///     .status("stable")
  ///     .version("1.0.0")
  ///     .build();
  ///
  /// assert_eq!(cmd.name().as_str(), ".my_command");
  /// ```
  pub fn build( self ) -> CommandDefinition
  {
    let name_str = self.name.unwrap();
    let namespace_str = self.namespace.unwrap();
    let status_str = self.status.unwrap();
    let version_str = self.version.unwrap();

    CommandDefinition
    {
      name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
      description : self.description.unwrap(),
      namespace : namespace_str,
      hint : self.hint.unwrap(),
      status : match status_str.to_lowercase().as_str()
      {
        "experimental" => CommandStatus::Experimental,
        "internal" => CommandStatus::Internal,
        "deprecated" => CommandStatus::Deprecated
        {
          reason : self.deprecation_message.clone(),
          since : None,
          replacement : None,
        },
        _ => CommandStatus::Active,
      },
      version : VersionType::new( &version_str ).expect( "builder version should be valid" ),
      arguments : self.arguments,
      routine_link : self.routine_link,
      tags : self.tags,
      aliases : self.aliases,
      permissions : self.permissions,
      idempotent : self.idempotent,
      deprecation_message : self.deprecation_message,
      http_method_hint : self.http_method_hint,
      examples : self.examples,
      auto_help_enabled : self.auto_help_enabled,
      category : self.category,
      short_desc : self.short_desc,
      hidden_from_list : self.hidden_from_list,
      priority : self.priority,
      group : self.group,
      show_version_in_help : self.show_version_in_help,
    }
  }
}
