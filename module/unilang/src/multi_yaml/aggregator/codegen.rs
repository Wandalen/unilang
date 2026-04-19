//! Static code generation for compile-time command registry output.

#[ allow( unused_imports ) ]
use crate::*;
use super::core::MultiYamlAggregator;
use std::path::PathBuf;
use std::fs;

impl MultiYamlAggregator
{
  /// Generate a string array constant
  fn generate_string_array( items : &[ String ], const_name : &str ) -> String
  {
    let mut content = String::new();
    content.push_str( &format!( "const {}: &[&str] = &[", const_name ) );
    for item in items
    {
      content.push_str( &format!( "\"{}\", ", Self::escape_string( item ) ) );
    }
    content.push_str( "];\n" );
    content
  }

  /// Generate argument definition for a single argument
  fn generate_argument_definition(
    arg : &ArgumentDefinition,
    const_name_base : &str,
    arg_idx : usize,
  ) -> String
  {
    let mut content = String::new();
    let arg_const_name = format!( "{}_{}_ARG", const_name_base, arg_idx );
    let attrs_const_name = format!( "{}_{}_ATTRS", const_name_base, arg_idx );
    let aliases_const_name = format!( "{}_{}_ALIASES", const_name_base, arg_idx );
    let tags_const_name = format!( "{}_{}_TAGS", const_name_base, arg_idx );

    // Generate aliases and tags arrays
    if !arg.aliases.is_empty()
    {
      content.push_str( &Self::generate_string_array( &arg.aliases, &aliases_const_name ) );
    }
    if !arg.tags.is_empty()
    {
      content.push_str( &Self::generate_string_array( &arg.tags, &tags_const_name ) );
    }

    // Generate attributes
    content.push_str( &format!( "const {}: StaticArgumentAttributes = StaticArgumentAttributes::new()\n", attrs_const_name ) );
    content.push_str( &format!( "  .with_optional( {} )\n", arg.attributes.optional ) );
    content.push_str( &format!( "  .with_sensitive( {} )\n", arg.attributes.sensitive ) );
    content.push_str( &format!( "  .with_interactive( {} )\n", arg.attributes.interactive ) );
    content.push_str( &format!( "  .with_multiple( {} )", arg.attributes.multiple ) );
    if let Some( ref default ) = arg.attributes.default
    {
      content.push_str( &format!( "\n  .with_default( \"{}\" )", Self::escape_string( default ) ) );
    }
    content.push_str( ";\n\n" );

    // Generate kind
    let kind_str = Self::generate_kind_string( &arg.kind, const_name_base, arg_idx, &mut content );

    // Generate argument definition
    content.push_str( &format!(
      "const {}: StaticArgumentDefinition = StaticArgumentDefinition::new(\n",
      arg_const_name
    ) );
    content.push_str( &format!( "  \"{}\",\n", Self::escape_string( &arg.name ) ) );
    content.push_str( &format!( "  {},\n", kind_str ) );
    content.push_str( &format!( "  \"{}\",\n", Self::escape_string( &arg.description ) ) );
    content.push_str( ")\n" );
    content.push_str( &format!( ".with_hint( \"{}\" )\n", Self::escape_string( &arg.hint ) ) );
    content.push_str( &format!( ".with_attributes( {} )", attrs_const_name ) );

    if !arg.aliases.is_empty()
    {
      content.push_str( &format!( "\n.with_aliases( {} )", aliases_const_name ) );
    }
    if !arg.tags.is_empty()
    {
      content.push_str( &format!( "\n.with_tags( {} )", tags_const_name ) );
    }
    content.push_str( ";\n\n" );

    content
  }

  /// Generate kind string representation with any additional const definitions
  fn generate_kind_string(
    kind : &Kind,
    const_name_base : &str,
    arg_idx : usize,
    content : &mut String,
  ) -> String
  {
    match kind
    {
      Kind::String => "StaticKind::String".to_string(),
      Kind::Integer => "StaticKind::Integer".to_string(),
      Kind::Float => "StaticKind::Float".to_string(),
      Kind::Boolean => "StaticKind::Boolean".to_string(),
      Kind::Path => "StaticKind::Path".to_string(),
      Kind::File => "StaticKind::File".to_string(),
      Kind::Directory => "StaticKind::Directory".to_string(),
      Kind::Url => "StaticKind::Url".to_string(),
      Kind::DateTime => "StaticKind::DateTime".to_string(),
      Kind::Pattern => "StaticKind::Pattern".to_string(),
      Kind::JsonString => "StaticKind::JsonString".to_string(),
      Kind::Enum( ref values ) =>
      {
        let enum_values_name = format!( "{}_{}_ENUM_VALUES", const_name_base, arg_idx );
        content.push_str( &Self::generate_string_array( values, &enum_values_name ) );
        format!( "StaticKind::Enum( &{} )", enum_values_name )
      }
      Kind::List( _, delim ) =>
      {
        let delim_str = match delim
        {
          Some( c ) => format!( "Some( '{}' )", c ),
          None => "None".to_string(),
        };
        format!( "StaticKind::List( &StaticKind::String, {} )", delim_str )
      }
      Kind::Map( _, _, entry_delim, kv_delim ) =>
      {
        let entry_delim_str = match entry_delim
        {
          Some( c ) => format!( "Some( '{}' )", c ),
          None => "None".to_string(),
        };
        let kv_delim_str = match kv_delim
        {
          Some( c ) => format!( "Some( '{}' )", c ),
          None => "None".to_string(),
        };
        format!( "StaticKind::Map( &StaticKind::String, &StaticKind::String, {}, {} )", entry_delim_str, kv_delim_str )
      }
      Kind::Object => "StaticKind::JsonString".to_string(),
    }
  }

