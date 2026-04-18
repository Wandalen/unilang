<!-- {{# generate.module_header{} #}} -->

# Module :: unilang
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/unilang/actions/workflows/module_unilang_push.yml/badge.svg)](https://github.com/Wandalen/unilang/actions/workflows/module_unilang_push.yml) [![docs.rs](https://img.shields.io/docsrs/unilang?color=e3e8f0&logo=docs.rs)](https://docs.rs/unilang) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Funilang%2Fexamples%2F00_pipeline_basics.rs,RUN_POSTFIX=--example%2000_pipeline_basics/https://github.com/Wandalen/unilang) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

**Zero-overhead command framework with compile-time command registration**

unilang processes command definitions at compile-time, generating optimized static command registries with O(1) lookups (~80ns), zero runtime overhead, and SIMD-accelerated parsing. Commands are defined in YAML (default), JSON, or Rust DSL; the build system auto-discovers and validates them before your binary ships.

## Features

- **50x faster command resolution** — static PHF map vs runtime HashMap (~80ns vs ~4,000ns)
- **Compile-time validation** — all command definitions checked before deployment
- **SIMD parsing** — 4-25x parsing performance improvement
- **Multiple definition styles** — YAML, JSON, Rust DSL (builder or const fn)
- **Multi-file aggregation** — auto-discover commands across files with conflict detection
- **Hybrid mode** — static base + runtime plugins in one registry
- **Built-in REPL** — interactive shell with history, completion, secure input
- **CLI aggregation** — unify multiple tools under one interface with namespace isolation

## Installation

```toml
[dependencies]
unilang = "0.51"
```

The default configuration enables multi-YAML build-time static registration (Approach #2 — recommended for 95% of users).

## Minimal Example

Create `unilang.commands.yaml`:

```yaml
- name: ".greet"
  description: "Greeting command"
  arguments:
    - name: "name"
      kind: "String"
      attributes:
        optional: true
        default: "World"
```

Use it in `src/main.rs`:

```rust,ignore
use unilang::prelude::*;

include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".greet name::Alice" );
  println!( "{}", result.outputs[ 0 ].content );
  Ok( () )
}
```

```bash
cargo run  # Builds, generates static registry, runs
```

## Parameter Syntax

Named parameters use `name::value` — **double colon** is required. The `::` operator
activates value context, preserving special characters (`/`, `.`, `#`, `?`) until
whitespace:

```bash
.greet name::Alice
.run   file::./examples/plan.md          # file paths — fully supported
.fetch url::https://example.com/path     # URLs — fully supported
.find  pattern::"multi word value"       # spaces → quote the value
```

Single colon (`name:value`) is not valid syntax and produces a parse error.
See [docs/parameter_syntax.md](docs/parameter_syntax.md) for the full reference.

## Documentation

| Document | Contents |
|----------|----------|
| [docs/quick_start.md](docs/quick_start.md) | Step-by-step setup guide |
| [docs/parameter_syntax.md](docs/parameter_syntax.md) | `::` operator, value context, file paths, quoting |
| [docs/cli_definition_approaches.md](docs/cli_definition_approaches.md) | All 21 approaches (YAML/JSON/DSL, build/runtime) |
| [docs/cli_aggregation.md](docs/cli_aggregation.md) | CLI aggregation with namespace isolation |
| [docs/migration.md](docs/migration.md) | Runtime → build-time migration (50x speedup) |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Common errors and solutions |
| [docs/features.md](docs/features.md) | Full feature tracking table |
| [docs/optimization_guide.md](docs/optimization_guide.md) | Performance tuning guidelines |
| [docs/phf_reexport.md](docs/phf_reexport.md) | PHF re-export for `static_registry` users |
| [docs/feature/](docs/feature/) | Feature requirements (FR-REG, FR-ARG, FR-PIPE, FR-HELP) |
| [docs/invariant/](docs/invariant/) | System invariants, NFRs, governing principles |
| [docs/api/](docs/api/) | API contracts, data structures, implementation details |
| [examples/](examples/) | Runnable examples with learning path |

## Approach Selection

| # | Approach | Feature Flag | Default | Lookup |
|---|----------|--------------|---------|--------|
| **2** | **Multi-YAML → Build-time static** | `approach_yaml_multi_build` | **✅** | **~80ns** |
| 1 | Single YAML → Build-time static | `approach_yaml_single_build` | ❌ | ~80ns |
| 3 | YAML → Runtime loading | `approach_yaml_runtime` | ❌ | ~4,200ns |
| 4–6 | JSON variants (same as 1–3) | `approach_json_*` | ❌ | 80/4,200ns |
| 7 | Rust DSL builder API | *(always available)* | ✅ | ~4,200ns |
| 8 | Rust DSL const fn static | `approach_rust_dsl_const` | ❌ | ~80ns |
| 18 | Hybrid (static + runtime) | `approach_hybrid` | ❌ | Mixed |

See [docs/cli_definition_approaches.md](docs/cli_definition_approaches.md) for all 21 approaches.
