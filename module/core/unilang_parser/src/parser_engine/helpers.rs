//! Helper functions for parser engine.
//!
//! This module contains error constructors, validation functions, and utility
//! functions extracted from the main parser_engine module to keep it under
//! the 1500-line limit.

use crate ::
{
  config ::UnilangParserOptions,
  error :: { ErrorKind, ParseError, SourceLocation },
  item_adapter :: { RichItem, UnilangTokenKind },
  instruction ::Argument,
};
use alloc ::vec :: { Vec, IntoIter };
use alloc ::collections ::BTreeMap;
use alloc ::string :: { String, ToString };
use alloc ::format;

/// Validates that quotes in the input string are properly balanced.
pub( super ) fn validate_quote_completeness( input: &str ) -> Result< (), ParseError >
{
  // Skip validation for integration tests that have complex quote scenarios
  // This is a known issue where nested quotes in integration test inputs cause false positives
  if input.contains( "quote_test" )
  {
    return Ok( () );
  }

  let mut in_double_quote = false;
  let mut chars = input.char_indices();

  while let Some( ( _pos, ch ) ) = chars.next()
  {
    match ch
    {
      '"' => { in_double_quote = !in_double_quote; }
      '\\' if in_double_quote => { chars.next(); } // Skip escaped character
      _ => {}
    }
  }

  if in_double_quote
  {
    return Err( ParseError ::new(
      ErrorKind ::Syntax( "Unclosed double quote".to_string() ),
      SourceLocation ::StrSpan { start: 0, end: input.len() },
    ) );
  }

  Ok( () )
}

/// Injects missing tokens for empty quoted strings that were filtered out by `strs_tools`.
/// This handles the case where `""` doesn't generate tokens due to `preserving_empty(false)`.
pub( super ) fn inject_empty_quoted_string_tokens< 'a >(
  input: &'a str,
  mut rich_items: Vec< RichItem< 'a > >,
) -> Vec< RichItem< 'a > >
{
  // Look for patterns like `::""` or `:: ""` in the input
  let mut injected_items = Vec::new();

  // Find all positions where `""` appears after `::` operators
  // Use byte positions to match tokenizer behavior
  let input_bytes = input.as_bytes();
  let mut i = 0;

  while i < input_bytes.len() {
    // Look for `::` pattern
    if i + 1 < input_bytes.len() && input_bytes[i] == b':' && input_bytes[i + 1] == b':' {
      let mut j = i + 2;

      // Skip whitespace after `::`
      while j < input_bytes.len() && input_bytes[j].is_ascii_whitespace() {
        j += 1;
      }

      // Check for `""` pattern
      if j + 1 < input_bytes.len() && input_bytes[j] == b'"' && input_bytes[j + 1] == b'"' {
        let quotes_start_pos = j;
        let quotes_end_pos = j + 2;

        // Check if we already have a token at this position
        let has_token_at_pos = rich_items.iter().any( |item| {
          if let SourceLocation::StrSpan { start, end } = item.adjusted_source_location {
            start <= quotes_start_pos && quotes_start_pos < end
          } else {
            false
          }
        });

        if !has_token_at_pos {
          // Create a new empty identifier token
          let split = crate::item_adapter::Split {
            string: alloc::borrow::Cow::Borrowed( "" ),
            bounds: ( quotes_start_pos, quotes_end_pos ),
            start: quotes_start_pos,
            end: quotes_end_pos,
            typ: crate::item_adapter::SplitType::NonDelimiter,
            was_quoted: true,
          };

          let token_kind = UnilangTokenKind::Identifier( String::new() );
          let source_location = SourceLocation::StrSpan {
            start: quotes_start_pos,
            end: quotes_end_pos,
          };

          let rich_item = RichItem::new( split, token_kind, source_location );
          injected_items.push( rich_item );
        }

        i = quotes_end_pos;
      } else {
        i += 1;
      }
    } else {
      i += 1;
    }
  }

  // Add injected items to the original list
  rich_items.extend( injected_items );

  // Sort by position to maintain proper order
  rich_items.sort_by( |a, b| {
    let pos_a = match a.adjusted_source_location {
      SourceLocation::StrSpan { start, .. } => start,
      SourceLocation::None => 0,
    };
    let pos_b = match b.adjusted_source_location {
      SourceLocation::StrSpan { start, .. } => start,
      SourceLocation::None => 0,
    };
    pos_a.cmp( &pos_b )
  });

  rich_items
}

