# Feature: `.check` Health Check Command

`cargo_unilang` must provide a `.check [path::<dir>]` command that detects common anti-patterns in existing `unilang` projects — custom `build.rs` that duplicates unilang's build logic, duplicate dependencies already provided transitively by `unilang`, and deprecated API usage — reporting each issue with location and suggested fix, and exiting with code `1` when any issues are found.
