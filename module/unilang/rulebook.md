# rulebook.md

Project-local override for the `unilang` crate. Per the Local-First Mandate, this file takes
precedence over global `$GENAI` rulebook conventions for this crate.

## Overrides

**Exception to `layout/l2_imp.rulebook.md` § Absolute Prohibitions (no test code in examples/):**

`examples/wasm-repl/` is a self-contained, independently-built nested example sub-crate, not
the parent crate's own example code. It is permitted to keep its own `tests/` directory
(currently `tests/wasm.rs`, browser-side WASM tests via `wasm-bindgen-test`), provided:

- It remains independently buildable: own `[package]` (`unilang-wasm-repl`), own `Cargo.lock`,
  and an explicit empty `[workspace]` block ("Empty workspace to avoid conflicts with parent
  workspace") — confirmed absent from the parent workspace's `[workspace] members` list
  (`unilang/dev/Cargo.toml`).
- Its `tests/` directory is never invoked by the parent `unilang` crate's own `cargo test` — it
  only runs via its own build tooling (e.g. `wasm-pack test`).
- Its tests exercise only the sub-crate's own WASM bindings code, not the parent `unilang`
  crate's internals directly.

If this sub-crate is ever added to the parent workspace's `members`, or its tests start
covering parent-crate logic, this exception no longer applies and its tests must move to a
location that doesn't sit under `examples/`.
