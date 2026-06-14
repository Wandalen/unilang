//!
//! Handles loading command definitions from external files (YAML/JSON).
//!

/// Internal namespace.
mod private
{
  use crate::
  {
    data::{ ErrorCode, ErrorData, OutputData },
    error::Error,
    registry::CommandRoutine,
  };
  #[cfg(any(feature = "yaml_parser", feature = "json_parser"))]
  use crate::data::CommandDefinition;

///
/// Loads command definitions from a YAML string.
///
/// **Requires feature**: `yaml_parser` (enabled by YAML approaches)
///
/// # Errors
///
/// Returns an `Error::Yaml` if the YAML string is invalid.
///
#[ cfg( feature = "yaml_parser" ) ]
pub fn load_command_definitions_from_yaml_str( yaml_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_yaml_ng::from_str( yaml_str ).map_err( Error::Yaml )?;
  Ok( definitions )
}

///
/// Loads command definitions from a JSON string.
///
/// **Requires feature**: `json_parser` (enabled by JSON approaches)
///
/// # Errors
///
/// Returns an `Error::Json` if the JSON string is invalid.
///
#[ cfg( feature = "json_parser" ) ]
pub fn load_command_definitions_from_json_str( json_str : &str ) -> Result< Vec< CommandDefinition >, Error >
{
  let definitions : Vec< CommandDefinition > = serde_json::from_str( json_str ).map_err( Error::Json )?;
  Ok( definitions )
}

///
/// Resolves a routine link string to a `CommandRoutine`.
///
/// This is a placeholder for now. In a later increment, this will handle
/// dynamic loading of routines from shared libraries or Rust modules.
///
/// # Errors
///
/// Returns an `Error::Execution` if the link is not recognized or if
/// dynamic loading fails (in future increments).
///
pub fn resolve_routine_link( link : &str ) -> Result< CommandRoutine, Error >
{
  // Dynamic library loading is not yet implemented. Return a stub routine that fails loudly
  // at call time rather than returning silent empty output. Failing at load time would
  // prevent the registry from being built at all — failing at call time correctly surfaces
  // the "not implemented" error when the command is actually invoked.
  let owned_link = link.to_owned();
  Ok
  (
    Box::new
    (
      move | _args, _context | -> Result< OutputData, ErrorData >
      {
        Err
        (
          ErrorData::new
          (
            ErrorCode::CommandNotImplemented,
            format!
            (
              "Routine link '{}' cannot be resolved: dynamic library loading is not yet implemented",
              owned_link
            ),
          )
        )
      }
    )
  )
}

}


mod_interface::mod_interface!
{
  #[ cfg( feature = "yaml_parser" ) ]
  exposed use private::load_command_definitions_from_yaml_str;
  #[ cfg( feature = "json_parser" ) ]
  exposed use private::load_command_definitions_from_json_str;
  exposed use private::resolve_routine_link;

  #[ cfg( feature = "yaml_parser" ) ]
  prelude use private::load_command_definitions_from_yaml_str;
  #[ cfg( feature = "json_parser" ) ]
  prelude use private::load_command_definitions_from_json_str;
}
