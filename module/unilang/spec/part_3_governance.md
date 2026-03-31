## Part III: Project & Process Governance
*This part of the specification defines the project's goals, scope, and the rules governing its development process.*

### 13. Project Goals & Success Metrics
*   **Primary Goal:** To create a stable, performant, and ergonomic framework for building multi-modal command-line utilities in Rust that allows developers to define a command interface once and deploy it everywhere with zero-overhead for static commands.
*   **Success Metric 1 (Performance):** The framework **must** meet all performance NFRs defined in Section 5, verified by the project's benchmark suite.
*   **Success Metric 2 (Adoption):** The framework is considered successful if it is used to build at least three distinct `utility1` applications with different modalities within 12 months of the v1.0 release.

### 14. Deliverables

Upon completion, the project will deliver the following artifacts:

1.  The published `unilang` Rust crate on crates.io.
2.  The published `unilang_parser` Rust crate on crates.io.
3.  The published `unilang_meta` Rust crate on crates.io.
4.  A compiled WebAssembly (`.wasm`) package and associated JavaScript bindings for the core framework, enabling client-side execution.
5.  Full access to the source code repository, including all examples and benchmarks.
6.  Generated API documentation hosted on docs.rs for all public crates.

### 15. Open Questions
1.  **Custom Type Registration:** What is the API and process for an `Integrator` to define a new custom `Kind` and register its associated parsing and validation logic with the framework?
2.  **Plugin System:** What would a formal plugin system look like, allowing third-party crates to provide `unilang` commands to a host application?

### 15.1. Governing Principles

The unilang framework is built on fundamental principles that guide all architectural decisions and implementation details:

#### 15.1.1. Minimum Implicit Magic
The framework **must** minimize implicit behavior and transformations to maximize predictability:
- **Explicit Operations**: All operations should be explicit rather than implicit
- **Predictable Behavior**: What you specify is exactly what you get - no hidden transformations
- **Clear APIs**: Function behavior should be obvious from signatures and documentation
- **No Surprising Side Effects**: Commands and functions should behave exactly as documented

#### 15.1.2. Single Source of Truth
Each piece of information **must** have exactly one authoritative source:
- **Command Definitions**: Commands registered exactly as specified, used exactly as registered
- **Configuration**: One canonical location for each configuration setting
- **Documentation**: Single authoritative source for each concept or procedure

#### 15.1.3. Fail-Fast Validation
The framework **must** detect and report errors as early as possible:
- **Registration Time**: Invalid command definitions rejected immediately during registration
- **Parse Time**: Syntax errors detected during parsing phase
- **Semantic Analysis**: Type and validation errors caught before execution
- **Clear Error Messages**: All errors include actionable guidance for resolution

#### 15.1.4. Explicit Dependencies
All dependencies and relationships **must** be made explicit:
- **Command Dependencies**: Clear specification of required arguments and constraints
- **Type Dependencies**: Explicit type requirements and conversions
- **System Dependencies**: Clear documentation of external requirements

#### 15.1.5. Consistent Help Access
The framework **must** provide standardized, predictable help access for all commands:
- **Universal Help Commands**: Every command `.command` automatically generates a `.command.help` counterpart
- **Uniform Help Parameter**: The `??` parameter provides consistent help access across all commands
- **Help Convention APIs**: Developer-friendly APIs make following help conventions effortless
- **Discoverability**: Users can always find help through predictable patterns

These principles serve as the foundation for all design decisions and implementation choices throughout the framework.

### 16. Core Principles of Development

#### 16.1. Single Source of Truth
The project's Git repository **must** be the absolute single source of truth for all project-related information. This includes specifications, documentation, source code, configuration files, and architectural diagrams.

#### 16.2. Documentation-First Development
All changes to the system's functionality or architecture **must** be documented in the relevant specification files *before* implementation begins.

#### 16.3. Review-Driven Change Control
All modifications to the repository, without exception, **must** go through a formal Pull Request review.

#### 16.4. Radical Transparency and Auditability
The development process **must** be fully transparent and auditable. All significant decisions and discussions **must** be captured in writing within the relevant Pull Request or a linked issue tracker. The repository's history should provide a clear, chronological narrative of the project's evolution.

#### 16.5. File Naming Conventions
All file names within the project repository **must** use lowercase `snake_case`.

