//! T06: Unsupported parameter type `Vec<String>` produces a compile error.

#[ unilang_meta::command( name = ".bulk" ) ]
fn bulk( names : Vec< String > ) -> String
{
  names.join( ", " )
}

fn main() {}
