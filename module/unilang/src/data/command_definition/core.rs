//! Core CommandDefinition struct definition and constructors.
//!
//! # Design Rationale
//!
//! **Why private fields?**
//!
//! The old API had public `String` fields that could be mutated freely:
//!
//! ```ignore
//! let mut cmd = CommandDefinition { name: ".test".to_string(), ... };
//! cmd.name = "invalid"; // Compiles! No dot prefix, breaks at registration
//! cmd.namespace = "bad_ns"; // Compiles! Invalid namespace, breaks later
//! ```
//!
//! **Problems with public fields:**
//!
//! 1. **Mutation:** Commands could be invalidated after construction
//! 2. **No validation:** Invalid values compile fine, fail at runtime
//! 3. **Unclear invariants:** No way to know what values are valid
//! 4. **Scattered validation:** Checks in registry, help system, CLI builder...
//!
//! **Benefits of private fields + getters:**
//!
//! - **Immutability:** Once constructed, commands can't be invalidated
//! - **Single validation point:** Construction time only, not scattered everywhere
//! - **Clear contract:** Getters return guaranteed-valid values
//! - **Encapsulation:** Implementation can change without breaking API
//!
//! **Why validated newtypes?**
//!
//! Instead of runtime String validation, we use validated types:
//!
//! - `CommandName`: Guarantees dot prefix at construction
//! - `VersionType`: Guarantees valid semver format
//! - `CommandStatus`: Enum prevents typos like "activ" or "Active"
//!
//! This moves validation from runtime to construction time, catching bugs earlier.
//!
//! **Construction patterns:**
//!
//! 1. **Direct constructor** (simple commands):
//! ```ignore
//! let cmd = CommandDefinition::new(
//!   CommandName::new(".build").unwrap(),
//!   "Build the project".to_string(),
//! );
//! ```
//!
//! 2. **Builder with defaults** (tests, simple cases):
//! ```ignore
//! let cmd = CommandDefinition::former()
//!   .name(".build")
//!   .description("Build the project")
//!   .end(); // Provides defaults for namespace, hint, status, version
//! ```
//!
//! 3. **Builder fully explicit** (production):
//! ```ignore
//! let cmd = CommandDefinition::former()
//!   .name(".build")
//!   .description("Build the project")
//!   .namespace("")
//!   .hint("Build hint")
//!   .status("active")
//!   .version("1.0.0")
//!   .build(); // No defaults, all fields required
//! ```
//!
//! **Trade-off:** More verbose construction, but impossible to create invalid commands.
//! This is a **good trade-off** - bugs caught at compile time > bugs at runtime.
//!
//! # Examples
//! ```rust
//! use unilang::data::{ CommandDefinition, CommandName };
//!
//! // Create a new command with validation
//! let cmd = CommandDefinition::new(
//!   CommandName::new(".build").unwrap(),
//!   "Build the project".to_string(),
//! );
//!
//! // Access via getters (fields are private)
//! assert_eq!(cmd.name().as_str(), ".build");
//! assert_eq!(cmd.description(), "Build the project");
//!
//! // Using builder pattern
//! let cmd = CommandDefinition::former()
//!   .name(".test")
//!   .description("Test command")
//!   .end();
//!
//! assert_eq!(cmd.name().as_str(), ".test");
//! ```

use super::super::validated_types::{ CommandName, VersionType };
use super::super::command_status::{ CommandStatus, construct_full_command_name };
use super::super::argument_types::ArgumentDefinition;
use super::builder::{ CommandDefinitionBuilder, NotSet };

///
/// Type-safe command definition with validated newtypes and private fields.
///
/// This struct implements the "parse don't validate" pattern, making invalid states
/// impossible at compile time. All construction goes through validated builders or
/// constructors that enforce domain rules.
///
#[ derive( Debug, Clone ) ]
pub struct CommandDefinition
{
  /// Validated command name (always starts with '.' prefix)
  pub(in super) name : CommandName,
  /// Brief description of what the command does
  pub(in super) description : String,
  /// List of arguments the command accepts
  pub(in super) arguments : Vec< ArgumentDefinition >,
  /// Optional link to the routine that executes this command
  pub(in super) routine_link : Option< String >,
  /// Namespace for this command (public to allow validation tests to set invalid states).
  ///
  /// Kept as `String` rather than `NamespaceType` so that tests can construct out-of-contract
  /// values (e.g. namespace without dot prefix) to verify that the registry correctly rejects
  /// them. Validation is enforced at registration time, not construction time.
  pub namespace : String,
  /// Short hint for the command
  pub(in super) hint : String,
  /// Command status (Active, Deprecated, Experimental, Internal)
  pub(in super) status : CommandStatus,
  /// Validated version string
  pub(in super) version : VersionType,
  /// Tags associated with the command
  pub(in super) tags : Vec< String >,
  /// Aliases for the command
  pub(in super) aliases : Vec< String >,
  /// Permissions required to execute the command
  pub(in super) permissions : Vec< String >,
  /// Indicates if the command is idempotent
  pub(in super) idempotent : bool,
  /// Deprecation message (deprecated - use CommandStatus::Deprecated instead)
  pub(in super) deprecation_message : String,
  /// Suggested HTTP method for Web API modality
  pub(in super) http_method_hint : String,
  /// Usage examples for help text
  pub(in super) examples : Vec< String >,
  /// Whether to automatically generate a .command.help counterpart
  pub(in super) auto_help_enabled : bool,
  /// Category for grouping commands in help output
  pub(in super) category : String,
  /// Short one-line description for brief help listings
  pub(in super) short_desc : String,
  /// Hide this command from brief help listings
  pub(in super) hidden_from_list : bool,
  /// Sort priority within category (lower numbers first)
  pub(in super) priority : i32,
  /// Explicit group membership for related commands
  pub(in super) group : String,
  /// Control whether version is displayed in help output
  pub(in super) show_version_in_help : bool,
}

