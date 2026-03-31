# Unilang Features

## Metadata

- **Last Updated:** 2026-03-31
- **Version:** 0.49.0
- **Features:** 30
- **Categories:** 7
- **Status:** ✅ 28/30 (93.3% Complete — Production Ready)

## Column Legend

| Column | Description |
|--------|-------------|
| `#` | Sequential row number |
| `Category` | Feature category (short form from All Categories) |
| `Feature` | Concise capability name (3–10 words) |
| `Status` | ✅ Complete · ⚠️ Partial · ❌ Missing |
| `Easiness` | 1–5 (1 = very hard, 5 = trivial) |
| `Value` | 1–5 (1 = low value, 5 = critical) |
| `Priority` | 1–5 (1 = low, 5 = critical) |
| `Score` | Easiness × Value × Priority |
| `Notes` | Context, source reference, or blockers |

## All Categories

| Category | Full Name | Count | Source | Description |
|----------|-----------|-------|--------|-------------|
| `core` | Core Pipeline | 6 | Implementation | Parsing, semantic analysis, and execution engine |
| `approaches` | Command Definition Approaches | 9 | Specification | Ways to define commands (YAML/JSON/DSL + build/runtime) |
| `registry` | Registry Modes | 3 | Specification | Static, dynamic, and hybrid command registries |
| `performance` | Performance Features | 3 | Implementation | SIMD paths, string interning, zero-copy |
| `help_sys` | Help System | 3 | Specification | Auto-generated help text and verbosity control |
| `build_time` | Build-Time Tooling | 4 | Implementation | Code generation, conflict detection, type hints |
| `interactive` | Interactive / REPL | 2 | Specification | REPL and enhanced REPL modes |

## Features Table

| # | Category | Feature | Status | Easiness | Value | Priority | Score | Notes |
|---|----------|---------|--------|----------|-------|----------|-------|-------|
| 1 | `core` | Parse command input strings | ✅ | 5 | 5 | 5 | 125 | `src/semantic.rs`, `unilang_parser` crate |
| 2 | `core` | Semantic argument binding | ✅ | 4 | 5 | 5 | 100 | Named, positional, alias resolution |
| 3 | `core` | Command pipeline processing | ✅ | 4 | 5 | 5 | 100 | `src/pipeline.rs` — batch + single |
| 4 | `core` | Command definition validation | ✅ | 4 | 5 | 5 | 100 | `src/validation_core.rs` |
| 5 | `core` | Typo suggestion for unknown params | ✅ | 3 | 4 | 4 | 48 | Levenshtein distance in `src/semantic.rs` |
| 6 | `core` | Multiple same-name parameter collection | ✅ | 3 | 4 | 4 | 48 | `Kind::Collection` support |
| 7 | `approaches` | Multi-YAML build-time static (default) | ✅ | 4 | 5 | 5 | 100 | Approach #2; `feature = approach_yaml_multi_build` |
| 8 | `approaches` | Single YAML build-time static | ✅ | 4 | 5 | 5 | 100 | Approach #1; `feature = approach_yaml_single_build` |
| 9 | `approaches` | YAML runtime loading | ✅ | 4 | 4 | 4 | 64 | Approach #3; `feature = approach_yaml_runtime` |
| 10 | `approaches` | Multi-JSON build-time static | ✅ | 4 | 4 | 4 | 64 | Approach #5; `feature = approach_json_multi_build` |
| 11 | `approaches` | Single JSON build-time static | ✅ | 4 | 4 | 4 | 64 | Approach #4; `feature = approach_json_single_build` |
| 12 | `approaches` | JSON runtime loading | ✅ | 4 | 4 | 4 | 64 | Approach #6; `feature = approach_json_runtime` |
| 13 | `approaches` | Rust DSL builder API | ✅ | 5 | 5 | 5 | 125 | Approach #7; always available (no feature gate) |
| 14 | `approaches` | Rust DSL const fn static | ✅ | 3 | 4 | 4 | 48 | Approach #8; `feature = approach_rust_dsl_const` |
| 15 | `approaches` | Hybrid static + runtime registry | ✅ | 3 | 4 | 4 | 48 | Approach #18; `feature = approach_hybrid` |
| 16 | `registry` | Dynamic command registry | ✅ | 4 | 5 | 5 | 100 | `src/registry.rs` — `CommandRegistry` |
| 17 | `registry` | PHF static command registry | ✅ | 4 | 5 | 5 | 100 | `src/static_data.rs` + PHF codegen |
| 18 | `registry` | StaticCommandRegistry trait bridge | ✅ | 3 | 4 | 4 | 48 | Converts static → dynamic at runtime |
| 19 | `performance` | SIMD tokenizer | ✅ | 3 | 4 | 3 | 36 | `src/simd_tokenizer.rs`; `feature = simd` |
| 20 | `performance` | SIMD JSON parser | ✅ | 3 | 4 | 3 | 36 | `src/simd_json_parser.rs`; `feature = simd` |
| 21 | `performance` | String interning cache | ✅ | 4 | 4 | 3 | 48 | `src/interner.rs`; zero-copy command names |
| 22 | `help_sys` | Auto-generated help text | ✅ | 4 | 5 | 5 | 100 | `src/help.rs` |
| 23 | `help_sys` | Version display in help | ✅ | 5 | 4 | 4 | 80 | `UNILANG_HELP_HIDE_VERSION` env var |
| 24 | `help_sys` | Verbosity control | ✅ | 4 | 3 | 3 | 36 | `UNILANG_VERBOSITY`; CLI binary only |
| 25 | `build_time` | Multi-YAML conflict detection | ✅ | 4 | 5 | 5 | 100 | `src/multi_yaml/aggregator.rs` |
| 26 | `build_time` | Build-time Rust code generation | ✅ | 4 | 5 | 5 | 100 | PHF maps + static arrays via `build.rs` |
| 27 | `build_time` | Type mismatch detection hints | ✅ | 4 | 3 | 3 | 36 | `src/build_helpers/type_analyzer.rs` |
| 28 | `build_time` | Build-time hint emission | ✅ | 5 | 3 | 3 | 45 | `src/build_helpers/hint_generator.rs` |
| 29 | `interactive` | REPL interactive mode | ✅ | 4 | 4 | 4 | 64 | `feature = repl`; `src/bin/unilang_cli.rs` |
| 30 | `interactive` | Enhanced REPL | ✅ | 3 | 3 | 3 | 27 | `feature = enhanced_repl`; readline support |
