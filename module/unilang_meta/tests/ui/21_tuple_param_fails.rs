//! T21: Tuple parameter type `(String, i64)` produces a compile error.
//!
//! Tuple types are `syn::Type::Tuple`, not `syn::Type::Path`.  The first guard
//! in `type_to_kind_and_inner` must reject them with
//! "unsupported parameter type: expected a path type".

#[ unilang_meta::command( name = ".tuple_cmd" ) ]
fn tuple_cmd( pair : ( String, i64 ) ) -> String
{
  format!( "{} {}", pair.0, pair.1 )
}

fn main() {}
