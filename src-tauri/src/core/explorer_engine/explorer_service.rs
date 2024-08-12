/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: explorer_engine
 * File Name: explorer_service.rs
 * Author: B74Z3
 * Description: This file provides functionality to perform file system
 *              operations like create / delete / open the files and folders
 *              viewing informations etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::{fs, path::Path, process::Command};

// External Crates
use rayon::iter::{ParallelBridge, ParallelIterator};
use rusqlite::Result;

// Internal Modules
use crate::core::{search_engine::index_service::create_index, FileEntry};

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

/// Opens a folder and retrieves file entries in parallel.
#[tauri::command]
pub fn open_folder(folder_path: &str) -> Result<Vec<FileEntry>, String> {
    let path: &Path = Path::new(folder_path);

    if path.exists() && path.is_dir() {
        // Attempt to read the directory
        let read_dir_result = fs::read_dir(path);

        // Handle potential errors from fs::read_dir
        let entries = match read_dir_result {
            Ok(entries) => entries,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Process directory entries in parallel
        let items: Vec<FileEntry> = entries
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let metadata = entry.metadata().unwrap();
                create_index(&entry.path(), &metadata).ok()
            })
            .collect();
        Ok(items)
    } else {
        return Err(format!("Invalid path or not a directory: {}", folder_path));
    }
}

/// Opens a file with windows default file opener
#[tauri::command]
pub fn open_file(file_path: &str) -> Result<(), String> {
    let path: &Path = Path::new(file_path);

    if path.exists() && (!path.is_dir()) {
        let mut cmd = Command::new("cmd");
        match cmd.arg("/C").arg("start").arg("").arg(file_path).spawn() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to execute command: {}", e)),
        }
    } else {
        return Err(format!("Invalid path or not a directory: {}", file_path));
    }
}
