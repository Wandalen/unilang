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

See [migration.md](migration.md) for full migration guide.

---

## "Unknown parameter" error

**Symptom:** `Error: Unknown parameter 'nam'`

The error message includes a suggestion: `Did you mean 'name'?`

Parameter names are validated strictly. Use `??` operator to see valid parameters:

```bash
.greet ??
```

---

## Still having issues?

1. Run `cargo run --example static_01_basic_compile_time`
2. Enable verbose logging: `RUST_LOG=debug`
3. Verify feature flags: `cargo tree -f "{p} {f}"`
4. See [quick_start.md](quick_start.md) for the complete setup guide
