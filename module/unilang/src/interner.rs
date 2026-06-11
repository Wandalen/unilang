//! String Interning System
//!
//! This module provides high-performance string interning to optimize command name construction
//! in the semantic analysis hot path. Instead of repeatedly constructing command name strings
//! like ".command.subcommand", we cache them and return references to avoid allocation overhead.
//!
//! Performance target: 5-10x improvement in command name construction (38K → 190K-380K cmd/sec)

/// Internal namespace.
mod private
{
  use std::collections::HashMap;
  use std::sync::RwLock;

  /// Maximum number of strings to cache before evicting oldest entries
  const DEFAULT_CACHE_SIZE_LIMIT : usize = 10_000;

  /// Thread-safe string interner that caches strings and returns 'static references.
  ///
  /// Uses `Box::leak()` to produce `&'static str` references for zero-copy command name lookups.
  ///
  /// **Memory model:** Every unique string interned via `Box::leak()` is a permanent heap
  /// allocation that cannot be freed for the lifetime of the process. The LRU eviction policy
  /// only limits the size of the lookup cache (the HashMap); it does NOT reclaim the leaked
  /// memory. After `size_limit` unique strings have been interned, subsequent entries are dropped
  /// from the cache but their heap allocations remain. Use only for a bounded set of strings
  /// (e.g., command names known at startup) to avoid unbounded heap growth.
  #[ derive( Debug ) ]
  pub struct StringInterner
  {
    /// Storage for interned strings with thread-safe access
    storage : RwLock< InternerStorage >,
    /// Maximum cache size before eviction
    size_limit : usize,
  }

  #[ derive( Debug ) ]
  struct InternerStorage
  {
    /// Maps strings to their interned 'static references
    cache : HashMap< String, &'static str >,
    /// LRU access order tracking for eviction policy
    access_order : Vec< String >,
  }

  impl StringInterner
  {
    /// Creates a new string interner with default size limits.
    #[must_use] pub fn new() -> Self
    {
      Self::with_capacity( DEFAULT_CACHE_SIZE_LIMIT )
    }

    /// Creates a new string interner with specified cache capacity.
    #[must_use] pub fn with_capacity( size_limit : usize ) -> Self
    {
      Self
      {
        storage : RwLock::new( InternerStorage
        {
          cache : HashMap::new(),
          access_order : Vec::new(),
        }),
        size_limit,
      }
    }

    /// Interns a string, returning a 'static reference for zero-copy usage.
    ///
    /// If the string is already cached, returns the existing reference.
    /// Otherwise, allocates the string permanently on the heap via `Box::leak()` and caches it.
    /// Note: leaked allocations are never freed — only intern strings from a bounded, known set.
    #[allow(clippy::missing_panics_doc)]
    pub fn intern( &self, s : &str ) -> &'static str
    {
      // Fast path: check if already cached with read lock
      {
        let storage = self.storage.read().unwrap();
        if let Some( &interned ) = storage.cache.get( s )
        {
          return interned;
        }
      }

      // Slow path: insert new string with write lock
      let mut storage = self.storage.write().unwrap();
      
      // Double-check in case another thread inserted while waiting for write lock
      if let Some( &interned ) = storage.cache.get( s )
      {
        return interned;
      }

      // Permanently leak this allocation to produce a &'static str.
      // The LRU eviction below only removes this entry from the lookup cache;
      // the heap memory itself is never freed (Box::leak is irreversible).
      let interned : &'static str = Box::leak( s.to_string().into_boxed_str() );
      
      // Insert into cache
      storage.cache.insert( s.to_string(), interned );
      storage.access_order.push( s.to_string() );

      // Evict oldest entries if cache is too large
      if storage.cache.len() > self.size_limit
      {
        let cache_len = storage.cache.len();
        let to_remove = storage.access_order.drain( 0..( cache_len - self.size_limit ) ).collect::< Vec< _ > >();
        for key in to_remove
        {
          storage.cache.remove( &key );
        }
      }

      interned
    }

    /// Optimized command name construction and caching.
    /// 
    /// Constructs command names in the format ".command.subcommand" directly
    /// without intermediate string allocations when possible.
    #[allow(clippy::missing_panics_doc)]
    pub fn intern_command_name( &self, path_slices : &[ &str ] ) -> &'static str
    {
      if path_slices.is_empty()
      {
        return self.intern( "." );
      }

      // Handle the case where first slice is empty (leading dot)
      let effective_slices = if path_slices[ 0 ].is_empty() && path_slices.len() > 1
      {
        &path_slices[ 1.. ]
      }
      else
      {
        path_slices
      };

      // Construct command name with leading dot
      let command_name = format!( ".{}", effective_slices.join( "." ) );
      self.intern( &command_name )
    }

    /// Returns current cache statistics for monitoring and debugging.
    #[allow(clippy::missing_panics_doc)]
    pub fn stats( &self ) -> InternerStats
    {
      let storage = self.storage.read().unwrap();
      InternerStats
      {
        cached_strings : storage.cache.len(),
        size_limit : self.size_limit,
        memory_usage_estimate : storage.cache.iter()
          .map( | ( k, v ) | k.len() + v.len() )
          .sum::< usize >(),
      }
    }

    /// Clears all cached strings. Useful for testing and memory management.
    #[allow(clippy::missing_panics_doc)]
    pub fn clear( &self )
    {
      let mut storage = self.storage.write().unwrap();
      storage.cache.clear();
      storage.access_order.clear();
    }
  }

  impl Default for StringInterner
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Statistics about the string interner's current state.
  #[ derive( Debug, Clone ) ]
  pub struct InternerStats
  {
    /// Number of strings currently cached
    pub cached_strings : usize,
    /// Maximum cache size before eviction
    pub size_limit : usize,
    /// Estimated memory usage in bytes
    pub memory_usage_estimate : usize,
  }

  /// Global singleton interner for use throughout the application.
  /// 
  /// Using a global instance reduces the need to thread the interner through
  /// all APIs while maintaining the performance benefits.
  static GLOBAL_INTERNER : std::sync::OnceLock< StringInterner > = std::sync::OnceLock::new();

  /// Returns a reference to the global string interner instance.
  pub fn global_interner() -> &'static StringInterner
  {
    GLOBAL_INTERNER.get_or_init( StringInterner::new )
  }

  /// Convenience function to intern a string using the global interner.
  #[must_use] #[allow(clippy::missing_panics_doc)]
 pub fn intern( s : &str ) -> &'static str
  {
    global_interner().intern( s )
  }

  /// Convenience function to intern command names using the global interner.
  #[must_use] #[allow(clippy::missing_panics_doc)]
 pub fn intern_command_name( path_slices : &[ &str ] ) -> &'static str
  {
    global_interner().intern_command_name( path_slices )
  }

}

mod_interface::mod_interface!
{
  exposed use private::StringInterner;
  exposed use private::InternerStats;
  exposed use private::global_interner;
  exposed use private::intern;
  exposed use private::intern_command_name;
}