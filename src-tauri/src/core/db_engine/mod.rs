pub mod backup_service;
pub mod cache_handler;
pub mod database_connector;
pub mod managers;
pub mod message_queue;
pub mod query_processor;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Query {
    pub sql: String,
    pub is_update: bool,
    pub table_name: String,
}

#[derive(Clone)]
pub struct QueryResult {
    pub data: Vec<Vec<String>>,
}

pub enum QueryMessage {
    ThemeQuery(String),
    LocalizationQuery(String),
    MetadataQuery(String),
}