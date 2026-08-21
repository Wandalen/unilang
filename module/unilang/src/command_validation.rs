//! Command registration validation and utilities.
//!
//! This module centralizes all validation logic used across different
//! registration approaches (runtime, YAML, JSON, Rust DSL) to ensure
//! consistent behavior and avoid code duplication.
//!
//! # Validation Rules
//!
//! All commands must follow these rules:
//! - Command names must start with '.' prefix
//! - Non-empty namespaces must start with '.' prefix
//! - Empty namespaces are allowed for root-level commands
//! - An `Enum` parameter's default must be one of its own choices (hard error)
//! - A `String` parameter whose description embeds an `a|b|c` choice list
//!   should be declared `Kind::Enum` instead (non-fatal warning)
//!
//! # Examples
//!
//! ```rust
//! use unilang::command_validation::{ validate_command_name, validate_namespace };
//!
//! // Valid names
//! assert!( validate_command_name( ".hello" ).is_ok() );
//! assert!( validate_command_name( ".cmd1.process" ).is_ok() );
//!
//! // Invalid names
//! assert!( validate_command_name( "hello" ).is_err() );
//! assert!( validate_command_name( "hello.world" ).is_err() );
//!
//! // Valid namespaces
//! assert!( validate_namespace( "" ).is_ok() );           // Empty is OK
//! assert!( validate_namespace( ".cmd1" ).is_ok() );
//!
//! // Invalid namespaces
//! assert!( validate_namespace( "cmd1" ).is_err() );     // Missing dot
//! ```

