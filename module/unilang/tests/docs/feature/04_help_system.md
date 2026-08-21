# Feature Spec: Help System

### Scope

- **Purpose:** Verify all FR-HELP behavioral requirements for help generation and access
- **Responsibility:** Test cases covering command list generation, detailed help, the `??` help token, `.cmd.help` commands, parameter detail pages, automatic help API, and verbosity levels
- **In Scope:** FR-HELP-1 (command list), FR-HELP-2 (detailed command help), FR-HELP-3 (help token interception), FR-HELP-4 (`.cmd.help` commands), FR-HELP-5 (the `??` help token), FR-HELP-6 (automatic help API), FR-HELP-7 (verbosity levels 0–4 via `UNILANG_HELP_VERBOSITY`, `HelpGenerator` verbosity API, `UNILANG_HELP_HIDE_VERSION`, `show_version_in_help`), FR-HELP-8 (`.help` self-exclusion from listing), FR-HELP-9 (parameter detail pages), FR-HELP-10 (detection opt-out), FR-HELP-12 (help-on-error nudges) — FT-18..FT-23 map to `tests/help/detection_matrix.rs`
- **Out of Scope:** Registry initialization (FR-REG); argument parsing (FR-ARG); REPL state (FR-REPL)

### FT-1: Command list returns all registered command names

- **Given:** A registry with three commands: `.foo`, `.bar`, `.baz`
- **When:** The help API is called to generate the command list (e.g., `help_list(&registry)`)
- **Then:** The output contains `.foo`, `.bar`, and `.baz`; no other names appear; no panic on empty registry

### FT-2: Detailed help output includes argument names and descriptions

- **Given:** A registry containing `.greet` with one argument `"name"` described as "The recipient's name"
- **When:** `help_command(&registry, ".greet")` is called
- **Then:** Output contains both `"name"` and `"The recipient's name"` in the help text

### FT-3: bare `??` alone produces command list output

- **Given:** A `Pipeline` with a registry containing `.foo`
- **When:** `pipeline.run("??")` is called
- **Then:** Output contains `.foo` in a list format; returns without error. Only the exact argument-free form is help — `pipeline.run("?? extra")` fails command lookup instead of silently dropping arguments

### FT-4: positional `??` token causes help output and no command execution

- **Given:** A `Pipeline` with `.greet` registered and input `".greet ??"`
- **When:** `pipeline.run(".greet ??")` is called
- **Then:** Help text for `.greet` is included in output; the command's handler function is not called

### FT-5: UNILANG_HELP_VERBOSITY=0 produces minimal output; =4 produces full output

- **Given:** A command `.greet` with one argument `"name"` that has a description, a type, and a default value
- **When:** `help_command` is called first with `UNILANG_HELP_VERBOSITY=0` set, then with `UNILANG_HELP_VERBOSITY=4`
- **Then:** At level 0 the output contains only command name; at level 4 the output additionally contains argument type and default value; the two outputs are observably different strings

### FT-6: `.greet.help` command is automatically available and returns help

- **Given:** A registry where `.greet` is registered and automatic help API is enabled
- **When:** `pipeline.run(".greet.help")` is called
- **Then:** Returns help text for `.greet` equivalent to calling the help API directly; no error

### FT-7: `??` on an unknown command returns not-found, never a help page

- **Given:** A `Pipeline` with a registry that does not contain `.unknown`
- **When:** `pipeline.run(".unknown ??")` is called
- **Then:** Returns `CommandNotFound` for `.unknown` (with optional Levenshtein suggestions); help rendering happens only after successful command lookup; no panic

### FT-8: `.help` command does not appear in its own listing

- **Given:** A `Pipeline` whose registry includes the `.help` system command
- **When:** `pipeline.run(".help")` is called (or `pipeline.run("??")`)
- **Then:** The returned output does NOT contain `.help` as a listed command entry; all other registered commands are still visible in the listing

### FT-9: Default verbosity level is Level 2 (Standard)

- **Given:** A `Pipeline` with `.greet` registered (one argument `"name"` with description, type, and default); `UNILANG_HELP_VERBOSITY` is NOT set in the environment
- **When:** `pipeline.run(".greet ??")` is called (triggering help with default verbosity)
- **Then:** Output includes USAGE line and PARAMETERS section with argument descriptions (Level 2 content); output does NOT include version, aliases, or tags metadata (Level 3+ content)

