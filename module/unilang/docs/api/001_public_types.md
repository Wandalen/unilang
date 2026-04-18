# API: Public Types

### Scope

- **Purpose:** Document the public data structures, API reference, and environment variables exposed to integrators
- **Responsibility:** Public types, finalized data models, environment variable reference
- **In Scope:** Public structs, enums, environment variables, API method signatures, compatibility guarantees
- **Out of Scope:** Internal implementation details, vision/scope, migration history

### Abstract

The `unilang` public API surface provides integrators with: `CommandDefinition`, `ArgumentDefinition`, `CommandRegistry`, `Pipeline`, `Value`, `Kind`, `StaticCommandDefinition`, and related types for building multi-modal command-line utilities.

### Compatibility Guarantees

- Semver: Breaking changes increment major version
- `StaticCommandDefinition` fields are additive — new fields have defaults (no breaking change)
- Environment variable names are stable after v1.0

### Core Data Structures

The public API **must** include the following data structures. See `src/data.rs` for the authoritative source definitions.

- `CommandDefinition`: Defines a command's metadata, including the `auto_help_enabled: bool` field for help convention support.
- `ArgumentDefinition`: Defines an argument's metadata.
- `ArgumentAttributes`: Defines behavioral flags for an argument.
- `Kind`: Defines the data type of an argument.
- `ValidationRule`: Defines a validation constraint for an argument.
- `OutputData`: Standardized structure for successful command output.
- `ErrorData`: Standardized structure for command failure information.
- `StaticCommandMap`: Opaque wrapper for compile-time optimized command maps.
- `StaticCommandDefinition`: Const-compatible version of CommandDefinition for static storage.
- `StaticArgumentDefinition`: Const-compatible version of ArgumentDefinition for static storage.

### Phase 2 Type-Safe Redesign

**Status:** ✅ **IMPLEMENTED** (v3.1.0)

The `CommandDefinition` structure underwent a complete type-safe redesign implementing the "parse don't validate" pattern. Invalid states are now impossible to represent at compile time.

**Design Philosophy:** "Invalid States Should Be Impossible". The old API allowed commands to be constructed in invalid states that only failed at runtime during registration. The new API catches errors at construction time, moving bugs from runtime to compile time.

### Key Changes

**Private Fields with Getter Methods**
- All `CommandDefinition` fields are now private
- Access via getter methods only (e.g., `cmd.name()` instead of `cmd.name`)
- Prevents mutation after construction; guarantees immutability and validity

**Validated Newtypes**
- `CommandName`: Wrapper type guaranteeing dot prefix (e.g., `.build`)
- `NamespaceType`: Wrapper type guaranteeing valid namespace (empty or dot-prefixed)
- `VersionType`: Wrapper type guaranteeing non-empty version string
- `CommandStatus`: Enum (`Active`, `Deprecated`, `Experimental`, `Internal`) replacing String

**Type-State Builder Pattern**
- `CommandDefinition::former()` returns a type-state builder
- `end()` method: Requires only `name` + `description`, provides defaults (ergonomic)
- `build()` method: Requires ALL fields explicitly set (explicit for production)
- Compile-time enforcement of required fields via phantom types

### Construction Patterns

```rust
// Pattern 1: Direct constructor (simple commands)
let name = CommandName::new(".build").unwrap();
let cmd = CommandDefinition::new(name, "Build the project".to_string());

// Pattern 2: Builder with defaults (tests, simple cases)
let cmd = CommandDefinition::former()
  .name(".build")
  .description("Build the project")
  .end(); // Provides defaults: namespace="", status=Active, version="1.0.0"

// Pattern 3: Builder fully explicit (production)
let cmd = CommandDefinition::former()
  .name(".build")
  .description("Build the project")
  .namespace("")
  .hint("Build hint")
  .status("active")
  .version("1.0.0")
  .build(); // No defaults, all fields required
```

### Validated Types API