#### 16.6. Explicit Command Naming Principle
The framework **must** adhere to the principle of explicit command naming with minimal implicit transformations:

- **Commands as Registered**: Command names **must** be used exactly as registered, without automatic prefix addition or name transformation
- **Dot Prefix Requirement**: All commands **must** be registered with explicit dot prefix (e.g., `.chat`, `.session.list`)  
- **Validation Enforcement**: The framework **must** reject command registrations that do not start with a dot prefix
- **No Implicit Behavior**: The system **must not** automatically add dots, modify namespaces, or transform command names during registration or execution
- **Principle of Least Surprise**: Command behavior should be predictable - what you register is exactly what gets executed

---
### Appendix: Addendum
*This appendix is intended for developer use during implementation. It captures as-built details and serves as a living document during the development cycle.*

#### Purpose
This document is intended to be completed by the **Developer** during the implementation phase. It is used to capture the final, as-built details of the **Internal Design**, especially where the implementation differs from the initial `Design Recommendations` in `spec.md`.

#### Instructions for the Developer
As you build the system, please use this document to log your key implementation decisions, the final data models, environment variables, and other details. This creates a crucial record for future maintenance, debugging, and onboarding.

---

#### Conformance Checklist
*This checklist is the definitive list of acceptance criteria for the project. Before final delivery, each item must be verified as complete and marked with `✅`. Use the 'Verification Notes' column to link to evidence (e.g., test results, screen recordings).*

