# Static Data Module

Split from `static_data.rs`. Compile-time optimized static command definitions.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point, `Static*` struct definitions, `StaticCommandMap` |
| `private/conversions.rs` | `From` impls converting static structs to dynamic data types |
