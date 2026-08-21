//! Routing matrix for the `??` help token.
//!
//! One test per routing rule of the single-token help design:
//! - bare `??` → global command listing (mirror of bare `.`)
//! - positional unquoted `??` (any position) → command help page
//! - named `param::??` (unquoted) → parameter detail page; beats positional
//! - several named `??` → first parameter in command-definition order wins
//! - alias `alias::??` resolves like the canonical name
//! - unknown `param::??` → valid-parameter listing, never a dead end
//! - quoted `"??"` (named or positional) → literal value, no help
//! - `with_help_detection( false )` → every `??` is a plain value
//! - `.cmd.help` / `.cmd.help <param>` spelled routes render the same pages
//! - interception happens before argument binding: broken sibling arguments
//!   never surface coercion errors when help is requested

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::pipeline::Pipeline;
use unilang::interpreter::ExecutionContext;

/// Builds the matrix command: `.matrix mode::enum count::integer tag::string`.
fn build_registry() -> CommandRegistry
{
  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former()
    .name( ".matrix" )
    .namespace( "" )
    .description( "Routing matrix probe command" )
    .hint( "Matrix probe" )
    .status( "stable" )
    .version( "1.0.0" )
    .arguments( vec![
      ArgumentDefinition
      {
        name : "mode".to_string(),
        description : "Processing mode".to_string(),
        kind : Kind::Enum( vec![ "fast".to_string(), "slow".to_string() ] ),
        hint : "Mode".to_string(),
        attributes : ArgumentAttributes
        {
          optional : true,
          default : Some( "fast".to_string() ),
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition
      {
        name : "count".to_string(),
        description : "How many times to run".to_string(),
        kind : Kind::Integer,
        hint : "Count".to_string(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![],
        tags : vec![],
      },
      ArgumentDefinition
      {
        name : "tag".to_string(),
        description : "Free-form tag".to_string(),
        kind : Kind::String,
        hint : "Tag".to_string(),
        attributes : ArgumentAttributes { optional : true, ..Default::default() },
        validation_rules : vec![],
        aliases : vec![ "t".to_string() ],
        tags : vec![],
      },
    ])
    .end();

  let routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx : ExecutionContext |
  {
    let tag = cmd.get_string( "tag" ).unwrap_or_default();
    Ok( OutputData
    {
      content : format!( "ran tag={tag}" ),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });
  registry.register_with_routine( &cmd, routine ).expect( "Registration must succeed" );
  registry
}

#[ test ]
fn test_bare_help_token_lists_commands()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( "??", ExecutionContext::default() );
  assert!( result.success, "Bare ?? must produce the global listing; error: {:?}", result.error );
  assert!( result.outputs[ 0 ].content.contains( ".matrix" ), "Listing must include registered commands" );
}

#[ test ]
fn test_bare_help_token_with_arguments_is_not_help()
{
  let pipeline = Pipeline::new( build_registry() );
  // `??` with arguments is not the argument-free global-help form; the token
  // is looked up as a command and fails — arguments are never silently dropped.
  let result = pipeline.process_command( "?? extra", ExecutionContext::default() );
  assert!( !result.success, "?? with arguments must not be swallowed as global help" );
}

#[ test ]
fn test_positional_help_token_renders_command_page()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix ??", ExecutionContext::default() );
  assert!( result.success, "Positional ?? must render command help; error: {:?}", result.error );
  let page = &result.outputs[ 0 ].content;
  assert!( page.contains( "Usage: .matrix" ), "Command page must show usage line; got: {page:?}" );
  assert!( page.contains( "mode" ) && page.contains( "count" ) && page.contains( "tag" ),
    "Command page must list all parameters; got: {page:?}" );
}

#[ test ]
fn test_positional_help_token_after_named_arguments()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix count::5 ??", ExecutionContext::default() );
  assert!( result.success, "?? after named args must still render command help; error: {:?}", result.error );
  assert!( result.outputs[ 0 ].content.contains( "Usage: .matrix" ) );
}

#[ test ]
fn test_named_help_token_renders_parameter_page()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix count::??", ExecutionContext::default() );
  assert!( result.success, "param::?? must render parameter help; error: {:?}", result.error );
  let page = &result.outputs[ 0 ].content;
  assert!( page.contains( "Parameter: count" ), "Parameter page must name the parameter; got: {page:?}" );
  assert!( page.contains( "How many times to run" ), "Parameter page must show the description" );
}

#[ test ]
fn test_named_help_token_beats_positional()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix count::?? ??", ExecutionContext::default() );
  assert!( result.success );
  assert!( result.outputs[ 0 ].content.contains( "Parameter: count" ),
    "Named ?? must take precedence over positional ??" );
}

#[ test ]
fn test_multiple_named_help_tokens_use_definition_order()
{
  let pipeline = Pipeline::new( build_registry() );
  // `mode` is declared before `count` — definition order, not input order, wins.
  let result = pipeline.process_command( ".matrix count::?? mode::??", ExecutionContext::default() );
  assert!( result.success );
  assert!( result.outputs[ 0 ].content.contains( "Parameter: mode" ),
    "First parameter in definition order must win; got: {:?}", result.outputs[ 0 ].content );
}

#[ test ]
fn test_alias_help_token_resolves_canonical_parameter()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix t::??", ExecutionContext::default() );
  assert!( result.success, "alias::?? must resolve like the canonical name; error: {:?}", result.error );
  assert!( result.outputs[ 0 ].content.contains( "Parameter: tag" ),
    "Alias lookup must render the canonical parameter page" );
}

