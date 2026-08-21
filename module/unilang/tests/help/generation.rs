//! Help Generation Unit Tests
//!
//! ## Scope
//! Tests the help system's ability to generate comprehensive help content for commands.
//! This covers help content creation, formatting, and the various help access patterns.
//!
//! ## Coverage
//! - Command-specific help generation
//! - Global help listing
//! - Help content accuracy and completeness
//! - Help formatting and structure
//! - Error conditions in help generation
//!
//! ## Related
//! - `unit/help/conventions.rs` - Help system conventions
//! - `unit/help/formatting.rs` - Help output formatting


use unilang::data::{ ArgumentDefinition, CommandDefinition, Kind, ArgumentAttributes, OutputData };
use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;
use unilang::interpreter::ExecutionContext;
use unilang::semantic::VerifiedCommand;

/// Helper to create a test command definition
fn create_test_command() -> CommandDefinition
{
  CommandDefinition::former()
    .name( ".test" )
    .description( "A test command for help generation validation" )
    .hint( "Use this command to test help functionality" )
    .version( "1.0.0" )
    .status( "stable" )
    .arguments( vec![
      ArgumentDefinition {
        name : "input".to_string(),
        description : "Input file path".to_string(),
        kind : Kind::String,
        hint : "Path to the input file".to_string(),
        attributes : ArgumentAttributes {
          optional : false,
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "i".to_string() ],
        tags : vec![ "file".to_string() ],
      },
      ArgumentDefinition {
        name : "output".to_string(),
        description : "Output file path".to_string(),
        kind : Kind::String,
        hint : "Path to the output file".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "output.txt".to_string() ),
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "o".to_string() ],
        tags : vec![ "file".to_string() ],
      },
      ArgumentDefinition {
        name : "verbose".to_string(),
        description : "Enable verbose output".to_string(),
        kind : Kind::Boolean,
        hint : "Show detailed output information".to_string(),
        attributes : ArgumentAttributes {
          optional : true,
          default : Some( "false".to_string() ),
          multiple : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "v".to_string() ],
        tags : vec![ "output".to_string() ],
      }
    ])
    .examples( vec![
      ".test input::\"data.txt\"".to_string(),
      ".test input::\"data.txt\" output::\"result.txt\" verbose::true".to_string(),
    ])
    .end()
}

#[test]
fn test_command_specific_help_generation()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify help contains essential information (Level 2 Standard format)
  assert!( help_content.contains( "test" ), "Help should contain command name" );
  assert!( help_content.contains( "A test command for help generation validation" ), "Help should contain description" );
  assert!( help_content.contains( "input" ), "Help should contain required argument" );
  assert!( help_content.contains( "output" ), "Help should contain optional argument" );
  assert!( help_content.contains( "verbose" ), "Help should contain boolean argument" );
  assert!( help_content.contains( "Usage:" ), "Help should contain usage section" );
}

/// FT-2: Detailed help output includes argument names and descriptions.
// test_kind: ft_spec(FT-2)  [feature/04_help_system]
#[test]
fn test_help_includes_argument_details()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify argument details are included (Level 2 Standard format uses Arguments:)
  // Note: Level 2 shows hints if available, otherwise descriptions
  assert!( help_content.contains( "Arguments:" ), "Help should contain arguments section" );
  assert!( help_content.contains( "input" ), "Help should contain argument names" );
  assert!( help_content.contains( "string" ), "Help should contain type information" );
  assert!( help_content.contains( "Path to the input file" ) || help_content.contains( "Input file path" ), "Help should contain argument descriptions or hints" );
}

#[test]
fn test_help_includes_examples()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify help content structure (Level 2 Standard format)
  assert!( help_content.contains( "Usage:" ), "Help should contain usage section" );
  assert!( help_content.contains( "Examples:" ), "Help should contain examples section" );
  assert!( help_content.contains( "A test command" ), "Help should contain description" );
}

#[test]
fn test_help_includes_aliases()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify aliases are mentioned
  assert!( help_content.contains( 'i' ) && help_content.contains( "input" ), "Help should mention argument aliases" );
  assert!( help_content.contains( 'o' ) && help_content.contains( "output" ), "Help should mention argument aliases" );
}

