# unilang_help Source Directory

## Responsibility

Source code for the `unilang_help` crate — help-page data model, verbosity levels, and renderers.

## File Responsibility Table

| File / Directory | Responsibility |
|------------------|---------------|
| `lib.rs` | Crate root: module declarations and prelude exports |
| `model.rs` | Renderer-independent help data (`HelpCommandData`, `HelpParamData`) |
| `verbosity.rs` | Verbosity levels and display options (`HelpVerbosity`, `HelpDisplayOptions`) |
| `plain.rs` | Plain-text renderer with five verbosity levels (`PlainRenderer`) |
| `cli_fmt_renderer.rs` | Column-aligned renderer via `cli_fmt` (`CliFmtRenderer`) |
