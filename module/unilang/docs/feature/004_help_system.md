# Feature: Help System

### Scope

- **Purpose:** Define behavioral requirements for command help generation and discovery
- **Responsibility:** FR-HELP-1 through FR-HELP-7: help generation, auto-help, ?? parameter
- **In Scope:** Help generation requirements, auto-help conventions, help parameter behavior
- **Out of Scope:** Help rendering implementation, UI formatting specifics

Functional requirements governing the help generator, help access methods, and verbosity control.

### FR-HELP-1 (Command List)

The `HelpGenerator` **must** be able to produce a formatted list of all registered commands, including their names, namespaces, and hints.

**Implementation status:** ✅ Implemented with comprehensive formatting and namespace-aware command listing.

### FR-HELP-2 (Detailed Command Help)

The `HelpGenerator` **must** be able to produce detailed, formatted help for a specific command, including its description, arguments (with types, defaults, and validation rules), aliases, and examples.

**Implementation status:** ✅ Implemented with hierarchical help formatting including all metadata, validation rules, and usage examples.

### FR-HELP-3 (Help Operator)

The parser **must** recognize the `?` operator. When present, the `Semantic Analyzer` **must** return a `HELP_REQUESTED` error containing the detailed help text for the specified command, bypassing all argument validation.

**Implementation status:** ✅ Implemented with Pipeline enhancement to convert HELP_REQUESTED errors to successful help output.

### FR-HELP-4 (Standardized Help Commands)

For every registered command `.command`, the framework **must** provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. This standardization ensures consistent help access across all commands.

**Implementation status:** ✅ Implemented via `register_with_auto_help()` and `auto_help_enabled` field with automatic help command generation.

### FR-HELP-5 (Double Question Mark Parameter)

The framework **must** recognize a special parameter `??` that can be appended to any command to trigger help display (e.g., `.command "??"`). When this parameter is detected, the system **must** return help information identical to calling `.command.help`, providing an alternative help access method.

*Implementation Note: The `??` parameter must be quoted to avoid parser conflicts with the `?` help operator.*

**Implementation status:** ✅ Implemented with semantic analyzer support for `??` parameter.

### FR-HELP-6 (Automatic Help Command Generation API)

The framework **must** provide APIs (`CommandDefinition::with_auto_help`) that automatically generate `.command.help` commands and enable `??` parameter processing with minimal developer effort. Help generation is now mandatory for all commands — no opt-out mechanism exists.

**Implementation Notes:**
- Automatic `.command.help` command registration via `register_with_auto_help()`
- Help generation is mandatory and always enabled
- Per-command control via `auto_help_enabled` field (for configuration only — help still generated)
- Pipeline enhancement converts `HELP_REQUESTED` errors to successful help output
- Comprehensive help formatting with all command metadata, validation rules, and examples
- Three help access methods: `?` operator, `"??"` parameter, and `.command.help` commands

**Implementation status:** ✅ Implemented with `register_with_auto_help()` and `auto_help_enabled` field — help generation is mandatory for all commands.

### FR-HELP-7 (Help Verbosity Levels)

The framework **must** support configurable help verbosity levels to accommodate different user preferences and use cases. The `HelpGenerator` **must** provide five verbosity levels (0-4) controlling the amount of information displayed:

- **Level 0 (Minimal):** Command name and brief description only — for quick reference
- **Level 1 (Basic):** Add parameters list with types — for syntax lookup
- **Level 2 (Standard — DEFAULT):** Concise format with USAGE, PARAMETERS with descriptions, and EXAMPLES sections — optimized for terminal use like unikit
- **Level 3 (Detailed):** Full metadata including version, aliases, tags, validation rules — comprehensive documentation
- **Level 4 (Comprehensive):** Extensive format with rationale, use cases, and detailed explanations — for learning and documentation

The default verbosity **must** be Level 2 (Standard) to provide concise, actionable help without overwhelming users. The API **must** provide methods to:
- Create generators with specific verbosity: `HelpGenerator::with_verbosity(level)`
- Set verbosity dynamically: `set_verbosity(level)`
- Query current verbosity: `verbosity()`

The verbosity level **must** be parseable from integers 0-4 via `HelpVerbosity::from_level`, with values above 4 capped at Comprehensive.

**Configurable via environment variable:** `UNILANG_HELP_VERBOSITY` (0=Minimal, 1=Basic, 2=Standard/DEFAULT, 3=Detailed, 4=Comprehensive).

**Implementation status:** ✅ Implemented in `src/help.rs` with `HelpVerbosity` enum (Minimal, Basic, Standard, Detailed, Comprehensive), `HelpGenerator::with_verbosity()`, `set_verbosity()`, and `verbosity()` methods. Default is Standard (Level 2). Comprehensive test coverage in `tests/help_verbosity.rs` with 9 tests verifying all verbosity levels and progressive information display.

### CLI Language Syntax for Help Access

From the CLI modality language spec (§6):

- **Rule 4 (Help Operator):** The `?` operator, if present, **must** be the final token and triggers the help system.
- **Rule 5 (Double Question Mark Parameter):** The `??` parameter, if present as any argument, **must** trigger help display for the command, identical to calling `.command.help`.
- **Rule 6 (Special Case — Discovery):** A standalone dot (`.`) **must** be interpreted as a request to list all available commands.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [feature/001_command_registry.md](001_command_registry.md) | Commands that help describes |
| doc | [architecture/005_help_decoupling.md](../architecture/005_help_decoupling.md) | Help system decoupling migration |
| doc | [invariant/003_governing_principles.md](../invariant/003_governing_principles.md) | Consistent help access principle |
