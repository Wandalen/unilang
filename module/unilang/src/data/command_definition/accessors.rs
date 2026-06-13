//! Getter and setter methods for CommandDefinition.

use super::super::command_name::CommandName;
use super::super::version_type::VersionType;
use super::super::command_status::CommandStatus;
use super::super::argument_types::ArgumentDefinition;
use super::core::CommandDefinition;

impl CommandDefinition
{
  // ===================================================================
  // Getter Methods - Read-only access to private fields
  // ===================================================================

  /// Returns a reference to the validated command name
  #[ must_use ]
  pub fn name( &self ) -> &CommandName
  {
    &self.name
  }

  /// Returns the command description
  #[ must_use ]
  pub fn description( &self ) -> &str
  {
    &self.description
  }

  /// Returns a reference to the command arguments
  #[ must_use ]
  pub fn arguments( &self ) -> &Vec< ArgumentDefinition >
  {
    &self.arguments
  }

  /// Returns the routine link if set
  #[ must_use ]
  pub fn routine_link( &self ) -> Option< &String >
  {
    self.routine_link.as_ref()
  }

  /// Returns a reference to the namespace string
  #[ must_use ]
  pub fn namespace( &self ) -> &str
  {
    &self.namespace
  }

  /// Returns the command hint
  #[ must_use ]
  pub fn hint( &self ) -> &str
  {
    &self.hint
  }

  /// Returns a reference to the command status
  #[ must_use ]
  pub fn status( &self ) -> &CommandStatus
  {
    &self.status
  }

  /// Returns a reference to the validated version
  #[ must_use ]
  pub fn version( &self ) -> &VersionType
  {
    &self.version
  }

  /// Returns a reference to the command tags
  #[ must_use ]
  pub fn tags( &self ) -> &Vec< String >
  {
    &self.tags
  }

  /// Returns a reference to the command aliases
  #[ must_use ]
  pub fn aliases( &self ) -> &Vec< String >
  {
    &self.aliases
  }

  /// Returns a reference to the permissions
  #[ must_use ]
  pub fn permissions( &self ) -> &Vec< String >
  {
    &self.permissions
  }

  /// Returns whether the command is idempotent
  #[ must_use ]
  pub fn idempotent( &self ) -> bool
  {
    self.idempotent
  }

  /// Returns the deprecation message
  #[ must_use ]
  pub fn deprecation_message( &self ) -> &str
  {
    &self.deprecation_message
  }

  /// Returns the HTTP method hint
  #[ must_use ]
  pub fn http_method_hint( &self ) -> &str
  {
    &self.http_method_hint
  }

  /// Returns a reference to the usage examples
  #[ must_use ]
  pub fn examples( &self ) -> &Vec< String >
  {
    &self.examples
  }

  /// Returns whether auto-help is enabled
  #[ must_use ]
  pub fn auto_help_enabled( &self ) -> bool
  {
    self.auto_help_enabled
  }

  /// Returns the command category
  #[ must_use ]
  pub fn category( &self ) -> &str
  {
    &self.category
  }

  /// Returns the short description
  #[ must_use ]
  pub fn short_desc( &self ) -> &str
  {
    &self.short_desc
  }

  /// Returns whether the command is hidden from listings
  #[ must_use ]
  pub fn hidden_from_list( &self ) -> bool
  {
    self.hidden_from_list
  }

  /// Returns the command priority
  #[ must_use ]
  pub fn priority( &self ) -> i32
  {
    self.priority
  }

  /// Returns the command group
  #[ must_use ]
  pub fn group( &self ) -> &str
  {
    &self.group
  }

  /// Returns whether version should be displayed in help output
  #[ must_use ]
  pub fn show_version_in_help( &self ) -> bool
  {
    self.show_version_in_help
  }

  // ===================================================================
  // Setter Methods - Fluent API with validation
  // ===================================================================

  /// Sets the command name (validated)
  #[ must_use ]
  pub fn with_name( mut self, name : CommandName ) -> Self
  {
    self.name = name;
    self
  }

