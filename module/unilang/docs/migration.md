# Migration: Runtime to Build-Time Registration

Migrate from runtime command registration (10-50x slower) to build-time registration (⚡ 50x faster).

## When to Migrate

**Use build-time (compile-time) when:**
- ⚡ Production CLIs — performance-critical applications
- ⚡ Large command sets (100+ commands benefit from ~80ns PHF lookups)
- ⚡ Embedded systems — zero-overhead static dispatch

**Keep runtime registration when:**
- ✅ REPL applications — commands defined interactively
- ✅ Plugin systems — commands loaded dynamically
- ✅ Prototyping — rapid iteration

---

## Step 1: Extract Command Definitions to YAML

**Before (Runtime):** ⚠️ Not recommended for production CLIs (10-50x slower)

```rust,ignore
let mut registry = CommandRegistry::new();

let greet_cmd = CommandDefinition {
  name: ".greet".to_string(),
  description: "Greeting command".to_string(),
  arguments: vec![
    ArgumentDefinition {
      name: "name".to_string(),
      kind: Kind::String,
      attributes: ArgumentAttributes {
        optional: true,
        default: Some("World".to_string()),
        ..Default::default()
      },
      ..Default::default()
    }
  ],
  ..Default::default()
};

registry.command_add_runtime(&greet_cmd, greet_routine)?;
```

**After (Build-Time), `unilang.commands.yaml`:**

```yaml
- name: ".greet"
  description: "Greeting command"
  arguments:
    - name: "name"
      kind: "String"
      attributes:
        optional: true
        default: "World"
```

## Step 2: Update Cargo.toml

```toml
[dependencies]
# Default already includes approach_yaml_multi_build
unilang = "0.35"
```

## Step 3: Configure Build Script (Single-File Only)

For `approach_yaml_single_build` only — multi-YAML approach needs no `build.rs`:

```rust,ignore
fn main()
{
  println!( "cargo:rerun-if-changed=unilang.commands.yaml" );
}
```

## Step 4: Update Code to Use Static Registry

**Before (Runtime):**

```rust,ignore
let mut registry = CommandRegistry::new();
registry.command_add_runtime(&greet_cmd, greet_routine)?;
let pipeline = Pipeline::new(registry);
```

**After (Build-Time):**

```rust,ignore
use unilang::prelude::*;

include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  let registry = StaticCommandRegistry::from_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );
  let result = pipeline.process_command_simple( ".greet name::Alice" );
  Ok( () )
}
```

## Step 5: Verify Performance

```bash
cargo run --example static_03_performance_comparison
```

Expected results:
- Runtime registration: ~4,000ns per lookup
- Compile-time registration: ~80-100ns per lookup
- Performance gain: ~50x
