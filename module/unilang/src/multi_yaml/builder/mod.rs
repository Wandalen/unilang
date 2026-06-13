//! CliBuilder API for ergonomic CLI aggregation
//!
//! This module provides the `CliBuilder` fluent API for combining multiple CLI tools
//! into unified commands with prefix management, namespace isolation, and conflict detection.
//! Supports both static and dynamic command sources with zero-overhead lookup when using
//! `StaticCommandRegistry`.

/// Internal namespace.
mod private
{
  use crate::data::CommandDefinition;
  use crate::error::Error;
  use crate::registry::{ CommandRegistry, StaticCommandRegistry, RegistryMode };
  use std::collections::HashMap;
  use std::path::PathBuf;

  mod types;
  pub use types::*;

  /// Ergonomic CLI builder for simple and complex aggregation scenarios
  #[derive(Debug, Clone)]
  pub struct CliBuilder
  {
    /// Registry mode for aggregation
    mode: AggregationMode,
    /// Static command modules
    static_modules: Vec< StaticModule >,
    /// Dynamic YAML modules
    dynamic_modules: Vec< DynamicModule >,
    /// Conditional modules based on features
    conditional_modules: Vec< ConditionalModule >,
    /// Global configuration
    config: CliConfig,
  }

  impl CliBuilder
  {
    /// Create a new CLI builder with intelligent defaults
    pub fn new() -> Self
    {
      Self
      {
        mode: AggregationMode::Auto,
        static_modules: Vec::new(),
        dynamic_modules: Vec::new(),
        conditional_modules: Vec::new(),
        config: CliConfig
        {
          app_name: "app".to_string(),
          global_prefix: None,
          auto_help: true,
          detect_conflicts: true,
          env_overrides: HashMap::new(),
          exclude_env_overrides: Vec::new(),
        },
      }
    }

    /// Set aggregation mode
    pub fn mode( mut self, mode: AggregationMode ) -> Self
    {
      self.mode = mode;
      self
    }

    /// Add a static module
    pub fn static_module( mut self, name: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.static_modules.push( StaticModule
      {
        name: name.to_string(),
        commands,
        prefix: None,
        enabled: true,
      } );
      self
    }

    /// Add a static module with prefix
    pub fn static_module_with_prefix( mut self, name: &str, prefix: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.static_modules.push( StaticModule
      {
        name: name.to_string(),
        commands,
        prefix: Some( prefix.to_string() ),
        enabled: true,
      } );
      self
    }

    /// Add a dynamic YAML module
    pub fn dynamic_module( mut self, name: &str, yaml_path: PathBuf ) -> Self
    {
      self.dynamic_modules.push( DynamicModule
      {
        name: name.to_string(),
        yaml_path,
        prefix: None,
        enabled: true,
      } );
      self
    }

    /// Add a dynamic YAML module with prefix
    pub fn dynamic_module_with_prefix( mut self, name: &str, yaml_path: PathBuf, prefix: &str ) -> Self
    {
      self.dynamic_modules.push( DynamicModule
      {
        name: name.to_string(),
        yaml_path,
        prefix: Some( prefix.to_string() ),
        enabled: true,
      } );
      self
    }

    /// Add a conditional module
    pub fn conditional_module( mut self, name: &str, feature: &str, commands: Vec< CommandDefinition > ) -> Self
    {
      self.conditional_modules.push( ConditionalModule
      {
        name: name.to_string(),
        feature: feature.to_string(),
        module: Box::new( StaticModule
        {
          name: name.to_string(),
          commands,
          prefix: None,
          enabled: true,
        } ),
      } );
      self
    }

    /// Set application name
    pub fn app_name( mut self, name: &str ) -> Self
    {
      self.config.app_name = name.to_string();
      self
    }

    /// Set global prefix
    pub fn global_prefix( mut self, prefix: &str ) -> Self
    {
      self.config.global_prefix = Some( prefix.to_string() );
      self
    }

    /// Enable or disable auto-help
    pub fn auto_help( mut self, enabled: bool ) -> Self
    {
      self.config.auto_help = enabled;
      self
    }

    /// Enable or disable conflict detection
    pub fn detect_conflicts( mut self, enabled: bool ) -> Self
    {
      self.config.detect_conflicts = enabled;
      self
    }

    /// Detect and report command prefix conflicts at build time
    pub fn detect_conflicts_report( &self ) -> Vec< ConflictReport >
    {
      if !self.config.detect_conflicts
      {
        return Vec::new();
      }

      let mut conflicts = Vec::new();
      let mut all_commands: HashMap< String, Vec< String > > = HashMap::new();

      // Check static modules for conflicts
      for module in &self.static_modules
      {
        if !module.enabled
        {
          continue;
        }

        for cmd in &module.commands
        {
          let final_name = self.compute_final_command_name( cmd, module.prefix.as_ref() );
          all_commands
            .entry( final_name )
            .or_default()
            .push( module.name.clone() );
        }
      }

      // Check dynamic modules for conflicts
      for module in &self.dynamic_modules
      {
        if !module.enabled
        {
          continue;
        }

        // Load actual commands from YAML file for conflict detection
        if module.yaml_path.exists()
        {
          if let Ok( yaml_content ) = std::fs::read_to_string( &module.yaml_path )
          {
            if let Ok( temp_registry_builder ) = CommandRegistry::builder().load_from_yaml_str( &yaml_content )
            {
              let temp_registry = temp_registry_builder.build();
              for ( _name, cmd ) in temp_registry.commands()
              {
                let final_name = self.compute_final_command_name( &cmd, module.prefix.as_ref() );
                all_commands
                  .entry( final_name )
                  .or_default()
                  .push( module.name.clone() );
              }
            }
          }
        }
      }

      // Check conditional modules for conflicts
      for cond_module in &self.conditional_modules
      {
        if self.is_feature_enabled( &cond_module.feature )
        {
          for cmd in &cond_module.module.commands
          {
            let final_name = self.compute_final_command_name( cmd, cond_module.module.prefix.as_ref() );
            all_commands
              .entry( final_name )
              .or_default()
              .push( cond_module.name.clone() );
          }
        }
      }

      // Generate conflict reports
      for ( cmd_name, sources ) in all_commands
      {
        if sources.len() > 1
        {
          conflicts.push( ConflictReport
          {
            command_name: cmd_name,
            modules: sources,
            conflict_type: ConflictType::NameCollision,
          } );
        }
      }

      conflicts
    }