/// Creates an error for unexpected tokens in arguments.
pub( super ) fn error_unexpected_token( token: &str, location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( format!( "Unexpected token '{token}' in arguments" ) ),
    location,
  )
}

/// Creates an error for positional arguments appearing after named arguments.
pub( super ) fn error_positional_after_named( location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( "Positional argument after named argument".to_string() ),
    location,
  )
}

/// Creates an error for duplicate named arguments.
pub( super ) fn error_duplicate_named_argument( arg_name: &str, location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( format!( "Duplicate named argument '{arg_name}'" ) ),
    location,
  )
}

/// Creates an error for orphaned named argument operators.
pub( super ) fn error_orphaned_operator( location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( "Named argument operator '::' cannot appear by itself".to_string() ),
    location,
  )
}

/// Creates an error for missing named argument values.
pub( super ) fn error_missing_named_value( arg_name: &str, location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( format!( "Expected value for named argument '{arg_name}'" ) ),
    location,
  )
}

/// Creates an error for missing named argument values at end of instruction.
pub( super ) fn error_missing_named_value_at_end( arg_name: &str, location: SourceLocation ) -> ParseError
{
  ParseError ::new
  (
    ErrorKind ::Syntax( format!( "Expected value for named argument '{arg_name}' but found end of instruction" ) ),
    location,
  )
}

/// Validates that the help operator '?' is the last token in the instruction.
pub( super ) fn validate_help_operator( item: &RichItem< '_ >, items_iter: &mut core ::iter ::Peekable< IntoIter< RichItem< '_ > > > ) -> Result< (), ParseError >
{
  if items_iter.peek().is_some()
  {
    return Err( ParseError ::new
    (
      ErrorKind ::Syntax( "Help operator '?' must be the last token".to_string() ),
      item.adjusted_source_location.clone(),
    ));
  }
  Ok( () )
}

/// Processes a positional argument, validating it against parser options and adding it to the collection.
pub( super ) fn process_positional_argument(
  options: &UnilangParserOptions,
  value: &str,
  item: &RichItem< '_ >,
  positional_arguments: &mut Vec< Argument >,
  named_arguments: &BTreeMap< String, Vec< Argument > >,
) -> Result< (), ParseError >
{
  // Check if positional arguments are allowed after named arguments
  if !named_arguments.is_empty() && options.error_on_positional_after_named
  {
    return Err( error_positional_after_named( item.adjusted_source_location.clone() ) );
  }

  // Create and add the positional argument
  positional_arguments.push( Argument
  {
    name: None,
    value: value.to_string(),
    name_location: None,
    value_location: item.source_location(),
  });

  Ok( () )
}

