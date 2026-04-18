# Feature: `.new` Project Scaffolding Command

`cargo_unilang` must provide a `.new project::<name>` command that creates a correctly structured `unilang` CLI project containing `Cargo.toml`, `src/main.rs`, and `commands.yaml` — with no `build.rs` generated, since `unilang` provides build logic automatically — and that accepts `template::minimal|full`, `author`, `license`, and `verbosity` parameters.
