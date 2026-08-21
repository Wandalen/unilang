use crate::data::{ ArgumentDefinition, CommandDefinition, ErrorData, ErrorCode };
use crate::error::Error;
use crate::types::Value;
use unilang_parser::GenericInstruction;
use std::collections::HashMap;
use super::core::SemanticAnalyzer;
use regex::Regex;

impl SemanticAnalyzer< '_ >
{
  pub( in super ) fn validate_bound_argument( bound_arguments : &HashMap< String, Value >, arg_def : &ArgumentDefinition ) -> Result< (), Error >
  {
    if let Some( value ) = bound_arguments.get( &arg_def.name )
    {
      for rule in &arg_def.validation_rules
      {
        if !Self::apply_validation_rule( value, rule )
        {
          let error_message = Self::format_validation_error( &arg_def.name, value, rule, arg_def.attributes.sensitive );
          return Err( Error::Execution( ErrorData::new(
            ErrorCode::ValidationRuleFailed,
            error_message,
          )));
        }
      }
    }

    Ok( () )
  }

  /// Formats a detailed validation error message with actual vs expected values.
  ///
  /// This provides user-friendly error messages that explain exactly what validation
  /// rule failed, what value was provided, and what was expected.
  /// When `sensitive` is true the raw value is replaced with `[REDACTED]` to prevent
  /// credential leakage in error logs (NFR-SEC-1).
  pub( in super ) fn format_validation_error( arg_name : &str, value : &Value, rule : &crate::data::ValidationRule, sensitive : bool ) -> String
  {
    use crate::data::ValidationRule;

    let value_str = if sensitive
    {
      "[REDACTED]".to_string()
    }
    else
    {
      match value
      {
        Value::String( s ) => format!( "\"{}\"", s ),
        Value::Integer( i ) => i.to_string(),
        Value::Float( f ) => f.to_string(),
        Value::Boolean( b ) => b.to_string(),
        Value::List( l ) => format!( "[{} items]", l.len() ),
        Value::Path( p ) | Value::File( p ) | Value::Directory( p ) => format!( "\"{}\"", p.display() ),
        Value::Enum( s ) | Value::JsonString( s ) => format!( "\"{}\"", s ),
        Value::Url( u ) => format!( "\"{}\"", u ),
        Value::DateTime( dt ) => format!( "\"{}\"", dt.to_rfc3339() ),
        Value::Pattern( r ) => format!( "\"{}\"", r.as_str() ),
        Value::Map( m ) => format!( "{{{} entries}}", m.len() ),
        #[ cfg( feature = "json_parser" ) ]
        Value::Object( o ) => o.to_string(),
      }
    };

    match rule
    {
      ValidationRule::Min( min_val ) => format!(
        "Validation Error: Argument '{}' has value {} which is less than the minimum allowed value of {}. Please provide a value >= {}.",
        arg_name, value_str, min_val, min_val
      ),
      ValidationRule::Max( max_val ) => format!(
        "Validation Error: Argument '{}' has value {} which exceeds the maximum allowed value of {}. Please provide a value <= {}.",
        arg_name, value_str, max_val, max_val
      ),
      ValidationRule::MinLength( min_len ) =>
      {
        let actual_len = match value
        {
          Value::String( s ) | Value::Enum( s ) | Value::JsonString( s ) => s.len(),
          Value::List( l ) => l.len(),
          Value::Map( m ) => m.len(),
          Value::Url( u ) => u.as_str().len(),
          Value::DateTime( dt ) => dt.to_rfc3339().len(),
          Value::Pattern( r ) => r.as_str().len(),
          Value::Integer( _ ) | Value::Float( _ ) | Value::Boolean( _ )
          | Value::Path( _ ) | Value::File( _ ) | Value::Directory( _ ) => 0,
          #[ cfg( feature = "json_parser" ) ]
          Value::Object( _ ) => 0,
        };
        format!(
          "Validation Error: Argument '{}' has length {} which is less than the minimum required length of {}. Please provide a value with at least {} characters/items.",
          arg_name, actual_len, min_len, min_len
        )
      },
      ValidationRule::MaxLength( max_len ) =>
      {
        let actual_len = match value
        {
          Value::String( s ) | Value::Enum( s ) | Value::JsonString( s ) => s.len(),
          Value::List( l ) => l.len(),
          Value::Map( m ) => m.len(),
          Value::Url( u ) => u.as_str().len(),
          Value::DateTime( dt ) => dt.to_rfc3339().len(),
          Value::Pattern( r ) => r.as_str().len(),
          Value::Integer( _ ) | Value::Float( _ ) | Value::Boolean( _ )
          | Value::Path( _ ) | Value::File( _ ) | Value::Directory( _ ) => 0,
          #[ cfg( feature = "json_parser" ) ]
          Value::Object( _ ) => 0,
        };
        format!(
          "Validation Error: Argument '{}' has length {} which exceeds the maximum allowed length of {}. Please provide a value with at most {} characters/items.",
          arg_name, actual_len, max_len, max_len
        )
      },
      ValidationRule::Pattern( pattern_str ) => format!(
        "Validation Error: Argument '{}' with value {} does not match the required pattern '{}'. Please provide a value matching this pattern.",
        arg_name, value_str, pattern_str
      ),
      ValidationRule::MinItems( min_items ) =>
      {
        let actual_items = match value
        {
          Value::List( l ) => l.len(),
          Value::Map( m ) => m.len(),
          Value::String( _ ) | Value::Integer( _ ) | Value::Float( _ ) | Value::Boolean( _ )
          | Value::Path( _ ) | Value::File( _ ) | Value::Directory( _ )
          | Value::Enum( _ ) | Value::Url( _ ) | Value::DateTime( _ )
          | Value::Pattern( _ ) | Value::JsonString( _ ) => 0,
          #[ cfg( feature = "json_parser" ) ]
          Value::Object( _ ) => 0,
        };
        format!(
          "Validation Error: Argument '{}' has {} items which is less than the minimum required {} items. Please provide at least {} items.",
          arg_name, actual_items, min_items, min_items
        )
      },
    }
  }

