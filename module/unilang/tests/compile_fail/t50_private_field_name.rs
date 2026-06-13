// T50: Private field enforcement — direct `name` field access must not compile.
// CommandDefinition.name is pub(in super), inaccessible outside the module.
fn main()
{
  let cmd = unilang::data::CommandDefinition::former()
  .name( ".test" )
  .description( String::from( "test" ) )
  .end();
  let _ = cmd.name;
}