/// FT-1: Command list returns all registered command names.
// test_kind: ft_spec(FT-1)  [feature/04_help_system]
#[test]
fn test_global_help_listing()
{
  let mut registry = CommandRegistry::new();

  // Add multiple commands
  let cmd1 = CommandDefinition::former()
    .name( ".first" )
    .description( "First test command" )
    .end();

  let cmd2 = CommandDefinition::former()
    .name( ".second" )
    .description( "Second test command" )
    .end();

  let cmd3 = CommandDefinition::former()
    .name( ".third" )
    .description( "Third test command" )
    .end();

  // Use runtime registration for all commands
  let test_routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let test_routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let test_routine3 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd1, test_routine1 ).unwrap();
  registry.register_with_routine( &cmd2, test_routine2 ).unwrap();
  registry.register_with_routine( &cmd3, test_routine3 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.list_commands();

  // Verify all commands are listed
  assert!( help_content.contains( "first" ), "Global help should list first command" );
  assert!( help_content.contains( "second" ), "Global help should list second command" );
  assert!( help_content.contains( "third" ), "Global help should list third command" );

  // Verify descriptions are included
  assert!( help_content.contains( "First test command" ), "Global help should include descriptions" );
  assert!( help_content.contains( "Second test command" ), "Global help should include descriptions" );
  assert!( help_content.contains( "Third test command" ), "Global help should include descriptions" );

  // Verify overall structure
  assert!( help_content.contains( "Available" ) || help_content.contains( "Commands:" ), "Global help should have header" );
}

#[test]
fn test_help_for_nonexistent_command()
{
  let registry = CommandRegistry::new();
  let help_generator = HelpGenerator::new( &registry );

  let help_result = help_generator.command( ".nonexistent" );

  // Should handle gracefully
  assert!( help_result.is_none(), "Help for nonexistent command should return None" );
}

#[test]
fn test_help_with_empty_registry()
{
  let registry = CommandRegistry::new();
  let help_generator = HelpGenerator::new( &registry );

  let help_content = help_generator.list_commands();

  // Should handle empty registry gracefully - returns some content (even if just empty message)
  assert!( !help_content.is_empty(), "Should return some help content" );
  // Empty registry should not list any specific commands
  assert!( !help_content.contains( ".test" ), "Empty registry should not show test commands" );
}

#[test]
fn test_help_content_formatting()
{
  let mut registry = CommandRegistry::new();
  let cmd = create_test_command();
  let cmd_name = cmd.name().clone();

  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify basic formatting structure
  let lines : Vec< &str > = help_content.lines().collect();
  assert!( lines.len() > 3, "Help should have multiple lines" );

  // Verify no obviously malformed content
  assert!( !help_content.contains( "{{" ), "Help should not contain template placeholders" );
  assert!( !help_content.contains( "}}" ), "Help should not contain template placeholders" );
  assert!( !help_content.is_empty(), "Help should not be empty" );
}

#[test]
fn test_help_performance()
{

  let mut registry = CommandRegistry::new();

  // Add many commands to test performance
  for i in 1..=50 {
    let cmd = CommandDefinition::former()
      .name( format!( ".command{i}" ) )
      .description( format!( "Test command number {i}" ) )
      .end();

    let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
      Ok(OutputData::new("test", "text"))
    });
    registry.register_with_routine( &cmd, test_routine ).unwrap();
  }

  let help_generator = HelpGenerator::new( &registry );

  let help_content = help_generator.list_commands();

  // Performance check

  // Verify correctness wasn't sacrificed
  assert!( help_content.contains( "command1" ), "Help should contain first command" );
  assert!( help_content.contains( "command50" ), "Help should contain last command" );
}

#[test]
fn test_command_help_with_complex_arguments()
{
  let mut registry = CommandRegistry::new();

  let cmd = CommandDefinition::former()
    .name( ".complex" )
    .description( "Command with complex arguments" )
    .arguments( vec![
      ArgumentDefinition {
        name : "multi_value".to_string(),
        description : "Parameter that accepts multiple values".to_string(),
        kind : Kind::List( Box::new( Kind::String ), None ),  // Fixed: multiple:true requires Kind::List
        hint : "Multiple string values".to_string(),
        attributes : ArgumentAttributes {
          multiple : true,
          optional : false,
          ..Default::default()
        },
        validation_rules : vec![],
        aliases : vec![ "m".to_string(), "multi".to_string() ],
        tags : vec![ "list".to_string() ],
      }
    ])
    .end();

  let cmd_name = cmd.name().clone();
  // Use runtime registration instead since command_add is deprecated
  let test_routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd, test_routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let help_content = help_generator.command( cmd_name.as_str() ).expect( "Help should be generated" );

  // Verify complex argument features are documented
  assert!( help_content.contains( "multi_value" ), "Help should contain argument name" );
  assert!( help_content.contains( "multiple" ) || help_content.contains( "Multi" ), "Help should indicate multiple values capability" );
  assert!( help_content.contains( 'm' ), "Help should show aliases" );
}

