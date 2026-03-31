# PHF Re-export

When using the `static_registry` feature, unilang generates code using Perfect Hash Functions (PHF) for compile-time command lookup. To simplify dependency management, unilang re-exports PHF types publicly via `unilang::phf`.

## Usage

```toml
[dependencies]
unilang = { version = "0.46", features = ["static_registry"] }
# No phf dependency needed — it's re-exported by unilang
```

```rust,ignore
use unilang::phf::{ self, Map };

static COMMANDS : Map< &str, u32 > = phf::phf_map!
{
  "help" => 1,
  "version" => 2,
};
```

**Important:** Import the `phf` module itself with `self` and use qualified macro calls (`phf::phf_map!`), not `phf_map!` directly.

## Migration from Direct PHF Dependency

**Before:**
```toml
unilang = { version = "0.45", features = ["static_registry"] }
phf = "0.11"  # Remove this
```

**After:**
```toml
unilang = { version = "0.46", features = ["static_registry"] }
# No phf needed
```

**Import update:**
```rust,ignore
// Before
use phf::{ phf_map, Map };
static MAP : Map< &str, i32 > = phf_map! { "key" => 1 };

// After
use unilang::phf::{ self, Map };
static MAP : Map< &str, i32 > = phf::phf_map! { "key" => 1 };
```

## Troubleshooting

**`unresolved import unilang::phf`** — Enable the `static_registry` feature.

**`multiple candidates for Map`** — Remove direct `phf` dependency; use `unilang::phf::Map` exclusively.

**`cannot find macro phf_map`** — Import both module and macro:
```rust,ignore
use unilang::phf::{ phf_map, Map };
```
