use crate::data::{ ArgumentDefinition, CommandDefinition, ErrorData, ErrorCode };
use crate::error::Error;
use crate::types::{ parse_value, Value };
use unilang_parser::{ Argument, GenericInstruction };
use std::collections::HashMap;
use super::core::{ SemanticAnalyzer, VerifiedCommand };

impl SemanticAnalyzer< '_ >
{
  ///
  /// Binds the arguments from a statement to the command definition.
  /// This function checks for the correct number and types of arguments,
  /// returning an error if validation fails.
  ///
  /// ## Validation Order (Critical for UX)
  /// 1. Try to bind all arguments (collect missing, but don't error yet)
  /// 2. Check for unknown parameters FIRST → provides "Did you mean" suggestions
  /// 3. Then check for missing required arguments
  ///
  /// This ordering ensures users get helpful typo suggestions instead of
  /// generic "missing argument" errors when they make typos.
  pub( in super ) fn bind_arguments( instruction : &GenericInstruction, command_def : &CommandDefinition ) -> Result< HashMap< String, Value >, Error >
  {
    let mut bound_arguments = HashMap::new();
    let mut positional_idx = 0;
    let mut missing_args = Vec::new();

    // Pass 1: Try to bind all arguments, collect which ones are missing
    for arg_def in command_def.arguments()
    {
      let value_found = Self::try_bind_named_argument( instruction, arg_def, &mut bound_arguments )?
        || Self::try_bind_positional_argument( instruction, arg_def, &mut bound_arguments, &mut positional_idx )?;

      if value_found
      {
        Self::validate_bound_argument( &bound_arguments, arg_def )?;
      }
      else
      {
        missing_args.push( arg_def );
      }
    }

    // Pass 2: Check for unknown parameters FIRST (provides helpful typo suggestions)
    Self::check_unknown_named_arguments( instruction, command_def )?;
    Self::check_excess_positional_arguments( instruction, positional_idx )?;

    // Pass 3: Now handle missing arguments (after unknown parameters are checked)
    for arg_def in missing_args
    {
      Self::handle_missing_argument( arg_def, &mut bound_arguments )?;
    }

    Ok( bound_arguments )
  }