| Status | Requirement | Verification Notes |
| :--- | :--- | :--- |
| ✅ | **FR-REG-1:** The framework must provide a mechanism, via a `build.rs` script, to register commands at compile-time from a manifest file (e.g., `unilang.commands.yaml`). | Implemented via `build.rs` parsing `unilang.commands.yaml` and generating `StaticCommandRegistry` with PHF maps. Two YAML formats supported. Phase 4 complete (M4.2, M4.3). All tests pass. |
| ✅ | **FR-REG-2:** The framework must expose a public API (`CommandRegistry::command_add_runtime`) for registering new commands and their routines at runtime. | Implemented as `CommandRegistry::command_add_runtime()` in `src/registry.rs:815`. Supports dynamic registration with full validation including dot-prefix enforcement, duplicate detection, and parameter validation. |
| ✅ | **FR-REG-3:** The framework must provide functions (`load_from_yaml_str`, `load_from_json_str`) to load `CommandDefinition`s from structured text at runtime. | Implemented as `CommandRegistry::load_from_yaml_str()` (`src/registry.rs:307`) and `load_from_json_str()` (`src/registry.rs:335`). Runtime loading from structured text into `CommandDefinition`s. |
| ✅ | **FR-REG-4:** The framework must support hierarchical command organization through dot-separated namespaces (e.g., `.math.add`). | Dot-separated hierarchical namespaces supported throughout. Build-time `compute_full_name()` in `build.rs` handles namespace construction. Semantic analyzer resolves fully-qualified names at runtime. |
| ✅ | **FR-REG-5:** The framework must support command aliases. When an alias is invoked, the framework must execute the corresponding canonical command. | Aliases implemented in `CommandDefinition::aliases: Vec<String>`. Resolved in `src/semantic.rs` during named argument binding via alias lookup. `From<StaticCommandRegistry> for CommandRegistry` preserves aliases through conversion. |
| ✅ | **FR-REG-6:** The framework must enforce explicit command naming with dot-prefixed command names. Runtime API must reject registrations lacking dot prefix. Build-time YAML manifests may use two valid formats (compound names or separate namespace) that both produce dot-prefixed commands. | Implemented with runtime validation in `src/command_validation.rs:48-76` and build.rs transformations in `build.rs:208-223`. Two YAML formats documented and tested: Format 1 (compound names) recommended for examples, Format 2 (separate namespace) valid for production. All 608 tests passing including test data files using both formats. |
| ✅ | **FR-REG-7:** The framework must provide a CliBuilder API for aggregating multiple CLI modules with namespace isolation, conflict detection, and prefix application. Supports static/hybrid build modes for performance. | Implemented in `src/multi_yaml/aggregator.rs` and `src/multi_yaml/cli_builder.rs`. Comprehensive test coverage in `tests/cli/cli_builder_api.rs` (25+ tests) covering module registration, prefix application, conflict detection, namespace isolation, and build modes. Examples: `examples/22_minimal_cli_aggregation.rs`. All tests passing. |
| ✅ | **FR-REG-8:** StaticCommandRegistry must have complete feature parity with CommandRegistry: validation during registration, auto-help generation, global .help registration, alias resolution, complete field set (including short_desc, hidden_from_list, priority, group), and From<StaticCommandRegistry> for CommandRegistry conversion bridge. | Task 087. Implemented: (1) `From<StaticCommandRegistry> for CommandRegistry` conversion bridge in `src/registry.rs:1028-1045`, (2) Validation happens in `CommandDefinition` builder at earliest point, (3) Auto-help generated during conversion via `CommandRegistry::register()`, (4) Aliases preserved through conversion. Tests: `tests/feature_parity_test.rs` (9 tests), `tests/static_registry_conversion_test.rs` (4 tests). |
| ✅ | **FR-REG-9:** build.rs must validate all command definitions at compile time with actionable error messages: command name validation, version validation, duplicate detection, parameter storage validation (wplan bug), complete field extraction including validation_rules, no silent failures (no unwrap on user data), shared validation logic, and compile-time guarantee that From conversion cannot panic. | Task 085 + Task 087. Implemented: (1) `build_validation` module in `build.rs:287-373` with `validate_command()`, `validate_version()`, `compute_full_name()`, (2) Duplicate detection via HashMap tracking (`build.rs:619-676`), (3) Parameter storage validation (`build.rs:678-743` - prevents wplan bug), (4) Shared validation logic in `src/validation_core.rs` (6 functions), (5) Build-time validation at line 631 with actionable error boxes. Tests: 833 tests pass with w3 .test l::3. |
| ✅ | **FR-ARG-1:** The framework must support parsing and type-checking for the following `Kind`s: `String`, `Integer`, `Float`, `Boolean`, `Path`, `File`, `Directory`, `Enum`, `Url`, `DateTime`, `Pattern`, `List`, `Map`, `JsonString`, and `Object`. | All 15 `Kind` variants implemented in `src/data/kind.rs`. Type checking enforced in `SemanticAnalyzer` during argument binding. |
| ✅ | **FR-ARG-2:** The framework must correctly bind positional arguments from a `GenericInstruction` to the corresponding `ArgumentDefinition`s in the order they are defined. | Positional binding implemented in `src/semantic.rs` `bind_arguments()`. Arguments bound in definition order when no name qualifier is provided. |
| ✅ | **FR-ARG-3:** The framework must correctly bind named arguments (`name::value`) from a `GenericInstruction` to the corresponding `ArgumentDefinition`, regardless of order. | Named `name::value` binding implemented in `src/semantic.rs`. Arguments bound by name regardless of order. Comprehensive test coverage in `tests/semantic/`. |
| ✅ | **FR-ARG-4:** The framework must correctly bind named arguments specified via an alias to the correct `ArgumentDefinition`. | Alias binding implemented in `src/semantic.rs` `bind_arguments()`. Named arguments checked against both primary name and all aliases via `find_argument_by_name_or_alias()`. |
| ✅ | **FR-ARG-5:** If an optional argument with a default value is not provided, the framework must use the default value during semantic analysis. | Default value injection implemented in `src/semantic.rs` `bind_arguments()`. When optional arguments are absent, `ArgumentDefinition::default_value` is used to populate `bound_args`. |
| ✅ | **FR-ARG-6:** The `Semantic Analyzer` must enforce all `ValidationRule`s (`Min`, `Max`, `MinLength`, `MaxLength`, `Pattern`, `MinItems`) defined for an argument. If a rule is violated, a `UNILANG_VALIDATION_RULE_FAILED` error must be returned. | ValidationRule enforcement implemented in `src/semantic.rs`. All six constraint types validated. Returns `UNILANG_VALIDATION_RULE_FAILED` error on violation. |
| ✅ | **FR-ARG-7:** When the same parameter name appears multiple times in a command invocation, the `Semantic Analyzer` must automatically collect all values into a `Value::List`, regardless of the argument definition's `multiple` attribute. Single parameters must remain as single values to maintain backward compatibility. | Implemented in `src/semantic.rs` with comprehensive test coverage in `tests/task_024_comprehensive_test_suite.rs` and `tests/tokenization_failure_reproduction_test.rs`. Resolves Task 024 critical tokenization failure. |
| ✅ | **FR-ARG-8:** The `Semantic Analyzer` must reject any command invocation containing named parameters not defined in the `CommandDefinition` (including aliases). Must return `UNILANG_UNKNOWN_PARAMETER` error with "Did you mean...?" suggestions (Levenshtein distance <= 2) and command-specific help references. Validation is mandatory with no bypass mechanisms. | Implemented in `src/semantic.rs` with `check_unknown_named_arguments()`, `find_closest_parameter_name()`, and `levenshtein_distance()` functions. Comprehensive test coverage: 21 tests across `tests/semantic/unknown_parameters.rs` (5 core tests) and `tests/semantic/unknown_parameters_edge_cases.rs` (16 edge case tests) covering all boundary conditions, alias matching, distance thresholds, and complex scenarios. All 564 tests passing. |
| ✅ | **FR-PIPE-1:** The `Pipeline` API must correctly orchestrate the full sequence: Parsing -> Semantic Analysis -> Interpretation. | Implemented as `Pipeline::process_command()` in `src/pipeline.rs`. Orchestrates full Parse → SemanticAnalysis → Interpretation sequence. Comprehensive test coverage in `tests/pipeline/`. |
| ✅ | **FR-PIPE-2:** The `Pipeline::process_batch` method must execute a list of commands independently, collecting results for each and not stopping on individual failures. | Implemented as `Pipeline::process_batch()` in `src/pipeline.rs:960`. Executes commands independently, collects all results, continues on individual failures. Returns `BatchResult`. |
| ✅ | **FR-PIPE-3:** The `Pipeline::process_sequence` method must execute a list of commands in order and must terminate immediately upon the first command failure. | Implemented as `Pipeline::process_sequence()` in `src/pipeline.rs:1002`. Executes commands in order, terminates immediately on first failure. Returns `BatchResult`. |
| ✅ | **FR-PIPE-4:** The framework must provide argv-based parsing and execution APIs that accept command-line arguments as `&[String]` arrays, intelligently combining consecutive argv elements to preserve argument boundaries and eliminate information loss in CLI applications. | Implemented in `unilang_parser/src/parser_engine.rs:1076-1169` (`parse_from_argv`) and `unilang/src/pipeline.rs:738-908` (`process_command_from_argv`, `process_command_from_argv_simple`). Comprehensive test coverage in `tests/argv_api.rs` with 9 tests covering all argv scenarios. Resolves Task 080 CLI integration issues. |
| ✅ | **FR-HELP-1:** The `HelpGenerator` must be able to produce a formatted list of all registered commands, including their names, namespaces, and hints. | Implemented with comprehensive formatting and namespace-aware command listing |
| ✅ | **FR-HELP-2:** The `HelpGenerator` must be able to produce detailed, formatted help for a specific command, including its description, arguments (with types, defaults, and validation rules), aliases, and examples. | Implemented with hierarchical help formatting including all metadata, validation rules, and usage examples |
| ✅ | **FR-HELP-3:** The parser must recognize the `?` operator. When present, the `Semantic Analyzer` must return a `HELP_REQUESTED` error containing the detailed help text for the specified command, bypassing all argument validation. | Implemented with Pipeline enhancement to convert HELP_REQUESTED errors to successful help output |
| ✅ | **FR-HELP-4:** For every registered command `.command`, the framework must provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. | Implemented via `register_with_auto_help()` and `auto_help_enabled` field with automatic help command generation |
| ✅ | **FR-HELP-5:** The framework must recognize a special parameter `??` that can be appended to any command to trigger help display (e.g., `.command ??`). When this parameter is detected, the system must return help information identical to calling `.command.help`. | Implemented with semantic analyzer support for `??` parameter (requires quoting as `"??"` to avoid parser conflicts) |
| ✅ | **FR-HELP-6:** The framework must provide APIs (`CommandDefinition::with_auto_help`) that automatically generate `.command.help` commands and enable `??` parameter processing with minimal developer effort. Help generation is now mandatory. | Implemented with `register_with_auto_help()` and `auto_help_enabled` field - help generation is mandatory for all commands |
| ✅ | **FR-HELP-7:** The framework must support configurable help verbosity levels (0-4) to accommodate different user preferences. Default verbosity is Level 2 (Standard - concise like unikit). Provides methods to create, set, and query verbosity levels. | Implemented in `src/help.rs` with `HelpVerbosity` enum (Minimal, Basic, Standard, Detailed, Comprehensive), `HelpGenerator::with_verbosity()`, `set_verbosity()`, and `verbosity()` methods. Default is Standard (Level 2). Comprehensive test coverage in `tests/help_verbosity.rs` with 9 tests verifying all verbosity levels and progressive information display. All tests passing. |
| ✅ | **FR-REPL-1:** The framework's core components (`Pipeline`, `Parser`, `SemanticAnalyzer`, `Interpreter`) must be structured to support a REPL-style execution loop. They must be reusable for multiple, sequential command executions within a single process lifetime. | Implemented with comprehensive examples and verified stateless operation |
| ✅ | **FR-INTERACTIVE-1:** When a mandatory argument with the `interactive: true` attribute is not provided, the `Semantic Analyzer` must return a distinct, catchable error (`UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`). This allows the calling modality to intercept the error and prompt the user for input. | Implemented in semantic analyzer with comprehensive test coverage and REPL integration |
| ❌ | **FR-MOD-WASM-REPL:** The framework must support a web-based REPL modality that can operate entirely on the client-side without a backend server. This requires the core `unilang` library to be fully compilable to the `wasm32-unknown-unknown` target. | |

