# CLI Aggregation

unilang excels at aggregating multiple CLI tools into a single unified command interface with namespace isolation and conflict detection.

**Feature required:** `multi_file` (automatically enabled by default `approach_yaml_multi_build`)

## Basic Usage

```rust,ignore
use unilang::multi_yaml::CliBuilder;

let unified_cli = CliBuilder::new()
  .static_module_with_prefix( "database", "db", database_commands )
  .static_module_with_prefix( "filesystem", "fs", file_commands )
  .static_module_with_prefix( "network", "net", network_commands )
  .detect_conflicts( true )
  .build_static();
```

**Before aggregation:**
```bash
db-cli migrate --direction up
file-cli copy --src ./source --dest ./target
net-cli ping google.com --count 10
```

**After aggregation:**
```bash
unified-cli .db.migrate direction::up
unified-cli .fs.copy source::./source destination::./target
unified-cli .net.ping host::google.com count::10
```

## Key Features

### Namespace Isolation

Each CLI module maintains its own namespace with automatic prefix application:

```text
Database commands → .db.migrate, .db.backup
File commands     → .fs.copy, .fs.move
Network commands  → .net.ping, .net.trace
```

### Conflict Detection

```rust,ignore
let registry = CliBuilder::new()
  .static_module_with_prefix( "tools", "tool", cli_a_commands )
  .static_module_with_prefix( "utils", "tool", cli_b_commands )  // Conflict!
  .detect_conflicts( true )  // Caught at build time
  .build_static();
```

### Hybrid Loading

```rust,ignore
let registry = CliBuilder::new()
  .static_module_with_prefix( "core", ".core", core_commands )
  .dynamic_module_with_prefix( "plugins", PathBuf::from( "plugins.yaml" ), ".plugins" )
  .build_hybrid();
```

- `static_module_with_prefix(name, prefix, Vec<CommandDefinition>)` → O(1) lookup (~80-100ns)
- `dynamic_module_with_prefix(name, PathBuf, prefix)` → runtime load (~4,000ns, 50x slower)

## Performance

| Approach | Lookup Time | Conflict Detection |
|----------|-------------|-------------------|
| Build-Time | O(1) static ~80ns | Build-time |
| Runtime | O(log n) | Runtime |

With 10 modules and 100 commands each: ~80-100ns lookup regardless of module count.

## Help Integration

All aggregated commands support unified help:

```bash
unified-cli .db.migrate.help       # Detailed help
unified-cli .fs.copy ??            # Interactive help
unified-cli .net.ping ?            # Traditional operator
```

## Complete Example

See `examples/practical_cli_aggregation.rs`:

```bash
cargo run --example practical_cli_aggregation
```
