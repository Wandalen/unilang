//! T12: Unknown attribute key produces a compile error.
//!
//! `bogus = "bar"` is not a recognised attribute; the macro parser must
//! reject it with "unknown command attribute: bogus".

#[ unilang_meta::command( name = ".foo", bogus = "bar" ) ]
fn foo() -> String
{
  "ok".to_string()
}

fn main() {}
