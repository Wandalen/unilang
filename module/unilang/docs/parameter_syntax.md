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
.greet ??         # Full help page: usage, arguments, examples
.plan.phases ??   # Same for a namespaced command
```

---

## Discovering Valid Parameters

Append an unquoted `??` to any command to see its full help page, or ask for one
parameter's detail page with `name::??`:

```bash
.greet ??            # command page: usage, all arguments with types and defaults
.greet name::??      # parameter detail page: kind, default, aliases, validation, examples
.greet bogus::??     # unknown name → listing of valid parameters (never a dead end)
```

The spelled `.command.help` counterpart renders the identical pages:

```bash
.greet.help          # same page as `.greet ??`
.greet.help name     # same page as `.greet name::??`
```

---

## Help Forms: `??` and `.command.help`

A single token — an unquoted `??` — covers every help surface; its position
selects the scope. The spelled `.command.help` routes render the identical pages:

| Form | Scope | Output |
|------|-------|--------|
| `??` (alone) | Global | Command listing — same as bare `.` |
| `.command ??` | Command | Full help page: usage, arguments, examples |
| `.command name::??` | Parameter | Detail page: kind, default, aliases, validation, examples |
| `.command.help` / `.command.help name` | Command / parameter | Byte-identical to the `??` forms above |

### Usage examples

```bash
??                   # list all commands
.greet ??            # command help page (any position works)
.greet name::??      # parameter detail page (aliases resolve too)
.greet.help          # spelled route — same page as `.greet ??`
.greet.help name     # spelled route — same page as `.greet name::??`
```

### Key rules

- `??` is NOT a parser operator — the parser passes it through as an ordinary
  token, and the **semantic analyzer** intercepts the unquoted form *before*
  argument binding. A broken sibling argument never masks a help request, and
  command routines never observe the token.
- A **named** `name::??` beats a positional `??`; with several named `??`, the
  first parameter in command-definition order wins; `alias::??` resolves to the
  canonical parameter; an unknown `name::??` lists the valid parameters.
- **Quoting opts out:** `name::"??"` binds the literal string `??` as a value.
- **Embedders can opt out entirely:** `Pipeline::with_help_detection( false )`
  (also on `SemanticAnalyzer`) turns every `??` back into an ordinary value.
- There is no `?` help form. `?` is an ordinary value; if it fails coercion, the
  error nudges: `Did you mean 'name::??' for parameter help?`
- `.command.help` is a **separate registered command** with its own dispatch
  path, automatically generated for every command — it renders through the same
  code path as `??`, which is what keeps the pages byte-identical.

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
