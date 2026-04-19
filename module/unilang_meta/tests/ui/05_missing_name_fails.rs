//! T05: Missing required `name` attribute produces a compile error.

#[ unilang_meta::command( namespace = ".app" ) ]
fn no_name() -> String
{
  "unused".to_string()
}

fn main() {}
