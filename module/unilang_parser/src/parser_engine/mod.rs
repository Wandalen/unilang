//! Parser for Unilang instructions.
//!
//! This module provides the core logic for parsing Unilang instructions from a string input.
//! It handles tokenization, command path parsing, argument parsing, and error reporting.
//!
//! ## Known Pitfalls
//!
//! ### Iterator Lookahead Pattern with `Peekable`
//!
//! Both `parse_command_path` and `parse_arguments` use `Peekable` iterators with outer
//! loops that call `peek()`. When implementing lookahead within such loops, calling `peek()`
//! again returns the SAME item, not the next one.
//!
//! **Wrong pattern (returns current item):**
//! ```rust,ignore
//! while let Some(item) = iter.peek() {
//!     if let Some(next) = iter.peek() { } // ❌ Returns 'item' again!
//! }
//! ```
//!
//! **Correct pattern (returns next item):**
//! ```rust,ignore
//! while let Some(item) = iter.peek() {
//!     let mut lookahead = iter.clone();
//!     lookahead.next(); // Skip current item
//!     if let Some(next) = lookahead.peek() { } // ✅ Returns truly next item
//! }
//! ```
//!
//! This pattern is used in:
//! - `parse_command_path` (lines 407-410) - Detects `name::value` patterns
//! - `parse_arguments` (lines 955-963) - Detects named argument operators
//!
//! ### Operator Variant Handling
//!
//! The tokenizer (via `strs_tools`) produces TWO variants of the named argument operator
//! based on whitespace in the input:
//! - `"::"` - No surrounding spaces (e.g., `cmd::value`)
//! - `" :: "` - With surrounding spaces (e.g., `cmd :: value`)
//!
//! Both variants are defined in the default config (see `config.rs` operators list).
//! Any code that checks for the operator MUST check both variants:
//!
//! ```rust,ignore
//! let is_named_arg_operator = match &token.kind {
//!     ZeroCopyTokenKind::Operator(op) => *op == "::" || *op == " :: ",
//!     _ => false,
//! };
//! ```
//!
//! This affects:
//! - Command path parser lookahead (lines 415-420)
//! - Argument parser operator detection (lines 958-960)
//!
//! ### Borrow Checker Patterns with Lookahead
//!
//! When implementing lookahead that needs data from the current item, clone the data
//! BEFORE performing lookahead to avoid multiple mutable borrows:
//!
//! ```rust,ignore
//! // Clone data before lookahead
//! let segment = s.clone();
//! let location = item.location.clone();
//!
//! // Now safe to do lookahead with peek()
//! let mut lookahead = iter.clone();
//! lookahead.next();
//! if let Some(next) = lookahead.peek() { ... }
//!
//! // Can use cloned data in error handling
//! return Err(ParseError::new(..., location));
//! ```
//!
//! ### API Consistency Requirement
//!
//! Both `parse_from_argv()` and `parse_repl_input()` must produce identical
//! results for equivalent inputs. Workarounds or special handling in one path but not
//! the other create inconsistencies and violate user expectations.
//!
//! Always verify both API paths with tests (see `test_api_path_consistency` in
//! `tests/diagnostic_real_bug.rs`).

mod validation_utilities;

use crate ::
{
  config ::UnilangParserOptions,
  error :: { ErrorKind, ParseError, SourceLocation },
  item_adapter :: { RichItem, ZeroCopyTokenKind },
};
use crate ::instruction :: { Argument, GenericInstruction };
use alloc ::collections ::BTreeMap;
use alloc ::vec :: { Vec, IntoIter };
use alloc ::string :: { String, ToString };
use alloc ::format;


/// The main parser struct.
#[ derive( Debug ) ]
pub struct Parser
{
  options: UnilangParserOptions,
}

impl Parser
{
  /// Creates a new `Parser` instance with the given options.
  #[ must_use ]
  pub fn new( options: UnilangParserOptions ) -> Self
  {
  Self { options }
 }

