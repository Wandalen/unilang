# Feature: Help System

### Scope

- **Purpose:** Define behavioral requirements for command help generation and discovery
- **Responsibility:** FR-HELP-1 through FR-HELP-8: help generation, auto-help, ?? parameter, self-exclusion
- **In Scope:** Help generation requirements, auto-help conventions, help parameter behavior
- **Out of Scope:** Help rendering implementation, UI formatting specifics

### Design

The help system uses a decoupled `HelpGenerator` that operates exclusively on `CommandDefinition` metadata without any knowledge of the domain or application using the framework. This makes the generator reusable across any command set without modification.

Three access methods exist for every registered command: the `?` operator (appended as the final token to any command string), the `??` quoted parameter (passable as any argument value), and the `.command.help` auto-generated command. The `?` operator and `??` parameter share one code path (`HelpGenerator`, dispatching by verbosity level) and always produce identical output for the same command and verbosity level. The `.command.help` companion command uses a separate, fixed-format renderer (`format_command_help()`) that is not verbosity-aware — its output has a different section structure (`Command:`/`Description:`/`Hint:`/`Version:`/`Status:`/`Arguments:`/`Examples:`/`Aliases:`/`Usage:`) from `HelpGenerator`'s output and does not vary with `UNILANG_HELP_VERBOSITY`. All three access methods expose the same underlying command metadata, but only `?` and `??` are output-identical to each other. Help generation is mandatory — every registered command automatically receives a `.command.help` companion with no opt-out.

The five verbosity levels (Minimal through Comprehensive) form a progressive disclosure ladder. Higher levels add more detail rather than replacing existing information, so a user who finds Level 2 output sufficient need not learn new output structure when switching to Level 3. The default is Level 2 (Standard), which is optimized for terminal use: concise, with a USAGE line, parameter list with descriptions, and example invocations. The verbosity level is configurable via environment variable `UNILANG_HELP_VERBOSITY`.

For the full CLI command language syntax including the `?` operator and `??` parameter parsing rules, see the canonical specification in `architecture/003_vision_scope.md § CLI Modality: Language Syntax & Processing`.

### FR-HELP-1 (Command List)

The `HelpGenerator` **must** be able to produce a formatted list of all registered commands, including their names, namespaces, and hints.

**Implementation status:** ✅ Implemented with comprehensive formatting and namespace-aware command listing.

### FR-HELP-2 (Detailed Command Help)

The `HelpGenerator` **must** be able to produce detailed, formatted help for a specific command via the `?`/`??` access methods, including its description, arguments (with types, defaults, and validation rules), aliases, and examples, scaled to the active verbosity level (see FR-HELP-7).

**Implementation Note:** This requirement governs `HelpGenerator::command()` (`src/help/mod.rs`), the formatter behind the `?` operator and `??` parameter. The `.command.help` auto-generated command (FR-HELP-4) uses a separate, non-verbosity-aware renderer — see the Design section above for the distinction.

**Implementation status:** ✅ Implemented with hierarchical help formatting including all metadata, validation rules, and usage examples.

### FR-HELP-3 (Help Operator)

The parser **must** recognize the `?` operator. When present, the `Semantic Analyzer` **must** return a `HELP_REQUESTED` error containing the detailed help text for the specified command, bypassing all argument validation.

**Implementation status:** ✅ Implemented with Pipeline enhancement to convert HELP_REQUESTED errors to successful help output.

### FR-HELP-4 (Standardized Help Commands)

For every registered command `.command`, the framework **must** provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. This standardization ensures consistent help access across all commands.

For a command registered under a non-empty namespace, the generated help command's fully-qualified name **must** include that namespace (e.g. a command `.delete` under namespace `.session` generates `.session.delete.help`, not `.delete.help`) — the help companion's qualification always mirrors its parent's. See `invariant/005_command_naming.md` for the namespace-concatenation algorithm this depends on.

**Implementation Note:** Rendered via `format_command_help()` (`src/registry/traits.rs`), called from the routine `create_help_routine()` builds for each generated `.command.help` command (`src/registry/help.rs`).

