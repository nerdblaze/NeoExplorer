use crate::core::db_engine::{managers::Manager, Query};

pub struct LocalizationManager;

impl LocalizationManager {
    pub fn new() -> Self {
        Self
    }
}

impl Manager for LocalizationManager {
    fn handle_message(&self, message: String) -> Query {
        // Process localization message and create a query
        Query { sql: message, is_update: false, table_name: "Table".to_string() }
    }
}