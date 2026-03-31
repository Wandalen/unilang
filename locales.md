> **Generated.** Do not edit manually. Maintained by `.locale.doc.generate`.
> Source of truth: `locales.config.yml` + `.persistent/locale.toml`.

# Locales — unilang

A **locale** is a named, bounded directory representing a self-contained unit of development work. See [`willbe/locate/module/locate/docs/locale.md`](../../willbe/locate/module/locate/docs/locale.md) for the full specification.

All paths are relative to `~/pro/lib/wip_core/unilang/dev`. `task` — Y = `task/` directory initialized.

## Summary

| # | rel-path | name | type | lang | purpose | task | last_active |
|---|----------|------|------|------|---------|------|-------------|
| 1 | `module/unilang` | unilang | rust_crate | rs | — | N | 2026-02-07 |
| 2 | `module/unilang_parser` | unilang_parser | rust_crate | rs | — | N | 2026-02-07 |
| 3 | `module/unilang_meta` | unilang_meta | rust_crate | rs | Proc-macro support for unilang | N | 2026-02-07 |
| 4 | `module/cargo_unilang` | cargo_unilang | rust_crate | rs | Scaffolding and health check tool for unilang CLI projects | N | 2026-02-07 |

---

## Profile

### workspace :: unilang

| field | value |
|-------|-------|
| path | `lib/wip_core/unilang/dev` |
| parent | `lib/wip_core` |
| type | rust_workspace |
| lang | rs |
| canonical | Y |
| task | N |
| last_active | 2026-02-07 |

**Purpose.** Implements the Unilang universal configuration format. Core parser (`unilang`, `unilang_parser`), metadata layer (`unilang_meta`), and Cargo integration (`cargo_unilang`). Supports JSON, YAML, TOML, RON interchange with a unified schema.