  /// Parses a single Unilang instruction from the input string.
  ///
  /// **Deprecated since next version.** Use [`parse_repl_input`] for REPL/string input.
  ///
  /// # Errors
  /// Returns a `ParseError` if the input string cannot be parsed into a valid instruction.
  ///
  /// [`parse_repl_input`]: Self::parse_repl_input
  #[ deprecated(
    since = "0.33.0",
    note = "Use parse_repl_input() for REPL/string input. \
            Use parse_from_argv() for CLI argv input."
  ) ]
  pub fn parse_single_instruction( &self, input: &str ) -> Result< crate ::instruction ::GenericInstruction, ParseError >
  {
    self.parse_repl_input( input )
  }

  /// Parses a single Unilang instruction from a raw input string.
  ///
  /// Use for REPL, configuration files, or any source where the string has NOT
  /// been tokenized by the shell. For shell argv, use [`parse_from_argv`].
  ///
  /// # Errors
  ///
  /// Returns `ParseError` if the input cannot be parsed.
  ///
  /// [`parse_from_argv`]: Self::parse_from_argv
  pub fn parse_repl_input( &self, input : &str )
    -> Result< crate ::instruction ::GenericInstruction, ParseError >
  {
  // Validate quote completeness before processing
  validation_utilities::validate_quote_completeness( input )?;

  // Use strs_tools as mandated by the architecture specification
  let mut all_delimiters = alloc::vec::Vec::new();
  all_delimiters.extend_from_slice( &[ " ", "\n", "\t", "\r", "#" ] );
  all_delimiters.extend( self.options.main_delimiters.iter().copied() );
  all_delimiters.extend( self.options.operators.iter().copied() );

  let splits_iter = strs_tools::string::split::split()
    .delimiters( all_delimiters.iter().map(core::convert::AsRef::as_ref).collect::<Vec<_>>().as_slice() )
    .quoting( true )
    .preserving_empty( false )
    .src( input )
    .perform();

  let splits: Vec< crate ::item_adapter ::Split< '_ > > = splits_iter
    .map( | s | crate ::item_adapter ::Split {
      string: s.string,
      bounds: ( s.start, s.end ),
      start: s.start,
      end: s.end,
      typ: match s.typ {
        strs_tools::string::split::SplitType::Delimited => crate ::item_adapter ::SplitType::Delimiter,
        strs_tools::string::split::SplitType::Delimiter => crate ::item_adapter ::SplitType::NonDelimiter,
      },
      was_quoted: s.was_quoted,
    })
    .collect();


  let rich_items: Vec< RichItem< '_ > > = splits
  .into_iter()
  .map( | s |
  {
   let ( kind, adjusted_source_location ) = crate ::item_adapter ::classify_split( &s )?;
   Ok( RichItem ::new( s, kind, adjusted_source_location ) )
 })
  .collect :: < Result< Vec< RichItem< '_ > >, ParseError > >()?;

  // Fix for Bug #006/#007: Merge tokens in value context (after ::)
  // This must happen BEFORE whitespace filtering so the merge logic can detect
  // whitespace delimiters as value terminators
  let rich_items = Self::merge_value_context_tokens( rich_items );

  let rich_items: Vec< RichItem< '_ > > = rich_items
  .into_iter()
  .filter( | item | !matches!( item.kind, ZeroCopyTokenKind ::Delimiter( " " | "\n" | "\t" | "\r" ) ) )
  .collect();

  // Fix for Task 026: Handle empty quoted strings that were filtered out by strs_tools
  let rich_items = validation_utilities::inject_empty_quoted_string_tokens( input, rich_items );

  self.parse_single_instruction_from_rich_items( rich_items )
  }

  /// Type-safe CLI entry point. Accepts pre-tokenized shell argv.
  ///
  /// Delegates to [`parse_from_argv`] after unwrapping the [`crate::argv_types::ShellArgv`]
  /// marker. Use this when you want compile-time enforcement that shell-tokenized
  /// input is not accidentally passed to the REPL parser.
  ///
  /// # Errors
  ///
  /// Returns `ParseError` if the argv cannot be parsed.
  ///
  /// [`parse_from_argv`]: Self::parse_from_argv
  pub fn parse_cli( &self, argv : &crate ::argv_types ::ShellArgv )
    -> Result< crate ::instruction ::GenericInstruction, ParseError >
  {
    self.parse_from_argv( argv.as_slice() )
  }

  /// Type-safe REPL entry point. Accepts a raw command string.
  ///
  /// Delegates to [`parse_repl_input`] after unwrapping the [`crate::argv_types::ReplInput`]
  /// marker. Use this when you want compile-time enforcement that a raw string
  /// is not accidentally passed to the argv parser.
  ///
  /// # Errors
  ///
  /// Returns `ParseError` if the input cannot be parsed.
  ///
  /// [`parse_repl_input`]: Self::parse_repl_input
  pub fn parse_repl( &self, input : &crate ::argv_types ::ReplInput )
    -> Result< crate ::instruction ::GenericInstruction, ParseError >
  {
    self.parse_repl_input( input.as_str() )
  }

  /// Merges tokens in value context (after :: operators) to protect special characters.
  ///
  /// This implements value-context-aware tokenization as specified in spec.md Section 2.4, Rule 5.
  /// After a `::` named argument operator, all subsequent tokens are merged into a single value
  /// until the next whitespace delimiter is encountered.
  ///
  /// # Examples
  ///
  /// ```text
  /// Input:  [query, ::, Bug, #, 003, status, ::, open?]
  /// Output: [query, ::, Bug #003, status, ::, open?]
  /// ```
  ///
  /// # Algorithm
  ///
  /// 1. Scan tokens for :: operator (both "::" and " :: " variants)
  /// 2. Enter value context after :: operator
  /// 3. Accumulate all following tokens until whitespace delimiter
  /// 4. Merge accumulated tokens into single Identifier with combined text
  /// 5. Return to normal context after whitespace
  ///
  /// # Design Notes
  ///
  /// - Fixes Bug #006 (# character in values) and Bug #007 (? character in values)
  /// - Aligns with spec.md: "Special characters after :: are value content, not delimiters"
  /// - Whitespace always terminates value collection (per spec.md Rule 0)
  /// - Preserves source location information for error reporting
  fn merge_value_context_tokens(
    rich_items: Vec< RichItem< '_ > >,
  ) -> Vec< RichItem< '_ > >
  {
    let mut result = Vec::new();
    let mut iter = rich_items.into_iter().peekable();

    while let Some( item ) = iter.next()
    {
      // Check if this is a :: operator (both variants)
      let is_named_arg_operator = matches!(
        &item.kind,
        ZeroCopyTokenKind::Operator( "::" | " :: " )
      );

      if is_named_arg_operator
      {
        // Push the :: operator itself
        result.push( item );

        // Enter value context - collect tokens until whitespace
        let mut value_parts = Vec::new();
        let mut value_start: Option< usize > = None;
        let mut value_end: usize = 0;
        let mut any_part_quoted = false;

        while let Some( next_item ) = iter.peek()
        {
          // Check if this is a whitespace delimiter - if so, stop collecting
          let is_whitespace = matches!(
            &next_item.kind,
            ZeroCopyTokenKind::Delimiter( " " | "\t" | "\n" | "\r" )
          );

          if is_whitespace
          {
            break;
          }

          // Take the token and add to value parts
          let token = iter.next().unwrap();

          // Quoting anywhere in the merged value signals literal intent
          // (distinguishes `param::??` help from `param::"??"` literal)
          any_part_quoted |= token.inner.was_quoted;

          // Track source location bounds
          if let SourceLocation::StrSpan { start, end } = token.adjusted_source_location
          {
            if value_start.is_none()
            {
              value_start = Some( start );
            }
            value_end = end;
          }

          // Extract text from token
          let text = match &token.kind
          {
            ZeroCopyTokenKind::Identifier( s )
            | ZeroCopyTokenKind::Number( s )
            | ZeroCopyTokenKind::Unrecognized( s ) => s.as_ref().to_string(),
            ZeroCopyTokenKind::Operator( s )
            | ZeroCopyTokenKind::Delimiter( s ) => (*s).to_string(),
          };

          value_parts.push( text );
        }

        // If we collected any value parts, create a merged Identifier token
        if !value_parts.is_empty()
        {
          let merged_value = value_parts.join( "" );
          let source_location = SourceLocation::StrSpan
          {
            start: value_start.unwrap_or( 0 ),
            end: value_end,
          };

          // Create a synthetic Split for the merged value
          let split = crate::item_adapter::Split
          {
            string: alloc::borrow::Cow::Owned( merged_value.clone() ),
            bounds: ( value_start.unwrap_or( 0 ), value_end ),
            start: value_start.unwrap_or( 0 ),
            end: value_end,
            typ: crate::item_adapter::SplitType::NonDelimiter,
            was_quoted: any_part_quoted,
          };

          let merged_token = RichItem::new(
            split,
            ZeroCopyTokenKind::Identifier( alloc ::borrow ::Cow ::Owned( merged_value ) ),
            source_location,
          );

          result.push( merged_token );
        }
      }
      else
      {
        // Normal token - just pass through
        result.push( item );
      }
    }

    result
  }

  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  /// Parses multiple Unilang instructions from the input string, separated by `;;`.
  ///
  /// # Errors
  /// Returns a `ParseError` if any segment cannot be parsed into a valid instruction,
  /// or if there are empty instruction segments (e.g., `;;;;`) or trailing delimiters (`cmd;;`).
  ///
  /// # Panics
  /// Panics if `segments.iter().rev().find(|s| s.typ == SplitType ::Delimiter).unwrap()` fails,
  /// which indicates a logic error where a trailing delimiter was expected but not found.
  pub fn parse_multiple_instructions( &self, input: &str ) -> Result< Vec< crate ::instruction ::GenericInstruction >, ParseError >
  {
  // Use standard string split instead of simple_split to avoid interference with ::operator
  let parts: Vec< &str > = input.split(";;").collect();
  let mut instructions = Vec ::new();

  // Handle empty input
  if parts.is_empty() || (parts.len() == 1 && parts[0].trim().is_empty())
  {
   return Ok( Vec ::new() );
 }

  // Check for invalid patterns
  if input.starts_with(";;")
  {
   return Err( ParseError ::new
   (
  ErrorKind ::EmptyInstructionSegment,
  SourceLocation ::StrSpan { start: 0, end: 2 },
 ));
 }
  

  // Check for consecutive delimiters
  if input.contains(";;;;")
  {
   let pos = input.find(";;;;").unwrap();
   return Err( ParseError ::new
   (
  ErrorKind ::EmptyInstructionSegment,
  SourceLocation ::StrSpan { start: pos, end: pos + 4 },
 ));
 }

  // Parse each part as an instruction
  for (i, part) in parts.iter().enumerate()
  {
   let trimmed = part.trim();
   if trimmed.is_empty()
   {
  // Empty part - need to determine if this is trailing delimiter or empty segment
  if i == parts.len() - 1 && input.contains(";;")
  {
   // This is the last part and it's empty, which means we have a trailing delimiter
   let semicolon_pos = input.rfind(";;").unwrap();
   return Err( ParseError ::new
   (
  ErrorKind ::TrailingDelimiter,
  SourceLocation ::StrSpan 
  { 
   start: semicolon_pos, 
   end: semicolon_pos + 2
 },
 ));
 }
  // Empty part between delimiters  
  let part_start = input.find(part).unwrap_or(0);
  return Err( ParseError ::new
  (
   ErrorKind ::EmptyInstructionSegment,
   SourceLocation ::StrSpan 
   { 
  start: part_start, 
  end: part_start + part.len().max(1)
 },
 ));
 }
   let instruction = self.parse_repl_input( trimmed )?;
   instructions.push( instruction );
 }

  Ok( instructions )
 }

  /// Parses a single Unilang instruction from a list of rich items.
  fn parse_single_instruction_from_rich_items
  (
  &self,
  rich_items: Vec< RichItem< '_ > >,
 )
  -> Result< crate ::instruction ::GenericInstruction, ParseError >
  {
  // Handle empty input (after filtering whitespace)

  if rich_items.is_empty()
  {
   return Ok( GenericInstruction
   {
  command_path_slices: Vec ::new(),
  positional_arguments: Vec ::new(),
  named_arguments: BTreeMap ::new(),
  overall_location: SourceLocation ::None, // No specific location for empty input
 });
 }

  let instruction_start_location = rich_items.first().map_or( 0, | item | item.inner.start );
  let instruction_end_location = rich_items.last().map_or( instruction_start_location, | item | item.inner.end );

  let mut items_iter = rich_items.into_iter().peekable();

  // Handle optional leading dot as per spec.md Rule 3.1
  if let Some( first_item ) = items_iter.peek()
  {
   if let ZeroCopyTokenKind ::Delimiter( "." ) = &first_item.kind
   {
  if first_item.inner.start == 0
  {
   // Ensure it's truly a leading dot at the beginning of the input
   items_iter.next(); // Consume the leading dot
 }
 }
 }

  let command_path_slices = Self ::parse_command_path( &mut items_iter, instruction_end_location )?;

  let ( positional_arguments, named_arguments ) = self.parse_arguments( &mut items_iter )?;

  Ok( GenericInstruction
  {
   command_path_slices,
   positional_arguments,
   named_arguments,
   overall_location: SourceLocation ::StrSpan
   {
  start: instruction_start_location,
  end: instruction_end_location,
 },
 })
 }

  /// Parses the command path from a peekable iterator of rich items.
  fn parse_command_path
  (
  items_iter: &mut core ::iter ::Peekable< IntoIter< RichItem< '_ > > >,
  instruction_end_location: usize,
 )
  -> Result< Vec< String >, ParseError >
  {
  let mut command_path_slices = Vec ::new();
  let mut last_token_was_dot = false;

  while let Some( item ) = items_iter.peek()
  {
   match &item.kind
   {
  ZeroCopyTokenKind ::Identifier( ref s )
   if command_path_slices.is_empty() || last_token_was_dot =>
  {
  // Fix(issue-cmd-path): Lookahead to detect named argument pattern before consuming
  // Root cause: Parser was consuming identifiers without checking if they're part of
  //             the named argument pattern (name::value), violating spec.md:193 which
  //             mandates "::" ends command path and begins argument parsing. This
  //             caused "orphaned operator" errors when parsing named-only arguments
  //             like "cmd::value" because "cmd" was incorrectly added to command_path,
  //             leaving "::" as the first token for the argument parser.
  // Pitfall: Must check for BOTH operator variants: "::" and " :: ". The tokenizer
  //          produces different tokens based on whitespace in input (config line 37).
  //          Do NOT attempt to peek 2 tokens ahead for two separate ":" tokens - this
  //          breaks iterator state. Always rely on the tokenizer's single-token output.
  //          Pattern copied from argument parser (lines 955-963) which handles the same
  //          lookahead correctly.

  // Clone data before lookahead (avoids borrow conflicts with peek)
  let segment = s.as_ref().to_string();
  let item_location = item.adjusted_source_location.clone();

  // Peek ahead to check if this identifier is followed by named argument operator
  // Clone iterator to look at next item without consuming current
  let mut lookahead_iter = items_iter.clone();
  lookahead_iter.next(); // Skip current item (the identifier we're examining)

  if let Some( next_item ) = lookahead_iter.peek()
  {
   // Check for named argument operator pattern (per spec.md:193)
   let is_named_arg_operator = match &next_item.kind
   {
    // Match both operator variants from config
    ZeroCopyTokenKind ::Operator( op ) => *op == "::" || *op == " :: ",
    _ => false,
   };

   if is_named_arg_operator
   {
    // This identifier is the NAME in a "name::value" pattern, not a command segment
    // Break without consuming - let argument parser handle the complete pattern
    break;
   }
  }

  // Not followed by ::, so it's a valid command path segment
  // Validate identifier doesn't contain hyphen (per spec.md:187)
  if segment.contains( '-' )
  {
   return Err( ParseError ::new
   (
  ErrorKind ::Syntax( format!( "Invalid character '-' in command path segment '{segment}'" ) ),
  item_location,
 ));
 }

  // Add to command path and consume token
  command_path_slices.push( segment );
  last_token_was_dot = false;
  items_iter.next(); // Safe to consume now
 }
  ZeroCopyTokenKind ::Identifier( _ ) =>
  {
   break; // End of command path
 }
  ZeroCopyTokenKind ::Delimiter( "." ) =>
  {
   if last_token_was_dot
   // Consecutive dots, e.g., "cmd..sub"
   {
  return Err( ParseError ::new
  (
   ErrorKind ::Syntax( "Consecutive dots in command path".to_string() ),
   item.adjusted_source_location.clone(),
 ));
 }
   last_token_was_dot = true;
   items_iter.next(); // Consume item
 }
  ZeroCopyTokenKind ::Unrecognized( ref s ) | ZeroCopyTokenKind ::Number( ref s ) =>
  {
   if last_token_was_dot
   {
  return Err( ParseError ::new
  (
   ErrorKind ::Syntax( format!( "Invalid identifier '{s}' in command path" ) ),
   item.adjusted_source_location.clone(),
 ));
 }
   break; // End of command path
 }
  _ =>
  {
   break; // End of command path
 }
 }
 }

  if last_token_was_dot
  {
   // If the last token was a dot, and we are at the end of the command path,
   // it's a trailing dot error. The location should be the end of the instruction.
   return Err( ParseError ::new
   (
  ErrorKind ::Syntax( "Command path cannot end with a '.'".to_string() ),
  SourceLocation ::StrSpan
  {
   start: instruction_end_location - 1,
   end: instruction_end_location,
 },
 ));
 }

  Ok( command_path_slices )
 }

  /// Processes a named argument with complex value parsing including multi-word values and paths.
  #[ allow( clippy ::too_many_lines ) ]
  fn process_named_argument(
    &self,
    arg_name: &str,
    item: &RichItem< '_ >,
    items_iter: &mut core ::iter ::Peekable< IntoIter< RichItem< '_ > > >,
    named_arguments: &mut BTreeMap< String, Vec< Argument > >,
  ) -> Result< (), ParseError >
  {
    if let Some( value_item ) = items_iter.next()
    {
      match value_item.kind
      {
        ZeroCopyTokenKind ::Identifier( ref val )
        | ZeroCopyTokenKind ::Unrecognized( ref val )
        | ZeroCopyTokenKind ::Number( ref val ) =>
        {
          let mut current_value = val.as_ref().to_string();
          let mut current_value_end_location = match value_item.source_location()
          {
            SourceLocation ::StrSpan { end, .. } => end,
            SourceLocation ::None => 0, // Default or handle error appropriately
          };

          // First, consume any additional tokens for multi-word values
          // Continue until we hit another named argument or the end
          loop
          {
            // Check what the next token is without borrowing
            let should_continue = match items_iter.peek()
            {
              Some( next_token ) =>
              {
                match &next_token.kind
                {
                  ZeroCopyTokenKind ::Identifier( _ ) =>
                  {
                    // FIXED: More reliable lookahead to detect named arguments
                    // Convert iterator to vec for reliable indexing
                    let remaining_items: Vec<_> = items_iter.clone().collect();
                    if remaining_items.len() >= 2
                    {
                      // Check if next two items form a named argument pattern
                      if let ZeroCopyTokenKind ::Operator( op ) = &remaining_items[1].kind
                      {
                        if *op == " :: " || *op == "::"
                        {
                          // This is definitely another named argument, stop consuming
                          false
                        }
                        else
                        {
                          // Different operator, continue consuming
                          true
                        }
                      }
                      else
                      {
                        // Not an operator after identifier, this is likely a positional argument, stop consuming
                        false
                      }
                    }
                    else
                    {
                      // Less than 2 items remaining, stop consuming to avoid taking positional args
                      false
                    }
                  }
                  ZeroCopyTokenKind ::Number( _ ) => true, // Numbers can be part of multi-word values
                  _ => false, // Other token types end the value
                }
              }
              None => false, // No more tokens
            };

            if !should_continue
            {
              break;
            }

            // Now safely consume the token
            if let Some( consumed_token ) = items_iter.next()
            {
              current_value.push( ' ' );
              current_value.push_str( &consumed_token.inner.string );
              current_value_end_location = match consumed_token.source_location()
              {
                SourceLocation ::StrSpan { end, .. } => end,
                SourceLocation ::None => current_value_end_location,
              };
            }
            else
            {
              break;
            }
          }

          // Loop to consume subsequent path segments
          while let Some( peeked_dot ) = items_iter.peek()
          {
            if let ZeroCopyTokenKind ::Delimiter( "." ) = &peeked_dot.kind
            {
              let _dot_item = items_iter.next().unwrap(); // Consume the dot
              let Some( peeked_segment ) = items_iter.peek() else
              {
                break;
              };
              if let ZeroCopyTokenKind ::Identifier( ref s ) = &peeked_segment.kind
              {
                current_value.push( '.' );
                current_value.push_str( s.as_ref() );
                current_value_end_location = match peeked_segment.source_location()
                {
                  SourceLocation ::StrSpan { end, .. } => end,
                  SourceLocation ::None => current_value_end_location, // Keep previous if None
                };
                items_iter.next(); // Consume the segment
              }
              else if let ZeroCopyTokenKind ::Unrecognized( ref s ) = &peeked_segment.kind
              {
                current_value.push( '.' );
                current_value.push_str( s.as_ref() );
                current_value_end_location = match peeked_segment.source_location()
                {
                  SourceLocation ::StrSpan { end, .. } => end,
                  SourceLocation ::None => current_value_end_location, // Keep previous if None
                };
                items_iter.next(); // Consume the segment
              }
              else if let ZeroCopyTokenKind ::Number( ref s ) = &peeked_segment.kind
              {
                current_value.push( '.' );
                current_value.push_str( s.as_ref() );
                current_value_end_location = match peeked_segment.source_location()
                {
                  SourceLocation ::StrSpan { end, .. } => end,
                  SourceLocation ::None => current_value_end_location, // Keep previous if None
                };
                items_iter.next(); // Consume the segment
              }
              else
              {
                // Not a valid path segment after dot, break
                break;
              }
            }
            else
            {
              break; // Next item is not a dot, end of path segments
            }
          }

          // Support multiple values for the same argument name
          let argument = Argument
          {
            name: Some( arg_name.to_string() ),
            value: current_value,
            name_location: Some( item.source_location() ),
            value_location: SourceLocation ::StrSpan
            {
              start: match value_item.source_location()
              {
                SourceLocation ::StrSpan { start, .. } => start,
                SourceLocation ::None => 0,
              },
              end: current_value_end_location,
            },
            was_quoted: value_item.inner.was_quoted,
          };

          // Check for duplicate named arguments if the option is set
          if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key( arg_name )
          {
            return Err( validation_utilities::error_duplicate_named_argument( arg_name, item.adjusted_source_location.clone() ) );
          }

          // Insert or append to existing vector
          named_arguments.entry( arg_name.to_string() )
            .or_default()
            .push( argument );
        }
        ZeroCopyTokenKind ::Delimiter( "." ) =>
        {
          // Handle file paths that start with "./" or "../"
          let mut current_value = ".".to_string();
          let mut current_value_end_location = match value_item.source_location()
          {
            SourceLocation ::StrSpan { end, .. } => end,
            SourceLocation ::None => 0,
          };

          // Continue building the path starting with "."
          // Look for the next token after "."
          if let Some( next_item ) = items_iter.peek()
          {
            match &next_item.kind
            {
              ZeroCopyTokenKind ::Unrecognized( ref s ) =>
              {
                // This handles cases like "./examples" where "/examples" is unrecognized
                current_value.push_str( s.as_ref() );
                current_value_end_location =  match next_item.source_location()
                {
                  SourceLocation ::StrSpan { end, .. } => end,
                  SourceLocation ::None => current_value_end_location,
                };
                items_iter.next(); // Consume the unrecognized token
              }
              ZeroCopyTokenKind ::Delimiter( "." ) =>
              {
                // This handles "../" patterns
                current_value.push( '.' );
                current_value_end_location =  match next_item.source_location()
                {
                  SourceLocation ::StrSpan { end, .. } => end,
                  SourceLocation ::None => current_value_end_location,
                };
                items_iter.next(); // Consume the second dot

                // Look for the next token after ".."
                if let Some( third_item ) = items_iter.peek()
                {
                  if let ZeroCopyTokenKind ::Unrecognized( ref s ) = &third_item.kind
                  {
                    current_value.push_str( s.as_ref() );
                    current_value_end_location =  match third_item.source_location()
                    {
                      SourceLocation ::StrSpan { end, .. } => end,
                      SourceLocation ::None => current_value_end_location,
                    };
                    items_iter.next(); // Consume the unrecognized token
                  }
                }
              }
              _ =>
              {
                // Other cases - not a file path, just leave as is
              }
            }

            // Continue with the normal path-building loop for any additional dots
            while let Some( peeked_dot ) = items_iter.peek()
            {
              if let ZeroCopyTokenKind ::Delimiter( "." ) = &peeked_dot.kind
              {
                let _dot_item = items_iter.next().unwrap(); // Consume the dot
                let Some( peeked_segment ) = items_iter.peek() else
                {
                  break;
                };
                if let ZeroCopyTokenKind ::Identifier( ref s ) = &peeked_segment.kind
                {
                  current_value.push( '.' );
                  current_value.push_str( s.as_ref() );
                  current_value_end_location = match peeked_segment.source_location()
                  {
                    SourceLocation ::StrSpan { end, .. } => end,
                    SourceLocation ::None => current_value_end_location,
                  };
                  items_iter.next(); // Consume the segment
                }
                else if let ZeroCopyTokenKind ::Unrecognized( ref s ) = &peeked_segment.kind
                {
                  current_value.push( '.' );
                  current_value.push_str( s.as_ref() );
                  current_value_end_location = match peeked_segment.source_location()
                  {
                    SourceLocation ::StrSpan { end, .. } => end,
                    SourceLocation ::None => current_value_end_location,
                  };
                  items_iter.next(); // Consume the segment
                }
                else if let ZeroCopyTokenKind ::Number( ref s ) = &peeked_segment.kind
                {
                  current_value.push( '.' );
                  current_value.push_str( s.as_ref() );
                  current_value_end_location = match peeked_segment.source_location()
                  {
                    SourceLocation ::StrSpan { end, .. } => end,
                    SourceLocation ::None => current_value_end_location,
                  };
                  items_iter.next(); // Consume the segment
                }
                else
                {
                  break;
                }
              }
              else
              {
                break;
              }
            }
          }

          // Support multiple values for the same argument name
          let argument = Argument
          {
            name: Some( arg_name.to_string() ),
            value: current_value,
            name_location: Some( item.source_location() ),
            value_location: SourceLocation ::StrSpan
            {
              start: match value_item.source_location()
              {
                SourceLocation ::StrSpan { start, .. } => start,
                SourceLocation ::None => 0,
              },
              end: current_value_end_location,
            },
            was_quoted: false, // path values are assembled from unquoted dot/segment tokens
          };

          // Check for duplicate named arguments if the option is set
          if self.options.error_on_duplicate_named_arguments && named_arguments.contains_key( arg_name )
          {
            return Err( validation_utilities::error_duplicate_named_argument( arg_name, item.adjusted_source_location.clone() ) );
          }

          // Insert or append to existing vector
          named_arguments.entry( arg_name.to_string() )
            .or_default()
            .push( argument );
        }
        _ =>
        {
          return Err( validation_utilities::error_missing_named_value( arg_name, value_item.source_location() ) )
        }
      }
    }
    else
    {
      return Err( validation_utilities::error_missing_named_value_at_end( arg_name, item.adjusted_source_location.clone() ) );
    }

    Ok( () )
  }

  /// Parses arguments from a peekable iterator of rich items.
  #[ allow( clippy ::type_complexity ) ]
  #[ allow( clippy ::too_many_lines ) ]
  fn parse_arguments
  (
  &self,
  items_iter: &mut core ::iter ::Peekable< IntoIter< RichItem< '_ > > >,
 )
  -> Result< ( Vec< Argument >, BTreeMap< String, Vec< Argument > > ), ParseError >
  {
  let mut positional_arguments = Vec ::new();
  let mut named_arguments = BTreeMap ::new();

  while let Some( item ) = items_iter.next()
  {
   match item.kind
   {
  ZeroCopyTokenKind ::Unrecognized( ref s ) =>
  {
   return Err( validation_utilities::error_unexpected_token( s.as_ref(), item.adjusted_source_location.clone() ) );
 }

  ZeroCopyTokenKind ::Identifier( ref s ) =>
  {
   // First, check if we have consecutive ":" delimiters by looking ahead
   let has_consecutive_colons = {
    let mut lookahead_iter = items_iter.clone();
    if let Some( first_item ) = lookahead_iter.next()
    {
     if matches!(first_item.kind, ZeroCopyTokenKind::Delimiter(":"))
     {
      if let Some( second_item ) = lookahead_iter.peek()
      {
       matches!(second_item.kind, ZeroCopyTokenKind::Delimiter(":"))
      }
      else
      {
       false
      }
     }
     else
     {
      false
     }
    }
    else
    {
     false
    }
   };

   if let Some( next_item ) = items_iter.peek()
   {
  // Check if this looks like a named argument pattern
  let is_named_argument = match &next_item.kind
  {
   ZeroCopyTokenKind ::Operator( op ) => *op == " :: " || *op == "::",
   ZeroCopyTokenKind ::Delimiter( ":" ) => has_consecutive_colons,
   _ => false,
  };

  if is_named_argument
  {
   // Named argument - consume the "::" operator (either single token or two ":" delimiters)
   match &next_item.kind
   {
    ZeroCopyTokenKind ::Operator( _ ) => {
     items_iter.next(); // Consume single "::" operator
    },
    ZeroCopyTokenKind ::Delimiter( ":" ) => {
     items_iter.next(); // Consume first ":"
     items_iter.next(); // Consume second ":"
    },
    _ => unreachable!(),
   }
   let arg_name = s.as_ref();

   self.process_named_argument( arg_name, &item, items_iter, &mut named_arguments )?;
}
  else
  {
   // Positional argument
   validation_utilities::process_positional_argument( &self.options, s.as_ref(), &item, &mut positional_arguments, &named_arguments )?;
 }
 }
   else
   {
  // Last token, must be positional
  validation_utilities::process_positional_argument( &self.options, s.as_ref(), &item, &mut positional_arguments, &named_arguments )?;
 }
 }
  ZeroCopyTokenKind ::Number( ref s ) =>
  {
   // Positional argument
   validation_utilities::process_positional_argument( &self.options, s.as_ref(), &item, &mut positional_arguments, &named_arguments )?;
 }
  ZeroCopyTokenKind::Operator("::" | " :: ") =>
  {
   return Err( validation_utilities::error_orphaned_operator( item.adjusted_source_location.clone() ) );
 }
  ZeroCopyTokenKind::Delimiter(":") =>
  {
   // Check if the next token is also ":" to form "::"
   if let Some( next_item ) = items_iter.peek()
   {
    if let ZeroCopyTokenKind::Delimiter(":") = &next_item.kind
    {
     // This is an orphaned "::" operator (no preceding identifier)
     return Err( validation_utilities::error_orphaned_operator( item.adjusted_source_location.clone() ) );
    }
   }
   // Single ":" without following ":" is unexpected
   return Err( validation_utilities::error_unexpected_token( ":", item.adjusted_source_location.clone() ) );
 }
  _ =>
  {
   return Err( validation_utilities::error_unexpected_token( &item.inner.string, item.adjusted_source_location.clone() ) );
 }
 }
 }

  Ok( ( positional_arguments, named_arguments ) )
 }

  /// Detects potential argv misuse patterns that suggest re-tokenization.
  ///
  /// Parses a single Unilang instruction from an argv array (OS command-line arguments).
  ///
  /// This method provides proper CLI integration by preserving the original argv structure
  /// from the operating system, avoiding information loss from string joining and re-tokenization.
  ///
  /// # Algorithm
  ///
  /// The argv parser intelligently combines consecutive argv elements that belong together:
  /// 1. The first element is treated as the command name
  /// 2. Elements containing `::` start named arguments (`key::value`)
  /// 3. Following elements without `::` or `.` prefix are combined into the parameter value
  /// 4. Combining stops when another `::` or `.` prefix is encountered
  ///
  /// # Examples
  ///
  /// ```ignore
  /// use unilang_parser::{Parser, UnilangParserOptions};
  ///
  /// let parser = Parser::new(UnilangParserOptions::default());
  ///
  /// // Shell: ./app command::ls -la
  /// // OS provides: ["command::ls", "-la"]
  /// let argv = vec!["command::ls".to_string(), "-la".to_string()];
  /// let instruction = parser.parse_from_argv(&argv).unwrap();
  ///
  /// // Result: command = "ls -la" (correctly combined)
  /// assert_eq!(instruction.named_arguments.get("command").unwrap()[0].value, "ls -la");
  /// ```
  ///
  /// # Errors
  ///
  /// Returns a `ParseError` if:
  /// - The argv array is malformed (e.g., orphaned `::` operators)
  /// - The command path structure is invalid
  /// - Arguments don't follow the expected syntax
  ///
  /// # See Also
  ///
  /// - [`parse_repl_input`] - For parsing REPL or string input
  /// - Task 080: Argv-Based API Request - Full specification and rationale
  pub fn parse_from_argv( &self, argv: &[String] ) -> Result< GenericInstruction, ParseError >
  {
    // Handle empty argv
    if argv.is_empty()
    {
      return Ok( GenericInstruction
      {
        command_path_slices: Vec ::new(),
        positional_arguments: Vec ::new(),
        named_arguments: BTreeMap ::new(),
        overall_location: SourceLocation ::None,
      });
    }

    // Detect potential argv misuse (emits warning if suspicious patterns found)
    validation_utilities::detect_argv_misuse( &self.options, argv );

    // Process argv into a reconstructed command string with proper token boundaries
    // We need to quote values that contain spaces to preserve argv boundaries
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < argv.len()
    {
      let arg = &argv[i];

      // Fix(manual-test-2026-02-12): Validate parameter syntax before attempting to parse
      // Root cause: split_once("::") returns None for single-colon strings, causing
      // malformed parameters to be silently treated as positional arguments instead
      // of producing clear syntax errors
      // Pitfall: Parameter parsers must validate syntax BEFORE attempting to match
      // parameters. Treating malformed syntax as positional args creates confusing
      // error messages that mislead users about the actual problem
      if arg.contains(':') && !arg.contains("::")
      {
        return Err( ParseError
        {
          kind: ErrorKind::Syntax( format!(
            "Invalid parameter syntax: '{}'. Parameters must use '::' separator (e.g., 'param::value')",
            arg
          ) ),
          location: Some( SourceLocation::None ),
        });
      }

      // Check if this is a named argument (contains ::)
      if let Some( ( key, initial_value ) ) = arg.split_once( "::" )
      {
        // Start building the value
        let mut value = initial_value.to_string();

        // Combine subsequent argv elements that are part of this value
        // Stop when we hit another :: or a dot-prefixed command
        while i + 1 < argv.len()
        {
          let next_arg = &argv[i + 1];

          // Fix(manual-test-2026-02-12): Validate parameter syntax for next arg before consuming it
          // This catches single-colon args that would otherwise be silently consumed as values
          if next_arg.contains(':') && !next_arg.contains("::")
          {
            return Err( ParseError
            {
              kind: ErrorKind::Syntax( format!(
                "Invalid parameter syntax: '{}'. Parameters must use '::' separator (e.g., 'param::value')",
                next_arg
              ) ),
              location: Some( SourceLocation::None ),
            });
          }

          // Stop if next arg contains :: (it's another named argument)
          if next_arg.contains( "::" )
          {
            break;
          }

          // Stop if next arg starts with . (it's a command or path separator)
          if next_arg.starts_with( '.' )
          {
            break;
          }

          // Fix(manual-test-2026-08-20): A standalone `??` argv element is the positional
          // help token — never absorb it into a preceding named value.
          // Root cause: the multiword absorption loop only broke on `::`, dot-prefixed, and
          //   path-bearing tokens, so `app .cmd a::1 ??` glued into a::"1 ??" and the help
          //   request surfaced as a coercion error instead of the command help page.
          // Pitfall: the equivalent in-process string form `.cmd a::1 ??` already yields a
          //   separate positional `??` token; argv parsing must match, or CLI binaries and
          //   in-process pipelines disagree on help behavior. A quoted literal (`'"??"'`)
          //   arrives with its own inner quotes, never equals bare `??`, and still absorbs.
          if next_arg == "??"
          {
            break;
          }

          // Fix(issue-087): Stop absorbing when the accumulated value already contains '/'.
          // Root cause: Path/URL values are complete in their first token; the absorption loop
          //   had no way to distinguish "continuation of a multi-word value" from "a separate
          //   positional that happens to look like a path", silently corrupting both.
          // Pitfall: Check the ACCUMULATED value, not the next_arg. Multi-word plain-text values
          //   (e.g., "message::hello" + "world") have no '/' and must continue absorbing normally.
          //   Removing this check breaks intentional multi-word absorption for non-path params.
          if value.contains( '/' )
          {
            break;
          }

          // Combine this argument into the value
          if !value.is_empty()
          {
            value.push( ' ' );
          }
          value.push_str( next_arg );
          i += 1;
        }

        // NOTE: Intentionally NOT stripping surrounding quotes from `value` here.
        //
        // Task 083 explored adding quote stripping to handle over-quoting like:
        //   'param::"value"' → strip quotes → param::value
        //
        // However, this has critical problems (22 identified):
        //
        // FUNDAMENTAL ISSUE: Cannot distinguish user intent from argv alone:
        //   Case A: 'param::"value"'   → over-quoting (wants: value)
        //   Case B: param::\"value\"   → escaped quotes (wants: "value")
        //   Both produce IDENTICAL argv: param::"value"
        //
        // CRITICAL RISK: Silent data corruption
        //   If we strip quotes, Case B breaks with NO error:
        //   - Book titles: 'title::"Chapter 1"' → loses quotes → DB corruption
        //   - CSV fields: 'field::"Smith, John"' → splits into two fields!
        //   - SQL literals: 'value::"admin"' → identifier instead of literal
        //   - Code/JSON: 'template::'"name": "value"' → invalid JSON
        //   Silent corruption propagates and persists - worse than crashes!
        //
        // RECOMMENDATION: Use warning-only approach (Alternative 3):
        //   - Detect quoted boundaries and warn user
        //   - NO modification to values (preserves existing behavior)
        //   - Gather data on frequency before making breaking changes
        //
        // See:
        //   - tests/argv_multiword_bug_test.rs::test_argv_multiword_parameter_with_shell_quotes_preserved
        //     (ignored test with extensive documentation)

        // Add the complete named argument as a single token: key::"value"
        // Quote the value if it contains whitespace or is empty. If the value contains quotes,
        // escape them before wrapping to avoid nested quote errors.
        //
        // Fix(issue-084): Prevents double-quoting bug where values like `cld -p "/start"`
        // would get wrapped as `"cld -p "/start""`, causing tokenizer errors on nested quotes.
        //
        // Root cause: Unconditional quoting when whitespace detected, without checking for
        // existing quotes. When value contains both whitespace AND quotes (e.g., shell commands
        // with quoted arguments), adding outer quotes creates: `cmd::"cld -p "/start""` where
        // the inner `"` terminates the outer quote prematurely, leaving `/start""` as unexpected
        // token.
        //
        // Solution: Escape inner quotes by doubling them before adding outer quotes. This
        // preserves the value integrity while preventing quote confusion.
        //
        // Pitfall: Don't assume edge cases are independent. Values can have BOTH whitespace AND
        // quotes simultaneously (common in shell commands, paths with spaces, etc.). Always test
        // combinations of characteristics, not just individual edge cases.
        if value.chars().any( char::is_whitespace ) || value.is_empty()
        {
          // Escape any existing quotes by replacing " with \"
          let escaped_value = value.replace( '"', "\\\"" );
          tokens.push( format!( "{key}::\"{escaped_value}\"" ) );
        }
        else
        {
          tokens.push( format!( "{key}::{value}" ) );
        }
      }
      else
      {
        // Not a named argument - just add as-is
        // Quote if it contains whitespace to preserve the token boundary, or if it contains
        // characters not valid in a unilang identifier (e.g., '/', '@', uppercase). The
        // string parser classifies such tokens as Unrecognized, which triggers a parse error.
        // Command-path tokens starting with '.' must NOT be quoted — they are handled by
        // the command-path parser and quoting them would make them positional instead.
        //
        // Fix(issue-084): Escape inner quotes before wrapping to avoid nested quote errors.
        //
        // Fix(issue-087): Also quote tokens that the string parser would reject as Unrecognized.
        // Root cause: Tokens with '/', '@', uppercase, etc. are Unrecognized in the string
        //   parser, which returns an error instead of treating them as positional arguments.
        // Pitfall: Do not quote tokens starting with '.'. They are command-path tokens and
        //   quoting them would route them to the positional argument path instead.
        let is_unilang_identifier = !arg.is_empty()
          && arg.chars().next().is_some_and( | c | c.is_ascii_lowercase() || c == '_' )
          && !arg.ends_with( '-' )
          && arg.chars().all( | c | c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' );
        let is_number = arg.parse :: < i64 >().is_ok();
        let is_operator = self.options.operators.contains( &arg.as_str() );
        // `?`/`??` are value-capable tokens the string parser accepts unquoted; re-quoting
        // them here would flip `was_quoted` to true and turn a CLI help request (`app .cmd ??`)
        // into the literal string — the shell-literal form (`'"??"'`) arrives with its own
        // inner quotes and stays on the quoted path naturally.
        let is_question_token = matches!( arg.as_str(), "?" | "??" );
        let needs_quoting = arg.chars().any( char::is_whitespace )
          || ( !arg.starts_with( '.' ) && !is_unilang_identifier && !is_number && !is_operator && !is_question_token );
        if needs_quoting
        {
          // Escape any existing quotes by replacing " with \"
          let escaped_arg = arg.replace( '"', "\\\"" );
          tokens.push( format!( "\"{escaped_arg}\"" ) );
        }
        else
        {
          tokens.push( arg.clone() );
        }
      }

      i += 1;
    }

    // Now convert tokens into a space-separated string and parse it
    // This reuses the existing string parser infrastructure
    let command_str = tokens.join( " " );
    self.parse_repl_input( &command_str )
  }

}