  /// Generate command definition body with all its fields
  fn generate_command_definition_body(
    cmd : &CommandDefinition,
    const_name_base : &str,
    tags_const_name : &str,
    aliases_const_name : &str,
    permissions_const_name : &str,
    examples_const_name : &str,
  ) -> String
  {
    let mut content = String::new();

    content.push_str( &format!( "  name: \"{}\",\n", Self::escape_string( cmd.name().as_str() ) ) );
    content.push_str( &format!( "  namespace: \"{}\",\n", Self::escape_string( cmd.namespace() ) ) );
    content.push_str( &format!( "  description: \"{}\",\n", Self::escape_string( cmd.description() ) ) );

    // Arguments
    if cmd.arguments().is_empty()
    {
      content.push_str( "  arguments: &[],\n" );
    }
    else
    {
      content.push_str( &format!( "  arguments: {}_ARGS,\n", const_name_base ) );
    }

    content.push_str( "  routine_link: None,\n" );
    content.push_str( &format!( "  hint: \"{}\",\n", Self::escape_string( cmd.hint() ) ) );

    // Format status enum as string
    let status_str = match cmd.status()
    {
      crate::data::CommandStatus::Active => "active",
      crate::data::CommandStatus::Deprecated { .. } => "deprecated",
      crate::data::CommandStatus::Experimental => "experimental",
      crate::data::CommandStatus::Internal => "internal",
    };
    content.push_str( &format!( "  status: \"{}\",\n", Self::escape_string( status_str ) ) );
    content.push_str( &format!( "  version: \"{}\",\n", Self::escape_string( cmd.version().as_str() ) ) );

    // Arrays
    if cmd.tags().is_empty()
    {
      content.push_str( "  tags: &[],\n" );
    }
    else
    {
      content.push_str( &format!( "  tags: {},\n", tags_const_name ) );
    }

    if cmd.aliases().is_empty()
    {
      content.push_str( "  aliases: &[],\n" );
    }
    else
    {
      content.push_str( &format!( "  aliases: {},\n", aliases_const_name ) );
    }

    if cmd.permissions().is_empty()
    {
      content.push_str( "  permissions: &[],\n" );
    }
    else
    {
      content.push_str( &format!( "  permissions: {},\n", permissions_const_name ) );
    }

    content.push_str( &format!( "  idempotent: {},\n", cmd.idempotent() ) );
    content.push_str( &format!( "  deprecation_message: \"{}\",\n", Self::escape_string( cmd.deprecation_message() ) ) );
    content.push_str( &format!( "  http_method_hint: \"{}\",\n", Self::escape_string( cmd.http_method_hint() ) ) );

    if cmd.examples().is_empty()
    {
      content.push_str( "  examples: &[],\n" );
    }
    else
    {
      content.push_str( &format!( "  examples: {},\n", examples_const_name ) );
    }

    // Fix(issue-088): Include auto_help_enabled field
    // Root cause: MultiYamlAggregator was not updated when StaticCommandDefinition struct gained this field
    // Pitfall: When adding fields to StaticCommandDefinition, ALL code generators must be updated:
    //   1. build.rs (direct PHF generation) - FIXED
    //   2. MultiYamlAggregator::generate_command_definition_body() - FIXED HERE
    content.push_str( &format!( "  auto_help_enabled: {},\n", cmd.auto_help_enabled() ) );

    // Fix(issue-089): Include category field
    // Root cause: MultiYamlAggregator wasnt updated when StaticCommandDefinition gained category field
    // Pitfall: Same as issue-088. When adding fields to StaticCommandDefinition, ALL code generators
    // must be updated, including this method and any direct PHF generation in build.rs files
    content.push_str( &format!( "  category: \"{}\",\n", Self::escape_string( cmd.category() ) ) );

    // show_version_in_help: controls whether version is displayed in help output
    content.push_str( &format!( "  show_version_in_help: {},\n", cmd.show_version_in_help() ) );

    content
  }