```rust
// CommandName - guarantees dot prefix
pub struct CommandName(String);
impl CommandName {
  pub fn new(name: impl Into<String>) -> Result<Self, Error>;
  pub fn as_str(&self) -> &str;
  pub fn into_inner(self) -> String;
}

// NamespaceType - guarantees valid namespace
pub struct NamespaceType(String);
impl NamespaceType {
  pub fn new(namespace: impl Into<String>) -> Result<Self, Error>;
  pub fn as_str(&self) -> &str;
  pub fn is_root(&self) -> bool; // Returns true if namespace is empty
}

// VersionType - guarantees non-empty version
pub struct VersionType(String);
impl VersionType {
  pub fn new(version: impl Into<String>) -> Result<Self, Error>;
  pub fn as_str(&self) -> &str;
}

// CommandStatus - enum eliminates typos
pub enum CommandStatus {
  Active,
  Deprecated { reason: String, since: Option<String>, replacement: Option<String> },
  Experimental,
  Internal,
}
```

### Builder API

```rust
// Type-state builder with phantom types
impl CommandDefinition {
  pub fn former() -> CommandDefinitionBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet>;
}

impl CommandDefinitionBuilder<Set, Set, Namespace, Hint, Status, Version> {
  // Available when name + description are set (others optional)
  pub fn end(self) -> CommandDefinition;
}

impl CommandDefinitionBuilder<Set, Set, Set, Set, Set, Set> {
  // Only available when ALL fields are set
  pub fn build(self) -> CommandDefinition;
}
```

### Getter Methods

All `CommandDefinition` fields have getter methods:

```rust
impl CommandDefinition {
  pub fn name(&self) -> &CommandName;
  pub fn description(&self) -> &str;
  pub fn namespace(&self) -> &str;
  pub fn status(&self) -> &CommandStatus;
  pub fn version(&self) -> &VersionType;
  pub fn auto_help_enabled(&self) -> bool;
  // ... all other fields

  pub fn full_name(&self) -> String; // Returns namespace + name
  pub fn generate_help_command(&self) -> CommandDefinition;
}
```

### Migration Impact

- **BREAKING:** All `CommandDefinition` construction must use builder or `new()` method
- **BREAKING:** Field access changed from direct (`cmd.name`) to getters (`cmd.name()`)
- **BREAKING:** Invalid commands now panic at construction, not registration
- **BREAKING:** Status strings replaced with `CommandStatus` enum
- **BENEFIT:** Bugs caught at compile time instead of runtime
- **BENEFIT:** Type system documents valid states
- **BENEFIT:** IDE autocomplete guides correct usage

See `src/data.rs` module documentation for comprehensive design rationale.

### Finalized Internal Data Models

*The definitive, as-built schema for all databases, data structures, and objects used internally by the system.*

### CommandDefinition Structure (as of 2025_09_16)

```rust
pub struct CommandDefinition {
    pub name: String,                    // Required dot-prefixed command name
    pub namespace: String,               // Hierarchical namespace organization
    pub description: String,             // Human-readable command description
    pub arguments: Vec<ArgumentDefinition>, // Command parameters
    pub routine_link: Option<String>,    // Link to execution routine
    pub hint: String,                   // Short description for command lists
    pub status: String,                 // Command stability status
    pub version: String,                // Command version
    pub tags: Vec<String>,              // Categorization tags
    pub aliases: Vec<String>,           // Alternative command names
    pub permissions: Vec<String>,       // Access control permissions
    pub idempotent: bool,              // Whether command is side-effect free
    pub deprecation_message: String,    // Deprecation notice if applicable
    pub http_method_hint: String,       // HTTP method suggestion for web API
    pub examples: Vec<String>,          // Usage examples
    pub auto_help_enabled: bool,        // Controls automatic .command.help generation
}
```

### OutputData Structure (as of 2025_10_19)

```rust
pub struct OutputData {
    pub content : String,                  // The actual output content
    pub format : String,                   // Output format identifier (e.g., "text", "json", "xml")
    pub execution_time_ms : Option< u64 >, // Execution time in milliseconds (automatically populated by Interpreter)
}
```

**Performance Monitoring Implementation:**
The `execution_time_ms` field provides automatic performance monitoring for all command executions:
- **Automatic Capture:** The `Interpreter` automatically measures execution time using `std::time::Instant` and populates this field
- **Zero Developer Overhead:** Command routines dont need to track timing manually
- **Backward Compatible:** Optional field design ensures existing code continues to work
- **Precision:** Millisecond-level precision suitable for performance analysis and optimization
- **Consistency:** All commands use identical timing methodology for fair comparison

