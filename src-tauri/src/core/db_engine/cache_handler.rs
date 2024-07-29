// NXP-REQ-CORE-DB-FUN-H-5
// NXP-REQ-CORE-DB-FUN-H-6
// NXP-REQ-CORE-DB-FUN-H-8: Cache Handler shall maintain a separate cache for each table.
// NXP-REQ-CORE-DB-FUN-H-9: Cache Handler shall invalidate the cache entries related to a specific table upon update.
// NXP-REQ-CORE-DB-FUN-H-10: Cache Handler shall provide a function to clear the entire cache.

use std::collections::HashMap;
use std::num::NonZeroUsize;

use lru::LruCache;

use super::{Query, QueryResult};

pub struct CacheHandler {
    // A hashmap to store separate caches for each table
    caches: HashMap<String, LruCache<String, QueryResult>>,
    capacity: NonZeroUsize,
}

impl CacheHandler {
    // NXP-REQ-CORE-DB-FUN-H-11: Cache Handler shall initialize with a given capacity for each table's cache.
    pub fn new(capacity: usize) -> Self {
        Self {
            caches: HashMap::new(),
            capacity: NonZeroUsize::new(capacity).unwrap(),
        }
    }

    // NXP-REQ-CORE-DB-FUN-H-12: Cache Handler shall get the query result from the appropriate table cache if it exists.
    pub fn get(&mut self, query: &Query) -> Option<QueryResult> {
        if let Some(cache) = self.caches.get_mut(&query.table_name) {
            cache.get(&query.sql).cloned()
        } else {
            None
        }
    }

    // NXP-REQ-CORE-DB-FUN-H-13: Cache Handler shall set the query result in the appropriate table cache.
    pub fn set(&mut self, query: Query, result: QueryResult) {
        self.caches
            .entry(query.table_name.clone())
            .or_insert_with(|| LruCache::new(self.capacity))
            .put(query.sql, result);
    }

    // NXP-REQ-CORE-DB-FUN-H-14: Cache Handler shall invalidate the cache entries related to a specific table.
    pub fn invalidate(&mut self, table_name: &str) {
        self.caches.remove(table_name);
    }

    // NXP-REQ-CORE-DB-FUN-H-15: Cache Handler shall clear the entire cache.
    pub fn clear(&mut self) {
        self.caches.clear();
    }
}
