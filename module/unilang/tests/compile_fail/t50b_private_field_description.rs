// T50b: Private field enforcement — direct `description` field access must not compile.
// CommandDefinition.description is pub(in super), inaccessible outside the module.
fn main()
{
  let cmd = unilang::data::CommandDefinition::former()
  .name( ".test" )
  .description( String::from( "test" ) )
  .end();
  let _ = cmd.description;
}
