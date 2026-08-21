# unilang_help

Reusable help-page domain model, verbosity levels, and renderers for unilang-style CLIs.

## Why a separate crate

Help rendering has no reason to depend on a command framework's type system. This crate defines a small, pre-rendered-string data model — the producing side (for example `unilang`'s help adapter) converts its own command definitions into that model once, and every renderer here consumes it unchanged. Any CLI that can describe a command as strings can reuse the renderers.

## Components

| Component | Purpose |
|-----------|---------|
| `HelpCommandData` / `HelpParamData` | Renderer-independent help data for one command and its parameters |
| `HelpVerbosity` | Five detail levels, Minimal (0) to Comprehensive (4); default Standard (2); reads `UNILANG_HELP_VERBOSITY` |
| `HelpDisplayOptions` | Global visibility toggles (version, status, aliases, tags); reads `UNILANG_HELP_HIDE_VERSION` |
| `PlainRenderer` | Plain-text command pages at all five verbosity levels, plus a parameter detail page |
| `CliFmtRenderer` | Column-aligned, colour-aware command and parameter pages via `cli_fmt` (feature `cli_fmt_backend`, on by default) |

The `PlainRenderer` command-page formats are a line-faithful port of the original `unilang` `HelpGenerator` output — consumers migrating from that implementation get byte-identical text for the same data.

## Example

```rust
use unilang_help::{ HelpCommandData, HelpParamData, HelpVerbosity, PlainRenderer };

let mut param = HelpParamData::default();
param.name = "scope".into();
param.kind = "Enum".into();
param.kind_compact = "enum".into();
param.description = "Discovery strategy selector.".into();
param.optional = true;
param.choices = vec![ "local".into(), "global".into() ];

let mut cmd = HelpCommandData::default();
cmd.name = ".rollup".into();
cmd.description = "Aggregate readme files.".into();
cmd.params.push( param );

let renderer = PlainRenderer::default().with_verbosity( HelpVerbosity::Basic );
let page = renderer.render( &cmd );
assert!( page.contains( ".rollup - Aggregate readme files." ) );
assert!( page.contains( "  scope::enum" ) );
```

Parameter detail pages are rendered from the same data:

```rust
use unilang_help::{ HelpCommandData, HelpParamData, PlainRenderer };

let mut param = HelpParamData::default();
param.name = "scope".into();
param.kind = "Enum".into();
param.kind_compact = "enum".into();
param.choices = vec![ "local".into(), "global".into() ];

let mut cmd = HelpCommandData::default();
cmd.name = ".rollup".into();

let page = PlainRenderer::default().render_param( &cmd, &param );
assert!( page.contains( "Parameter: scope" ) );
assert!( page.contains( "Choices: local, global" ) );
```

## Features

| Feature | Default | Purpose |
|---------|---------|---------|
| `enabled` | yes | Core model, verbosity, and `PlainRenderer` |
| `cli_fmt_backend` | yes | `CliFmtRenderer` backed by `cli_fmt`'s detail-page template |
| `full` | — | All of the above |

Disable default features for a dependency-free plain-text-only build:

```toml
unilang_help = { version = "0.1", default-features = false, features = [ "enabled" ] }
```

## Environment variables

| Variable | Effect |
|----------|--------|
| `UNILANG_HELP_VERBOSITY=0..4` | Selects the verbosity level (`HelpVerbosity::from_env`) |
| `UNILANG_HELP_HIDE_VERSION=1` | Hides version lines globally (`HelpDisplayOptions::with_env_overrides`) |
