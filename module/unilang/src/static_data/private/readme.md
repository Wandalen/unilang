# static_data/private

Internal implementation of static command data structures and conversions.

## Files

| File | Responsibility |
|------|----------------|
| `argument_types.rs` | `StaticArgumentDefinition` and `StaticArgumentAttributes` structs |
| `command_map.rs` | `StaticCommandMap` — PHF-based static command lookup |
| `conversions.rs` | `From` impls: static structs → dynamic `CommandDefinition` types |
