//! Optional-field setter methods for CommandDefinitionBuilder.

use crate::data::argument_types::ArgumentDefinition;
use super::builder::CommandDefinitionBuilder;

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
