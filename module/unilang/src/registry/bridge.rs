use super::dynamic::CommandRegistry;
use super::static_reg::StaticCommandRegistry;

/// Conversion from StaticCommandRegistry to CommandRegistry.
///
/// This enables the pattern: `Pipeline::new(static_registry.into())`
///
/// # Fix(H15): StaticCommandRegistry now converts to CommandRegistry
///
/// Root cause: Pipeline requires CommandRegistry but static definitions produce
/// StaticCommandRegistry. This bridge enables seamless conversion.
///
/// Pitfall: If build.rs validation is disabled, conversion may fail on invalid commands.
///
/// # Examples
///
/// ```rust,ignore
/// use unilang::registry::{StaticCommandRegistry, CommandRegistry};
/// use unilang::pipeline::Pipeline;
///
/// // Create static registry from compile-time commands
/// let static_registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
///
/// // Convert to CommandRegistry for Pipeline
/// let command_registry: CommandRegistry = static_registry.into();
/// let pipeline = Pipeline::new(command_registry);
/// ```
impl From< StaticCommandRegistry > for CommandRegistry
{
  fn from( static_reg : StaticCommandRegistry ) -> Self
  {
    #[ allow( deprecated ) ]
    let mut registry = CommandRegistry::new();

    // Collect commands first (into_routines consumes static_reg)
    let all_cmds = static_reg.commands();
    let mut routines = static_reg.into_routines();

    for ( name, cmd ) in all_cmds
    {
      if let Some( routine ) = routines.remove( &name )
      {
        // Use command_add_runtime so the routine is attached directly
        // without requiring re-registration by the caller.
        if let Err( e ) = registry.command_add_runtime( &cmd, routine )
        {
          log::warn!(
            "Unexpected: Command '{}' failed during StaticCommandRegistry conversion: {}",
            name, e
          );
        }
      }
      else if let Err( e ) = registry.register( cmd )
      {
        log::warn!(
          "Unexpected: Command '{}' registration failed during StaticCommandRegistry conversion: {}",
          name, e
        );
      }
    }

    registry
  }
}