  /// Generate static command registry source code for build-time compilation.
  ///
  /// Returns Rust source code that defines a compile-time optimized command registry.
  /// This code should be written to a `.rs` file and included in your build output.
  ///
  /// # Performance
  /// Commands generated this way have **zero runtime overhead** for lookups (O(1) const-time).
  ///
  /// # Returns
  /// Rust source code string ready to be written to a file
  ///
  /// # Example
  /// ```no_run
  /// use unilang::multi_yaml::MultiYamlAggregator;
  /// use unilang::multi_yaml::AggregationConfig;
  ///
  /// let config = AggregationConfig::default();
  /// let aggregator = MultiYamlAggregator::new(config);
  /// let source_code = aggregator.generate_static_registry_source();
  /// // Write source_code to a .rs file in your build output
  /// ```
  pub fn generate_static_registry_source( &self ) -> String
  {
    let mut source_code = String::new();
    // Fix(dev-001): use {self, Map} not {phf_map, Map} so downstream sees qualified phf::phf_map!
    // Root cause: importing phf_map by name forced bare invocation that expands to ::phf:: absolute
    //   paths, requiring every downstream crate to add phf as a direct Cargo.toml dependency
    // Pitfall: phf::phf_map! works via re-export only with phf >= 0.11 ($crate:: hygiene)
    source_code.push_str( "use unilang::phf::{self, Map};\n" );
    source_code.push_str( "use unilang::static_data::{StaticCommandDefinition, StaticArgumentDefinition, StaticArgumentAttributes, StaticKind};\n\n" );

    // Generate each command
    for ( cmd_name, cmd ) in &self.commands
    {
      let const_name_base = cmd_name.replace( [ '.', '-' ], "_" ).to_uppercase();

      // Generate argument definitions
      for ( arg_idx, arg ) in cmd.arguments().iter().enumerate()
      {
        source_code.push_str( &Self::generate_argument_definition( arg, &const_name_base, arg_idx ) );
      }

      // Generate arguments array
      if !cmd.arguments().is_empty()
      {
        let args_array_name = format!( "{}_ARGS", const_name_base );
        source_code.push_str( &format!( "const {}: &[StaticArgumentDefinition] = &[", args_array_name ) );
        for arg_idx in 0..cmd.arguments().len()
        {
          source_code.push_str( &format!( "{}_{}_ARG, ", const_name_base, arg_idx ) );
        }
        source_code.push_str( "];\n\n" );
      }

      // Generate command-level arrays
      let tags_const_name = format!( "{}_TAGS", const_name_base );
      let aliases_const_name = format!( "{}_ALIASES", const_name_base );
      let permissions_const_name = format!( "{}_PERMISSIONS", const_name_base );
      let examples_const_name = format!( "{}_EXAMPLES", const_name_base );

      if !cmd.tags().is_empty()
      {
        source_code.push_str( &Self::generate_string_array( cmd.tags(), &tags_const_name ) );
      }
      if !cmd.aliases().is_empty()
      {
        source_code.push_str( &Self::generate_string_array( cmd.aliases(), &aliases_const_name ) );
      }
      if !cmd.permissions().is_empty()
      {
        source_code.push_str( &Self::generate_string_array( cmd.permissions(), &permissions_const_name ) );
      }
      if !cmd.examples().is_empty()
      {
        source_code.push_str( &Self::generate_string_array( cmd.examples(), &examples_const_name ) );
      }

      // Generate command definition
      let const_name = format!( "{}_CMD", const_name_base );
      source_code.push_str( &format!(
        "\nstatic {}: StaticCommandDefinition = StaticCommandDefinition {{\n",
        const_name
      ) );
      source_code.push_str( &Self::generate_command_definition_body(
        cmd,
        &const_name_base,
        &tags_const_name,
        &aliases_const_name,
        &permissions_const_name,
        &examples_const_name,
      ) );
      source_code.push_str( "};\n\n" );
    }

    // Fix(issue-001): Use phf_codegen struct-literal generation — no phf_map! macro.
    // Root cause: phf_map! proc-macro expands to ::phf::Map absolute paths at downstream
    //   compile time, forcing every consumer to list phf as a direct Cargo.toml dep even
    //   though unilang already re-exports phf via pub use phf. Qualifying as phf::phf_map!
    //   does not help — the macro's internal expansion hardcodes ::phf:: regardless.
    // Pitfall: If phf_codegen dep is removed or phf_path changes from "phf", downstream
    //   builds break; keep phf_codegen in the static_registry feature and path as "phf".

    // Generate optimized static map using phf_codegen struct literal (no macro invocation)
    #[ cfg( feature = "static_registry" ) ]
    {
      let mut sorted_names: Vec< String > = self.commands.keys().cloned().collect();
      sorted_names.sort();  // deterministic output order

      let mut map_builder = phf_codegen::Map::new();
      // phf_path("phf") → `phf::Map { ... }` where phf = use unilang::phf::{self, Map}
      map_builder.phf_path( "phf" );
      for cmd_name in &sorted_names
      {
        let const_name = format!(
          "{}_CMD",
          cmd_name.replace( [ '.', '-' ], "_" ).to_uppercase()
        );
        map_builder.entry( cmd_name.as_str(), format!( "&{}", const_name ) );
      }

      source_code.push_str( "pub static AGGREGATED_COMMANDS: Map<&'static str, &'static StaticCommandDefinition> = " );
      source_code.push_str( &format!( "{}", map_builder.build() ) );
      source_code.push_str( ";\n" );
    }
    #[ cfg( not( feature = "static_registry" ) ) ]
    {
      source_code.push_str( "// static_registry feature required for AGGREGATED_COMMANDS\n" );
    }

    source_code
  }