#[ test ]
fn test_unknown_parameter_help_token_lists_valid_parameters()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix bogus::??", ExecutionContext::default() );
  assert!( result.success, "unknown::?? must fall back to a listing; error: {:?}", result.error );
  let page = &result.outputs[ 0 ].content;
  assert!( page.contains( "bogus" ), "Listing must echo the unknown name" );
  assert!( page.contains( "mode" ) && page.contains( "count" ) && page.contains( "tag" ),
    "Listing must enumerate valid parameters; got: {page:?}" );
}

#[ test ]
fn test_help_interception_precedes_argument_binding()
{
  let pipeline = Pipeline::new( build_registry() );
  // `count::abc` cannot coerce to Integer — but help interception runs before
  // binding, so the broken sibling never produces a coercion error.
  let result = pipeline.process_command( ".matrix tag::?? count::abc", ExecutionContext::default() );
  assert!( result.success, "Help must win over sibling coercion failures; error: {:?}", result.error );
  assert!( result.outputs[ 0 ].content.contains( "Parameter: tag" ) );
}

#[ test ]
fn test_quoted_named_help_token_is_literal()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix tag::\"??\"", ExecutionContext::default() );
  assert!( result.success, "Quoted ?? must bind as a plain value; error: {:?}", result.error );
  assert_eq!( result.outputs[ 0 ].content, "ran tag=??",
    "The routine must observe the literal ?? value" );
}

#[ test ]
fn test_help_detection_disabled_makes_named_token_literal()
{
  let pipeline = Pipeline::new( build_registry() ).with_help_detection( false );
  let result = pipeline.process_command( ".matrix tag::??", ExecutionContext::default() );
  assert!( result.success, "With detection off, ?? must bind as a plain value; error: {:?}", result.error );
  assert_eq!( result.outputs[ 0 ].content, "ran tag=??" );
}

#[ test ]
fn test_help_detection_disabled_makes_bare_token_unknown_command()
{
  let pipeline = Pipeline::new( build_registry() ).with_help_detection( false );
  let result = pipeline.process_command( "??", ExecutionContext::default() );
  assert!( !result.success, "With detection off, bare ?? must not produce the global listing" );
}

#[ test ]
fn test_help_detection_disabled_positional_token_hits_coercion()
{
  let pipeline = Pipeline::new( build_registry() ).with_help_detection( false );
  // Positional `??` binds to the first parameter (`mode`, an enum) and fails
  // coercion — proving it reached binding as an ordinary value.
  let result = pipeline.process_command( ".matrix ??", ExecutionContext::default() );
  assert!( !result.success, "With detection off, positional ?? must be an ordinary value" );
  let error = result.error.unwrap();
  assert!( error.contains( "fast" ) && error.contains( "slow" ),
    "Enum coercion failure must list the allowed choices; got: {error:?}" );
}

#[ test ]
fn test_spelled_help_command_renders_command_page()
{
  let pipeline = Pipeline::new( build_registry() );
  let spelled = pipeline.process_command( ".matrix.help", ExecutionContext::default() );
  assert!( spelled.success, ".matrix.help must succeed; error: {:?}", spelled.error );

  let token = pipeline.process_command( ".matrix ??", ExecutionContext::default() );
  assert_eq!( spelled.outputs[ 0 ].content, token.outputs[ 0 ].content,
    ".cmd.help and .cmd ?? must render the identical command page" );
}

#[ test ]
fn test_spelled_help_command_with_parameter_argument()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix.help count", ExecutionContext::default() );
  assert!( result.success, ".matrix.help count must succeed; error: {:?}", result.error );
  assert!( result.outputs[ 0 ].content.contains( "Parameter: count" ),
    ".cmd.help <param> must render the parameter page; got: {:?}", result.outputs[ 0 ].content );

  // Unknown parameter falls back to the valid-parameter listing
  let fallback = pipeline.process_command( ".matrix.help bogus", ExecutionContext::default() );
  assert!( fallback.success, ".matrix.help bogus must not be a dead end; error: {:?}", fallback.error );
  assert!( fallback.outputs[ 0 ].content.contains( "mode" ),
    "Fallback listing must enumerate valid parameters" );
}

#[ test ]
fn test_parameter_page_carries_derived_examples()
{
  let pipeline = Pipeline::new( build_registry() );
  let result = pipeline.process_command( ".matrix mode::??", ExecutionContext::default() );
  assert!( result.success );
  // Synthesized canonical invocation: enum placeholder is the first choice.
  assert!( result.outputs[ 0 ].content.contains( ".matrix mode::fast" ),
    "Parameter page must carry the synthesized canonical example; got: {:?}", result.outputs[ 0 ].content );
}

#[ test ]
fn test_coercion_failure_of_question_value_nudges_to_help()
{
  let pipeline = Pipeline::new( build_registry() );
  // The retired `?` operator is now a plain value; when it fails coercion the
  // error points at the real help syntax.
  let result = pipeline.process_command( ".matrix count::?", ExecutionContext::default() );
  assert!( !result.success, "? must be an ordinary (failing) Integer value" );
  let error = result.error.unwrap();
  assert!( error.contains( "Did you mean 'count::??' for parameter help?" ),
    "Coercion failure of '?' must nudge toward param::??; got: {error:?}" );
}
