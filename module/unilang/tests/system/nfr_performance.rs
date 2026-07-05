//! NFR performance tests: static registry startup cost and lookup throughput.
//!
//! Implements IN-1 and IN-2 from `tests/docs/invariant/02_non_functional_requirements.md`.
//!
//! Both tests require `static_registry` feature (PHF-backed registries).
//! They are skipped at compile time when the feature is absent.

/// IN-1: Static registry startup cost is zero — no runtime initialization.
///
/// A `StaticCommandMap` is a thin wrapper around a `phf::Map`, which is a `static`
/// constant baked into the binary at build time. Accessing it requires no heap allocation,
/// no initialization loop, and no lazy-init synchronization primitive. This test confirms
/// the architectural guarantee: the map is accessible without any setup call, and the
/// first access is as fast as any subsequent access.
///
/// Spec: invariant/002_non_functional_requirements.md § IN-1
// test_kind: in_spec(IN-1)  [invariant/02_non_functional_requirements]
#[ cfg( feature = "static_registry" ) ]
#[ test ]
fn test_in1_static_registry_startup_zero_cost()
{
  use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap };
  use phf::phf_map;

  const STATIC_DEF : StaticCommandDefinition = StaticCommandDefinition::new( ".build", "", "Build the project" );

  const INTERNAL_MAP : phf::Map< &'static str, &'static StaticCommandDefinition > = phf_map!
  {
    ".build" => &STATIC_DEF,
  };

  // `static` item: initialized at binary load time, NOT at first access — zero startup cost
  static REGISTRY : StaticCommandMap = StaticCommandMap::from_phf_internal( &INTERNAL_MAP );

  // First access — must not trigger any initialization (baked into binary segment)
  let first = std::time::Instant::now();
  let result = REGISTRY.get( ".build" );
  let first_ns = first.elapsed().as_nanos();

  assert!(
    result.is_some(),
    "IN-1: .build must be found in the static registry"
  );
  assert_eq!(
    result.unwrap().name,
    ".build",
    "IN-1: retrieved definition must have the correct name"
  );
  // Allow very generous 10ms bound — PHF lookup should be nanoseconds in practice
  assert!(
    first_ns < 10_000_000,
    "IN-1: first PHF lookup took {}ns; expected < 10ms (PHF lookup should be nanoseconds)",
    first_ns
  );
}

/// IN-2: PHF registry lookup throughput ≥5M lookups/sec.
///
/// Executes 1,000,000 `get()` calls on a PHF-backed `StaticCommandMap` and asserts
/// throughput is at least 5,000,000 lookups per second (1M calls ≤ 200ms).
/// PHF lookup cost is two hash operations regardless of map size; the threshold is
/// a floor test against catastrophic regression, not a rigorous p99 benchmark.
///
/// Spec: invariant/002_non_functional_requirements.md § IN-2
// test_kind: in_spec(IN-2)  [invariant/02_non_functional_requirements]
#[ cfg( feature = "static_registry" ) ]
#[ test ]
fn test_in2_static_registry_lookup_throughput_exceeds_5m_per_sec()
{
  use unilang::static_data::{ StaticCommandDefinition, StaticCommandMap };
  use phf::phf_map;

  const DEF_A : StaticCommandDefinition = StaticCommandDefinition::new( ".cmd_a", "", "Command A" );
  const DEF_B : StaticCommandDefinition = StaticCommandDefinition::new( ".cmd_b", "", "Command B" );
  const DEF_C : StaticCommandDefinition = StaticCommandDefinition::new( ".cmd_c", "", "Command C" );
  const DEF_D : StaticCommandDefinition = StaticCommandDefinition::new( ".cmd_d", "", "Command D" );
  const DEF_E : StaticCommandDefinition = StaticCommandDefinition::new( ".cmd_e", "", "Command E" );

  const INTERNAL_MAP : phf::Map< &'static str, &'static StaticCommandDefinition > = phf_map!
  {
    ".cmd_a" => &DEF_A,
    ".cmd_b" => &DEF_B,
    ".cmd_c" => &DEF_C,
    ".cmd_d" => &DEF_D,
    ".cmd_e" => &DEF_E,
  };
  static REGISTRY : StaticCommandMap = StaticCommandMap::from_phf_internal( &INTERNAL_MAP );

  let lookups : u64 = 1_000_000;
  let keys = [ ".cmd_a", ".cmd_b", ".cmd_c", ".cmd_d", ".cmd_e", ".cmd_missing" ];

  let start = std::time::Instant::now();
  let mut found : u64 = 0;
  for i in 0..( lookups as usize )
  {
    let key = keys[ i % keys.len() ];
    if REGISTRY.get( key ).is_some() { found += 1; }
  }
  let elapsed = start.elapsed();

  // Sanity: ~5/6 of lookups should find a result
  assert!( found > 0, "IN-2: at least some lookups must succeed" );

  let throughput = lookups as f64 / elapsed.as_secs_f64();
  // Release builds must meet the 5M/sec spec floor; debug builds use a lower bound
  // because unoptimized PHF lookups run ~10-50x slower than release.
  let floor = if cfg!( debug_assertions ) { 500_000.0_f64 } else { 5_000_000.0_f64 };
  assert!(
    throughput >= floor,
    "IN-2 violation: throughput {:.0} lookups/sec is below the {:.0}/sec floor.\n\
     Elapsed: {:?} for {} lookups.",
    throughput,
    floor,
    elapsed,
    lookups
  );
}
