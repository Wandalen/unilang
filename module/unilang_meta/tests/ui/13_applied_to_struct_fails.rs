//! T13: Applying #[unilang_meta::command] to a struct produces a compile error.
//!
//! The macro can only be applied to functions; applying it to a struct must
//! produce the error "unilang::command can only be applied to functions".

#[ unilang_meta::command( name = ".foo" ) ]
struct Foo;

fn main() {}
