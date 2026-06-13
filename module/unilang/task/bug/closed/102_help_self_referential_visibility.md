# BUG-102: .help command visible in its own help listing

- **Severity:** Minor
- **State:** Fixed
- **Affects:** Any unilang-based CLI — `.help` output includes `.help` itself under the "Help:" category, inflating visible command count by 1
- **Component:** `src/registry/dynamic.rs` — `register_mandatory_global_help_command()`
- **Filed:** 2026-05-23

---

## Symptom

```bash
$ dream .help
# ...
Help:
  .help                 List available commands with descriptions
# ^^^ .help should NOT appear in its own listing — self-referential noise

# Expected: 32 visible commands (33 registered, 1 hidden)
# Actual:   33 visible commands (33 registered, 0 hidden)
```

The `.help` command lists itself under the "Help:" category. This is self-referential — the user
is already running `.help`, so showing it in the output adds no information. The visible command
count is 33 instead of the expected 32.

## Impact

Every invocation of `.help` in any unilang-based CLI shows the self-referential entry. No
functional data is lost — all other commands are listed correctly — but the inflated count
breaks downstream assertions (dream smoke test expects 32 visible commands, gets 33).

Entity scope: `None`.

## How Discovered

```bash
$ cd /home/user1/pro/lib/wip_core/willbe/dev/module/dream
$ cargo nextest run smoke -- smoke_test
# FAILED: assert_eq!( command_count, 32, ...) — got 33
# Dream smoke test at tests/smoke.rs:192 caught the count mismatch
```

## Minimum Reproducible Example

```bash
# Requires the `dream` binary (built from the dream crate, which uses unilang)
# The bug is in unilang's register_mandatory_global_help_command(), but dream
# is the concrete reproducer since unilang is a library crate.

mkdir -p /tmp/mre102
cd /tmp/mre102

# Run .help and check if .help appears in its own output
dream .help 2>&1 | grep -c '\.help'
# Expected: 0 (or only in header text, not as a listed command)
# Actual: 1+ (appears as a listed command under "Help:" category)

# Verify the visible command count
dream .help 2>&1 | grep -E '^\s+\.' | wc -l
# Expected: 32
# Actual: 33
```

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|--------|---------|----------|
| H1 | `.help` registered with `hidden_from_list: false` | ✅ Root Cause | `dynamic.rs:583` sets `false`; `help.rs:705` filters by `!cmd.hidden_from_list()` | E1, E2, E3 |
| H2 | Override from YAML/static/other registration path makes `.help` visible | ❌ Disproved | No `.help` in YAML; `register_with_routine` rejects duplicates; `from_static_commands` skips if exists | E4, E5 |
| H3 | Filter logic in `list_commands_filtered` ignores the `hidden_from_list` flag | ❌ Disproved | `help.rs:705` unconditionally checks `!cmd.hidden_from_list()`; `accessors.rs:142` is direct field return | E3 |
| H4 | Double registration — second registration with `hidden_from_list: false` wins | ❌ Disproved | `register_with_routine` returns `CommandAlreadyExists`; `from_static_commands` skips `.help`; single registration path | E5, E6 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `src/registry/dynamic.rs:583` (committed) | `.with_hidden_from_list( false )` — explicitly sets `.help` as visible | ✅ H1 Root Cause |
| E2 | `src/interpreter/interpreter.rs:84-87` | `.help` invocation special-cased: calls `list_commands_filtered(None)` | ✅ H1 Root Cause |
| E3 | `src/help/help.rs:705` | Filter: `let is_visible = !cmd.hidden_from_list()` — unconditional check; no bypass | ✅ H1 Root Cause, ❌ H3 |
| E4 | dream YAML files | No `.help` definition in any YAML command file | ❌ H2 |
| E5 | `src/registry/dynamic.rs:229-234` | `register_with_routine` rejects duplicates with `CommandAlreadyExists` error | ❌ H2, ❌ H4 |
| E6 | `src/registry/dynamic.rs:697-701` | `from_static_commands` explicitly skips `.help` if it already exists | ❌ H4 |

## Root Cause

```
CommandRegistry::new()                          (dynamic.rs:80)
  → register_mandatory_global_help_command()    (dynamic.rs:560-613)
    → CommandBuilder::new(".help")
      .with_hidden_from_list( false )           (dynamic.rs:583) ← BUG: should be true
      .build()
    → self.register_with_routine(.help_cmd)

When .help is invoked:
  interpreter.rs:84  → special-case match for ".help"
  interpreter.rs:87  → HelpGenerator::list_commands_filtered(None)  (help.rs:689)
  help.rs:705        → let is_visible = !cmd.hidden_from_list()
                     → false → !false → true  ← .help passes filter
                     → .help appears in output under "Help:" category
```

The `.help` command is registered with `hidden_from_list: false` (visible), but it should be
`true` (hidden) so it does not appear in its own listing. The filter at `help.rs:705`
correctly checks the flag — the flag value itself is wrong at the registration site.

## Why Not Caught

No test in unilang's test suite verifies that `.help` does NOT appear in its own help listing.
The dream crate's `tests/smoke.rs:192` catches the count mismatch (`assert_eq!( command_count, 32, ...)`)
but lives in a different crate, so it was not run during unilang-only development. The
`register_mandatory_global_help_command()` function has no unit test for the `hidden_from_list`
attribute value.

## Fix Location

`src/registry/dynamic.rs:583`:

```rust
// Before:
.with_hidden_from_list( false )

// After:
.with_hidden_from_list( true )
```

Single-line change. No other files affected.

## Prevention

1. Add a unit test in unilang's test suite that asserts `.help` does NOT appear in the output
   of `list_commands_filtered(None)` — verifying the hidden flag is correctly set.
2. When registering internal/meta commands (`.help`, `.version`, etc.), default to
   `hidden_from_list: true` and require an explicit justification comment for `false`.
3. Any command whose sole purpose is to list other commands should be hidden from its own
   output — self-referential listings add no information.

**Pitfall:** A boolean flag defaulting to the wrong value (`false` instead of `true`) is
invisible at the registration site — only observable in the rendered output, which has no
automated assertion.

## Generalized Version

**Broken assumption:** Meta-commands that enumerate other commands should not enumerate themselves.

Fails when:
1. A meta-command (e.g., `.help`) is registered with `hidden_from_list: false`, AND
2. The listing filter uses `hidden_from_list` as its sole visibility gate, AND
3. No test asserts the meta-command is absent from its own output

**Detection invariant:**
```
for each command C where C.purpose == "list commands":
  assert C.hidden_from_list == true
  assert C not in C.output()
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2026-05-23 | filed | Confirmed via dual-agent validation; root cause at `dynamic.rs:583` |
| 2026-05-23 | fixed | `dynamic.rs:583`: `.with_hidden_from_list( false )` → `true`; `bug_reproducer(BUG-102)` test added to `tests/help/enforcement.rs`; verified 153/153 nextest pass in container (task 103) |
