//! T20: `i8` parameter type produces a compile error.
//!
//! Only `i64`, `i32`, `u64`, `u32`, `usize`, `isize` are in the supported integer
//! set.  Narrower integer types (`i8`, `u8`, `i16`, `u16`) are not.  The catch-all
//! arm in `type_to_kind_and_inner` must reject `i8` with
//! "unsupported parameter type: i8".

#[ unilang_meta::command( name = ".narrow" ) ]
fn narrow( tiny : i8 ) -> String
{
  tiny.to_string()
}

fn main() {}