  pub( in super ) fn check_excess_positional_arguments( instruction : &GenericInstruction, positional_idx : usize ) -> Result< (), Error >
  {
    if positional_idx < instruction.positional_arguments.len()
    {
      return Err( Error::Execution( ErrorData::new(
        ErrorCode::TooManyArguments,
        "Argument Error: Too many arguments provided for this command. Please check the command usage and remove extra arguments.".to_string(),
      )));
    }

    Ok( () )
  }

  ///
  /// Checks for unknown named arguments that don't match any defined parameter.
  ///
  /// This function validates that all named parameters in the instruction correspond
  /// to actual parameter definitions (including aliases). If unknown parameters are found,
  /// it returns an error with helpful suggestions for similar parameter names.
  ///
  /// # Arguments
  /// * `instruction` - The parsed instruction containing named arguments
  /// * `command_def` - The command definition with valid parameter names
  ///
  /// # Returns
  /// * `Ok(())` if all named arguments are valid
  /// * `Err` with UNILANG_UNKNOWN_PARAMETER if invalid parameters are found
  ///
  /// # Error Format
  /// - Single unknown: "Unknown parameter 'drry'. Did you mean 'dry'?"
  /// - Multiple unknown: "Unknown parameters: 'drry', 'foo'. Check command help for valid parameters."
  pub( in super ) fn check_unknown_named_arguments( instruction : &GenericInstruction, command_def : &CommandDefinition ) -> Result< (), Error >
  {
    // Collect all valid parameter names (canonical names + aliases)
    let mut valid_names = std::collections::HashSet::new();
    for arg_def in command_def.arguments()
    {
      valid_names.insert( arg_def.name.as_str() );
      for alias in &arg_def.aliases
      {
        valid_names.insert( alias.as_str() );
      }
    }

    // Find unknown parameters in the instruction
    let mut unknown_params: Vec< &str > = Vec::new();
    for param_name in instruction.named_arguments.keys()
    {
      if !valid_names.contains( param_name.as_str() )
      {
        unknown_params.push( param_name );
      }
    }

    // If no unknown parameters, validation passes
    if unknown_params.is_empty()
    {
      return Ok( () );
    }

    // Generate helpful error message with suggestions
    let error_message = if unknown_params.len() == 1
    {
      let unknown = unknown_params[ 0 ];

      // Find best suggestion using Levenshtein distance
      let suggestion = Self::find_closest_parameter_name( unknown, &valid_names );

      if let Some( suggested_name ) = suggestion
      {
        format!(
          "Argument Error: Unknown parameter '{}'. Did you mean '{}'? Use '{} ??' for help.",
          unknown,
          suggested_name,
          command_def.full_name()
        )
      }
      else
      {
        format!(
          "Argument Error: Unknown parameter '{}'. Use '{} ??' to see valid parameters.",
          unknown,
          command_def.full_name()
        )
      }
    }
    else
    {
      // Multiple unknown parameters
      let params_list = unknown_params.iter()
        .map( | p | format!( "'{}'", p ) )
        .collect::< Vec< _ > >()
        .join( ", " );

      format!(
        "Argument Error: Unknown parameters: {}. Use '{} ??' to see valid parameters.",
        params_list,
        command_def.full_name()
      )
    };

    Err( Error::Execution( ErrorData::new(
      ErrorCode::UnknownParameter,
      error_message,
    )))
  }

