// T40: Type-state builder enforces .name() at compile time.
// Building a CommandDefinition with only .description() set must not compile
// because end() requires both Name=Set and Description=Set.
fn main()
{
  let _ = unilang::data::CommandDefinition::former()
  .description( String::from( "test description" ) )
  .end();
}
