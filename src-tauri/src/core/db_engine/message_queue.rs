// NXP-REQ-CORE-DB-FUN-H-1
// NXP-REQ-CORE-DB-FUN-H-2

use crate::core::db_engine::managers::{
    localization_manager::LocalizationManager, metadata_manager::MetadataManager,
    theme_manager::ThemeManager,
};
use crate::core::db_engine::query_processor::QueryProcessor;
use std::collections::VecDeque;

use super::{managers::Manager, QueryMessage};

pub struct MessageQueue {
    queue: VecDeque<QueryMessage>,
    theme_manager: ThemeManager,
    localization_manager: LocalizationManager,
    metadata_manager: MetadataManager,
    query_processor: QueryProcessor,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            theme_manager: ThemeManager::new(),
            localization_manager: LocalizationManager::new(),
            metadata_manager: MetadataManager::new(),
            query_processor: QueryProcessor::new(),
        }
    }

    pub fn push(&mut self, message: QueryMessage) {
        self.queue.push_back(message);
    }

    pub fn pop(&mut self) -> Option<QueryMessage> {
        self.queue.pop_front()
    }

    pub fn process_message(&mut self, message: QueryMessage) {
        let query = match message {
            QueryMessage::ThemeQuery(msg) => self.theme_manager.handle_message(msg),
            QueryMessage::LocalizationQuery(msg) => self.localization_manager.handle_message(msg),
            QueryMessage::MetadataQuery(msg) => self.metadata_manager.handle_message(msg),
        };

        self.query_processor.process_query(query);
    }
}
