//! Adapters for converting raw string splits into rich, classified tokens.

#![ allow( clippy ::std_instead_of_alloc ) ]
#![ allow( clippy ::std_instead_of_core ) ]

use crate ::error :: { ParseError, SourceLocation };
use alloc ::borrow ::Cow;
use core ::fmt;

/// Split representation compatible with `strs_tools` Split
#[ derive( Debug, Clone ) ]
pub struct Split< 'a >
{
  /// The string content of this split
  pub string: Cow< 'a, str >,
  /// The byte bounds in the original string  
  pub bounds: ( usize, usize ),
  /// Start position in the original string
  pub start: usize,
  /// End position in the original string
  pub end: usize,
  /// Type of this split segment
  pub typ: SplitType,
  /// Whether this segment was originally quoted
  pub was_quoted: bool,
}

/// Type of split segment
#[ derive( Debug, Clone, PartialEq ) ]
pub enum SplitType
{
  /// A delimiter segment
  Delimiter,
  /// A non-delimiter segment
  NonDelimiter,
}

/// Represents a token with its original split information and zero-copy classified kind.
#[ derive( Debug, Clone ) ]
pub struct ZeroCopyRichItem< 'a >
{
  /// The original string split.
  pub inner: Split< 'a >,
  /// The zero-copy classified kind of the token.
  pub kind: ZeroCopyTokenKind< 'a >,
  /// The source location adjusted for things like quotes.
  pub adjusted_source_location: SourceLocation,
}

impl< 'a > ZeroCopyRichItem< 'a >
{
  /// Creates a new `ZeroCopyRichItem`.
  #[ must_use ]
  pub fn new
  (
  inner: Split< 'a >,
  kind: ZeroCopyTokenKind< 'a >,
  adjusted_source_location: SourceLocation,
 )
  ->
  Self
  {
  Self
  {
   inner,
   kind,
   adjusted_source_location,
 }
 }

  /// Returns the source location of the item.
  #[ must_use ]
  pub fn source_location( &self ) -> SourceLocation
  {
  self.adjusted_source_location.clone()
 }

  /// Converts to an owned `RichItem`.
  #[ must_use ]
  pub fn to_owned( &self ) -> RichItem< 'a >
  {
  RichItem ::new( self.inner.clone(), self.kind.clone(), self.adjusted_source_location.clone() )
 }
}


/// Represents a token with its original split information and classified kind.
#[ derive( Debug, Clone ) ]
pub struct RichItem< 'a >
{
  /// The original string split.
  pub inner: Split< 'a >,
  /// The classified kind of the token.
  pub kind: ZeroCopyTokenKind< 'a >,
  /// The source location adjusted for things like quotes.
  pub adjusted_source_location: SourceLocation,
}

impl< 'a > RichItem< 'a >
{
  /// Creates a new `RichItem`.
  #[ must_use ]
  pub fn new
  (
  inner: Split< 'a >,
  kind: ZeroCopyTokenKind< 'a >,
  adjusted_source_location: SourceLocation,
 )
  ->
  Self
  {
  Self
  {
   inner,
   kind,
   adjusted_source_location,
 }
 }

  /// Returns the source location of the item.
  #[ must_use ]
  pub fn source_location( &self ) -> SourceLocation
  {
  self.adjusted_source_location.clone()
 }
}