See `src/data.rs` for the complete and authoritative structure definitions.

### API Reference: CommandRegistry Methods

**CommandRegistry Methods:**
- `register_with_auto_help(&mut self, command: CommandDefinition, routine: CommandRoutine)` — Registers a command with automatic help command generation (mandatory for all commands).
- `get_help_for_command(&self, command_name: &str) -> Option<String>` — Retrieves formatted help text for any registered command.

**CommandRegistryBuilder Methods:**
- `builder() -> CommandRegistryBuilder` — Creates a new builder for fluent command registration.
- `command_with_routine(name: &str, description: &str, routine: F) -> Self` — Adds a command with inline routine using fluent builder pattern.
- `build(self) -> CommandRegistry` — Builds and returns the CommandRegistry, ignoring any registration errors (for backward compatibility). **Warning:** Silently ignores registration errors.
- `build_checked(self) -> Result<CommandRegistry, Error>` — Builds and returns the CommandRegistry with proper error propagation. Returns an error if any command failed to register during the build process. **Recommended** for production code to ensure all commands registered successfully.

### API Reference: VerifiedCommand Helper Methods

The following helper methods eliminate boilerplate in command routines (eliminates ~90% of argument extraction code):

*String extraction:*
- `get_string(&self, name: &str) -> Option<&str>` — Extracts optional string argument, returns None if not found or wrong type.
- `require_string(&self, name: &str) -> Result<&str, Error>` — Extracts required string argument, returns error if missing or wrong type.
- `get_string_normalized<'a>(&'a self, name: &str) -> Option<&'a str>` — Extracts optional string, trimming leading/trailing Unicode whitespace. Returns `Some("")` for whitespace-only input. No allocation — borrows from self.
- `require_string_normalized<'a>(&'a self, name: &str) -> Result<&'a str, Error>` — Extracts required string, trimming whitespace. Returns error if argument is missing or wrong type.

*Integer extraction:*
- `get_integer(&self, name: &str) -> Option<i64>` — Extracts optional integer argument.
- `require_integer(&self, name: &str) -> Result<i64, Error>` — Extracts required integer argument.

*Float extraction:*
- `get_float(&self, name: &str) -> Option<f64>` — Extracts optional float argument.
- `require_float(&self, name: &str) -> Result<f64, Error>` — Extracts required float argument.

*Boolean extraction:*
- `get_boolean(&self, name: &str) -> Option<bool>` — Extracts optional boolean argument.
- `require_boolean(&self, name: &str) -> Result<bool, Error>` — Extracts required boolean argument.

*Path extraction:*
- `get_path(&self, name: &str) -> Option<&Path>` — Extracts optional path argument (works with Path, File, Directory variants).
- `require_path(&self, name: &str) -> Result<&Path, Error>` — Extracts required path argument.

*List extraction:*
- `get_list(&self, name: &str) -> Option<&Vec<Value>>` — Extracts optional list argument.
- `require_list(&self, name: &str) -> Result<&Vec<Value>, Error>` — Extracts required list argument.

*Generic helpers:*
- `has_argument(&self, name: &str) -> bool` — Returns true if argument exists (regardless of type).
- `get_value(&self, name: &str) -> Option<&Value>` — Gets raw Value reference for custom handling.

These helpers replace the verbose pattern:
```rust
// OLD (verbose, error-prone):
let name = cmd.arguments.get("name")
  .and_then(|v| if let Value::String(s) = v { Some(s) } else { None })
  .unwrap_or("default");

// NEW (concise, type-safe):
let name = cmd.get_string("name").unwrap_or("default");
```

### API Reference: StaticCommandMap

The `StaticCommandMap` struct is an opaque wrapper that hides compile-time optimization implementation details from the public API.

**Design Requirements:**
- **Opaque Wrapper:** Hides internal optimization types completely — no implementation-specific types in public signatures.
- **Zero Dependencies:** Downstream crates using `StaticCommandMap` do not require internal optimization library dependencies.
- **Zero Overhead:** All wrapper methods are `#[inline]` to ensure the wrapper compiles away with no performance cost.
- **Const Initialization:** Supports `const fn` initialization for compile-time map creation.

