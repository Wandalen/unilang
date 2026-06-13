//! `CommandRegistryTrait` implementation for `CommandRegistry`.

use super::dynamic::CommandRegistry;
use super::traits::CommandRoutine;

impl super::traits::CommandRegistryTrait for CommandRegistry
{
  fn command( &self, name : &str ) -> Option< crate::data::CommandDefinition >
  {
    self.command( name )
  }

  fn commands( &self ) -> std::collections::HashMap< String, crate::data::CommandDefinition >
  {
    self.commands()
  }

  fn routine( &self, name : &str ) -> Option< &CommandRoutine >
  {
    self.routine( name )
  }

  fn help_for_command( &self, command_name : &str ) -> Option< String >
  {
    self.help_for_command( command_name )
  }
}
