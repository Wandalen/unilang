//! T07: Integer type variants and PathBuf parameter type.
//!
//! Verifies that i32, u64, u32, usize, isize map to Kind::Integer and PathBuf
//! maps to Kind::Path; all inferred as non-optional with correct argument count.

use std::path::PathBuf;
use unilang::data::Kind;

#[ unilang_meta::command( name = ".calc" ) ]
fn calc( a : i32, b : u64, c : u32, d : usize, e : isize, path : PathBuf ) -> String
{
  format!( "{} {} {} {} {} {:?}", a, b, c, d, e, path )
}

fn main()
{
  let def = __unilang_register_calc();
  assert_eq!( def.arguments().len(), 6 );

  // All integer types map to Kind::Integer
  for i in 0..5
  {
    assert_eq!(
      def.arguments()[ i ].kind,
      Kind::Integer,
      "arg {} should be Kind::Integer",
      i
    );
    assert!( !def.arguments()[ i ].attributes.optional );
  }

  // PathBuf maps to Kind::Path
  assert_eq!( def.arguments()[ 5 ].kind, Kind::Path );
  assert!( !def.arguments()[ 5 ].attributes.optional );
}
