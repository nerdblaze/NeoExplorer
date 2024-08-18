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
use std::{fs::{self, File}, path::Path, process::Command};

// External Crates
use rayon::iter::{ParallelBridge, ParallelIterator};
use trash;

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
/// Delelte a file/folder
#[tauri::command]
pub async fn delete_file(file_path: &str, permanent: Option<bool>) -> Result<(), String> {
    let path = Path::new(file_path);
    let permanent = permanent.unwrap_or(false);

    if !path.exists() {
        return Err(format!("Invalid path: {}", file_path));
    }

    if permanent {
        // Permanent deletion
        if path.is_dir() {
            fs::remove_dir_all(path)
                .map_err(|_e| format!("Unable to delete directory: '{}'", file_path))
        } else if path.is_file() {
            fs::remove_file(path).map_err(|_e| format!("Unable to delete file: '{}'", file_path))
        } else {
            Err(format!("Unknown path type: {}", file_path))
        }
    } else {
        trash::delete(file_path).map_err(|_e| format!("Unable to delete '{}'", file_path))
    }
}

/// Delelte a list of files and folders
#[tauri::command]
pub async fn delete_files(file_paths: Vec<&str>, permanent: Option<bool>) -> Result<(), String> {
    let mut errors = Vec::new();

    for file_path in file_paths {
        match delete_file(&file_path, permanent).await {
            Ok(_) => {}
            Err(e) => errors.push(e),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join(", "))
    }
}

/// Create a file
#[tauri::command]
pub async fn create_file(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    if path.exists() {
        return Err(format!("File already exists: {}", file_path));
    }

    File::create(path)
        .map(|_| ())
        .map_err(|e| format!("Failed to create file '{}': {}", file_path, e))
}

/// Create  a folder
#[tauri::command]
pub async fn create_folder(folder_path: &str) -> Result<(), String> {
    let path = Path::new(folder_path);
    if path.exists() {
        return Err(format!("Directory already exists: {}", folder_path));
    }

    fs::create_dir(path)
        .map(|_| ())
        .map_err(|e| format!("Failed to create directory '{}': {}", folder_path, e))
}