  pub( in super ) fn try_bind_named_argument( instruction : &GenericInstruction, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< bool, Error >
  {
    // TASK 024 ENHANCEMENT: Collect all arguments matching canonical name AND all aliases
    let mut all_matching_args = Vec::new();
    let mut found_any = false;

    // Collect arguments by canonical name
    if let Some( parser_args ) = instruction.named_arguments.get( &arg_def.name )
    {
      all_matching_args.extend_from_slice( parser_args );
      found_any = true;
    }

    // Collect arguments by all aliases
    for alias in &arg_def.aliases
    {
      if let Some( parser_args ) = instruction.named_arguments.get( alias )
      {
        all_matching_args.extend_from_slice( parser_args );
        found_any = true;
      }
    }

    if found_any
    {
      Self::bind_argument_values( &all_matching_args, arg_def, bound_arguments )?;
      Ok( true )
    }
    else
    {
      Ok( false )
    }
  }

  pub( in super ) fn try_bind_positional_argument( instruction : &GenericInstruction, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value >, positional_idx : &mut usize ) -> Result< bool, Error >
  {
    if *positional_idx >= instruction.positional_arguments.len()
    {
      return Ok( false );
    }

    if arg_def.attributes.multiple
    {
      let mut values = Vec::new();
      while *positional_idx < instruction.positional_arguments.len()
      {
        let parser_arg = &instruction.positional_arguments[ *positional_idx ];
        values.push( coerce_arg_value( &parser_arg.value, arg_def )? );
        *positional_idx += 1;
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else
    {
      let parser_arg = &instruction.positional_arguments[ *positional_idx ];
      bound_arguments.insert( arg_def.name.clone(), coerce_arg_value( &parser_arg.value, arg_def )? );
      *positional_idx += 1;
    }

    Ok( true )
  }

  pub( in super ) fn bind_argument_values( parser_args : &Vec< Argument >, arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< (), Error >
  {
    // TASK 024 FIX: Automatic Multiple Parameter Collection
    // Always collect multiple values into a list, regardless of the `multiple` attribute
    // This implements requirement R1: "When the same parameter name appears multiple times, collect ALL values into a list"

    if parser_args.len() > 1
    {
      // Multiple values detected - always collect into a list
      let mut values = Vec::new();
      for parser_arg in parser_args
      {
        values.push( coerce_arg_value( &parser_arg.value, arg_def )? );
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else if arg_def.attributes.multiple
    {
      // Single value but multiple=true - wrap in list for consistency
      let mut values = Vec::new();
      if let Some( parser_arg ) = parser_args.first()
      {
        values.push( coerce_arg_value( &parser_arg.value, arg_def )? );
      }
      bound_arguments.insert( arg_def.name.clone(), Value::List( values ) );
    }
    else if let Some( parser_arg ) = parser_args.first()
    {
      // Single value and multiple=false - keep as single value
      bound_arguments.insert( arg_def.name.clone(), coerce_arg_value( &parser_arg.value, arg_def )? );
    }

    Ok( () )
  }

  pub( in super ) fn handle_missing_argument( arg_def : &ArgumentDefinition, bound_arguments : &mut HashMap< String, Value > ) -> Result< (), Error >
  {
    if !arg_def.attributes.optional
    {
      if arg_def.attributes.interactive
      {
        // Critical REPL Implementation: Interactive Argument Signaling
        // This is the core implementation of FR-INTERACTIVE-1 requirement
        // ✅ SPECIFICATION COMPLIANCE: Return exact error code as specified
        // This error is designed to be caught by REPL loops for secure input prompting
        //
        // ⚠️ SECURITY NOTE: The error message intentionally doesn't contain the argument value
        // to prevent sensitive data (passwords, API keys) from being logged or displayed
        //
        // 📝 REPL INTEGRATION: REPL implementations should:
        // 1. Catch this specific error code
        // 2. Present secure input prompt to user
        // 3. Mask input if arg_def.attributes.sensitive is true
        // 4. Re-execute the command with the provided interactive value
        return Err( Error::Execution( ErrorData::new(
          ErrorCode::ArgumentInteractiveRequired,
          format!( "Interactive Argument Required: The argument '{}' is marked as interactive and must be provided interactively. The application should prompt the user for this value.", arg_def.name ),
        )));
      }

      return Err( Error::Execution( ErrorData::new(
        ErrorCode::ArgumentMissing,
        format!( "Argument Error: The required argument '{}' is missing. Please provide a value for this argument.", arg_def.name ),
      )));
    }
    else if let Some( default_value ) = &arg_def.attributes.default
    {
      bound_arguments.insert( arg_def.name.clone(), coerce_arg_value( default_value, arg_def )? );
    }

    Ok( () )
  }
}

// Suppress unused import warnings — these imports are used by method bodies above
#[ allow( unused_imports ) ]
use VerifiedCommand as _;

/// Coerces a raw string value to a typed `Value` for a specific argument definition.
///
/// Produces `ErrorCode::ArgumentTypeMismatch` on failure, distinguishing argument-level
/// coercion errors from internal `TypeMismatch` errors (which come from `From<TypeError>`
/// for non-argument contexts). Sensitive argument values are excluded from the error
/// message to prevent credential leakage in logs.
fn coerce_arg_value( input : &str, arg_def : &ArgumentDefinition ) -> Result< Value, Error >
{
  parse_value( input, &arg_def.kind ).map_err( | type_err |
  {
    // Do not include the raw value in the message for sensitive arguments
    // (prevents passwords, API keys, and tokens from appearing in error logs)
    let detail = if arg_def.attributes.sensitive
    {
      "type coercion failed (value redacted — sensitive argument)".to_string()
    }
    else
    {
      type_err.reason.clone()
    };
    // A failing "?" or empty value usually means the user wanted help, not a
    // value — point at the real help syntax. Suppressed for sensitive
    // arguments so the message reveals nothing about the attempted value.
    let nudge = if !arg_def.attributes.sensitive && ( input == "?" || input.is_empty() )
    {
      format!( " Did you mean '{}::??' for parameter help?", arg_def.name )
    }
    else
    {
      String::new()
    };
    Error::Execution( ErrorData::new(
      ErrorCode::ArgumentTypeMismatch,
      format!(
        "Argument Error: Cannot coerce value for argument '{}' to {:?}. {}{}",
        arg_def.name, arg_def.kind, detail, nudge
      ),
    ))
  })
}
