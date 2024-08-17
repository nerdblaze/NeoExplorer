/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: file_service
 * File Name: info_handler.rs
 * Author: B74Z3
 * Description: This module handles all file informations
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::{collections::HashMap, ffi::OsStr, fs::FileType, path::Path};

// External Crates

// Internal Modules
use crate::core::explorer_engine::FileInfo;

/******************************************************************************
 * Constants:
 ******************************************************************************/

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

/******************************************************************************
 * Implementations:
 ******************************************************************************/

/******************************************************************************
 * Functions:
 ******************************************************************************/

fn get_file_type_description(file_type: &FileType) -> String {
    if file_type.is_file() {
        "File".to_string()
    } else if file_type.is_dir() {
        "Folder".to_string()
    } else if file_type.is_symlink() {
        "Link".to_string()
    } else {
        "Unknown".to_string()
    }
}

#[tauri::command]
pub async fn get_file_info(file_path: &str) -> Result<FileInfo, String> {
    let file_path: &Path = Path::new(&file_path);
    let meta = match file_path.metadata() {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };

    let mut general = HashMap::new();
    general.insert(
        "name".to_string(),
        file_path
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy()
            .into_owned(),
    );
    general.insert("size".to_string(), meta.len().to_string());
    general.insert(
        "attributes".to_string(),
        get_file_type_description(&meta.file_type()),
    );
    general.insert(
        "location".to_string(),
        file_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_string_lossy()
            .into_owned(),
    );

    let mut categories = HashMap::new();
    categories.insert("General".to_string(), general);
    let metadata = FileInfo { categories };

    Ok(metadata)
}
