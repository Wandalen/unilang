//! T16: `hint` attribute value is stored and retrievable via `def.hint()`.
//!
//! ## Test Matrix
//!
//! | Case | Input | Expected |
//! |------|-------|----------|
//! | hint provided | hint = "short help text" | def.hint() == "short help text" |
//! | empty hint | hint = "" | def.hint() == "" |
//! | hint absent | no hint attr | def.hint() == "" |
//!
//! T04 sets hint but never asserts its value — this test closes that gap and
//! confirms the `.former().hint(...)` builder call actually stores the value.

#[ unilang_meta::command(
  name  = ".with_hint",
  hint  = "short help text",
) ]
fn with_hint() -> String
{
  "ok".to_string()
}

#[ unilang_meta::command(
  name = ".empty_hint",
  hint = "",
) ]
fn empty_hint() -> String
{
  "ok".to_string()
}

#[ unilang_meta::command( name = ".no_hint" ) ]
fn no_hint() -> String
{
  "ok".to_string()
}

fn main()
{
  // hint provided → exact value stored
  let def = __unilang_register_with_hint();
  assert_eq!( def.hint(), "short help text", "hint must equal the provided string" );

  // empty hint → stored as empty string
  let def_empty = __unilang_register_empty_hint();
  assert_eq!( def_empty.hint(), "", "empty hint must be stored as empty string" );

  // hint absent → defaults to empty string
  let def_none = __unilang_register_no_hint();
  assert_eq!( def_none.hint(), "", "absent hint must default to empty string" );
}
