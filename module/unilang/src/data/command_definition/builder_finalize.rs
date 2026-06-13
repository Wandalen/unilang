//! `end()` and `build()` finalization impls for `CommandDefinitionBuilder`.

use super::builder::{ CommandDefinitionBuilder, Set };
use super::core::CommandDefinition;
use super::super::command_name::CommandName;
use super::super::version_type::VersionType;
use super::super::command_status::CommandStatus;

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
    let namespace_str = self.namespace.unwrap_or_default();
    let hint_str = self.hint.unwrap_or_default();
    let status_str = self.status.unwrap_or_else( || "active".to_string() );
    let version_str = self.version.unwrap_or_else( || "1.0.0".to_string() );

    CommandDefinition
    {
      name : CommandName::new( &name_str ).expect( "builder name should be valid" ),
      description : description_str,
      namespace : namespace_str,
      hint : hint_str,
      status : parse_status_from_str( &status_str, &self.deprecation_message ),
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

// Parses a status string and deprecation message into a `CommandStatus`.
// Both end() and build() share this logic to avoid duplication.
fn parse_status_from_str( status_str : &str, deprecation_message : &str ) -> CommandStatus
{
  if status_str.eq_ignore_ascii_case( "deprecated" )
  {
    CommandStatus::Deprecated
    {
      reason : deprecation_message.to_string(),
      since : None,
      replacement : None,
    }
  }
  else
  {
    CommandStatus::from_str_lossy( status_str )
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
      status : parse_status_from_str( &status_str, &self.deprecation_message ),
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