  ///
  /// Finds the closest matching parameter name using Levenshtein distance.
  ///
  /// This provides helpful "Did you mean..." suggestions when users make typos
  /// in parameter names. Only suggests if the similarity is high enough (distance <= 2).
  ///
  /// # Arguments
  /// * `unknown` - The unknown parameter name
  /// * `valid_names` - Set of all valid parameter names
  ///
  /// # Returns
  /// * `Some(name)` - Best matching parameter name if similarity threshold met
  /// * `None` - No close match found
  ///
  /// # Examples
  /// - "drry" → Some("dry") (distance: 1)
  /// - "verbse" → Some("verbose") (distance: 1)
  /// - "xyz" → None (no close matches)
  pub( in super ) fn find_closest_parameter_name( unknown : &str, valid_names : &std::collections::HashSet< &str > ) -> Option< String >
  {
    let mut best_match: Option< ( &str, usize ) > = None;

    for valid_name in valid_names
    {
      let distance = Self::levenshtein_distance( unknown, valid_name );

      // Only suggest if distance is small (good match)
      // and it's better than previous best
      if distance <= 2
      {
        match best_match
        {
          None => best_match = Some( ( valid_name, distance ) ),
          Some( ( _, prev_distance ) ) if distance < prev_distance =>
          {
            best_match = Some( ( valid_name, distance ) );
          },
          _ => {},
        }
      }
    }

    best_match.map( | ( name, _ ) | name.to_string() )
  }

  ///
  /// Calculates Levenshtein distance between two strings.
  ///
  /// Levenshtein distance is the minimum number of single-character edits
  /// (insertions, deletions, or substitutions) required to change one string
  /// into another. Used for fuzzy matching and typo detection.
  ///
  /// # Arguments
  /// * `a` - First string
  /// * `b` - Second string
  ///
  /// # Returns
  /// * `usize` - The edit distance between the strings
  ///
  /// # Algorithm
  /// Classic dynamic programming approach with O(n*m) time and space complexity.
  ///
  /// # Examples
  /// - levenshtein("drry", "dry") = 1 (delete 'r')
  /// - levenshtein("verbse", "verbose") = 1 (insert 'o')
  /// - levenshtein("cat", "dog") = 3 (substitute all)
  pub( in super ) fn levenshtein_distance( a : &str, b : &str ) -> usize
  {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0
    {
      return b_len;
    }
    if b_len == 0
    {
      return a_len;
    }

    // Create distance matrix
    let mut matrix = vec![ vec![ 0usize; b_len + 1 ]; a_len + 1 ];

    // Initialize first column (each row's [0] = row index)
    for ( i, row ) in matrix.iter_mut().enumerate().take( a_len + 1 )
    {
      row[ 0 ] = i;
    }
    // Initialize first row (each column's [0] = column index)
    for ( j, val ) in matrix[ 0 ].iter_mut().enumerate().take( b_len + 1 )
    {
      *val = j;
    }

    // Compute distances
    let a_chars: Vec< char > = a.chars().collect();
    let b_chars: Vec< char > = b.chars().collect();

    for i in 1..=a_len
    {
      for j in 1..=b_len
      {
        let cost = usize::from( a_chars[ i - 1 ] != b_chars[ j - 1 ] );

        matrix[ i ][ j ] = std::cmp::min(
          std::cmp::min(
            matrix[ i - 1 ][ j ] + 1,      // deletion
            matrix[ i ][ j - 1 ] + 1       // insertion
          ),
          matrix[ i - 1 ][ j - 1 ] + cost  // substitution
        );
      }
    }

    matrix[ a_len ][ b_len ]
  }

  /// Applies a single validation rule to a parsed value.
  #[ allow( clippy::cast_precision_loss ) ] // Allow casting i64 to f64 for min/max comparison
  pub( in super ) fn apply_validation_rule( value : &Value, rule : &crate::data::ValidationRule ) -> bool
  {
    use crate::data::ValidationRule;
    match rule
    {
      ValidationRule::Min( min_val ) => match value
      {
        Value::Integer( i ) => *i as f64 >= *min_val,
        Value::Float( f ) => *f >= *min_val,
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::Max( max_val ) => match value
      {
        Value::Integer( i ) => *i as f64 <= *max_val,
        Value::Float( f ) => *f <= *max_val,
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::MinLength( min_len ) => match value
      {
        Value::String( s ) => s.len() >= *min_len,
        Value::List( l ) => l.len() >= *min_len,
        _ => false,
      },
      ValidationRule::MaxLength( max_len ) => match value
      {
        Value::String( s ) => s.len() <= *max_len,
        Value::List( l ) => l.len() <= *max_len,
        _ => false,
      },
      ValidationRule::Pattern( pattern_str ) => match value
      {
        Value::String( s ) =>
        {
          if let Ok( regex ) = Regex::new( pattern_str )
          {
            regex.is_match( s )
          }
          else
          {
            false
          }
        },
        _ => false, // Rule not applicable or type mismatch
      },
      ValidationRule::MinItems( min_items ) => match value
      {
        Value::List( l ) => l.len() >= *min_items,
        _ => false,
      },
    }
  }
}

// Suppress unused import warnings — these imports are used by method bodies above
#[ allow( unused_imports ) ]
use ArgumentDefinition as _;
#[ allow( unused_imports ) ]
use HashMap as _;
#[ allow( unused_imports ) ]
use GenericInstruction as _;