### FT-10: UNILANG_HELP_VERBOSITY=1 produces Basic level output

- **Given:** A `Pipeline` with `.greet` registered (one argument `"name"` with description, type, default, and aliases); `UNILANG_HELP_VERBOSITY=1` set in the environment
- **When:** `pipeline.run(".greet ??")` is called
- **Then:** Output includes command name and parameters list with types (syntax lookup); output does NOT include full PARAMETERS descriptions, EXAMPLES sections, or version metadata (Level 1 = Basic, adding parameter types beyond Level 0's name-only output)

### FT-11: UNILANG_HELP_VERBOSITY=3 produces Detailed level output with metadata

- **Given:** A `Pipeline` with `.greet` registered (one argument `"name"` with description, type, default, version set to `"1.0"`, and aliases `["g"]`); `UNILANG_HELP_VERBOSITY=3` set in the environment
- **When:** `pipeline.run(".greet ??")` is called
- **Then:** Output includes USAGE line, PARAMETERS section with argument descriptions AND type information, and version metadata; output is strictly more detailed than Level 2 output

### FT-12: UNILANG_HELP_VERBOSITY=4 produces Comprehensive level output

- **Given:** A command `.greet` with one argument `"name"` that has a description, a type, a default value, a version, and tags; `UNILANG_HELP_VERBOSITY=4` set in the environment
- **When:** `help_command` (or `pipeline.run(".greet ??")`) is called
- **Then:** Output includes USAGE, DESCRIPTION, PARAMETERS (with type and validation detail), EXAMPLES, and TAGS sections; output is strictly more detailed than Level 3 output

### FT-13: HelpGenerator::with_verbosity, set_verbosity, and verbosity round-trip correctly

- **Given:** A `HelpGenerator` constructed via `HelpGenerator::with_verbosity(&registry, HelpVerbosity::Detailed)`
- **When:** `verbosity()` is queried, then `set_verbosity(HelpVerbosity::Minimal)` is called, then `verbosity()` is queried again
- **Then:** The first query returns `HelpVerbosity::Detailed`; the second query returns `HelpVerbosity::Minimal`; subsequent `command(...)` output reflects the newly-set Minimal level, not the original Detailed level

### FT-14: HelpVerbosity::from_level caps values above 4 at Comprehensive

- **Given:** No registry or pipeline setup required
- **When:** `HelpVerbosity::from_level(level)` is called with `level` values `4`, `5`, and `100`
- **Then:** All three calls return `HelpVerbosity::Comprehensive`; no panic or error for out-of-range input

### FT-15: UNILANG_HELP_HIDE_VERSION suppresses the version line in rendered help output

- **Given:** `UNILANG_HELP_HIDE_VERSION=1` set in the environment
- **When:** `HelpGenerator::from_env()` (or any `HelpGenerator` defaulting its `HelpDisplayOptions` via `with_env_overrides()`) renders a command's help at Level 2+ via `??`, or `registry.help_for_command()` renders the same command's `.command.help` output
- **Then:** The version line is absent from both rendered outputs; with `UNILANG_HELP_HIDE_VERSION` unset, the version line is present in both. The underlying `HelpDisplayOptions.show_version` field flips `false`/`true` accordingly, AND-composed with the command's own `show_version_in_help` (see FT-17).

### FT-16: Namespaced command's `.help` companion includes the parent's namespace

- **Given:** A command `delete` registered under namespace `.session` (e.g. via `CommandDefinition::with_namespace` or YAML `namespace: ".session"` + `name: "delete"`)
- **When:** `generate_help_command()` is called on the command definition, and separately `pipeline.run(".session.delete.help")` is called
- **Then:** The generated help command's `full_name()` is `.session.delete.help`, not `.delete.help`; the pipeline call succeeds with no "not found" error

### FT-17: show_version_in_help suppresses a command's version line in .command.help output

- **Given:** A command registered with a non-empty version (e.g., `"3.0.0"`) and `show_version_in_help` set to `false` (via `CommandDefinition::with_show_version_in_help(false)`)
- **When:** `registry.help_for_command()` is called for that command (the text backing `.command.help`)
- **Then:** The rendered output does NOT contain the version string; a command with `show_version_in_help` at its default (`true`) DOES show the version string
- **Coverage note:** the `unilang_help`-backed rendering path (`src/help/mod.rs`, serving `??` and `.command.help` alike) also reads `show_version_in_help` at each verbosity level, AND-composed with the registry-wide `HelpDisplayOptions.show_version` — exercised end-to-end by T05 in `tests/help/show_version.rs` (task 113), alongside the `.command.help` path above.

### FT-18: named `name::??` renders the parameter detail page and beats positional `??`

- **Given:** A `Pipeline` with `.matrix` registered (parameters `mode`, `count`, `tag` in that definition order)
- **When:** `pipeline.run(".matrix count::??")`, then `pipeline.run(".matrix count::?? ??")`, then `pipeline.run(".matrix count::?? mode::??")` are called
- **Then:** The first two render the `count` detail page (named beats positional); the third renders `mode` — with several named `??`, the first parameter in command-definition order wins

### FT-19: alias `alias::??` resolves canonically; unknown `name::??` lists valid parameters

- **Given:** `.matrix` whose `tag` parameter has alias `t`
- **When:** `pipeline.run(".matrix t::??")` and `pipeline.run(".matrix bogus::??")` are called
- **Then:** The first renders the `tag` detail page (canonical name shown); the second succeeds with a listing that echoes `bogus` and enumerates `mode`, `count`, `tag` — never a dead end

### FT-20: quoted `"??"` is a literal value; `with_help_detection(false)` disables all interception

- **Given:** `.matrix` with a routine echoing its `tag` value; a second `Pipeline` built `.with_help_detection( false )`
- **When:** `pipeline.run(".matrix tag::\"??\"")` on the first; `".matrix tag::??"`, `"??"`, and `".matrix ??"` on the second
- **Then:** The quoted form executes the routine with literal `??`; with detection off, `tag::??` also executes with literal `??`, bare `??` fails command lookup, and positional `??` reaches binding as an ordinary (coercion-failing) value

### FT-21: parameter detail pages carry synthesized canonical examples

- **Given:** `.matrix` whose `mode` parameter is `Enum` with choices `fast`, `slow`
- **When:** `pipeline.run(".matrix mode::??")` is called
- **Then:** The page contains the synthesized invocation `.matrix mode::fast` — the enum placeholder is its first choice, and the command name is the full invocable name

### FT-22: help interception precedes argument binding

- **Given:** `.matrix` with Integer parameter `count`
- **When:** `pipeline.run(".matrix tag::?? count::abc")` is called
- **Then:** The `tag` detail page renders successfully; the broken sibling `count::abc` never surfaces a coercion error, because interception runs before binding

### FT-23: coercion failure of `?` or an empty value nudges toward `name::??`

- **Given:** `.matrix` with Integer parameter `count`, and a command with a `sensitive` parameter
- **When:** `pipeline.run(".matrix count::?")` is called; separately an empty/`?` value fails coercion on the sensitive parameter
- **Then:** The count error appends `Did you mean 'count::??' for parameter help?`; the sensitive parameter's error carries no nudge (and no echoed value)

### FT-24: CLI binary parity — argv `??`, spelled-route printing, byte-identity, footer

- **Given:** The `unilang_cli` binary with `.math.add` registered (Integer parameters `a`, `b`)
- **When:** The binary is invoked as `unilang_cli .math.add a::1 ??`, `unilang_cli .math.add.help`, `unilang_cli .math.add ??`, and bare `unilang_cli`
- **Then:** `a::1 ??` renders the command page (argv absorption never glues a standalone `??` into a named value — no coercion error); `.math.add.help` prints a non-empty page (the binary prints the interpreter's returned `OutputData`); `.math.add ??` and `.math.add.help` stdout are byte-identical; the bare listing's footer advertises `'<command> ??' or '<command>.help'` with example `.list ??`
- **Maps to:** `tests/help/cli_invocation.rs` (T8.8–T8.11) and `module/unilang_parser/tests/parse_from_argv_boundary_test.rs` (T07–T09)
