# Feature: Help System

### Scope

- **Purpose:** Define behavioral requirements for command help generation and discovery
- **Responsibility:** FR-HELP-1 through FR-HELP-12: help generation, auto-help, the `??` help token, parameter detail pages, opt-out, lints, self-exclusion
- **In Scope:** Help generation requirements, auto-help conventions, help parameter behavior
- **Out of Scope:** Help rendering implementation, UI formatting specifics

### Design

The help system uses a decoupled `HelpGenerator` that operates exclusively on `CommandDefinition` metadata without any knowledge of the domain or application using the framework. This makes the generator reusable across any command set without modification.

Two access routes exist for every registered command, and they render byte-identical pages: the unquoted `??` token, whose position selects scope (bare `??` lists all commands, `.command ??` renders the command page, `.command name::??` renders the parameter detail page), and the spelled `.command.help` / `.command.help <param>` auto-generated companion command. There is no parser-level help operator: `?` and `??` are ordinary tokens, and the semantic analyzer intercepts an unquoted `??` before argument binding — so a broken sibling argument never masks a help request, and routines never observe the token. A quoted `"??"` is a literal value.

Rendering is delegated to the reusable `unilang_help` crate: `PlainRenderer` produces command pages and listings; the `cli_fmt`-backed `CliFmtRenderer` produces parameter detail pages. `unilang::help` adapts `CommandDefinition` metadata into `unilang_help`'s renderer-agnostic model (`help_command_data`, `help_param_data`) and exposes the text entry points (`command_help_text`, `parameter_help_text`, `parameter_help_or_listing`) that both access routes call — which is what guarantees the byte-identical pages. Both routes honor `UNILANG_HELP_VERBOSITY` and the display-option environment overrides. Command pages use the full invocable name (`Usage: .ns.cmd`), so every rendered usage line is directly typeable. Help generation is mandatory — every registered command automatically receives a `.command.help` companion with no opt-out (detection of the `??` token, by contrast, is suppressible — FR-HELP-10).

The five verbosity levels (Minimal through Comprehensive) form a progressive disclosure ladder. Higher levels add more detail rather than replacing existing information, so a user who finds Level 2 output sufficient need not learn new output structure when switching to Level 3. The default is Level 2 (Standard), which is optimized for terminal use: concise, with a USAGE line, parameter list with descriptions, and example invocations. The verbosity level is configurable via environment variable `UNILANG_HELP_VERBOSITY`.

For the full CLI command language syntax including the `??` help-token rules, see the canonical specification in `architecture/003_vision_scope.md § CLI Modality: Language Syntax & Processing`.

### FR-HELP-1 (Command List)

The `HelpGenerator` **must** be able to produce a formatted list of all registered commands, including their names, namespaces, and hints.

**Implementation status:** ✅ Implemented with comprehensive formatting and namespace-aware command listing.

### FR-HELP-2 (Detailed Command Help)

The framework **must** be able to produce detailed, formatted help for a specific command via the `??` access route, including its description, arguments (with types, defaults, and validation rules), aliases, and examples, scaled to the active verbosity level (see FR-HELP-7).

**Implementation Note:** This requirement governs `command_help_text()` and `HelpGenerator::command()` (`src/help/mod.rs`) — the same `unilang_help`-backed rendering path also serves the `.command.help` route (FR-HELP-4), so both routes' pages are byte-identical.

**Implementation status:** ✅ Implemented with hierarchical help formatting including all metadata, validation rules, and usage examples.

### FR-HELP-3 (Help Token Interception)

The semantic analyzer **must** intercept an unquoted `??` argument before argument binding and return a `HELP_REQUESTED` error whose message carries the rendered help text — bypassing all argument validation, so a failing sibling argument can never mask a help request. Framework pipelines **must** convert this error into successful output (`Pipeline::process_command` and the free `process_single_command` both do). Routing: a named `name::??` beats a positional `??`; several named `??` resolve to the first parameter in command-definition order; an alias resolves to its canonical parameter; an unknown `name::??` renders the valid-parameter listing instead of failing.

**Implementation status:** ✅ Implemented in `src/semantic/core.rs` (`first_help_requested_parameter`, `positional_help_requested`); the routing matrix is pinned test-per-rule in `tests/help/detection_matrix.rs`.

