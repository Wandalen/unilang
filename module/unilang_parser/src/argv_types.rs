//! Input marker newtypes for type-safe parser entry points.
//!
//! [`ShellArgv`] wraps pre-tokenized shell argv; [`ReplInput`] wraps a raw
//! command string.  Passing the wrong type to a parser method is a compile-time
//! error rather than a silent runtime misuse.

use alloc ::{ borrow ::ToOwned, string ::String, vec ::Vec };

/// Marker newtype wrapping shell-tokenized argv.
///
/// Created from the OS argv (e.g. via [`ShellArgv::from_env`]) or from a
/// pre-built `Vec<String>` (e.g. via [`ShellArgv::from_vec`]).
/// Pass to [`crate::parser_engine::Parser::parse_cli`] for type-safe CLI
/// parsing.
#[ derive( Debug, Clone ) ]
pub struct ShellArgv( Vec< String > );

impl ShellArgv
{
  /// Create from a pre-built argv vector.
  pub fn from_vec( argv : Vec< String > ) -> Self
  {
    Self( argv )
  }

  /// Return the argv items as a slice.
  pub fn as_slice( &self ) -> &[ String ]
  {
    &self.0
  }

  /// Create from OS-provided argv (`std::env::args()`).
  ///
  /// Not available in `no_std` builds.
  #[ cfg( not( feature = "no_std" ) ) ]
  pub fn from_env() -> Self
  {
    Self( ::std ::env ::args().collect() )
  }
}

impl From< Vec< String > > for ShellArgv
{
  fn from( v : Vec< String > ) -> Self { Self( v ) }
}

/// Marker newtype wrapping a raw command string for REPL or script input.
///
/// Created from a `&str` or `String` that has **not** been shell-tokenized.
/// Pass to [`crate::parser_engine::Parser::parse_repl`] for type-safe string
/// parsing.
#[ derive( Debug, Clone ) ]
pub struct ReplInput( String );

impl ReplInput
{
  /// Create from a string slice.
  pub fn new( s : &str ) -> Self { Self( s.to_owned() ) }

  /// Return the inner string as a `&str`.
  pub fn as_str( &self ) -> &str { &self.0 }
}

impl From< &str > for ReplInput
{
  fn from( s : &str ) -> Self { Self( s.to_owned() ) }
}

impl From< String > for ReplInput
{
  fn from( s : String ) -> Self { Self( s ) }
}
