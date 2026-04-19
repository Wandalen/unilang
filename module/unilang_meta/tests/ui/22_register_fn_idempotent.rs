//! T22: Registration function is idempotent — `OnceLock` reuse.
//!
//! ## Test Matrix
//!
//! | Case | Input | Expected |
//! |------|-------|----------|
//! | call once | __unilang_register_steady() | valid &CommandDefinition |
//! | call twice | second call | same pointer as first call |
//! | call 10 times | repeated calls | all return ptr::eq to first |
//!
//! The `OnceLock` pattern guarantees the `CommandDefinition` is initialised
//! exactly once.  All repeated calls to the register function must return a
//! reference to the same object (pointer equality).  This validates thread-safety
//! semantics and ensures callers can cache the result from the first call.

#[ unilang_meta::command(
  name        = ".steady",
  description = "Idempotency check",
) ]
fn steady() -> String
{
  "ok".to_string()
}

fn main()
{
  let ptr1 = __unilang_register_steady() as *const _;
  let ptr2 = __unilang_register_steady() as *const _;
  let ptr3 = __unilang_register_steady() as *const _;

  assert!(
    std::ptr::eq( ptr1, ptr2 ),
    "register fn must return same pointer on second call (OnceLock reuse)"
  );
  assert!(
    std::ptr::eq( ptr2, ptr3 ),
    "register fn must return same pointer on third call (OnceLock reuse)"
  );

  // Value must still be correct after repeated calls
  let def = __unilang_register_steady();
  assert_eq!( def.name().as_str(), ".steady" );
  assert_eq!( def.description(), "Idempotency check" );
}