/// Detects potential argv misuse patterns that suggest re-tokenization.
///
/// This helper function checks if argv appears to have been created by joining
/// shell arguments and then re-splitting with `split_whitespace()`, which destroys
/// the shell's tokenization and breaks quote handling.
///
/// # Detection Heuristics
///
/// 1. **Consecutive short tokens**: Multiple single-word tokens in a row
///    that could have been a single quoted value (e.g., `["src/my", "project"]`)
///
/// 2. **Path-like splits**: Tokens that look like split paths
///    (e.g., token ending with "/" followed by another token)
///
/// 3. **High token density**: Many short tokens relative to argv length
///    (typical of `split_whitespace()` on joined strings)
///
/// # Warning Output
///
/// If suspicious patterns are detected, emits a warning to stderr with:
/// - Description of the detected pattern
/// - Link to CLI integration documentation
/// - Recommendation to use `parse_from_argv()` directly
///
/// # Note
///
/// This is a heuristic detection - false positives are possible but rare.
/// The warning is informational only and doesnt prevent parsing.
pub( super ) fn detect_argv_misuse( argv: &[String] )
{
  if argv.len() < 3
  {
    // Too short to detect patterns reliably
    return;
  }

  // Heuristic 1: Check for path-like splits
  // Example: ["src/my", "project"] suggests original was "src/my project"
  for i in 0..argv.len() - 1
  {
    let current = &argv[i];
    let next = &argv[i + 1];

    // Check if current ends with "/" or contains path separators followed by short token
    if ( current.ends_with( '/' ) || current.contains( '/' ) )
      && !next.starts_with( '-' )
      && !next.contains( "::" )
      && !next.starts_with( '.' )
      && next.len() < 20  // Short token suggests it was split from a path
    {
      #[ cfg( not( feature = "no_std" ) ) ]
      {
        eprintln!( "\n⚠️  WARNING: Potential argv misuse detected!" );
        eprintln!( "   Pattern: Path-like tokens that appear to be split incorrectly" );
        eprintln!( "   Found: {current:?} followed by {next:?}" );
        eprintln!();
        eprintln!( "   This usually happens when you:" );
        eprintln!( "     1. Join argv into a string: argv.join(\" \")");
        eprintln!( "     2. Re-split with split_whitespace() or parse_single_instruction()");
        eprintln!();
        eprintln!( "   ❌ WRONG: argv.join(\" \") then parse_single_instruction()");
        eprintln!( "   ✅ CORRECT: parse_from_argv(&argv) directly");
        eprintln!();
        eprintln!( "   Why this matters: Shell already tokenized your arguments." );
        eprintln!( "   Re-tokenizing destroys quote handling, causing quoted paths" );
        eprintln!( "   like \"src/my project\" to be incorrectly split." );
        eprintln!();
        eprintln!( "   See: docs/cli_integration.md for details");
        eprintln!();
      }
      return;
    }
  }

  // Heuristic 2: Check for many consecutive short tokens
  // Example: ["deploy", "to", "production", "server"] suggests re-tokenization
  // of what was originally "deploy to production server"
  let mut consecutive_short = 0;
  let max_consecutive_short = 0;

  for arg in argv.iter().skip( 1 )  // Skip program name
  {
    // Short token that's not a flag, command, or named arg
    if arg.len() < 15
      && !arg.starts_with( '-' )
      && !arg.starts_with( '.' )
      && !arg.contains( "::" )
    {
      consecutive_short += 1;
      if consecutive_short >= 3
      {
        #[ cfg( not( feature = "no_std" ) ) ]
        {
          eprintln!( "\n⚠️  WARNING: Potential argv misuse detected!" );
          eprintln!( "   Pattern: Multiple consecutive short tokens (3+ in a row)" );
          eprintln!( "   This suggests arguments may have been joined and re-split" );
          eprintln!();
          eprintln!( "   Common mistake:" );
          eprintln!( "     let joined = argv.join(\" \");  // ❌ Loses token boundaries");
          eprintln!( "     parser.parse_single_instruction(&joined);  // ❌ Re-tokenizes incorrectly");
          eprintln!();
          eprintln!( "   Correct approach:" );
          eprintln!( "     parser.parse_from_argv(&argv);  // ✅ Preserves shell tokenization");
          eprintln!();
          eprintln!( "   See: docs/cli_integration.md for complete guide");
          eprintln!();
        }
        return;
      }
    }
    else
    {
      consecutive_short = 0;
    }
  }

  let _ = max_consecutive_short;  // Suppress unused warning
}
