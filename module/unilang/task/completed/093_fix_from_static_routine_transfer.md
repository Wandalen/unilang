# Fix `From<StaticCommandRegistry>` Routine Transfer + Doctest

✅ (Completed)

<!-- task_metadata
value: 8
easiness: 7
priority: 2
safety: 8
advisability: 0
-->

## Goal

`Pipeline::from_static()` exists (pipeline.rs:604) but is currently **broken for any caller that
registers routines**: `From<StaticCommandRegistry> for CommandRegistry` silently drops all routines
(registry.rs:1367–1368). Fix the transfer and expose the method to do it. Also fix the
`Pipeline::from_static()` doctest from `ignore` to `no_run`.

Success: a `StaticCommandRegistry` with routines registered via `register_with_routine()` produces
a fully-functional `Pipeline` via `Pipeline::from_static()` — routines callable, no re-registration
required.

## In Scope

Three changes, all `#[cfg(feature = "static_registry")]`:

| File | Line(s) | Change |
|------|---------|--------|
| `src/registry.rs` | ~1460 impl block | Add `pub fn into_routines(self) -> HashMap<String, CommandRoutine>` to `StaticCommandRegistry` |
| `src/registry.rs` | 1345–1372 | Fix `From<StaticCommandRegistry> for CommandRegistry` to call `into_routines()` and re-register each routine |
| `src/pipeline.rs` | 591 | Change ` ```ignore ` → ` ```rust,no_run ` in `Pipeline::from_static()` doctest |

## Out of Scope

- Changing `Pipeline::from_static()` signature or behavior
- Changing how `register_with_routine()` works
- Migrating callers (will_clean handles its own ARCH-3 cleanup separately)
- Adding `into_routines()` without consuming self (a non-consuming accessor already exists: `get_routine()`)

## Root Cause

`From<StaticCommandRegistry> for CommandRegistry` (registry.rs:1346) only iterates
`static_reg.commands()` and registers command definitions. It intentionally skips routines
(comment: *"Routines are not transferred because they are runtime-specific"*).

This comment reflects an outdated design assumption. `StaticCommandRegistry` already stores
routines in its `routines: HashMap<String, CommandRoutine>` field (line 1453). Callers use
`register_with_routine()` (line 1614) to attach routines at registration time. Without
transferring these, `Pipeline::from_static()` produces a pipeline that can parse commands but
cannot execute any of them — silent, fatal functional failure.

The second issue: `routines` is a private field with no consuming accessor, so the `From` impl
cannot currently access them even if it wanted to. `get_routine(name)` exists but requires knowing
each name; there is no `into_routines()` or equivalent.

## Fix

**Step 1 — Add `into_routines()` to `StaticCommandRegistry`** (registry.rs, inside `#[cfg(feature = "static_registry")] impl StaticCommandRegistry`):

```rust
/// Consume this registry and return its routines map.
///
/// Used by `From<StaticCommandRegistry> for CommandRegistry` to transfer
/// routines without cloning.
#[ must_use ]
pub fn into_routines( self ) -> HashMap< String, CommandRoutine >
{
  self.routines
}
```

**Step 2 — Fix `From<StaticCommandRegistry> for CommandRegistry`** (registry.rs:1346):

```rust
#[ cfg( feature = "static_registry" ) ]
impl From< StaticCommandRegistry > for CommandRegistry
{
  fn from( static_reg : StaticCommandRegistry ) -> Self
  {
    #[ allow( deprecated ) ]
    let mut registry = CommandRegistry::new();

    for ( name, cmd ) in static_reg.commands()
    {
      if let Err( e ) = registry.register( cmd.clone() )
      {
        log::warn!(
          "Unexpected: Command '{}' failed during StaticCommandRegistry conversion: {}",
          name, e
        );
      }
    }

    // Transfer routines so callers don't need to re-register after conversion
    for ( name, routine ) in static_reg.into_routines()
    {
      registry.register_routine( name, routine );
    }

    registry
  }
}
```

**Step 3 — Fix doctest on `Pipeline::from_static()`** (pipeline.rs:591):

Change:
```
  /// ```ignore
```
to:
```
  /// ```rust,no_run