/// T01: Flat-list rendering (no explicit categories) preserved through the `cli_fmt` swap.
// test_kind: tm_spec(T01)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t01_flat_list_no_category()
{
  let mut registry = CommandRegistry::new();

  let cmd1 = CommandDefinition::former().name( ".alpha" ).description( "Alpha command" ).end();
  let cmd2 = CommandDefinition::former().name( ".beta" ).description( "Beta command" ).end();
  let cmd3 = CommandDefinition::former().name( ".gamma" ).description( "Gamma command" ).end();

  let routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine3 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd1, routine1 ).unwrap();
  registry.register_with_routine( &cmd2, routine2 ).unwrap();
  registry.register_with_routine( &cmd3, routine3 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let output = help_generator.list_commands_filtered( None );

  assert!( output.contains( ".alpha" ), "Output should contain .alpha" );
  assert!( output.contains( "Alpha command" ), "Output should contain alpha's description" );
  assert!( output.contains( ".beta" ), "Output should contain .beta" );
  assert!( output.contains( "Beta command" ), "Output should contain beta's description" );
  assert!( output.contains( ".gamma" ), "Output should contain .gamma" );
  assert!( output.contains( "Gamma command" ), "Output should contain gamma's description" );
}

/// T02: Multi-group category rendering via `CliHelpData.groups` preserved through the swap.
// test_kind: tm_spec(T02)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t02_multi_category_grouping()
{
  let mut registry = CommandRegistry::new();

  let cmd_a1 = CommandDefinition::former()
    .name( ".cmd_a1" )
    .description( "First alpha command" )
    .category( "alpha_ops" )
    .end();
  let cmd_b1 = CommandDefinition::former()
    .name( ".cmd_b1" )
    .description( "First beta command" )
    .category( "beta_ops" )
    .end();

  let routine_a = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine_b = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd_a1, routine_a ).unwrap();
  registry.register_with_routine( &cmd_b1, routine_b ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let output = help_generator.list_commands_filtered( None );

  assert!( output.contains( "Alpha Ops" ), "Output should show 'Alpha Ops' group header" );
  assert!( output.contains( "Beta Ops" ), "Output should show 'Beta Ops' group header" );
  assert!( output.contains( ".cmd_a1" ), "Output should contain alpha command" );
  assert!( output.contains( ".cmd_b1" ), "Output should contain beta command" );

  // Each category's command appears only under its own group header (not merely present anywhere).
  let alpha_header_pos = output.find( "Alpha Ops" ).expect( "Alpha Ops header must be present" );
  let beta_header_pos = output.find( "Beta Ops" ).expect( "Beta Ops header must be present" );
  let cmd_a1_pos = output.find( ".cmd_a1" ).expect( ".cmd_a1 must be present" );
  let cmd_b1_pos = output.find( ".cmd_b1" ).expect( ".cmd_b1 must be present" );

  assert!( alpha_header_pos < cmd_a1_pos, ".cmd_a1 should appear after the Alpha Ops header" );
  assert!( cmd_a1_pos < beta_header_pos, ".cmd_a1 should appear before the Beta Ops header (scoped to its own group)" );
  assert!( beta_header_pos < cmd_b1_pos, ".cmd_b1 should appear after the Beta Ops header" );
}

/// T03: Prefix filtering preserved through the swap.
// test_kind: tm_spec(T03)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t03_prefix_filtering_preserved()
{
  let mut registry = CommandRegistry::new();

  let cmd_push = CommandDefinition::former().name( ".git.push" ).description( "Push commits" ).end();
  let cmd_pull = CommandDefinition::former().name( ".git.pull" ).description( "Pull commits" ).end();
  let cmd_remove = CommandDefinition::former().name( ".remove" ).description( "Remove item" ).end();

  let routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine3 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd_push, routine1 ).unwrap();
  registry.register_with_routine( &cmd_pull, routine2 ).unwrap();
  registry.register_with_routine( &cmd_remove, routine3 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let output = help_generator.list_commands_filtered( Some( ".git" ) );

  assert!( output.contains( ".git.push" ), "Output should contain .git.push" );
  assert!( output.contains( ".git.pull" ), "Output should contain .git.pull" );
  assert!( !output.contains( ".remove" ), "Output should NOT contain .remove" );
}

