//! Type-state builder for CommandDefinition with compile-time required field enforcement.

use std::marker::PhantomData;
use super::super::argument_types::ArgumentDefinition;

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
/// ```text
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
/// ```text
/// CommandDefinitionBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet> // Initial
///   .name(".test") → CommandDefinitionBuilder<Set, NotSet, NotSet, NotSet, NotSet, NotSet>
///   .description("Test") → CommandDefinitionBuilder<Set, Set, NotSet, NotSet, NotSet, NotSet>
///   .end() // OK - only requires Name and Description to be Set
/// ```
///
/// The `build()` method is only available when ALL type parameters are `Set`:
///
/// ```text
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
  pub( super ) name : Option< String >,
  pub( super ) description : Option< String >,
  pub( super ) namespace : Option< String >,
  pub( super ) hint : Option< String >,
  pub( super ) status : Option< String >,
  pub( super ) version : Option< String >,
  pub( super ) arguments : Vec< ArgumentDefinition >,
  pub( super ) routine_link : Option< String >,
  pub( super ) tags : Vec< String >,
  pub( super ) aliases : Vec< String >,
  pub( super ) permissions : Vec< String >,
  pub( super ) idempotent : bool,
  pub( super ) deprecation_message : String,
  pub( super ) http_method_hint : String,
  pub( super ) examples : Vec< String >,
  pub( super ) auto_help_enabled : bool,
  pub( super ) category : String,
  pub( super ) short_desc : String,
  pub( super ) hidden_from_list : bool,
  pub( super ) priority : i32,
  pub( super ) group : String,
  pub( super ) show_version_in_help : bool,
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