```

Add minimal hidden context so it compiles:
```
  /// ```rust,no_run
  /// # use unilang::pipeline::Pipeline;
  /// # use unilang::registry::StaticCommandRegistry;
  /// let static_registry = StaticCommandRegistry::new();
  /// let pipeline = Pipeline::from_static(static_registry);
  /// ```
```

## Requirements

- All work must adhere to applicable rulebooks (`kbase .rulebooks`)
- `into_routines()` must be `#[cfg(feature = "static_registry")]` and `#[must_use]`
- `From` fix must be inside the existing `#[cfg(feature = "static_registry")]` gate
- Doctest must use `rust,no_run`, not `ignore`
- No changes to existing public API signatures
- `cargo nextest run -p unilang --all-features` must pass

## Work Procedure

Execute in order.

1. **Read** `src/registry.rs` lines 1446–1720 (`StaticCommandRegistry` impl block).
2. **Add** `into_routines(self) -> HashMap<String, CommandRoutine>` method inside the impl.
3. **Fix** `From<StaticCommandRegistry> for CommandRegistry` (lines 1345–1372): call `into_routines()`, loop, register each routine.
4. **Read** `src/pipeline.rs` lines 575–610 (`from_static` method).
5. **Fix** doctest tag from `ignore` to `rust,no_run`; supply hidden context lines.
6. **Write tests** — see Test Matrix.
7. **Verify** — `RUSTFLAGS="-D warnings" cargo nextest run -p unilang --all-features`.
8. **Verify doctest** — `RUSTDOCFLAGS="-D warnings" cargo test --doc -p unilang --all-features`.
9. **Update task status** — set ✅, set advisability=0, move to `task/completed/`.

## Test Matrix

| # | Scenario | Expected |
|---|----------|----------|
| T01 | `StaticCommandRegistry::into_routines()` on empty registry | Returns empty `HashMap` |
| T02 | `into_routines()` after `register_with_routine()` for N commands | Returns `HashMap` with N entries |
| T03 | `From<StaticCommandRegistry> for CommandRegistry` preserves routines | `get_routine(name)` returns `Some` for every transferred routine |
| T04 | `Pipeline::from_static(reg)` with routines → dispatch command | Routine executes; correct output |
| T05 | Existing `Pipeline::new()` path unaffected | All pre-existing tests pass |
| T06 | Doctest on `Pipeline::from_static()` compiles under `RUSTDOCFLAGS="-D warnings"` | Exit 0, no `ignored` count |

## Acceptance Criteria

- `StaticCommandRegistry::into_routines()` is public under `static_registry` feature
- `From<StaticCommandRegistry> for CommandRegistry` transfers routines
- `Pipeline::from_static(reg)` produces a pipeline that can execute commands from `reg`
- `Pipeline::from_static()` doctest uses `rust,no_run` (not `ignore`)
- `RUSTFLAGS="-D warnings" cargo nextest run -p unilang --all-features` exits 0
- `RUSTDOCFLAGS="-D warnings" cargo test --doc -p unilang --all-features` exits 0

## Validation Checklist

- [ ] Does `StaticCommandRegistry` have `pub fn into_routines(self) -> HashMap<String, CommandRoutine>`?
- [ ] Is `into_routines` `#[must_use]` and inside `#[cfg(feature = "static_registry")]`?
- [ ] Does the `From` impl loop over `into_routines()` and call `register_routine()`?
- [ ] Does `pipeline.rs` `from_static` doctest use `rust,no_run`?
- [ ] Do all 6 test scenarios pass?
- [ ] Does `RUSTFLAGS="-D warnings" cargo nextest run -p unilang --all-features` exit 0?

## Consumer Impact

Directly unblocks **will_clean ARCH-3** tech debt: `will_clean/src/cli/parser.rs` forces 3×
duplication of routing logic (documented at lines 63–75) because `Pipeline::from_static()` was
non-functional without this fix. Once 0.49.0 ships with this fix:

1. will_clean upgrades to unilang 0.49.0
2. Replaces `Pipeline::new(registry)` (deprecated `CommandRegistry`) with
   `Pipeline::from_static(static_reg)` (modern `StaticCommandRegistry`)
3. Deletes the duplicate `register_routines()` in `cli/mod.rs` and `cli/parser.rs`

**Note**: `approach_yaml_multi_build` (will_clean's current feature) already enables
`static_registry` transitively — no feature change needed on the consumer side.