impl CommandDefinition
{
  /// Creates a new command using the builder pattern.
  ///
  /// This method returns a CommandDefinitionBuilder that provides a fluent API
  /// for constructing commands with compile-time verification of required fields.
  ///
  /// # Returns
  /// * `CommandDefinitionBuilder` - A builder instance for constructing a CommandDefinition
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::CommandDefinition;
  ///
  /// let cmd = CommandDefinition::former()
  ///   .name( ".greet" )
  ///   .description( "Greets the user" )
  ///   .end();
  /// ```
  #[ must_use ]
  pub fn former() -> CommandDefinitionBuilder< NotSet, NotSet, NotSet, NotSet, NotSet, NotSet >
  {
    CommandDefinitionBuilder::new()
  }

  ///
  /// Creates a new command with sensible defaults.
  ///
  /// This is the recommended way to create commands. It requires only the essential
  /// validated fields (name and description) and provides reasonable defaults for
  /// all optional fields.
  ///
  /// # Arguments
  /// * `name` - Validated command name (must start with '.')
  /// * `description` - Brief description of what the command does
  ///
  /// # Returns
  /// * `Self` - A new CommandDefinition with defaults applied
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".greet").unwrap();
  /// let cmd = CommandDefinition::new(name, "Greets the user".to_string());
  ///
  /// assert_eq!(cmd.name().as_str(), ".greet");
  /// assert_eq!(cmd.description(), "Greets the user");
  /// ```
  #[ must_use ]
  pub fn new( name : CommandName, description : String ) -> Self
  {
    Self
    {
      name,
      description,
      arguments : Vec::new(),
      routine_link : None,
      namespace : String::new(),
      hint : String::new(),
      status : CommandStatus::Active,
      version : VersionType::new( "1.0.0" ).expect( "default version valid" ),
      tags : Vec::new(),
      aliases : Vec::new(),
      permissions : Vec::new(),
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(),
      examples : Vec::new(),
      auto_help_enabled : true,
      category : String::new(),
      short_desc : String::new(),
      hidden_from_list : false,
      priority : 0,
      group : String::new(),
      show_version_in_help : true,
    }
  }

  // ===================================================================
  // Helper Methods (ported from CommandDefinition)
  // ===================================================================

  ///
  /// Returns true if this command should automatically generate a help counterpart.
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".test").unwrap();
  /// let cmd = CommandDefinition::new(name, "Test".to_string())
  ///   .with_auto_help(true);
  ///
  /// assert!(cmd.has_auto_help());
  /// ```
  #[ must_use ]
  pub fn has_auto_help( &self ) -> bool
  {
    self.auto_help_enabled
  }

  ///
  /// Constructs the full command name from namespace and name components.
  ///
  /// # Returns
  /// * `String` - The fully qualified command name with dot prefix
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// // Simple command (no namespace)
  /// let name = CommandName::new(".help").unwrap();
  /// let cmd1 = CommandDefinition::new(name, "Help".to_string());
  /// assert_eq!(cmd1.full_name(), ".help");
  ///
  /// // Namespaced command
  /// let name2 = CommandName::new(".list").unwrap();
  /// let cmd2 = CommandDefinition::new(name2, "List".to_string())
  ///   .with_namespace(".session".to_string());
  /// assert_eq!(cmd2.full_name(), ".session.list");
  /// ```
  #[ must_use ]
  pub fn full_name( &self ) -> String
  {
    construct_full_command_name( self.namespace.as_str(), self.name.as_str() )
  }

  ///
  /// Generates a corresponding help command definition for this command.
  ///
  /// # Returns
  /// * `CommandDefinition` - A new command definition for the help counterpart
  ///
  /// # Examples
  /// ```rust
  /// use unilang::data::{ CommandDefinition, CommandName };
  ///
  /// let name = CommandName::new(".example").unwrap();
  /// let cmd = CommandDefinition::new(name, "Example".to_string());
  ///
  /// let help_cmd = cmd.generate_help_command();
  /// assert_eq!(help_cmd.name().as_str(), ".example.help");
  /// assert!(help_cmd.description().contains(".example"));
  /// ```
  #[ must_use ]
  pub fn generate_help_command( &self ) -> CommandDefinition
  {
    let help_name = CommandName::new( format!( "{}.help", self.name.as_str() ) )
      .expect( "help command name should be valid" );

    CommandDefinition
    {
      name : help_name,
      namespace : self.namespace.clone(),
      description : format!( "Display help information for the '{}' command", self.name.as_str() ),
      hint : format!( "Help for {}", self.name.as_str() ),
      status : CommandStatus::Active,
      version : self.version.clone(),
      arguments : vec![], // Help commands typically take no arguments
      routine_link : None, // Will be set during registration
      tags : vec![ "help".to_string(), "documentation".to_string() ],
      aliases : vec![ format!( "{}.h", self.name.as_str() ) ],
      permissions : vec![], // Help commands accessible to all
      idempotent : true, // Help commands always idempotent
      deprecation_message : String::new(),
      http_method_hint : "GET".to_string(), // Help is read-only
      examples : vec![
        format!( "{}.help", self.name.as_str() ),
        format!( "{} ??", self.name.as_str() )
      ],
      auto_help_enabled : false, // Prevent recursive help generation
      category : "help".to_string(),
      short_desc : format!( "Help for {}", self.name.as_str() ),
      hidden_from_list : true, // Hide .help variants from brief listings
      priority : 999, // Low priority (shown last if visible)
      group : String::new(),
      show_version_in_help : true, // Inherit default behavior
    }
  }
}
