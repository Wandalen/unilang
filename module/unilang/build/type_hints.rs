use serde_yaml_ng::Value;

/// Analyzes argument definitions for potential type issues
pub struct TypeAnalyzer
{
  suppress_warnings : bool,
}

impl TypeAnalyzer
{
  pub fn new() -> Self
  {
    Self
    {
      suppress_warnings : std::env::var( "UNILANG_SUPPRESS_TYPE_HINTS" )
        .map( | v | v == "1" )
        .unwrap_or( false ),
    }
  }

  pub fn analyze_argument( &self, arg : &Value ) -> Vec< TypeHint >
  {
    let mut hints = Vec::new();

    let name = arg[ "name" ].as_str().unwrap_or( "" );
    let kind = arg[ "kind" ].as_str().unwrap_or( "" );

    let default = arg[ "attributes" ][ "default" ].as_str()
      .or_else( || arg[ "default" ].as_str() );

    let suppress = arg[ "attributes" ][ "suppress_type_hint" ]
      .as_bool()
      .unwrap_or( false );

    if self.suppress_warnings || suppress
    {
      return hints;
    }

    // Check 1: Boolean-like default with String kind
    if kind == "String"
    {
      if let Some( def ) = default
      {
        if ( def == "true" || def == "false" ) && Self::context_suggests_boolean( name, arg )
        {
          hints.push( TypeHint::BooleanAsString
          {
            argument_name : name.to_string(),
            default_value : def.to_string(),
          } );
        }
      }
    }

    // Check 2: Integer-like default with String kind
    if kind == "String"
    {
      if let Some( def ) = default
      {
        if def.parse::< i64 >().is_ok() &&
           !def.starts_with( '0' ) &&
           !def.contains( '.' ) &&
           !def.is_empty() &&
           def.chars().all( | c | c.is_ascii_digit() ) &&
           Self::context_suggests_integer( name )
        {
          hints.push( TypeHint::IntegerAsString
          {
            argument_name : name.to_string(),
            default_value : def.to_string(),
          } );
        }
      }
    }

    hints
  }

  fn context_suggests_boolean( name : &str, arg : &Value ) -> bool
  {
    let name_lower = name.to_lowercase();
    let desc = arg[ "description" ].as_str().unwrap_or( "" );

    let boolean_keywords = [
      "enable", "disable", "flag", "is_", "has_", "can_",
      "should_", "dry_run", "dry-run", "force", "quiet", "verbose",
      "clone", "parallel", "recursive", "skip", "ignore",
    ];

    let name_suggests = boolean_keywords.iter()
      .any( | kw | name_lower.contains( kw ) );

    let desc_suggests = desc.contains( "true/false" ) ||
                       desc.contains( "(true|false)" ) ||
                       desc.contains( "true or false" );

    name_suggests || desc_suggests
  }

  fn context_suggests_integer( name : &str ) -> bool
  {
    let name_lower = name.to_lowercase();

    let integer_keywords = [
      "count", "limit", "max", "min", "size", "length",
      "verbosity", "level", "timeout", "retry", "retries",
      "attempts", "depth", "width", "height", "num",
    ];

    integer_keywords.iter().any( | kw | name_lower.contains( kw ) )
  }
}

#[derive(Debug, Clone)]
pub enum TypeHint
{
  BooleanAsString
  {
    argument_name : String,
    default_value : String,
  },
  IntegerAsString
  {
    argument_name : String,
    default_value : String,
  },
}

pub struct HintGenerator;

impl HintGenerator
{
  pub fn generate_warning( hint : &TypeHint ) -> String
  {
    match hint
    {
      TypeHint::BooleanAsString { argument_name, default_value } =>
      {
        format!(
          "💡 Type Hint: Argument '{argument_name}' might be better as Boolean kind\n\
           \n\
           Current:\n\
           - name: \"{argument_name}\"\n\
             kind: \"String\"\n\
             attributes:\n\
               default: \"{default_value}\"  # String literal\n\
           \n\
           Suggestion:\n\
           - name: \"{argument_name}\"\n\
             kind: \"Boolean\"\n\
             attributes:\n\
               default: {default_value}  # Boolean value (no quotes)\n\
           \n\
           Benefits:\n\
           - Automatic validation (rejects invalid values like 'yes', '1')\n\
           - Type-safe: cmd.get_boolean(\"{argument_name}\") instead of manual parsing\n\
           - Better error messages for users\n\
           \n\
           If intentional (e.g., code template): Add suppress_type_hint: true\n\
           To suppress all hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n"
        )
      },

      TypeHint::IntegerAsString { argument_name, default_value } =>
      {
        format!(
          "💡 Type Hint: Argument '{argument_name}' might be better as Integer kind\n\
           \n\
           Current:\n\
           - name: \"{argument_name}\"\n\
             kind: \"String\"\n\
             attributes:\n\
               default: \"{default_value}\"  # String literal\n\
           \n\
           Suggestion:\n\
           - name: \"{argument_name}\"\n\
             kind: \"Integer\"\n\
             attributes:\n\
               default: {default_value}  # Integer value (no quotes)\n\
             validation_rules:\n\
               - Min: 0  # Add appropriate range\n\
               - Max: 100\n\
           \n\
           Benefits:\n\
           - Automatic range validation\n\
           - Type-safe: cmd.get_integer(\"{argument_name}\") instead of manual parsing\n\
           \n\
           If intentional (version/ID/code): Add suppress_type_hint: true\n\
           To suppress all hints: export UNILANG_SUPPRESS_TYPE_HINTS=1\n"
        )
      },
    }
  }

  pub fn emit_hints( hints : Vec< TypeHint > )
  {
    if hints.is_empty()
    {
      return;
    }

    eprintln!();
    eprintln!( "{}", "=".repeat( 80 ) );
    eprintln!( "📋 Unilang Type Hints ({} suggestion{})",
      hints.len(),
      if hints.len() == 1 { "" } else { "s" }
    );
    eprintln!( "{}", "=".repeat( 80 ) );
    eprintln!();

    for hint in hints
    {
      eprintln!( "{}", Self::generate_warning( &hint ) );
      eprintln!( "{}", "-".repeat( 80 ) );
      eprintln!();
    }

    eprintln!(
      "ℹ️  Type hints help you choose appropriate argument types.\n\
       These are suggestions, not errors. Your build continues normally.\n"
    );
  }
}
