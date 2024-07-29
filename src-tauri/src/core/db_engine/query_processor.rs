// NXP-REQ-CORE-DB-FUN-H-3
// NXP-REQ-CORE-DB-FUN-H-4

use crate::core::db_engine::cache_handler::CacheHandler;
use crate::core::db_engine::database_connector::DatabaseConnector;

use super::{Query, QueryResult};

pub struct QueryProcessor {
    cache_handler: CacheHandler,
    db_connector: DatabaseConnector,
}

impl QueryProcessor {
    pub fn new() -> Self {
        Self {
            cache_handler: CacheHandler::new(100),
            db_connector: DatabaseConnector::new(),
        }
    }

    pub fn process_query(&mut self, query: Query) -> QueryResult {
        if let Some(result) = self.cache_handler.get(&query) {
            return result;
        }

        let result = self.db_connector.execute_query(query.clone());

        if query.is_update {
            self.cache_handler.invalidate(&query.table_name);
        }

        self.cache_handler.set(query, result.clone());

        result
    }
}