#### Finalized Internal Design Decisions
*This section documents key architectural decisions and implementation choices.*

The framework's implementation is fully documented through:
- **Functional Requirements (Section 4):** Complete specification of all features and capabilities
- **Conformance Checklist (below):** Verification status and implementation details for each requirement
- **Git History:** Detailed commit messages documenting all design decisions, bug fixes, and refactoring

Key architectural decisions:
- **Hybrid Registry:** `StaticCommandMap` wrapper (compile-time optimized) for static commands + dynamic HashMap for runtime commands - downstream crates require no internal optimization dependencies
- **Two-Phase Validation:** Parse-time syntax validation + semantic-time type and constraint validation
- **Explicit Naming:** Commands require dot prefix (`.command`); YAML manifests support two valid formats
- **Help Conventions:** Three access methods (`?` operator, `??` parameter, `.command.help` commands)
- **Argv-Based API:** Native `&[String]` array support for CLI applications alongside string-based API
- **Automatic Performance Monitoring:** Interpreter-level execution timing capture with `execution_time_ms` field in `OutputData` - provides zero-overhead timing instrumentation without manual tracking in command routines

#### Finalized Internal Data Models
*The definitive, as-built schema for all databases, data structures, and objects used internally by the system.*

**CommandDefinition Structure (as of 2025-09-16):**
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
    pub auto_help_enabled: bool,        // NEW: Controls automatic .command.help generation
}
```

**OutputData Structure (as of 2025-10-19):**
```rust
pub struct OutputData {
    pub content : String,                  // The actual output content
    pub format : String,                   // Output format identifier (e.g., "text", "json", "xml")
    pub execution_time_ms : Option< u64 >, // NEW: Execution time in milliseconds (automatically populated by Interpreter)
}
```

**Performance Monitoring Implementation:**
The `execution_time_ms` field provides automatic performance monitoring for all command executions:
- **Automatic Capture:** The `Interpreter` automatically measures execution time using `std::time::Instant` and populates this field
- **Zero Developer Overhead:** Command routines dont need to track timing manually
- **Backward Compatible:** Optional field design ensures existing code continues to work
- **Precision:** Millisecond-level precision suitable for performance analysis and optimization
- **Consistency:** All commands use identical timing methodology for fair comparison

*See `src/data.rs` for the complete and authoritative structure definitions.*

#### Environment Variables
*List all environment variables required to run the application. Include the variable name, a brief description of its purpose, and an example value (use placeholders for secrets).*

| Variable | Description | Example |
| :--- | :--- | :--- |
| `UNILANG_VERBOSITY` | Sets logging verbosity for the `unilang_cli` demo binary (0=quiet, 1=normal, 2=debug). CLI binary only — library callers configure logging via their own `tracing` subscriber. | `2` |
| `UNILANG_HELP_VERBOSITY` | Controls help output detail level (0=Minimal, 1=Basic, 2=Standard/DEFAULT, 3=Detailed, 4=Comprehensive). | `2` |
| `UNILANG_HELP_HIDE_VERSION` | When set (any value), suppresses the version line in command help output. Implemented in `src/help.rs:270`. | `1` |

#### Finalized Library & Tool Versions
*List the critical libraries, frameworks, or tools used and their exact locked versions (e.g., from `Cargo.lock`).*

-   `rustc`: `1.70.0` (MSRV)
-   `phf`: `0.11`
-   `serde`: `1.0`
-   `serde_yaml`: `0.9`

#### Deployment Checklist
*A step-by-step guide for deploying the application from scratch. This is not applicable for a library, but would be used by an `Integrator`.*

1.  Set up the `.env` file using the template above.
2.  Run `cargo build --release`.
3.  Place the compiled binary in `/usr/local/bin`.

---

## Appendix A: Internal Implementation Details (For Maintainers)

### A.1 Compile-Time Optimization Strategy

**For Library Maintainers Only** - This section documents internal implementation choices that are intentionally hidden from downstream crates.

#### Perfect Hash Functions (PHF)

The `StaticCommandMap` wrapper uses Perfect Hash Functions (PHF) internally to achieve zero-overhead command lookups. This is an implementation detail that must remain hidden from the public API.

**Why PHF:**
- O(1) guaranteed lookup time (not average case)
- Zero runtime memory allocation
- Generated at compile-time via `build.rs`
- No hash computation at runtime
- Typically 10-50x faster than `HashMap` for static data

**Implementation Pattern:**
```rust
// Internal (generated by build.rs, never exposed to users)
const STATIC_COMMANDS_INTERNAL: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {
    ".command1" => &CMD_DEF_1,
    ".command2" => &CMD_DEF_2,
    // ...
};

