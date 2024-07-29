// NXP-REQ-CORE-DB-FUN-H-7

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use super::{Query, QueryResult};

pub struct DatabaseConnector {
    pool: Pool<SqliteConnectionManager>,
}

impl DatabaseConnector {
    pub fn new() -> Self {
        let manager = SqliteConnectionManager::file("database.db");
        let pool = Pool::new(manager).expect("Failed to create pool.");
        Self { pool }
    }

    pub fn execute_query(&self, query: Query) -> QueryResult { //
        let conn = self.pool.get().unwrap();
        // Execute query using the connection and return result
        QueryResult { data: vec![] }
    }
}