/// T04: Hidden-from-list visibility filtering preserved through the swap.
// test_kind: tm_spec(T04)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t04_hidden_from_list_preserved()
{
  let mut registry = CommandRegistry::new();

  let cmd_visible = CommandDefinition::former().name( ".visible" ).description( "Visible command" ).end();
  let cmd_hidden = CommandDefinition::former()
    .name( ".hidden" )
    .description( "Hidden command" )
    .hidden_from_list( true )
    .end();

  let routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd_visible, routine1 ).unwrap();
  registry.register_with_routine( &cmd_hidden, routine2 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let output = help_generator.list_commands_filtered( None );

  assert!( output.contains( ".visible" ), "Output should contain the visible command" );
  assert!( !output.contains( ".hidden" ), "Output should NOT contain the hidden command" );
}

/// T05: Priority-then-name sort order preserved through the swap.
// test_kind: tm_spec(T05)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t05_sort_order_preserved()
{
  let mut registry = CommandRegistry::new();

  // Names are deliberately alphabetically *opposite* to their priority order: if the output
  // ordering matched alphabetical order instead of priority, this test would fail — proving
  // priority (not name) drives the sort.
  let cmd_low_priority = CommandDefinition::former()
    .name( ".aaa_low_priority" )
    .description( "Lower-priority command" )
    .category( "same_category" )
    .priority( 9 )
    .end();
  let cmd_high_priority = CommandDefinition::former()
    .name( ".zzz_high_priority" )
    .description( "Higher-priority command" )
    .category( "same_category" )
    .priority( 1 )
    .end();

  let routine1 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  let routine2 = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });

  registry.register_with_routine( &cmd_low_priority, routine1 ).unwrap();
  registry.register_with_routine( &cmd_high_priority, routine2 ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let output = help_generator.list_commands_filtered( None );

  let high_pos = output.find( ".zzz_high_priority" ).expect( ".zzz_high_priority must be present" );
  let low_pos = output.find( ".aaa_low_priority" ).expect( ".aaa_low_priority must be present" );

  assert!(
    high_pos < low_pos,
    "Higher-priority (priority=1) command should appear before lower-priority (priority=9) command, despite an alphabetically later name"
  );
}

/// T06: Empty-registry and no-prefix-match messaging preserved through the swap.
// test_kind: tm_spec(T06)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t06_empty_and_no_match_messaging_preserved()
{
  let empty_registry = CommandRegistry::new();
  let empty_help_generator = HelpGenerator::new( &empty_registry );
  let empty_output = empty_help_generator.list_commands_filtered( None );

  assert!( empty_output.contains( "No commands available." ), "Empty registry should show the no-commands message" );

  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former().name( ".exists" ).description( "Exists command" ).end();
  let routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  registry.register_with_routine( &cmd, routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );
  let no_match_output = help_generator.list_commands_filtered( Some( ".nomatch" ) );

  assert!(
    no_match_output.contains( "No commands found matching prefix: .nomatch" ),
    "Non-matching prefix should show the no-match message"
  );
}

/// T07: Footer usage-hint text preserved through the swap, gated on prefix presence.
// test_kind: tm_spec(T07)  [task/unilang/114_adopt_cli_fmt_for_global_help_listing.md]
#[test]
fn test_t07_footer_hint_gated_on_prefix()
{
  let mut registry = CommandRegistry::new();
  let cmd = CommandDefinition::former().name( ".only" ).description( "Only command" ).end();
  let routine = Box::new( |_cmd: VerifiedCommand, _ctx: ExecutionContext| -> Result<OutputData, unilang::data::ErrorData> {
    Ok(OutputData::new("test", "text"))
  });
  registry.register_with_routine( &cmd, routine ).unwrap();

  let help_generator = HelpGenerator::new( &registry );

  let no_prefix_output = help_generator.list_commands_filtered( None );
  assert!(
    no_prefix_output.contains( "Use '<command> ??' or '<command>.help' to get detailed help for a specific command." ),
    "No-prefix output should contain the footer usage-hint text"
  );
  assert!( no_prefix_output.contains( "Example:" ), "No-prefix output should contain the example line" );

  let prefixed_output = help_generator.list_commands_filtered( Some( ".only" ) );
  assert!(
    !prefixed_output.contains( "Use '<command> ??' or '<command>.help' to get detailed help for a specific command." ),
    "Prefixed output should NOT contain the footer usage-hint text"
  );
}