/// Internal namespace.
mod private
{
  use crate::{ error::Error, data::CommandDefinition };

/// Validates command name follows dot-prefix naming convention.
///
/// # Errors
///
/// Returns `Error::Registration` if name doesn't start with '.'
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::validate_command_name;
///
/// assert!( validate_command_name( ".hello" ).is_ok() );
/// assert!( validate_command_name( "hello" ).is_err() );
/// ```
pub fn validate_command_name( name : &str ) -> Result< (), Error >
{
  crate::validation_core::validate_command_name_core( name ).map_err( Error::Registration )
}

/// Validates namespace follows dot-prefix naming convention.
///
/// Empty namespaces are allowed (for root-level commands).
/// Non-empty namespaces must start with '.'.
///
/// # Errors
///
/// Returns `Error::Registration` if non-empty namespace doesn't start with '.'
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::validate_namespace;
///
/// assert!( validate_namespace( "" ).is_ok() );         // Empty OK
/// assert!( validate_namespace( ".session" ).is_ok() );
/// assert!( validate_namespace( "session" ).is_err() ); // Missing dot
/// ```
pub fn validate_namespace( namespace : &str ) -> Result< (), Error >
{
  crate::validation_core::validate_namespace_core( namespace ).map_err( Error::Registration )
}

/// Validates parameter storage types match their multiple attribute.
///
/// Prevents silent data loss where `multiple: true` is used with
/// non-List storage types, causing multiple values to silently
/// overwrite each other.
///
/// # Errors
///
/// Returns `Error::Registration` if any parameter has `multiple: true`
/// but kind is not `Kind::List`.
///
/// # Examples
///
/// ```rust
/// use unilang::prelude::*;
///
/// // CORRECT: multiple:true with Kind::List
/// let cmd = CommandDefinition::former()
///   .name( ".test" )
///   .description( "Test command" )
///   .arguments( vec![
///     ArgumentDefinition {
///       name: "files".to_string(),
///       description: "Input files".to_string(),
///       kind: Kind::List( Box::new( Kind::String ), None ),
///       hint: String::new(),
///       attributes: ArgumentAttributes {
///         multiple: true,
///         optional: true,
///         ..Default::default()
///       },
///       validation_rules: vec![],
///       aliases: vec![],
///       tags: vec![],
///     }
///   ])
///   .end();
///
/// assert!( validate_parameter_storage_types( &cmd ).is_ok() );
/// ```
pub fn validate_parameter_storage_types( cmd : &CommandDefinition ) -> Result< (), Error >
{
  use crate::data::Kind;

  for arg in cmd.arguments()
  {
    if arg.attributes.multiple
    {
      match &arg.kind
      {
        Kind::List( _, _ ) => {
          // Correct - multiple:true with List storage
        }
        _ => {
          return Err( Error::Registration( format!(
            "Parameter '{}' in command '{}' has multiple:true but storage type is {:?}. \
            Parameters accepting multiple values must use Kind::List storage to prevent data loss. \
            \n\nWithout List storage, multiple values silently overwrite each other instead of accumulating. \
            \n\nChange to: Kind::List( Box::new( Kind::String ), None ) or similar List variant.",
            arg.name,
            cmd.name().as_str(),
            arg.kind
          )));
        }
      }
    }
  }
  Ok(())
}

/// Returns true when `text` reads as a bare `a|b|c` choice list.
///
/// Whole-text match needs >= 2 pipe-separated bare words; an embedded token
/// needs >= 3 so prose that merely contains one pipe (e.g. "name|size" inside
/// a sentence) does not trigger the help-convention warning.
fn looks_like_choice_list( text : &str ) -> bool
{
  fn is_bare_choice_segment( segment : &str ) -> bool
  {
    !segment.is_empty()
      && segment.len() <= 20
      && segment.chars().all( | c | c.is_ascii_alphanumeric() || c == '_' || c == '-' )
  }

  let is_list = | candidate : &str, min_segments : usize |
  {
    let segments : Vec< &str > = candidate.split( '|' ).collect();
    segments.len() >= min_segments && segments.iter().all( | s | is_bare_choice_segment( s ) )
  };

  let trimmed = text.trim().trim_end_matches( [ '.', ',' ] );
  if is_list( trimmed, 2 )
  {
    return true;
  }
  trimmed.split_whitespace().any( | token | is_list( token.trim_end_matches( [ '.', ',' ] ), 3 ) )
}

/// Lints help-affecting parameter conventions.
///
/// Hard errors for definitions that can never work (an `Enum` default outside
/// its own choices — the default could never pass coercion); non-fatal
/// warnings for definitions that degrade `??` help quality (a `String`
/// parameter whose description embeds an `a|b|c` choice list instead of
/// declaring `Kind::Enum`, which would let help list the choices and coercion
/// reject invalid values).
///
/// # Errors
///
/// Returns `Error::Registration` if any `Enum` parameter's default value is
/// not one of its own choices.
///
/// # Examples
///
/// ```rust
/// use unilang::prelude::*;
/// use unilang::command_validation::validate_help_conventions;
///
/// let cmd = CommandDefinition::former()
///   .name( ".render" )
///   .description( "Render output" )
///   .arguments( vec![
///     ArgumentDefinition {
///       name: "format".to_string(),
///       description: "Output format: json|yaml|table".to_string(),
///       kind: Kind::String,
///       hint: String::new(),
///       attributes: ArgumentAttributes { optional: true, ..Default::default() },
///       validation_rules: vec![],
///       aliases: vec![],
///       tags: vec![],
///     }
///   ])
///   .end();
///
/// let warnings = validate_help_conventions( &cmd ).unwrap();
/// assert_eq!( warnings.len(), 1 ); // String kind with embedded choice list
/// ```
pub fn validate_help_conventions( cmd : &CommandDefinition ) -> Result< Vec< String >, Error >
{
  use crate::data::Kind;

  let mut warnings = Vec::new();
  for arg in cmd.arguments()
  {
    if let Kind::Enum( choices ) = &arg.kind
    {
      if let Some( default ) = &arg.attributes.default
      {
        if !choices.iter().any( | c | c == default )
        {
          return Err( Error::Registration( format!(
            "Parameter '{}' in command '{}' declares default '{}' which is not among its enum choices {:?}. \
            The default could never pass coercion. Add it to the choices or change the default.",
            arg.name,
            cmd.full_name(),
            default,
            choices
          )));
        }
      }
    }

    if arg.kind == Kind::String && looks_like_choice_list( &arg.description )
    {
      warnings.push( format!(
        "Parameter '{}' in command '{}' is Kind::String but its description looks like a choice list ('{}'). \
        Declare it as Kind::Enum so '{} {}::??' help can list the choices and invalid values are rejected.",
        arg.name,
        cmd.full_name(),
        arg.description,
        cmd.full_name(),
        arg.name
      ));
    }
  }
  Ok( warnings )
}

/// Validates entire command definition for registration.
///
/// Checks:
/// - Command name has dot prefix
/// - Namespace has dot prefix (if non-empty)
/// - Parameter storage types match multiple attribute
/// - Help conventions (enum default within choices — error; `String`
///   parameter with an `a|b|c` choice-list description — warning printed to
///   stderr, suppressible via `UNILANG_NO_LINT_WARNINGS`)
///
/// This is the primary validation function used by all registration paths.
///
/// # Errors
///
/// Returns `Error::Registration` if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::prelude::*;
///
/// let cmd = CommandDefinition::former()
///   .name( ".hello".to_string() )
///   .description( "Test command".to_string() )
///   .end();
///
/// assert!( validate_command_for_registration( &cmd ).is_ok() );
/// ```
pub fn validate_command_for_registration( cmd : &CommandDefinition ) -> Result< (), Error >
{
  // Validate the final full name (which combines namespace + name)
  let full_name = cmd.full_name();
  if !full_name.starts_with( '.' )
  {
    return Err( Error::Registration( format!(
      "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
      This enforces explicit naming with minimal implicit transformations.",
      full_name
    )));
  }

  validate_namespace( cmd.namespace() )?;
  validate_parameter_storage_types( cmd )?;
  for warning in validate_help_conventions( cmd )?
  {
    if std::env::var_os( "UNILANG_NO_LINT_WARNINGS" ).is_none()
    {
      eprintln!( "unilang: warning: {warning}" );
    }
  }
  Ok(())
}

/// Checks if command name ends with ".help" suffix.
///
/// Used to avoid creating help commands for help commands (prevent recursion).
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::is_help_command;
///
/// assert!( is_help_command( ".hello.help" ) );
/// assert!( !is_help_command( ".hello" ) );
/// ```
#[ must_use ]
#[ allow( clippy::case_sensitive_file_extension_comparisons ) ] // .help is not a file extension
pub fn is_help_command( full_name : &str ) -> bool
{
  full_name.ends_with( ".help" )
}

/// Builds help command name from command name.
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::make_help_command_name;
///
/// assert_eq!( make_help_command_name( ".hello" ), ".hello.help" );
/// assert_eq!( make_help_command_name( ".cmd1.process" ), ".cmd1.process.help" );
/// ```
#[ must_use ]
pub fn make_help_command_name( full_name : &str ) -> String
{
  format!( "{}.help", full_name )
}

}

mod_interface::mod_interface!
{
  exposed use private::validate_command_name;
  exposed use private::validate_namespace;
  exposed use private::validate_parameter_storage_types;
  exposed use private::validate_help_conventions;
  exposed use private::validate_command_for_registration;
  exposed use private::is_help_command;
  exposed use private::make_help_command_name;

  prelude use private::validate_command_name;
  prelude use private::validate_namespace;
  prelude use private::validate_parameter_storage_types;
  prelude use private::validate_help_conventions;
  prelude use private::validate_command_for_registration;
  prelude use private::is_help_command;
  prelude use private::make_help_command_name;
}