**Implementation status:** ✅ Implemented via `register_with_auto_help()` and `auto_help_enabled` field with automatic help command generation. Namespace qualification of the generated help name was fixed in BUG-103 (previously, any embedded dot in the parent's local name — including the `.help` suffix itself — was misread as "already fully qualified," silently dropping the namespace).

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
- **Level 2 (Standard — DEFAULT):** Concise format with USAGE, PARAMETERS with descriptions, and EXAMPLES sections — optimized for terminal use
- **Level 3 (Detailed):** Full metadata including version, aliases, tags, validation rules — comprehensive documentation
- **Level 4 (Comprehensive):** Extensive format with rationale, use cases, and detailed explanations — for learning and documentation

The default verbosity **must** be Level 2 (Standard) to provide concise, actionable help without overwhelming users. The API **must** provide methods to:
- Create generators with specific verbosity: `HelpGenerator::with_verbosity(level)`
- Set verbosity dynamically: `set_verbosity(level)`
- Query current verbosity: `verbosity()`

The verbosity level **must** be parseable from integers 0-4 via `HelpVerbosity::from_level`, with values above 4 capped at Comprehensive.

**Configurable via environment variable:** `UNILANG_HELP_VERBOSITY` (0=Minimal, 1=Basic, 2=Standard/DEFAULT, 3=Detailed, 4=Comprehensive).

**Per-command version visibility:** Independent of verbosity level, the `show_version_in_help` field (set via `CommandDefinition::with_show_version_in_help()` in Rust, or `show_version_in_help` on `StaticCommandDefinition`/YAML) controls whether a specific command's version line appears in its help output — defaults to `true`. This applies to both `HelpGenerator` output and `.command.help` output (FR-HELP-4), since both renderers consult this field directly. `HelpDisplayOptions` (constructed via `HelpGenerator::with_display_options()`, or defaulted from the `UNILANG_HELP_HIDE_VERSION` environment variable through `with_env_overrides()`) provides the registry-wide toggle for version, status, aliases, and tags, consulted by both renderers alongside the per-command field — the two are AND-composed, so either one suppressing the version line is enough to hide it.

**Implementation status:** ✅ Implemented with `HelpVerbosity` enum (Minimal, Basic, Standard, Detailed, Comprehensive), `HelpGenerator::with_verbosity()`, `set_verbosity()`, and `verbosity()` methods. Default is Standard (Level 2). Comprehensive test coverage with 9 tests verifying all verbosity levels and progressive information display. Per-command version visibility implemented and tested in `tests/help/show_version.rs`; `HelpDisplayOptions` (including `UNILANG_HELP_HIDE_VERSION`) is wired into `HelpGenerator`'s renderers and `.command.help`'s `format_command_help()`, and covered by the Test Matrix in task 113.

### FR-HELP-8 (Help Command Self-Exclusion)

The `.help` system command **must not** appear in its own listing when the user invokes `.help` (or the `?` operator). Including `.help` in its own output is a self-referential display artefact that adds noise without value, since the user already successfully invoked `.help` to see the list.

**Implementation Note:** Achieved by registering `.help` with `hidden_from_list: true` so `format_command_listing()` suppresses it from the output it generates.

**Implementation status:** ✅ Implemented — `.help` is registered with `hidden_from_list: true` in the dynamic registry build (fixed in BUG-102).

### Analyses

| File | Relationship |
|------|--------------|
| [001_api_analysis.md](../analysis/001_api_analysis.md) | Analysis of help request detection patterns |

### APIs

| File | Relationship |
|------|--------------|
| [001_public_types.md](../api/001_public_types.md) | HelpGenerator and HelpVerbosity public types |

### Architectures

| File | Relationship |
|------|--------------|
| [005_help_decoupling.md](../architecture/005_help_decoupling.md) | Migration that decoupled help from domain |

### Features

| File | Relationship |
|------|--------------|
| [001_command_registry.md](001_command_registry.md) | Commands that help describes |
| [003_pipeline.md](003_pipeline.md) | Pipeline that intercepts help requests |

### Invariants

| File | Relationship |
|------|--------------|
| [003_governing_principles.md](../invariant/003_governing_principles.md) | Consistent help access principle |

### Sources

| File | Relationship |
|------|--------------|
| `src/help/` | Help text generation and formatting for `?`/`??` (`HelpGenerator`, verbosity-aware) |
| `src/registry/traits.rs` | `format_command_help()` — fixed-format renderer backing `.command.help` |
| `src/registry/help.rs` | `create_help_routine()`, `register_mandatory_global_help_command()` — help command registration and the literal `.help` command's own renderer |
| `src/data/command_status.rs` | `construct_full_command_name()` — namespace qualification for generated help command names |

### Tests

| File | Relationship |
|------|--------------|
| `tests/help/` | Help generation, formatting, conventions, verbosity tests |
| `tests/regression/namespace_split_and_help_qualification.rs` | Namespaced help command qualification regression tests (BUG-103) |
