# unilang_help Test Suite

Tests are organized by source module: each renderer and the verbosity module get their own integration test file exercising the public API only.

## File Responsibility Table

| File | Responsibility |
|------|---------------|
| `inc/` | Shared help-data fixtures used by both renderer test files |
| `verbosity_test.rs` | `HelpVerbosity` level parsing, env-var reading, `HelpDisplayOptions` builders and env overrides |
| `plain_renderer_test.rs` | `PlainRenderer` command pages at all five verbosity levels and the parameter detail page |
| `cli_fmt_renderer_test.rs` | `CliFmtRenderer` command and parameter pages against `tty_detect: false` goldens |
