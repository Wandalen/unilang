# Module :: cargo_unilang

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/unilang/actions/workflows/module_cargo_unilang_push.yml/badge.svg)](https://github.com/Wandalen/unilang/actions/workflows/module_cargo_unilang_push.yml) [![docs.rs](https://img.shields.io/docsrs/cargo_unilang?color=e3e8f0&logo=docs.rs)](https://docs.rs/cargo_unilang) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

**Scaffolding and health-check tool for unilang projects**

`cargo_unilang` scaffolds new unilang projects with the correct structure and detects common anti-patterns in existing ones — duplicate dependencies, redundant `build.rs`, deprecated API calls. It is itself built on unilang and serves as a reference implementation.

## Why This Exists

A real-world project wrote **220 lines of custom `build.rs`** duplicating logic unilang provides automatically, resulting in:

- **50× performance degradation** — `OnceLock<HashMap>` at runtime instead of compile-time PHF maps
- **Duplicate dependencies** — `serde_yaml`, `walkdir`, `phf_codegen` already re-exported by unilang
- **4+ hours of wasted development time**

`cargo_unilang` prevents this class of mistake.

## Installation

```bash
# Install from the unilang workspace
cargo install --path module/cargo_unilang

# Or run directly without installing
cargo run --manifest-path module/cargo_unilang/Cargo.toml -- .help
```

## Quick Start

```bash
# Scaffold a new project
cargo_unilang .new project::my-cli

# Validate an existing project
cargo_unilang .check path::./my-project
```

## Commands

### `.new` — Create a project

```bash
cargo_unilang .new project::<name> [template::<type>] [author::<name>] [license::<id>] [verbosity::<0-5>]
```

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| `project` | yes | — | Package name (valid Rust identifier, max 64 chars) |
| `template` | no | `minimal` | `minimal` or `full` |
| `author` | no | — | Author field in generated `Cargo.toml` |
| `license` | no | `MIT` | License identifier |
| `verbosity` | no | `2` | Output level 0–5 |

**Generated structure** (`minimal`):

```
my-cli/
├── Cargo.toml       ← unilang = "0.53" with usage warnings
├── src/main.rs      ← working example (~15 lines)
└── commands.yaml    ← example command definition
```

No `build.rs` — unilang provides it automatically.

**Exit codes**: `0` success · `1` I/O error · `2` invalid parameters · `3` directory already exists

### `.check` — Validate a project

```bash
cargo_unilang .check [path::<dir>] [verbosity::<0-5>]
```

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| `path` | no | `.` | Project directory to check |
| `verbosity` | no | `2` | Output level 0–5 |

Runs three checks:

1. **Custom `build.rs`** — detects unilang keywords, signals duplication of unilang's built-in build system
2. **Duplicate dependencies** — `serde_yaml`, `walkdir`, `phf` are already provided transitively by unilang
3. **Deprecated API** — `CommandRegistry::new()` replaced by `StaticCommandRegistry::from_commands()`

**Example output** (verbosity 2):

```
Checking unilang project: ./my-project

❌ PROBLEMS DETECTED:

  1. Custom build.rs found
     Location: ./build.rs (220 lines)
     Issue: Duplicates unilang's built-in build system
     Fix: Delete build.rs — unilang provides this automatically

  2. Duplicate dependencies
     Location: Cargo.toml [dependencies]
     Issue: serde_yaml, walkdir already provided by unilang
     Fix: Remove serde_yaml, walkdir from Cargo.toml

Summary: 2 issue(s) found
```

**Exit codes**: `0` all checks passed · `1` issues found · `2` invalid parameters · `3` path not accessible

### `.help` / `.`

```bash
cargo_unilang .             # General help
cargo_unilang .help         # General help
cargo_unilang .new.help     # Help for .new
cargo_unilang .check.help   # Help for .check
```

## Verbosity

All commands support `verbosity::<0-5>`:

| Level | Output | Use case |
|-------|--------|----------|
| `0` | Silent — exit code only | CI pipelines |
| `1` | Single line | Shell scripts |
| `2` | Concise multi-line **(default)** | Normal use |
| `3` | + debug info | Troubleshooting |
| `4`–`5` | Maximum debug | Deep diagnostics |

**CI example**:

```bash
cargo_unilang .check verbosity::0
[ $? -eq 0 ] || { echo "unilang project has issues — run .check for details"; exit 1; }
```

## Anti-Patterns Detected

### Custom `build.rs`

```rust
// ❌ Wrong — 220 lines duplicating what unilang does for free
fn main() {
  let files = discover_yaml_files();
  // ... manual PHF generation ...
}
```

```
# ✅ Correct — no build.rs at all
```

### Duplicate dependencies

```toml
# ❌ Wrong
[dependencies]
unilang    = "0.53"
serde_yaml = "0.9"   # already inside unilang
walkdir    = "2"     # already inside unilang

[build-dependencies]
phf_codegen = "0.13" # already inside unilang
```

```toml
# ✅ Correct
[dependencies]
unilang = "0.53"     # only dependency needed
```

### Deprecated API

```rust
// ❌ Deprecated
let registry = CommandRegistry::new();

// ✅ Current
let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
```

## Architecture

```
cargo_unilang/
├── src/
│   ├── main.rs                  # Entry point and command dispatcher
│   ├── commands/
│   │   ├── help.rs              # Help strings for all commands
│   │   ├── new.rs               # .new handler — parameter parsing + file generation
│   │   └── check.rs             # .check handler — orchestrates health checks
│   ├── templates/
│   │   ├── cargo_toml.rs        # Cargo.toml template (minimal/full)
│   │   ├── main_rs.rs           # src/main.rs templates
│   │   └── commands_yaml.rs     # commands.yaml templates
│   └── checks/
│       ├── build_rs.rs          # Detect custom build.rs with unilang keywords
│       ├── deps.rs              # Detect duplicate transitive dependencies
│       └── api.rs               # Detect deprecated CommandRegistry::new()
└── tests/
    └── integration_test.rs      # Integration test suite
```

## Testing

```bash
# Full test suite
cargo test

# Integration tests only
cargo test --test integration_test

# Specific test
cargo test test_new_creates_correct_structure
```

## Meta-Compliance

`cargo_unilang` practices what it preaches:

- No `build.rs` — uses unilang's built-in build system
- No duplicate dependencies — only `unilang`, `toml_edit`, `walkdir`
- Uses current API — `StaticCommandRegistry::from_commands()`
- Follows CLI conventions — dot-prefix commands, `param::value` format

## Troubleshooting

**"Unknown command"** — commands require dot-prefix: `.new` not `new`.

**"Invalid parameter format"** — use `key::value` not `--key value`.

**"Project already exists"** — choose a different name or remove the directory first.

**False positive in `.check`** — `deps` check uses TOML parsing, not text matching; file an issue if you hit one.

## Links

- [unilang documentation](https://docs.rs/unilang)
- [unilang repository](https://github.com/Wandalen/unilang)
- [Discord](https://discord.gg/m3yKnHRAGr)

## License

MIT