// Public API (what users see)
pub static STATIC_COMMANDS: StaticCommandMap = 
    StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_INTERNAL);
```

**Critical Requirements:**
1. **Never expose `phf::Map` in public signatures** - Always wrap in `StaticCommandMap`
2. **Mark PHF constructor as `#[doc(hidden)]`** - Only build.rs should use it
3. **All wrapper methods must be `#[inline]`** - Ensure zero-cost abstraction
4. **Generated constant names end with `_INTERNAL`** - Signals implementation detail

**Dependencies:**
- `phf = "0.11"` - Build dependency only (not required by downstream crates)
- `phf_codegen = "0.11"` - Build script generation

### A.2 Performance Characteristics

**Static Registry (PHF-based):**
- Startup overhead: ~5μs (map initialization)
- Lookup latency: ~50-200ns (P99 < 100ns in optimized builds)
- Memory overhead: Zero runtime allocation
- Throughput: >10M lookups/second

**Dynamic Registry (HashMap-based):**
- Startup overhead: ~10-100μs (depends on command count)
- Lookup latency: ~500-5000ns (P99 < 1μs)
- Memory overhead: ~48 bytes per command + allocation overhead
- Throughput: ~1M lookups/second

**Why 10-50x Performance Difference:**
1. PHF has no hash computation (precomputed at build time)
2. PHF has perfect collision-free lookups (guaranteed O(1))
3. PHF data is in read-only memory (better cache locality)
4. No allocator involvement (zero malloc/free overhead)