/// Represents the classified kind of a unilang token with zero-copy string slices.
#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub enum ZeroCopyTokenKind< 'a >
{
  /// An identifier (e.g., a command name, argument name, or unquoted value).
  Identifier( alloc ::borrow ::Cow< 'a, str > ),
  /// A number literal.
  Number( alloc ::borrow ::Cow< 'a, str > ),

  /// An operator (e.g., `::`, ` :: `).
  Operator( &'static str ),
  /// A delimiter (e.g., space, dot, newline).
  Delimiter( &'static str ),
  /// An unrecognized token, indicating a parsing error.
  Unrecognized( alloc ::borrow ::Cow< 'a, str > ),
}

/// Represents the classified kind of a unilang token.
#[ derive( Debug, PartialEq, Eq, Clone ) ]
pub enum UnilangTokenKind< 'a >
{
  /// An identifier (e.g., a command name, argument name, or unquoted value).
  Identifier( Cow< 'a, str > ),
  /// A number literal.
  Number( Cow< 'a, str > ),

  /// An operator (e.g., `::`, ` :: `).
  Operator( &'static str ),
  /// A delimiter (e.g., space, dot, newline).
  Delimiter( &'static str ),
  /// An unrecognized token, indicating a parsing error.
  Unrecognized( Cow< 'a, str > ),
}

impl< 'a > ZeroCopyTokenKind< 'a >
{
  /// Converts a zero-copy token to an owned token.
  #[ must_use ]
  pub fn to_owned( &self ) -> UnilangTokenKind< 'a >
  {
  match self
  {
   ZeroCopyTokenKind ::Identifier( s ) => UnilangTokenKind ::Identifier( s.clone() ),
   ZeroCopyTokenKind ::Number( s ) => UnilangTokenKind ::Number( s.clone() ),
   ZeroCopyTokenKind ::Operator( s ) => UnilangTokenKind ::Operator( s ),
   ZeroCopyTokenKind ::Delimiter( s ) => UnilangTokenKind ::Delimiter( s ),
   ZeroCopyTokenKind ::Unrecognized( s ) => UnilangTokenKind ::Unrecognized( s.clone() ),
 }
 }
}

impl fmt ::Display for ZeroCopyTokenKind< '_ >
{
  fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
  {
  match self
  {
   ZeroCopyTokenKind ::Identifier( s )
   | ZeroCopyTokenKind ::Unrecognized( s )
   | ZeroCopyTokenKind ::Number( s ) => write!( f, "{}", s.as_ref() ),
   ZeroCopyTokenKind ::Operator( s )
   | ZeroCopyTokenKind ::Delimiter( s ) => write!( f, "{s}" ),
 }
 }
}

impl fmt ::Display for UnilangTokenKind< '_ >
{
  fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> fmt ::Result
  {
  match self
  {
   UnilangTokenKind ::Identifier( s ) | UnilangTokenKind ::Unrecognized( s ) | UnilangTokenKind ::Number( s ) => write!( f, "{s}" ),
   UnilangTokenKind ::Operator( s ) | UnilangTokenKind ::Delimiter( s ) => write!( f, "{s}" ),
 }
 }
}

/// Checks if a character is a valid part of a Unilang identifier.
/// Valid characters are lowercase alphanumeric (`a-z`, `0-9`) and underscore (`_`).
fn is_valid_identifier( s: &str ) -> bool
{
  !s.is_empty()
  && s.chars()
  .next()
  .is_some_and( | c | c.is_ascii_lowercase() || c == '_' )
  && !s.ends_with( '-' )
  && s
  .chars()
  .all( | c | c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' )
}

/// Classifies a `strs_tools ::Split` into a zero-copy `ZeroCopyTokenKind` and returns its adjusted source location.
///
/// Token strings are `Cow::Borrowed` — no heap allocation for input-derived tokens.
///
/// # Errors
/// Returns a `ParseError` if the split represents an invalid token.
pub fn classify_split_zero_copy< 'a >( s: &Split< 'a > ) -> Result< ( ZeroCopyTokenKind< 'a >, SourceLocation ), ParseError >
{
  // Fix(parser-001): return Cow::Borrowed for Identifier/Number/Unrecognized — zero allocation on hot path
  // Root cause: prior ZeroCopyTokenKind variants held &'a str, preventing use in synthetic merged
  //   tokens (merge_value_context_tokens) which create new Strings; fix changed variants to
  //   Cow<'a, str> so both borrowed (input) and owned (synthetic) tokens share one type
  // Pitfall: s.string.clone() on Cow::Borrowed copies the &str pointer (cheap); on Cow::Owned it
  //   clones the String (expected only for synthetic tokens that bypass classify_split entirely)
  let original_location = SourceLocation ::StrSpan
  {
  start: s.start,
  end: s.end,
 };

  let result = match s.string
  {
  Cow ::Borrowed( " :: " ) => Ok( ( ZeroCopyTokenKind ::Operator( " :: " ), original_location ) ),
  Cow ::Borrowed( "::" ) => Ok( ( ZeroCopyTokenKind ::Operator( "::" ), original_location ) ),
  Cow ::Borrowed( " : " ) => Ok( ( ZeroCopyTokenKind ::Operator( " : " ), original_location ) ),
  Cow ::Borrowed( "." ) => Ok( ( ZeroCopyTokenKind ::Delimiter( "." ), original_location ) ),
  Cow ::Borrowed( " " ) => Ok( ( ZeroCopyTokenKind ::Delimiter( " " ), original_location ) ),
  Cow ::Borrowed( "\t" ) => Ok( ( ZeroCopyTokenKind ::Delimiter( "\t" ), original_location ) ),
  Cow ::Borrowed( "\r" ) => Ok( ( ZeroCopyTokenKind ::Delimiter( "\r" ), original_location ) ),
  Cow ::Borrowed( "\n" ) => Ok( ( ZeroCopyTokenKind ::Delimiter( "\n" ), original_location ) ),
  Cow ::Borrowed( "#" ) => Ok( ( ZeroCopyTokenKind ::Delimiter( "#" ), original_location ) ),
  Cow ::Borrowed( "!" ) => Ok( ( ZeroCopyTokenKind ::Unrecognized( alloc ::borrow ::Cow ::Borrowed( "!" ) ), original_location ) ),
  _ =>
  {
   if s.typ == SplitType ::Delimiter
   {
  if s.was_quoted
  {
   Ok( ( ZeroCopyTokenKind ::Identifier( s.string.clone() ), original_location ) )
 }
  else if s.string.parse :: < i64 >().is_ok()
  {
   Ok( ( ZeroCopyTokenKind ::Number( s.string.clone() ), original_location ) )
 }
  else if is_valid_identifier( s.string.as_ref() )
    || matches!( s.string.as_ref(), "?" | "??" )
  {
   // `?` and `??` are ordinary value-capable tokens (not operators): `??` is the
   // semantic-layer help token when unquoted; a lone `?` is a plain literal value.
   Ok( ( ZeroCopyTokenKind ::Identifier( s.string.clone() ), original_location ) )
 }
  else
  {
   Ok( ( ZeroCopyTokenKind ::Unrecognized( s.string.clone() ), original_location ) )
 }
 }
   else
   {
  Ok( ( ZeroCopyTokenKind ::Unrecognized( s.string.clone() ), original_location ) )
 }
 }
 };
  result
}

/// Classifies a `strs_tools ::Split` into a `ZeroCopyTokenKind` and returns its adjusted source location.
///
/// # Errors
/// Returns a `ParseError` if the split represents an invalid escape sequence.
pub fn classify_split< 'a >( s: &Split< 'a > ) -> Result< ( ZeroCopyTokenKind< 'a >, SourceLocation ), ParseError >
{
  classify_split_zero_copy( s )
}
