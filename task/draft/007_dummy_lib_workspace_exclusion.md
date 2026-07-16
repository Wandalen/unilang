# 007: Fix dummy_lib workspace membership configuration

## Execution State

- **Executor Type:** any
- **filed_by:** user1@w002/home/user1/pro/lib/yrd_core/unilang/dev/
- **actor:** null
- **started_at:** null
- **expires_at:** null
- **round:** 1
- **state:** 📝 (Draft)
- **closes:** null
- **unit_type:** workspace
- **unit:** lib/yrd_core/unilang/dev
- **validated_by:** null
- **validation_date:** null
- **blocked_by:** null

## Goal

`module/unilang/tests/dynamic_libs/dummy_lib/Cargo.toml` is missing workspace-exclusion handling: building it directly (`cargo check --manifest-path module/unilang/tests/dynamic_libs/dummy_lib/Cargo.toml`) fails with "current package believes it's in a workspace when it's not" because it's a nested `Cargo.toml` under `module/unilang/tests/` that is neither listed in the root `Cargo.toml`'s `[workspace.members]` nor excluded via `[workspace.exclude]`, and it lacks its own empty `[workspace]` table (contrast: `module/unilang/examples/wasm-repl/Cargo.toml` has an explicit empty `[workspace]` table with a documented rationale comment for exactly this situation).

Discovered incidentally on 2026-07-16 while verifying a workspace dependency update didn't break the two non-member crates nested in `module/unilang/` (`examples/wasm-repl` and `tests/dynamic_libs/dummy_lib`).

Currently dormant/non-blocking: grep across `module/unilang/tests/*.rs` and `src/` finds zero references to `dummy_lib` or `dynamic_libs` outside two `readme.md` mentions, so nothing in the passing test suite currently invokes it — but the fixture as configured cannot be built stand-alone the way its sibling `wasm-repl` can.

Needs a decision among 3 remediation options before fixing:
- (a) add an empty `[workspace]` table to `dummy_lib/Cargo.toml` matching the `wasm-repl` pattern
- (b) add it to the root `Cargo.toml`'s `[workspace.exclude]`
- (c) add it to `[workspace.members]` — this last option would pull it into workspace-wide commands like `cargo test --workspace`, a behavior change beyond just fixing the standalone-build error, so is likely not the intended fix

## History

- **[2026-07-16]** `FILED` — Task filed by user1@w002/home/user1/pro/lib/yrd_core/unilang/dev/. Goal: fix dummy_lib's missing workspace-exclusion configuration so it can be built standalone like wasm-repl.