### A.3 Build System Integration

The `build.rs` script generates static registries using this process:

1. **Parse YAML manifests** - Load command definitions from YAML files
2. **Generate PHF map source** - Use `MultiYamlAggregator::generate_static_registry_source()`
3. **Write to `$OUT_DIR`** - Create `static_commands.rs` in build output
4. **Include in binary** - Application uses `include!(concat!(env!("OUT_DIR"), "/static_commands.rs"))`

This approach ensures:
- Compile-time validation of all command definitions
- Zero runtime parsing overhead
- Type-safe static command access
- No dependency on YAML parsing in production binary

---

**Note to Maintainers:** When updating this implementation, ensure that:
1. Public API never exposes PHF types
2. All examples use domain terms (not "PHF map")
3. User documentation focuses on capabilities, not implementation
4. Deprecation warnings guide users toward static registration

---

## Appendix B: Help System Decoupling Migration Plan

### B.1 Migration Overview

**Status:** ✅ COMPLETE (as of 2025-12-04)
**Final State:** 0 domain-specific patterns, 2 generic algorithms, 100% tests passing

This migration successfully removed all application-specific coupling from the unilang help system, making it truly generic and reusable across any domain. The help system is now completely domain-agnostic and implements only generic transformation algorithms.

### B.2 Migration Goals

