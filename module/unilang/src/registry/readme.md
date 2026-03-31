# Registry Module

Split from `registry.rs`. Manages command registration, lookup, and routing.

## Files

| File | Responsibility |
|------|----------------|
| `mod.rs` | Module entry point and public re-exports |
| `traits.rs` | `CommandRoutine`, `RegistryMode`, `CommandRegistryTrait`, help formatting |
| `metrics.rs` | `PerformanceMetrics` for tracking lookup statistics |
| `map.rs` | `DynamicCommandMap` — LRU-cached command storage |
| `builder.rs` | `CommandRegistryBuilder` — fluent registry construction |
| `dynamic.rs` | `CommandRegistry` — runtime dynamic command registry |
| `static_reg.rs` | `StaticCommandRegistry` — PHF static + dynamic hybrid registry |
| `bridge.rs` | `From<StaticCommandRegistry> for CommandRegistry` conversion |