  /// Sets the command description
  #[ must_use ]
  pub fn with_description( mut self, description : impl Into< String > ) -> Self
  {
    self.description = description.into();
    self
  }

  /// Sets the command arguments
  #[ must_use ]
  pub fn with_arguments( mut self, arguments : Vec< ArgumentDefinition > ) -> Self
  {
    self.arguments = arguments;
    self
  }

  /// Sets the routine link
  #[ must_use ]
  pub fn with_routine_link( mut self, link : Option< String > ) -> Self
  {
    self.routine_link = link;
    self
  }

  /// Sets the command namespace (String)
  #[ must_use ]
  pub fn with_namespace( mut self, namespace : String ) -> Self
  {
    self.namespace = namespace;
    self
  }

  /// Sets the command hint
  #[ must_use ]
  pub fn with_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.hint = hint.into();
    self
  }

  /// Sets the command status
  #[ must_use ]
  pub fn with_status( mut self, status : CommandStatus ) -> Self
  {
    self.status = status;
    self
  }

  /// Sets the command version (validated)
  #[ must_use ]
  pub fn with_version( mut self, version : VersionType ) -> Self
  {
    self.version = version;
    self
  }

  /// Sets the command tags
  #[ must_use ]
  pub fn with_tags( mut self, tags : Vec< String > ) -> Self
  {
    self.tags = tags;
    self
  }

  /// Sets the command aliases
  #[ must_use ]
  pub fn with_aliases( mut self, aliases : Vec< String > ) -> Self
  {
    self.aliases = aliases;
    self
  }

  /// Sets the permissions
  #[ must_use ]
  pub fn with_permissions( mut self, permissions : Vec< String > ) -> Self
  {
    self.permissions = permissions;
    self
  }

  /// Sets whether the command is idempotent
  #[ must_use ]
  pub fn with_idempotent( mut self, idempotent : bool ) -> Self
  {
    self.idempotent = idempotent;
    self
  }

  /// Sets the deprecation message
  #[ must_use ]
  pub fn with_deprecation_message( mut self, message : impl Into< String > ) -> Self
  {
    self.deprecation_message = message.into();
    self
  }

  /// Sets the HTTP method hint
  #[ must_use ]
  pub fn with_http_method_hint( mut self, hint : impl Into< String > ) -> Self
  {
    self.http_method_hint = hint.into();
    self
  }

  /// Sets the usage examples
  #[ must_use ]
  pub fn with_examples( mut self, examples : Vec< String > ) -> Self
  {
    self.examples = examples;
    self
  }

  /// Sets whether auto-help is enabled
  #[ must_use ]
  pub fn with_auto_help( mut self, enabled : bool ) -> Self
  {
    self.auto_help_enabled = enabled;
    self
  }

  /// Sets the command category
  #[ must_use ]
  pub fn with_category( mut self, category : impl Into< String > ) -> Self
  {
    self.category = category.into();
    self
  }

  /// Sets the short description
  #[ must_use ]
  pub fn with_short_desc( mut self, desc : impl Into< String > ) -> Self
  {
    self.short_desc = desc.into();
    self
  }

  /// Sets whether the command is hidden from listings
  #[ must_use ]
  pub fn with_hidden_from_list( mut self, hidden : bool ) -> Self
  {
    self.hidden_from_list = hidden;
    self
  }

  /// Sets the command priority
  #[ must_use ]
  pub fn with_priority( mut self, priority : i32 ) -> Self
  {
    self.priority = priority;
    self
  }

  /// Sets the command group
  #[ must_use ]
  pub fn with_group( mut self, group : impl Into< String > ) -> Self
  {
    self.group = group.into();
    self
  }

  /// Sets whether version is displayed in help output
  #[ must_use ]
  pub fn with_show_version_in_help( mut self, show : bool ) -> Self
  {
    self.show_version_in_help = show;
    self
  }
}
