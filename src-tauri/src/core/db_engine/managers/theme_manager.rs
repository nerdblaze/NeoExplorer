use crate::core::db_engine::{managers::Manager, Query};

pub struct ThemeManager;

impl ThemeManager {
    pub fn new() -> Self {
        Self
    }
}

impl Manager for ThemeManager {
    fn handle_message(&self, message: String) -> Query {
        // Process theme message and create a query
        Query { sql: message, is_update: false, table_name: "Table".to_string() }
    }
}
