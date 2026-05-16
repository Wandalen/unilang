//! T14: Option<Vec<String>> — unsupported inner type produces a compile error.
//!
//! The type inference recurses into the Option<T> inner type; Vec<String> is not
//! a supported type, so the macro must emit "unsupported parameter type: Vec".

#[ unilang_meta::command( name = ".bulk" ) ]
fn bulk( names : Option< Vec< String > > ) -> String
{
  names.unwrap_or_default().join( ", " )
}

fn main() {}
