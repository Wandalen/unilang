# Help Module

Thin adapter over the `unilang_help` crate: maps `CommandDefinition` into the renderer-agnostic help model and delegates rendering. `HelpVerbosity` and `HelpDisplayOptions` are re-exports of the `unilang_help` types.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | `HelpGenerator`, model mapping, and registry-free help text functions |