### FR-HELP-4 (Standardized Help Commands)

For every registered command `.command`, the framework **must** provide automatic registration of a corresponding `.command.help` command that returns detailed help information for the parent command. This standardization ensures consistent help access across all commands.

For a command registered under a non-empty namespace, the generated help command's fully-qualified name **must** include that namespace (e.g. a command `.delete` under namespace `.session` generates `.session.delete.help`, not `.delete.help`) — the help companion's qualification always mirrors its parent's. See `invariant/005_command_naming.md` for the namespace-concatenation algorithm this depends on.

**Implementation Note:** Rendered via `crate::help::command_help_text()` (no argument) or `crate::help::parameter_help_or_listing()` (`.command.help <param>`) from the routine `create_help_routine()` builds for each generated `.command.help` command (`src/registry/help.rs`) — the same entry points backing the `??` token, which is what makes the two routes' pages byte-identical.

**Implementation status:** ✅ Implemented via `register_with_auto_help()` and `auto_help_enabled` field with automatic help command generation. Namespace qualification of the generated help name was fixed in BUG-103 (previously, any embedded dot in the parent's local name — including the `.help` suffix itself — was misread as "already fully qualified," silently dropping the namespace).

### FR-HELP-5 (The `??` Help Token)

The framework **must** recognize a single help token — an unquoted `??` — whose position selects scope: bare `??` **must** mirror the `.` global listing, and only in the exact argument-free form (`?? extra` is not help — it fails command lookup rather than silently dropping arguments); `.command ??` (any position) **must** render help identical to calling `.command.help`; `.command name::??` **must** render the parameter detail page identical to `.command.help name`. A quoted `"??"` **must** bind as the literal string value. There is no `?` help form: `?` is an ordinary value, and when it fails coercion the error nudges toward `name::??` (FR-HELP-12).

**Implementation status:** ✅ Implemented; every routing rule, including the quoted-literal and argument-free-bare cases, is pinned in `tests/help/detection_matrix.rs`.

### FR-HELP-6 (Automatic Help Command Generation API)

The framework **must** provide APIs (`CommandDefinition::with_auto_help`) that automatically generate `.command.help` commands and enable `??` token processing with minimal developer effort. Help generation is now mandatory for all commands — no opt-out mechanism exists.

**Implementation Notes:**
- Automatic `.command.help` command registration via `register_with_auto_help()`
- Help generation is mandatory and always enabled
- Per-command control via `auto_help_enabled` field (for configuration only — help still generated)
- Pipeline enhancement converts `HELP_REQUESTED` errors to successful help output
- Comprehensive help formatting with all command metadata, validation rules, and examples
- Two byte-identical access routes: the `??` token and `.command.help` commands

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

**Per-command version visibility:** Independent of verbosity level, the `show_version_in_help` field (set via `CommandDefinition::with_show_version_in_help()` in Rust, or `show_version_in_help` on `StaticCommandDefinition`/YAML) controls whether a specific command's version line appears in its help output — defaults to `true`. This applies to both access routes (`??` and `.command.help`), which share one rendering path. `HelpDisplayOptions` (constructed via `HelpGenerator::with_display_options()`, or defaulted from the `UNILANG_HELP_HIDE_VERSION` environment variable through `with_env_overrides()`) provides the registry-wide toggle for version, status, aliases, and tags, AND-composed with the per-command field — either one suppressing the version line is enough to hide it. `HelpVerbosity` and `HelpDisplayOptions` are defined in the `unilang_help` crate and re-exported by `unilang::help`.

**Implementation status:** ✅ Implemented with `HelpVerbosity` enum (Minimal, Basic, Standard, Detailed, Comprehensive), `HelpGenerator::with_verbosity()`, `set_verbosity()`, and `verbosity()` methods. Default is Standard (Level 2). Comprehensive test coverage with 9 tests verifying all verbosity levels and progressive information display. Per-command version visibility implemented and tested in `tests/help/show_version.rs`; `HelpDisplayOptions` (including `UNILANG_HELP_HIDE_VERSION`) is wired through `unilang_help`'s renderers, serving both access routes, and covered by the Test Matrix in task 113.

### FR-HELP-8 (Help Command Self-Exclusion)

The `.help` system command **must not** appear in its own listing when the user invokes `.help` (or a bare `??`). Including `.help` in its own output is a self-referential display artefact that adds noise without value, since the user already successfully invoked `.help` to see the list.

**Implementation Note:** Achieved by registering `.help` with `hidden_from_list: true` so `format_command_listing()` suppresses it from the output it generates.

**Implementation status:** ✅ Implemented — `.help` is registered with `hidden_from_list: true` in the dynamic registry build (fixed in BUG-102).

### FR-HELP-9 (Parameter Detail Pages)

`name::??` (and `.command.help <param>`) **must** render a per-parameter page containing: the parameter name, a synthesized canonical invocation using the full command name, kind, required/multiple flags, default, aliases, validation rules, possible values for enums, and derived examples (an enum's synthesized placeholder is its first choice). Requests by alias **must** resolve to the canonical parameter's page.

**Implementation status:** ✅ Implemented via `help_param_data()` + `parameter_help_text()` (`src/help/mod.rs`), rendered by `unilang_help`'s `CliFmtRenderer` over cli_fmt's `DetailPageTemplate`; covered in `tests/help/detection_matrix.rs` and `tests/help/conventions.rs`.

### FR-HELP-10 (Detection Opt-Out)

`SemanticAnalyzer::with_help_detection( false )` and `Pipeline::with_help_detection( false )` **must** disable all `??` interception: bare `??` fails command lookup, and every `??` argument flows to binding as an ordinary value. Applications that need `??` as data get it back wholesale; quoting remains the per-value escape while detection is on.

**Implementation status:** ✅ Implemented; the three detection-off behaviors (named literal, bare unknown-command, positional-hits-coercion) are pinned in `tests/help/detection_matrix.rs`.

### FR-HELP-11 (Registration Lints)

At registration, `validate_help_conventions()` **must** reject an `Enum` parameter whose default is not among its own choices (such a default could never pass coercion), and **must** warn — non-fatally, on stderr, suppressible via `UNILANG_NO_LINT_WARNINGS` — when a `String` parameter's description embeds an `a|b|c` choice list, steering the author to `Kind::Enum` so `name::??` can list the choices and invalid values are rejected.

**Implementation status:** ✅ Implemented in `src/command_validation.rs`, wired into `validate_command_for_registration()`; covered in `tests/semantic/command_validation.rs`.

### FR-HELP-12 (Help-on-Error Nudges)

Errors adjacent to help **must** point at the working help syntax: a coercion failure of an empty or `?` value appends `Did you mean 'name::??' for parameter help?` (suppressed for sensitive parameters), and unknown-parameter errors reference `.command ??` using the full invocable name (never a leaf name that would render as `..command`).

**Implementation status:** ✅ Implemented in `src/semantic/argument_binding.rs` (nudge) and `src/semantic/validation.rs` (full-name hints); covered in `tests/semantic/argument_binding.rs`, `tests/semantic/parameter_typo_suggestion.rs`, and `tests/help/detection_matrix.rs`.

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
| `src/help/` | Adapter from `CommandDefinition` to `unilang_help`'s model; text entry points serving both access routes |
| `src/semantic/core.rs` | Unquoted-`??` interception before argument binding (`first_help_requested_parameter`, `positional_help_requested`) |
| `src/registry/help.rs` | `create_help_routine()`, `register_mandatory_global_help_command()` — help command registration and the literal `.help` command's own renderer |
| `src/command_validation.rs` | `validate_help_conventions()` — registration lints (FR-HELP-11) |
| `src/data/command_status.rs` | `construct_full_command_name()` — namespace qualification for generated help command names |

### Tests

| File | Relationship |
|------|--------------|
| `tests/help/` | Help generation, formatting, conventions, verbosity tests |
| `tests/help/detection_matrix.rs` | One test per `??` routing rule (FR-HELP-3, -5, -9, -10, -12) |
| `tests/help/cli_invocation.rs` | Binary-level (`unilang_cli`) checks: argv `??` after named values, spelled-route printing, `??`/`.help` byte-identity, listing footer |
| `module/unilang_parser/tests/parse_from_argv_boundary_test.rs` | Argv absorption never glues a standalone `??` into a named value (string/argv parity) |
| `tests/regression/namespace_split_and_help_qualification.rs` | Namespaced help command qualification regression tests (BUG-103) |
