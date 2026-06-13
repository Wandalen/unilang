//! Error message parsing helpers for the pipeline.
//!
//! Extracts structured information from unstructured error message strings
//! produced by the semantic analyzer and interpreter.

/// Extracts an interactive argument name from an error message.
pub( super ) fn extract_interactive_argument( error_msg : &str ) -> Option< &str >
{
  // Look for patterns like "The argument 'arg_name' is marked as interactive"
  if let Some( start ) = error_msg.find( "The argument '" )
  {
    let after = &error_msg[ start + "The argument '".len().. ];
    if let Some( end ) = after.find( '\'' )
    {
      return Some( &after[ ..end ] );
    }
  }

  // Fallback: look for "Interactive Argument Required: <arg_name>"
  if let Some( start ) = error_msg.find( "Interactive Argument Required:" )
  {
    let after_prefix = &error_msg[ start + "Interactive Argument Required:".len().. ];
    if let Some( arg_start ) = after_prefix.find( |c: char| !c.is_whitespace() )
    {
      let arg_part = &after_prefix[ arg_start.. ];
      if let Some( arg_end ) = arg_part.find( |c: char| c.is_whitespace() )
      {
        return Some( &arg_part[ ..arg_end ] );
      }
      return Some( arg_part );
    }
  }

  // Another fallback: look for "argument '" pattern
  if let Some( start ) = error_msg.find( "argument '" )
  {
    let after = &error_msg[ start + "argument '".len().. ];
    if let Some( end ) = after.find( '\'' )
    {
      return Some( &after[ ..end ] );
    }
  }

  None
}

/// Extracts a command name from an error message.
pub( super ) fn extract_command_from_error( error_msg : &str ) -> Option< &str >
{
  // Look for "for command <name>" pattern
  if let Some( start ) = error_msg.find( "for command " )
  {
    let after = &error_msg[ start + "for command ".len().. ];
    if let Some( end ) = after.find( |c: char| c.is_whitespace() )
    {
      return Some( &after[ ..end ] );
    }
    return Some( after );
  }

  // Look for "command '<name>'" pattern
  if let Some( start ) = error_msg.find( "command '" )
  {
    let after = &error_msg[ start + "command '".len().. ];
    if let Some( end ) = after.find( '\'' )
    {
      return Some( &after[ ..end ] );
    }
  }

  None
}

/// Extracts the list of available commands from a help error message.
pub( super ) fn extract_available_commands( error_msg : &str ) -> Vec< String >
{
  let mut commands = Vec::new();
  let mut in_commands_section = false;

  for line in error_msg.lines()
  {
    let line = line.trim();

    if line.contains( "Available commands:" )
    {
      in_commands_section = true;
      continue;
    }

    if in_commands_section
    {
      // Stop if we hit an empty line or different section
      if line.is_empty() || line.starts_with( "Use" ) || line.starts_with( "For" )
      {
        break;
      }

      // Extract command names - they typically start with '.'
      // Handle various indentation patterns
      if let Some( stripped ) = line.strip_prefix( '.' )
      {
        // Direct command line
        if let Some( cmd_end ) = stripped.find( ' ' )
        {
          commands.push( stripped[ ..cmd_end ].to_string() );
        }
        else
        {
          commands.push( stripped.to_string() );
        }
      }
      else if line.contains( '.' )
      {
        // Find the first '.' in the line and extract command
        if let Some( dot_pos ) = line.find( '.' )
        {
          let after_dot = &line[ dot_pos + 1.. ];
          if let Some( cmd_end ) = after_dot.find( ' ' )
          {
            commands.push( after_dot[ ..cmd_end ].to_string() );
          }
          else
          {
            commands.push( after_dot.to_string() );
          }
        }
      }
    }
  }

  commands
}

/// Extracts command suggestions from a "Did you mean:" error message.
pub( super ) fn extract_command_suggestions( error_msg : &str ) -> Vec< String >
{
  let mut suggestions = Vec::new();

  // Look for "Did you mean:" pattern
  if let Some( start ) = error_msg.find( "Did you mean:" )
  {
    let after = &error_msg[ start + "Did you mean:".len().. ];
    for word in after.split_whitespace()
    {
      if word.starts_with( '.' )
      {
        suggestions.push( word.trim_end_matches( ',' ).trim_end_matches( '?' ).to_string() );
      }
    }
  }

  suggestions
}

/// Formats a list of command names into a help content string.
pub( super ) fn format_help_content( commands : &[ String ] ) -> String
{
  if commands.is_empty()
  {
    "No commands available.".to_string()
  }
  else
  {
    let mut content = "Available commands:\n".to_string();
    for command in commands
    {
      content.push_str( &format!( "  .{}\n", command ) );
    }
    content
  }
}
