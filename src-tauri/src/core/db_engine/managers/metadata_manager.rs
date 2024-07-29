use crate::core::db_engine::{managers::Manager, Query};

pub struct MetadataManager;

impl MetadataManager {
    pub fn new() -> Self {
        Self
    }
}

impl Manager for MetadataManager {
    fn handle_message(&self, message: String) -> Query { //Query
        // Process metadata message and create a query
        Query { sql: message, is_update: false, table_name: "Table".to_string() }
    }
}
