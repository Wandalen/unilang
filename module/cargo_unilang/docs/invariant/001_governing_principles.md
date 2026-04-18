# Invariant: Governing Principles

`cargo_unilang` must be detection-only (no auto-fix), meta-compliant (it must itself use `unilang` as its CLI framework), and scoped exclusively to scaffolding and anti-pattern detection for `unilang`-based projects — general-purpose Rust scaffolding, auto-correction of detected issues, and IDE integration are permanently out of scope.
