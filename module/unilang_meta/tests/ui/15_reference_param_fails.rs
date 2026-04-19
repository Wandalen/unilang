//! T15: Reference parameter type `&str` produces a compile error.
//!
//! `&str` is a reference type (syn::Type::Reference), not a path type, so the
//! macro must reject it with "unsupported parameter type: expected a path type".

#[ unilang_meta::command( name = ".ref_param" ) ]
fn ref_param( name : &str ) -> String
{
  name.to_string()
}

fn main() {}
