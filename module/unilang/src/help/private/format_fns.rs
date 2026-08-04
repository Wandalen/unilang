//! Help text format methods for HelpGenerator.

use super::*;
use core::fmt::Write;

impl< 'a > HelpGenerator< 'a >
{
  /// Helper function to format argument kind as lowercase string
  fn format_kind( kind : &crate::data::Kind ) -> String
  {
    format!( "{:?}", kind ).to_lowercase()
  }

  /// Format Level 0: Minimal - Just name and brief description
  pub( super ) fn format_minimal( &self, command : &crate::CommandDefinition ) -> String
  {
    format!( "{} - {}", command.name().as_str(), command.description() )
  }

  /// Format Level 1: Basic - Add parameters list with types
  pub( super ) fn format_basic( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}", command.name().as_str(), command.description() ).unwrap();

    if !command.arguments().is_empty()
    {
      writeln!( &mut help, "\nPARAMETERS:" ).unwrap();
      for arg in command.arguments()
      {
        writeln!( &mut help, "  {}::{}", arg.name, Self::format_kind( &arg.kind ) ).unwrap();
      }
    }
    help
  }

  /// Format Level 2: Standard (DEFAULT) - Concise like unikit
  pub( super ) fn format_standard( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();

    // Command header with optional version
    if command.show_version_in_help() && self.display_options.show_version
    {
      writeln!( &mut help, "Usage: {} (v{})", command.name().as_str(), command.version().as_str() ).unwrap();
    }
    else
    {
      writeln!( &mut help, "Usage: {}", command.name().as_str() ).unwrap();
    }
    writeln!( &mut help, "{}\n", command.description() ).unwrap();

    // Status information
    if self.display_options.show_status
    {
      writeln!( &mut help, "Status: {}", command.status() ).unwrap();
    }
    if !command.aliases().is_empty() && self.display_options.show_aliases
    {
      writeln!( &mut help, "Aliases: {}", command.aliases().join( ", " ) ).unwrap();
    }

    // Arguments section with improved formatting
    if !command.arguments().is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in command.arguments()
      {
        write!( &mut help, "{}", arg.name ).unwrap();
        write!( &mut help, " (Type: {})", Self::format_kind( &arg.kind ) ).unwrap();

        let mut status_parts = Vec::new();
        if arg.attributes.optional {
          status_parts.push("Optional");
        }
        if arg.attributes.multiple {
          status_parts.push("Multiple");
        }
        if !status_parts.is_empty() {
          write!( &mut help, " - {}", status_parts.join(", ") ).unwrap();
        }
        writeln!( &mut help ).unwrap();

        // Show description, or hint if description is empty
        let desc_text = if !arg.description.is_empty() {
          &arg.description
        } else if !arg.hint.is_empty() {
          &arg.hint
        } else {
          ""
        };

        if !desc_text.is_empty() {
          writeln!( &mut help, "  {}", desc_text ).unwrap();
        }

        if !arg.validation_rules.is_empty() {
          writeln!(
            &mut help,
            "  Rules: [{}]",
            arg.validation_rules.iter().map(|r| format!("{r:?}")).collect::<Vec<_>>().join( ", " )
          ).unwrap();
        }

        writeln!( &mut help ).unwrap();
      }
    }

    // Examples section
    if !command.examples().is_empty()
    {
      writeln!( &mut help, "Examples:" ).unwrap();
      for (idx, example) in command.examples().iter().enumerate()
      {
        writeln!( &mut help, "  {}. {}", idx + 1, example ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    help
  }

  /// Format Level 3: Detailed - Full metadata (old default behavior)
  pub( super ) fn format_detailed( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    if command.show_version_in_help() && self.display_options.show_version
    {
      writeln!
      (
        &mut help,
        "Usage: {} (v{})",
        command.name().as_str(),
        command.version().as_str()
      )
      .unwrap();
    }
    else
    {
      writeln!( &mut help, "Usage: {}", command.name().as_str() ).unwrap();
    }
    if !command.aliases().is_empty() && self.display_options.show_aliases
    {
      writeln!( &mut help, "Aliases: {}", command.aliases().join( ", " ) ).unwrap();
    }
    if !command.tags().is_empty() && self.display_options.show_tags
    {
      writeln!( &mut help, "Tags: {}", command.tags().join( ", " ) ).unwrap();
    }
    writeln!( &mut help, "\n  Hint: {}", command.hint() ).unwrap();
    writeln!( &mut help, "  {}\n", command.description() ).unwrap();
    if self.display_options.show_status
    {
      writeln!( &mut help, "Status: {}", command.status() ).unwrap();
    }

    if !command.arguments().is_empty()
    {
      writeln!( &mut help, "\nArguments:" ).unwrap();
      for arg in command.arguments()
      {
        write!( &mut help, "{}", arg.name ).unwrap();
        write!( &mut help, " (Type: {})", arg.kind ).unwrap();

        let mut status_parts = Vec::new();
        if arg.attributes.optional {
          status_parts.push("Optional");
        }
        if arg.attributes.multiple {
          status_parts.push("Multiple");
        }
        if !status_parts.is_empty() {
          write!( &mut help, " - {}", status_parts.join(", ") ).unwrap();
        }
        writeln!( &mut help ).unwrap();

        if !arg.description.is_empty() {
          writeln!( &mut help, "  {}", arg.description ).unwrap();
          if !arg.hint.is_empty() && arg.hint != arg.description {
            writeln!( &mut help, "  ({})", arg.hint ).unwrap();
          }
        } else if !arg.hint.is_empty() {
          writeln!( &mut help, "  {}", arg.hint ).unwrap();
        }

        if !arg.validation_rules.is_empty() {
          writeln!(
            &mut help,
            "  Rules: [{}]",
            arg.validation_rules.iter().map(|r| format!("{r:?}")).collect::<Vec<_>>().join( ", " )
          ).unwrap();
        }

        writeln!( &mut help ).unwrap();
      }
    }

    help
  }

  /// Format Level 4: Comprehensive - Extensive like runbox
  pub( super ) fn format_comprehensive( &self, command : &crate::CommandDefinition ) -> String
  {
    let mut help = String::new();
    writeln!( &mut help, "{} - {}\n", command.name().as_str(), command.description() ).unwrap();

    // USAGE section
    write!( &mut help, "USAGE:\n  {}", command.name().as_str() ).unwrap();
    if !command.arguments().is_empty()
    {
      for arg in command.arguments()
      {
        if arg.attributes.optional
        {
          write!( &mut help, " [{}::<value>]", arg.name ).unwrap();
        }
        else
        {
          write!( &mut help, " {}::<value>", arg.name ).unwrap();
        }
      }
    }
    writeln!( &mut help, "\n" ).unwrap();

    // DESCRIPTION section with metadata
    writeln!( &mut help, "DESCRIPTION:" ).unwrap();
    writeln!( &mut help, "  {}", command.description() ).unwrap();
    if !command.hint().is_empty() && command.hint() != command.description()
    {
      writeln!( &mut help, "  {}", command.hint() ).unwrap();
    }
    if self.display_options.show_status
    {
      if command.show_version_in_help() && self.display_options.show_version
      {
        writeln!( &mut help, "\n  Status: {} (v{})", command.status(), command.version().as_str() ).unwrap();
      }
      else
      {
        writeln!( &mut help, "\n  Status: {}", command.status() ).unwrap();
      }
    }
    if !command.aliases().is_empty() && self.display_options.show_aliases
    {
      writeln!( &mut help, "  Aliases: {}", command.aliases().join( ", " ) ).unwrap();
    }
    writeln!( &mut help ).unwrap();

    // PARAMETERS section with detailed explanations
    if !command.arguments().is_empty()
    {
      writeln!( &mut help, "PARAMETERS:\n" ).unwrap();
      for arg in command.arguments()
      {
        writeln!( &mut help, "  {}::<value>", arg.name ).unwrap();

        // Description with indentation
        if !arg.description.is_empty()
        {
          writeln!( &mut help, "    {}", arg.description ).unwrap();
        }
        if !arg.hint.is_empty() && arg.hint != arg.description
        {
          writeln!( &mut help, "    {}", arg.hint ).unwrap();
        }

        // Type and attributes
        writeln!( &mut help, "    Type: {}", arg.kind ).unwrap();
        if arg.attributes.optional
        {
          writeln!( &mut help, "    Optional: yes" ).unwrap();
        }
        if arg.attributes.multiple
        {
          writeln!( &mut help, "    Multiple values: yes" ).unwrap();
        }

        // Validation rules
        if !arg.validation_rules.is_empty()
        {
          writeln!( &mut help, "    Validation:" ).unwrap();
          for rule in &arg.validation_rules
          {
            writeln!( &mut help, "      - {rule:?}" ).unwrap();
          }
        }

        writeln!( &mut help ).unwrap();
      }
    }

    // EXAMPLES section
    if !command.examples().is_empty()
    {
      writeln!( &mut help, "EXAMPLES:" ).unwrap();
      for example in command.examples()
      {
        writeln!( &mut help, "  {example}" ).unwrap();
      }
      writeln!( &mut help ).unwrap();
    }

    // TAGS section if present
    if !command.tags().is_empty() && self.display_options.show_tags
    {
      writeln!( &mut help, "TAGS: {}", command.tags().join( ", " ) ).unwrap();
    }

    help
  }
}
