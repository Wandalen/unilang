//! Mode detection and prefix application — internal  helpers.

use super::*;

impl CliBuilder
{
  /// Detect optimal aggregation mode based on environment
  pub fn detect_optimal_mode( &self ) -> RegistryMode
  {
    let has_static = !self.static_modules.is_empty();
    let has_dynamic = !self.dynamic_modules.is_empty();
    let has_conditional = !self.conditional_modules.is_empty();

    // If any modules are present that require dynamic registration, use Hybrid or DynamicOnly
    if has_static || has_conditional
    {
      if has_dynamic
      {
        RegistryMode::Hybrid
      }
      else
      {
        // Static or conditional modules exist (both use dynamic registration), use Hybrid
        RegistryMode::Hybrid
      }
    }
    else if has_dynamic
    {
      RegistryMode::DynamicOnly
    }
    else
    {
      // No modules configured, default to StaticOnly
      RegistryMode::StaticOnly
    }
  }

  /// Check if a feature is enabled using real Cargo features
  pub( super ) fn is_feature_enabled( &self, feature: &str ) -> bool
  {
    match feature
    {
      "enabled" => cfg!( feature = "enabled" ),
      "simd" => cfg!( feature = "simd" ),
      "repl" => cfg!( feature = "repl" ),
      "enhanced_repl" => cfg!( feature = "enhanced_repl" ),
      "static_registry" => cfg!( feature = "static_registry" ),
      "static_commands" => cfg!( feature = "static_registry" ), // Legacy alias
      "multi_file" => cfg!( feature = "multi_file" ),
      "multi_yaml" => cfg!( feature = "multi_file" ), // Legacy alias
      // NOTE: Benchmark features removed - see unilang_benchmarks workspace crate
      "advanced_benchmarks" => false,
      "advanced_cli_tests" => cfg!( feature = "advanced_cli_tests" ),
      "wasm" => cfg!( feature = "wasm" ),
      "benchmarks" => false,
      "on_unknown_suggest" => cfg!( feature = "on_unknown_suggest" ),
      "full" => cfg!( feature = "full" ),
      // Legacy compatibility for existing tests
      "test_feature" => cfg!( feature = "advanced_cli_tests" ),
      "advanced" => cfg!( feature = "full" ),
      _ => false,
    }
  }

  /// Apply prefixes to a command
  pub( super ) fn apply_prefixes( &self, mut cmd: CommandDefinition, module_prefix: Option< &String > ) -> CommandDefinition
  {
    // Apply module prefix
    if let Some( prefix ) = module_prefix
    {
      let new_namespace = if cmd.namespace().is_empty()
      {
        format!( ".{}", prefix )
      }
      else
      {
        format!( ".{}{}", prefix, cmd.namespace() )
      };
      cmd = cmd.with_namespace( new_namespace.clone() );
    }

    // Apply global prefix
    if let Some( global_prefix ) = &self.config.global_prefix
    {
      let new_namespace = if cmd.namespace().is_empty()
      {
        format!( ".{}", global_prefix )
      }
      else
      {
        format!( ".{}{}", global_prefix, cmd.namespace() )
      };
      cmd = cmd.with_namespace( new_namespace.clone() );
    }

    cmd
  }

  /// Register static modules
  pub( super ) fn register_static_modules( &self, registry: &mut CommandRegistry ) -> Result< (), Error >
  {
    for module in &self.static_modules
    {
      if !module.enabled
      {
        continue;
      }

      for cmd in module.commands.clone()
      {
        let processed_cmd = self.apply_prefixes( cmd, module.prefix.as_ref() );
        registry.register( processed_cmd )?;
      }
    }
    Ok(())
  }

  /// Register dynamic modules
  pub( super ) fn register_dynamic_modules( &self, registry: &mut CommandRegistry ) -> Result< (), Error >
  {
    for module in &self.dynamic_modules
    {
      if !module.enabled
      {
        continue;
      }

      // Attempt to load commands from YAML file
      if module.yaml_path.exists()
      {
        // Read file content
        match std::fs::read_to_string( &module.yaml_path )
        {
          Ok( yaml_content ) => {
            // Create a temporary registry to load YAML commands
            let temp_registry = match CommandRegistry::builder()
              .load_from_yaml_str( &yaml_content )
            {
              Ok( builder ) => builder.build(),
              Err( e ) => {
                eprintln!( "Warning: Failed to parse YAML file {}: {}", module.yaml_path.display(), e );
                continue;
              }
            };

            // Register all commands from the YAML file with proper prefixes
            for ( _name, cmd ) in temp_registry.commands()
            {
              let processed_cmd = self.apply_prefixes( cmd, module.prefix.as_ref() );
              registry.register( processed_cmd )?;
            }
          }
          Err( e ) => {
            eprintln!( "Warning: Failed to read YAML file {}: {}", module.yaml_path.display(), e );
          }
        }
      }
      else
      {
        eprintln!( "Warning: YAML file {} does not exist", module.yaml_path.display() );
      }
    }
    Ok(())
  }

  /// Register conditional modules
  pub( super ) fn register_conditional_modules( &self, registry: &mut CommandRegistry ) -> Result< (), Error >
  {
    for cond_module in &self.conditional_modules
    {
      if self.is_feature_enabled( &cond_module.feature )
      {
        for cmd in cond_module.module.commands.clone()
        {
          let mut processed_cmd = cmd;
          // Apply conditional module namespace
          let new_namespace = format!( ".{}", cond_module.name );
          processed_cmd = processed_cmd.with_namespace( new_namespace.clone() );

          // Apply global prefix if configured
          if let Some( global_prefix ) = &self.config.global_prefix
          {
            let new_namespace = format!( ".{}{}", global_prefix, processed_cmd.namespace() );
            processed_cmd = processed_cmd.with_namespace( new_namespace.clone() );
          }

          registry.register( processed_cmd )?;
        }
      }
    }
    Ok(())
  }

}
