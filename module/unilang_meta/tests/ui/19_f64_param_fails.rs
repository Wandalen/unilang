//! T19: `f64` parameter type produces a compile error.
//!
//! Float types are not in the supported set (`String`, `bool`, `PathBuf`,
//! integer variants).  The catch-all arm in `type_to_kind_and_inner` must
//! reject `f64` with "unsupported parameter type: f64".

#[ unilang_meta::command( name = ".floaty" ) ]
fn floaty( value : f64 ) -> String
{
  value.to_string()
}

fn main() {}
