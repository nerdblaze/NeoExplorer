use super::Query;

pub mod localization_manager;
pub mod metadata_manager;
pub mod theme_manager;

pub trait Manager {
    fn handle_message(&self, message: String) -> Query;
}