**Public API Methods:**
- `get(name: &str) -> Option<&'static StaticCommandDefinition>` — Retrieve command by name (O(1) lookup).
- `contains_key(name: &str) -> bool` — Check if command exists.
- `keys() -> impl Iterator<Item = &&'static str>` — Iterate over command names.
- `entries() -> impl Iterator` — Iterate over (name, definition) pairs.
- `values() -> impl Iterator` — Iterate over command definitions.
- `len() -> usize` — Get number of commands.
- `is_empty() -> bool` — Check if map is empty.
- `Index<&str>` trait — Enable indexing syntax (`map["command"]`), panics if key not found.

**Registry Integration:**
- `StaticCommandRegistry::from_commands(commands: &'static StaticCommandMap)` — Primary API for creating registry from static map.

**Performance Characteristics:**
- Lookup time: O(1), approximately 80 nanoseconds per command.
- Memory overhead: Zero runtime allocation (all data is compile-time).
- Binary size impact: Minimal (<100 bytes for wrapper code).

**Build System Integration:**
```rust
// Generated code pattern (internal implementation hidden)
const STATIC_COMMANDS_INTERNAL: /* optimization structure */ = /* generated */;
pub static STATIC_COMMANDS: StaticCommandMap =
  StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_INTERNAL);
```

### API Reference: Config Value Extraction Utilities

**Requires feature**: `json_parser`

The `unilang` framework provides generic utilities for extracting typed values from configuration maps. These utilities work with `HashMap<String, (JsonValue, S)>` where `S` is any source-tracking type.

**Type Alias:**
- `ConfigMap<S>` — Alias for `HashMap<String, (JsonValue, S)>`

**Extraction Functions** (all generic over source type `S`, return `Option<T>`):
- `extract_u8<S>(config, key) -> Option<u8>`
- `extract_u16<S>(config, key) -> Option<u16>`
- `extract_u32<S>(config, key) -> Option<u32>`
- `extract_u64<S>(config, key) -> Option<u64>`
- `extract_i32<S>(config, key) -> Option<i32>`
- `extract_i64<S>(config, key) -> Option<i64>`
- `extract_f64<S>(config, key) -> Option<f64>`
- `extract_bool<S>(config, key) -> Option<bool>`
- `extract_string<S>(config, key) -> Option<String>`
- `extract_string_array<S>(config, key) -> Option<Vec<String>>`

### Environment Variables

| Variable | Purpose | Example |
| :--- | :--- | :--- |
| `UNILANG_VERBOSITY` | Sets logging verbosity for the `unilang_cli` demo binary (0=quiet, 1=normal, 2=debug). CLI binary only — library callers configure logging via their own `tracing` subscriber. | `2` |
| `UNILANG_HELP_VERBOSITY` | Controls help output detail level (0=Minimal, 1=Basic, 2=Standard/DEFAULT, 3=Detailed, 4=Comprehensive). | `2` |
| `UNILANG_HELP_HIDE_VERSION` | When set (any value), suppresses the version line in command help output. Implemented in `src/help.rs`. | `1` |

### Library & Tool Versions

Critical libraries, frameworks, and tools with locked versions:

- `rustc`: `1.70.0` (MSRV)
- `phf`: `0.11`
- `serde`: `1.0`
- `serde_yaml`: `0.9`

### Key Architectural Decisions

- **Hybrid Registry:** `StaticCommandMap` wrapper (compile-time optimized) for static commands + dynamic HashMap for runtime commands — downstream crates require no internal optimization dependencies
- **Two-Phase Validation:** Parse-time syntax validation + semantic-time type and constraint validation
- **Explicit Naming:** Commands require dot prefix (`.command`); YAML manifests support two valid formats
- **Help Conventions:** Three access methods (`?` operator, `??` parameter, `.command.help` commands)
- **Argv-Based API:** Native `&[String]` array support for CLI applications alongside string-based API
- **Automatic Performance Monitoring:** Interpreter-level execution timing capture with `execution_time_ms` field in `OutputData` — provides zero-overhead timing instrumentation without manual tracking in command routines

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_command_registry.md](../feature/001_command_registry.md) | FR-* requirements these types implement |
| doc | [architecture/004_implementation_details.md](../architecture/004_implementation_details.md) | Internal implementation of these types |
| doc | [invariant/002_non_functional_requirements.md](../invariant/002_non_functional_requirements.md) | NFRs that govern API performance |