1. **Generic Algorithm:** Replace pattern-matching `auto_categorize()` with algorithm that returns empty string (categories must be explicit via `CommandDefinition::category()`)
2. **Universal Formatting:** Replace hardcoded category mappings in `format_category_name()` with generic snake_case → Title Case transformation
3. **Self-Contained Documentation:** Remove all application-specific references (wip, wplan, dream, wish) from comments and documentation
4. **Test Independence:** Update test assertions to validate generic behavior, not specific CLI patterns

### B.3 Target Architecture

**Current State (Coupled):**
```rust
fn auto_categorize( &self, name : &str ) -> String
{
  if name.starts_with( ".git" ) { "git_operations".to_string() }
  else if name.starts_with( ".remove" ) { "removal_operations".to_string() }
  // ... 12+ more domain-specific patterns
}

fn format_category_name( &self, category : &str ) -> String
{
  match category {
    "repository_management" => "REPOSITORY MANAGEMENT".to_string(),
    "git_operations" => "GIT OPERATIONS".to_string(),
    // ... 15+ hardcoded mappings
  }
}
```

**Target State (Generic):**
```rust
fn auto_categorize( &self, name : &str ) -> String
{
  String::new()  // Categories must be explicit, never inferred
}

fn format_category_name( &self, category : &str ) -> String
{
  category
    .split( '_' )
    .map( |word| {
      let mut chars = word.chars();
      match chars.next() {
        None => String::new(),
        Some( first ) => first.to_uppercase().collect::<String>() + chars.as_str(),
      }
    })
    .collect::<Vec<_>>()
    .join( " " )
}
```

### B.4 Migration Phases

**Phase 0: Baseline Measurement** ✅ COMPLETE
- Baseline metrics: 37 old patterns identified
- Category 1 (auto_categorize): 6 old patterns
- Category 2 (format_category_name): 16 old patterns
- Category 3 (Documentation): 8 old patterns
- Category 4 (Tests): 7 old patterns

**Phase 1a: TDD - auto_categorize Simplification** ✅ COMPLETE
- Created failing tests expecting empty string return (5 tests)
- Replaced pattern matching with `String::new()`
- Documented architectural requirement: categories must be explicit
- Result: Eliminated all domain-specific pattern matching

**Phase 1b: TDD - format_category_name Genericization** ✅ COMPLETE
- Created failing tests for Title Case transformation (7 tests)
- Implemented generic split/map/join algorithm
- Documented transformation: snake_case → Title Case
- Result: Eliminated all hardcoded category mappings

**Phase 2-8:** ✅ COMPLETE
- All domain-specific coupling removed from help system
- Comprehensive genericization (25+ files updated):

  **Example Files (21 files):**
  - Namespaces: `.math`/`.file`/`.text`/`.fs`/`.db`/`.network` → `.cmd1`/`.cmd2`/`.cmd3`/`.svc1`
  - Hints: "Mathematical", "Text processing", "File system" → "Generic operation/processing/listing"
  - Comments: All domain references removed ("math namespace", "file system commands")
  - Variable names: `math_command`, `math_routine`, `create_math_commands` → `cmd1_*`, `create_cmd1_commands`
  - Example strings: `"math.add"`, `"text.upper"` → `"cmd1.add"`, `"cmd3.upper"`
  - Tags: `"math"`, `"arithmetic"` → `"cmd1"`, `"generic"`
  - Module names: `math_cli_static`, `MathCliModule` → `cmd1_cli_static`, `Cmd1CliModule`
  - Descriptions: "mathematical calculations" → "generic calculations"
  - Documentation: `cli_export_best_practices.md` genericized

  **Source Files (5 files):**
  - `help.rs`: Comment examples `.math.add` → `.cmd1.add`
  - `registry.rs`: Help examples `.video.search` → `.cmd1.process`, application attribution removed
  - `simd_tokenizer.rs`: Test strings `.math.add` → `.cmd1.add`
  - `command_validation.rs`: Doc examples `.video.search` → `.cmd1.process`, `.video` → `.cmd1`, "wplan bug pattern" → "silent data loss"
  - `pipeline.rs`: Doc comment examples `.fs.list` → `.cmd2.list`

  **Verification Results:**
  - Domain references in src/: 0
  - Domain references in examples/: 0
  - Application references (wplan/wip/dream/wish/wflow): 0
  - Test suite: 100% success rate (845+ tests)
  - Zero clippy warnings

  **Final Genericization (2025-12-04):**
  - `command_validation.rs:100-102`: "wplan bug pattern" → "silent data loss"
  - `command_validation.rs:155`: "wplan bug pattern" → generic description
  - `registry.rs:278`: Removed "wflow's .languages command" attribution
  - `examples/cli_export_best_practices.md`: `.math`/`.fs`/`.db` → `.cmd1`/`.cmd2`/`.svc1`