  /// Escape strings for Rust code generation
  fn escape_string( s: &str ) -> String
  {
    s.replace( '\\', "\\\\" )
     .replace( '"', "\\\"" )
     .replace( '\n', "\\n" )
     .replace( '\r', "\\r" )
     .replace( '\t', "\\t" )
  }

  /// Write static command registry to a build output file.
  ///
  /// Generates optimized compile-time command definitions and writes them
  /// to the specified file path. This file should be included in your
  /// build.rs output directory.
  ///
  /// # Arguments
  /// * `output_path` - Path where the generated `.rs` file will be written
  ///
  /// # Example
  /// ```no_run
  /// use unilang::multi_yaml::MultiYamlAggregator;
  /// use unilang::multi_yaml::AggregationConfig;
  /// use std::path::PathBuf;
  ///
  /// # fn example() -> Result<(), unilang::Error> {
  /// let config = AggregationConfig::default();
  /// let aggregator = MultiYamlAggregator::new(config);
  ///
  /// let out_dir = std::env::var("OUT_DIR").unwrap();
  /// let output = PathBuf::from(out_dir).join("static_commands.rs");
  /// aggregator.write_static_registry(&output)?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn write_static_registry( &self, output_path: &PathBuf ) -> Result< (), Error >
  {
    let source_code = self.generate_static_registry_source();
    fs::write( output_path, source_code )
      .map_err( |e| Error::Registration( format!( "Failed to write static registry file: {}", e ) ) )
  }

  /// Generate build.rs content for build-time integration
  pub fn generate_build_rs( &self ) -> String
  {
    let mut build_rs = String::new();

    build_rs.push_str( "//! Build script for multi-YAML command aggregation\n" );
    build_rs.push_str( "//! This file is auto-generated - do not edit manually\n\n" );

    build_rs.push_str( "fn main() {\n" );
    build_rs.push_str( "  println!(\"cargo:rerun-if-changed=build.rs\");\n\n" );

    // Add rerun-if-changed for all YAML files
    for module in &self.config.modules
    {
      if module.enabled
      {
        let yaml_path = self.config.base_dir.join( &module.yaml_path );
        build_rs.push_str( &format!(
          "  println!(\"cargo:rerun-if-changed={}\");\n",
          yaml_path.display()
        ) );
      }
    }

    build_rs.push_str( "\n  // Add feature detection\n" );
    build_rs.push_str( "  #[cfg(feature = \"multi_yaml\")]\n" );
    build_rs.push_str( "  {\n" );

    build_rs.push_str( "    // Generate aggregated commands at build time\n" );
    build_rs.push_str( "    let mut aggregator = unilang::multi_yaml::MultiYamlAggregator::from_cargo_metadata(\n" );
    build_rs.push_str( "      &std::path::PathBuf::from(\"Cargo.toml\")\n" );
    build_rs.push_str( "    ).expect(\"Failed to create aggregator\");\n\n" );

    build_rs.push_str( "    aggregator.aggregate().expect(\"Failed to aggregate YAML files\");\n\n" );

    build_rs.push_str( "    // Generate static registry file\n" );
    build_rs.push_str( "    let output_path = std::path::PathBuf::from(\n" );
    build_rs.push_str( "      std::env::var(\"OUT_DIR\").expect(\"OUT_DIR not set\")\n" );
    build_rs.push_str( "    ).join(\"generated_commands.rs\");\n\n" );

    build_rs.push_str( "    aggregator.write_static_registry(&output_path)\n" );
    build_rs.push_str( "      .expect(\"Failed to write static registry\");\n" );

    build_rs.push_str( "  }\n" );
    build_rs.push_str( "}\n" );

    build_rs
  }
}
