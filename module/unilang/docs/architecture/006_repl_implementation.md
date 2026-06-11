# Architecture: REPL Implementation

### Scope

- **Purpose:** Document the REPL feature implementation: feature flags, configuration, and usage patterns
- **Responsibility:** How to enable and configure REPL, integration patterns, feature combinations
- **In Scope:** Feature flag configuration, usage guide, REPL implementation patterns
- **Out of Scope:** REPL behavioral requirements (see feature/005_repl_interactive.md)

### Overview

The Unilang REPL functionality is organized into two feature levels: `repl` (base REPL with standard input/output) and `enhanced_repl` (advanced REPL with arrow keys, command history, and tab completion).

### Feature Dependencies

The `enhanced_repl` feature depends on `repl` (base REPL), the `rustyline` readline library for advanced input features, and `std::io::IsTerminal` for TTY detection (Rust 1.70+). The base `repl` feature has no additional dependencies beyond the standard library.

#### Important Notes

- `enhanced_repl` automatically enables `repl` (dependency relationship)
- `enhanced_repl` without `repl` is equivalent to neither feature enabled (shows error)
- Default configuration includes both `repl` and `enhanced_repl`

### Feature Combinations and Behavior

| Features Enabled | Behavior | Arrow Keys | Command History | Tab Completion |
|------------------|----------|------------|-----------------|----------------|
| `enhanced_repl` | Enhanced REPL | ✅ Full support | ✅ Up/Down arrows + `history` | ✅ Basic |
| `repl` only | Basic REPL | ❌ Shows `^[[A` | ✅ `history` command only | ❌ |
| Neither | Error message | ❌ N/A | ❌ N/A | ❌ N/A |

### Default Features

The default feature set includes `enabled`, `simd`, `repl`, and `enhanced_repl`. Running any example without explicit feature flags uses the full enhanced REPL experience.

### Usage Examples

#### 1. Enhanced REPL (Default)

Run any REPL example without feature flags to get the enhanced REPL. Features include arrow key navigation through history (up/down), line editing (cursor movement, Home/End, Ctrl+A/E), basic tab completion, Ctrl+C/Ctrl+D handling, a `history` command, and TTY detection with user guidance.

#### 2. Basic REPL Only

Enable only the `repl` feature (with `--no-default-features --features enabled,repl`) for basic REPL without arrow key support. Provides: the `history` command with a manual numbered list, all standard REPL commands, and standard input/output handling. Arrow keys show raw escape sequences rather than navigating history.

#### 3. No REPL Features

Running with `--no-default-features --features enabled` (without `repl`) shows an instructional error message explaining which feature flag to add and what each level provides.

### Implementation Details

#### Conditional Compilation

The implementation uses Rust conditional compilation attributes to activate the correct function body depending on which features are enabled. When `repl` is enabled, the REPL entry point runs. When neither feature is enabled, the entry point displays a guidance message with available options.

#### Function Feature Gates

- `register_interactive_commands`: guarded by the `repl` feature
- `run_enhanced_repl`: guarded by the `enhanced_repl` feature
- `run_basic_repl`: guarded by `repl` and not `enhanced_repl`
- `display_repl_help`: guarded by the `repl` feature
- `display_command_history`: guarded by `repl` and not `enhanced_repl`

#### Dependency Management

The `rustyline` library (readline with history/completion) is declared as an optional dependency in the manifest. The `enhanced_repl` feature activates it via `dep:rustyline` syntax. The base `repl` feature declares no optional dependencies.

#### How Arrow Keys Work

When `enhanced_repl` is enabled, the up arrow navigates backward through command history (most recent first), the down arrow navigates forward toward newer commands, Enter executes the displayed command, and recalled commands can be edited before execution.

#### When Arrow Keys Work

Arrow keys work in interactive terminal sessions, direct terminal execution, SSH sessions, and standard terminal emulators. Arrow keys do not work in non-interactive sessions such as piped input, redirected stdin/stdout, CI/CD environments, and automated scripts. The REPL automatically detects the environment and provides appropriate guidance.

#### Enhanced REPL History

History is handled by `rustyline` internally. Navigation uses up/down arrow keys. History is session-only (not saved to file). Only actual commands are added — meta-commands like `help`, `quit`, and `clear` are excluded.

#### Basic REPL History

History is maintained in a `Vec<String>` stored manually. Access is via the `history` command which displays a numbered list.

#### Commands Not Added to History

The following commands are not recorded in either REPL mode: `help`, `h`, `history`, `clear`, `quit`, `exit`, `q`, and empty input.

### Error Handling

#### Feature-Specific Error Handling

No REPL features: shows an instructional error message with usage options. Basic REPL: standard error messages with tips to use `help`. Enhanced REPL: advanced error handling with context-aware suggestions.

#### Interactive Argument Handling

All REPL modes detect the interactive argument required signal and present a secure input prompt. The signal is detected via the error code string in the returned error data and triggers the REPL to prompt for the missing argument value before retrying.

### REPL Implementation Performance Analysis

#### Enhanced REPL

Memory usage is higher due to rustyline dependencies. Startup is slightly slower due to terminal initialization. Runtime performance difference is negligible. User experience is significantly better.

#### Basic REPL

Memory usage is lower (standard library only). Startup is faster. Runtime overhead is minimal. User experience is functional but basic.

### Testing

#### Feature Combination Tests

Test the four combinations: default (enhanced REPL), basic REPL only (`--no-default-features --features enabled,repl`), explicit enhanced REPL (`--no-default-features --features enabled,enhanced_repl`), and no REPL (`--no-default-features --features enabled`).

#### Arrow Key Testing

Arrow keys can only be tested interactively in a terminal. Start the REPL, enter several commands, then use the up arrow to navigate backward through history and the down arrow to navigate forward. Edit a recalled command and press Enter to execute the modified version.

### Features

| File | Relationship |
|------|--------------|
| [005_repl_interactive.md](../feature/005_repl_interactive.md) | FR-REPL-* requirements this implements |

### Architectures

| File | Relationship |
|------|--------------|
| [003_vision_scope.md](003_vision_scope.md) | REPL as a supported modality |

### Sources

| File | Relationship |
|------|--------------|
| `src/bin/unilang_cli.rs` | REPL binary entry point |
| `src/interpreter.rs` | Interpreter used by REPL |

### Tests

| File | Relationship |
|------|--------------|
| `examples/wasm-repl/` | WASM REPL example |
| `tests/manual/readme.md` | Manual REPL testing plan |
