/// Validates version string is non-empty.
pub fn validate_version( version : &str, name : &str, file_path : &str ) -> Result< (), String >
{
  if version.is_empty()
  {
    return Err( format!(
      "In file '{file_path}': Command '{name}' has empty version. Version string cannot be empty."
    ));
  }

  Ok(())
}

/// Computes full command name from namespace and name.
/// Handles both YAML formats (per FR-REG-6):
/// - Format 1: name: ".version" (compound name with dot)
/// - Format 2: namespace: "system", name: "status" (separate, dots added)
///
/// This mirrors the logic in `generate_static_commands` for PHF key generation.
pub fn compute_full_name( namespace : &str, name : &str ) -> String
{
  if namespace.is_empty()
  {
    // If name already has dot, use as-is; otherwise add dot
    if name.starts_with( '.' )
    {
      name.to_string()
    }
    else
    {
      format!( ".{name}" )
    }
  }
  else
  {
    // Namespace present: add dot if missing
    let ns = if namespace.starts_with( '.' )
    {
      namespace.to_string()
    }
    else
    {
      format!( ".{namespace}" )
    };
    format!( "{ns}.{name}" )
  }
}

/// Validates a complete command definition.
/// This validates the FINAL `full_name` after dots are properly added.
/// Supports both YAML formats (FR-REG-6).
///
/// This is called for each command during build to ensure the From conversion
/// will not panic at runtime.
pub fn validate_command(
  name : &str,
  namespace : &str,
  version : &str,
  file_path : &str,
) -> Result< (), String >
{
  // Name must not be empty
  if name.is_empty()
  {
    return Err( format!(
      "In file '{file_path}': Command name cannot be empty"
    ));
  }

  // Validate version
  validate_version( version, name, file_path )?;

  // Compute and validate full name (after dot normalization)
  let full_name = compute_full_name( namespace, name );
  if !full_name.starts_with( '.' )
  {
    return Err( format!(
      "In file '{file_path}': Invalid command '{name}'. Final full name '{full_name}' must start with dot prefix."
    ));
  }

  Ok(())
}