- All tests passing (100% success rate)
- Comprehensive test coverage added (12 new tests)

**Migration Insights:**

1. **Test Data vs Documentation**: Test files (tests/*.rs) appropriately contain domain-specific test data as fixtures (e.g., `.video.search`). The migration plan specifically targeted "Examples" per Executive Summary objective 4, not test fixtures. Test data is distinct from documentation examples.

2. **Knowledge Preservation vs Coupling**: Bug patterns discovered in specific applications (wplan, wflow) should be documented generically to preserve the knowledge without creating coupling. Example:
   - ❌ Coupling: "Prevents the wplan bug pattern where..."
   - ✅ Generic: "Prevents silent data loss where..."
   The technical knowledge (multiple:true with non-List storage causes data loss) is preserved, but application attribution is removed. This principle applies to all documentation: preserve WHY bugs occur, not WHERE they were discovered.

3. **Comprehensive Genericization Scope**: Genericization must extend to ALL documentation artifacts, not just source code:
   - Source code comments (*.rs)
   - Example documentation (examples/*.md)
   - Error messages and diagnostic strings
   - Inline documentation comments
   Missing even one markdown file (e.g., `cli_export_best_practices.md`) violates domain-agnosticism.

### B.5 Acceptance Criteria

Migration is considered complete when ALL of the following conditions are met:

1. **Zero Old Patterns:** Measurement script reports 0 old patterns across all categories
2. **Full New Patterns:** Measurement script reports 4/4 new pattern score
3. **100% Migration Progress:** Measurement script reports 100% completion
4. **All Tests Passing:** Full test suite passes with `w3 .test l::3`
5. **No Application References:** Zero mentions of wip/wplan/dream/wish in `src/help.rs`
6. **Generic Documentation:** All comments are self-contained and domain-agnostic
7. **Test Validation:** Tests validate generic behavior, not specific CLI patterns

### B.6 Verification Strategy

The migration uses 7-layer verification:

1. **Quantitative Metrics:** Automated measurement script tracks old/new pattern counts
2. **Test-Driven Development:** RED-GREEN-REFACTOR cycle for each change
3. **Rulebook Compliance:** All changes follow CLAUDE.md rulebook requirements
4. **Absence Verification:** Explicit validation that old patterns are gone
5. **Authenticity Verification:** New code demonstrates truly generic behavior
6. **Impossibility Verification:** Architecture makes coupling impossible to reintroduce
7. **Irreversibility Verification:** Changes are complete replacements, not toggles

### B.7 Risk Mitigation

- **Baseline Established:** All tests passing before migration start (833+ tests)
- **Incremental Changes:** TDD approach with checkpoint verification after each phase
- **Quantitative Tracking:** Metrics script provides objective progress measurement
- **Rollback Capability:** Git history allows reverting to pre-migration state
- **Comprehensive Testing:** Full test suite execution after each change

### B.8 Success Metrics

Final verification results:
- ✅ Test suite: All 845+ tests passing with zero failures (100% success rate)
- ✅ Code review: No domain-specific pattern matching in help system
- ✅ Architectural validation: Generic algorithms incapable of domain inference
- ✅ Functional verification: `auto_categorize()` returns empty string for all inputs
- ✅ Functional verification: `format_category_name()` uses generic Title Case algorithm
- ✅ No hardcoded category mappings remain
- ✅ All clippy checks passing

**Status:** ✅ MIGRATION COMPLETE
**Completed:** 2025-12-04
**Files Modified:** `src/help.rs`, 2 new test files, 1 existing test updated
**Breaking Changes:** Applications relying on auto-categorization must now specify categories explicitly