    /// Compute the final command name after applying prefixes
    fn compute_final_command_name( &self, cmd: &CommandDefinition, module_prefix: Option< &String > ) -> String
    {
      let mut final_name = cmd.name().as_str().to_string();

      // Apply module prefix
      if let Some( prefix ) = module_prefix
      {
        final_name = if cmd.namespace().is_empty()
        {
          format!( ".{}.{}", prefix, final_name.strip_prefix( '.' ).unwrap_or( &final_name ) )
        }
        else
        {
          format!( ".{}{}.{}", prefix, cmd.namespace(), final_name.strip_prefix( '.' ).unwrap_or( &final_name ) )
        };
      }

      // Apply global prefix
      if let Some( global_prefix ) = &self.config.global_prefix
      {
        final_name = format!( ".{}{}", global_prefix, final_name );
      }

      final_name
    }

    /// Build the CLI registry with dynamic registration
    pub fn build( self ) -> Result< CommandRegistry, Error >
    {
      #[ allow( deprecated ) ]
      let mut registry = CommandRegistry::new();

      // Set registry mode based on aggregation mode
      let registry_mode = match self.mode
      {
        AggregationMode::Static => RegistryMode::Hybrid, // Static modules are registered dynamically
        AggregationMode::Dynamic => RegistryMode::DynamicOnly,
        AggregationMode::Hybrid => RegistryMode::Hybrid,
        AggregationMode::Auto => self.detect_optimal_mode(),
      };

      registry.set_mode( registry_mode );

      // Register all module types
      self.register_static_modules( &mut registry )?;
      self.register_dynamic_modules( &mut registry )?;
      self.register_conditional_modules( &mut registry )?;

      Ok( registry )
    }

    /// Build a static registry with zero-overhead lookup optimized for StaticCommandRegistry
    pub fn build_static( self ) -> Result< StaticCommandRegistry, Error >
    {
      let mut static_registry = StaticCommandRegistry::new();

      // Set registry mode for optimal static performance
      // Note: Use Hybrid mode to allow dynamic command registration while maintaining static optimizations
      static_registry.set_mode( RegistryMode::Hybrid );

      // Process static modules only for optimal performance
      for module in &self.static_modules
      {
        if !module.enabled
        {
          continue;
        }

        for cmd in module.commands.clone()
        {
          let cmd = self.apply_prefixes( cmd, module.prefix.as_ref() );

          // Register command with the static registry
          // Note: Using a placeholder routine since actual command execution logic
          // would be provided by the application using the CliBuilder
          let cmd_name = cmd.name().as_str().to_string();
          let cmd_description = cmd.description().to_string();
          let routine = Box::new( move |_cmd, _ctx| {
            Err( crate::data::ErrorData::new(
              crate::data::ErrorCode::CommandNotImplemented,
              format!(
                "Command '{}' ({}) is registered but not implemented. Applications using CliBuilder must provide their own command execution logic.",
                cmd_name, cmd_description
              ),
            ))
          });

          static_registry.register_with_routine( cmd, routine )?;
        }
      }

      // Note: Dynamic and conditional modules are skipped in static build for zero-overhead
      // Users should use build() for hybrid scenarios

      Ok( static_registry )
    }

    /// Returns the current aggregation mode.
    pub fn aggregation_mode( &self ) -> &AggregationMode
    {
      &self.mode
    }

    /// Returns the current CLI configuration.
    pub fn config( &self ) -> &CliConfig
    {
      &self.config
    }

    /// Get static modules count (for testing)
    pub fn static_modules_count( &self ) -> usize
    {
      self.static_modules.len()
    }

    /// Get dynamic modules count (for testing)
    pub fn dynamic_modules_count( &self ) -> usize
    {
      self.dynamic_modules.len()
    }

    /// Get conditional modules count (for testing)
    pub fn conditional_modules_count( &self ) -> usize
    {
      self.conditional_modules.len()
    }
  }

  impl Default for CliBuilder
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  mod prefixes;
}

mod_interface::mod_interface!
{
  exposed use private::AggregationMode;
  exposed use private::StaticModule;
  exposed use private::DynamicModule;
  exposed use private::ConditionalModule;
  exposed use private::ModuleConfig;
  exposed use private::ModuleSource;
  exposed use private::CliConfig;
  exposed use private::ConflictReport;
  exposed use private::ConflictType;
  exposed use private::CliBuilder;

  prelude use private::CliBuilder;
  prelude use private::AggregationMode;
  prelude use private::StaticModule;
  prelude use private::DynamicModule;
  prelude use private::ConditionalModule;
}