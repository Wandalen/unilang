# Troubleshooting

Common issues and solutions when using unilang.

## "Command not found" error

**Symptom:** `Error: Command '.greet' not found in registry`

**Cause:** Command name missing dot prefix.

```yaml
# Wrong
- name: "greet"

# Correct
- name: ".greet"
```

All command names MUST start with `.` (dot prefix).

---

## Type mismatch warnings during build

**Symptom:** Type hint warnings during `cargo build`

**Cause:** YAML default values quoted when they shouldn't be.

```yaml
# Wrong — quoted values
- name: "dry"
  kind: "Boolean"
  attributes:
    default: 'false'     # String, not Boolean!

# Correct — unquoted primitives
- name: "dry"
  kind: "Boolean"
  attributes:
    default: false       # Boolean type
```

Rule: Boolean, Integer, Float values must NOT be quoted in YAML.

---

## Build fails with "could not find static_commands.rs"

**Symptom:** `error: couldn't read .../static_commands.rs: No such file or directory`

**Cause:** Wrong feature flag or missing YAML command file.

Ensure you have at least one `.yaml` file with commands, or enable a build-time approach:

```toml
[dependencies]
unilang = "0.35"  # Default includes approach_yaml_multi_build
```

---

## Performance not improved after migration

**Symptom:** Lookups still slow after switching to build-time approach.

**Cause:** Using runtime registration in production CLI (10-50x performance penalty).

```rust,ignore
// Wrong for production CLIs — 10-50x slower
let registry = CommandRegistry::new();

// Correct — use static registry
let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
```

See [architecture/007_migration_guide.md](architecture/007_migration_guide.md) for full migration guide.

---

## "Unknown parameter" error

**Symptom:** `Error: Unknown parameter 'nam'`

The error message includes a suggestion: `Did you mean 'name'?`

Parameter names are validated strictly. Use the `??` token to see valid parameters:

```bash
.greet ??
```

---

## "Parse error" when passing a file path

**Symptom:** A command fails with "Parse error" or "Unexpected token" when you include a file path.

**Cause:** Single colon (`path:value`) is not valid unilang syntax. The missing second colon
causes the parser to reject the input before even reading the path value. The parser is not
rejecting the path — it never reaches the path.

```bash
# ❌ Wrong — produces parse error (invalid syntax)
.run file:tests/data/input.yaml

# ✅ Correct — double colon activates value context
.run file::tests/data/input.yaml
```

The `::` operator enables value context, which preserves `/`, `.`, `#`, `?` and other
special characters inside the value. File paths, URLs, and any string work as values.

See [parameter_syntax.md](parameter_syntax.md) for the full reference.

---

## Argument value not received by handler

**Symptom:** A command runs but the handler receives `None` for a parameter you provided.

**Cause:** If you wrote `name:value` (single colon), the parser did not create a named
argument for `name`. Instead it likely failed to parse, or parsed `name:value` as a
different token type entirely. The handler then sees the argument as missing.

```bash
# ❌ Wrong — 'name' argument will be missing in handler
.greet name:Alice

# ✅ Correct
.greet name::Alice
```

Check: if a required argument is missing, verify the command string uses `::` not `:`.

---

## Still having issues?

1. Run `cargo run --example static_01_basic_compile_time`
2. Enable verbose logging: `RUST_LOG=debug`
3. Verify feature flags: `cargo tree -f "{p} {f}"`
4. See [quick_start.md](quick_start.md) for the complete setup guide
