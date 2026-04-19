# Parameter Syntax

How to pass values to named parameters in unilang commands.

---

## The `::` Operator

All named parameters use `name::value` — **double colon**, not single colon.

```bash
.greet name::Alice
.run file::./examples/plan.md
.search url::https://example.com/path?q=foo
```

The `::` is the **value-context operator**. Everything after `::` until the next
whitespace is treated as a single, opaque value. Special characters inside the value
are preserved exactly.

---

## Value Context Protection

Once `::` is seen, the parser enters value context. In value context, these characters
are **preserved as-is** rather than being interpreted as syntax:

| Character | Example | Notes |
|-----------|---------|-------|
| `/` | `path::dir/sub/file.md` | Path separators |
| `.` | `file::./relative.md` | Relative paths, file extensions |
| `#` | `url::https://host/page#anchor` | URL fragments |
| `?` | `url::https://host/search?q=foo` | Query strings |
| `&` | `url::https://host/?a=1&b=2` | Query parameter separators |
| `=` | `param::key=value` | Assignment-like values |
| `~` | `path::~/home/file` | Home directory shorthand |
| `-` | `flag::--verbose` | Flags and dashes |
| `_` | `id::some_snake_case` | Underscores |
| `:` | `addr::localhost:8080` | Single colon inside value is fine |

Value context ends at the **first whitespace** (space or newline). Everything after
that whitespace is parsed as the next token.

---

## File Paths

File paths — including those with slashes, dots, and directory traversal — work
correctly with `::` syntax:

```bash
# Relative paths
.run file::tests/data/input.yaml
.open path::../parent/dir/file.md

# Absolute paths
.open path::/home/user/project/plan.md
.open path::~/projects/plan.md

# Paths with dots and slashes
.run file::./examples/rust_learning.yaml
.run file::src/lib/module.rs
```

The common misconception that "the parser can't handle file paths" arose from using
single-colon syntax (`path:value`), which produces a parse error. The error is always
about the missing `::` operator, never about the path content itself.

---

## URLs

URLs containing `://`, `?`, `#`, and `&` are fully preserved:

```bash
.fetch url::https://api.example.com/v1/data?format=json&limit=50
.open  url::https://docs.rust-lang.org/book/ch01.html#summary
```

---

## Values With Spaces

When a value must contain whitespace, wrap it in quotes:

```bash
.greet name::"Alice Bob"
.find pattern::"error: not found"
.run  args::"--verbose --output /tmp/out"
```

The quotes are consumed by the parser; the command handler receives the value without
the surrounding quotes.

---

## Multiple Parameters

Parameters are separated by whitespace. Each uses its own `::` operator:

```bash
.plan.phases path::plan.md output_mode::file phase_id::0
.greet name::Alice lang::en
```

---

## Wrong Syntax — Single Colon

Single colon is **not valid** unilang syntax for named parameters. It produces a
parse error regardless of the value content:

```bash
# ❌ WRONG — produces "Unexpected token" parse error
.plan.phases path:tests/file.md
.greet name:Alice

# ✅ CORRECT — double colon required
.plan.phases path::tests/file.md
.greet name::Alice
```

When a command fails with "Parse error" or "Unexpected token," check for a missing
`::` before concluding that the parser can't handle the value type.

---

## Migration: Single Colon → Double Colon

If you have existing commands or scripts using single-colon syntax, the fix is
mechanical — add a second colon to every named parameter:

| Before (wrong) | After (correct) |
|----------------|-----------------|
| `path:tests/file.md` | `path::tests/file.md` |
| `name:Alice` | `name::Alice` |
| `url:https://x.com` | `url::https://x.com` |
| `mode:dry-run` | `mode::dry-run` |
| `limit:50` | `limit::50` |

---

## Positional Arguments

Some commands accept positional (unnamed) arguments — values without a `name::` prefix.
Whether a command supports positional arguments depends on its definition. Check command
help to see which parameters are positional:

```bash
.greet ??         # Show parameter list with types
.plan.phases ??   # Show all accepted parameters
```

---

## Discovering Valid Parameters

Use the `??` parameter on any command to list all accepted parameters:

```bash
.greet ??
# name (String, optional, default: "World")
# lang (String, optional, default: "en")
```

Use the `.command.help` counterpart for full descriptions:

```bash
.greet.help
.plan.phases.help
```

---

## Help Forms: `?`, `??`, and `.command.help`

Three distinct mechanisms trigger help display. They operate at different layers
and return different levels of detail:

| Form | Layer | How it works | Output |
|------|-------|--------------|--------|
| `?` | Parser | Sets `help_requested = true` in `ParsedInstruction` | Brief inline help |
| `??` | Framework | Positional argument value `"??"` detected by the framework | Parameter list with types and defaults |
| `.command.help` | Command registry | Separate command auto-registered alongside every `.command` | Full command documentation |

### Usage examples

```bash
# ? — parser-level help flag
.greet ?

# ?? — framework-level parameter listing
.greet ??

# .command.help — dedicated help command
.greet.help
```

### When to use which

- **`?`** — quick reminder that you need help with a command; lightest weight
- **`??`** — see parameter names, types, and defaults before constructing a call
- **`.command.help`** — full reference: description, examples, constraints

### Key differences

- `?` is tokenized by the parser (same layer as `::` and `!`); the application
  receives `instruction.help_requested = true` and decides what to show.
- `??` is NOT a parser operator — it is the literal string value `"??"` passed
  as a positional argument; the framework intercepts it before dispatch.
- `.command.help` is a **separate registered command** with its own dispatch
  path. It is automatically generated for every command when `auto_help_enabled`
  is set (the default).

---

## Troubleshooting

### "Parse error" or "Unexpected token"

Most likely cause: missing `::` in a named parameter.

```bash
# ❌ Fails with parse error
.run file:tests/data.yaml

# ✅ Fix: add the second colon
.run file::tests/data.yaml
```

### "Unknown parameter 'nam'"

Parameter name typo. Use `??` to see valid names:

```bash
.greet nam::Alice    # typo
.greet ??            # shows valid names
.greet name::Alice   # correct
```

### Value truncated at special character

Some shells expand or intercept characters before they reach the binary. Use quotes:

```bash
# Shell may expand or strip ?
.fetch url::https://example.com?q=foo

# Quotes prevent shell interference
.fetch url::"https://example.com?q=foo"
```
