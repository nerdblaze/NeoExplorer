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
use std::{fs, path::Path, time::Instant};

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
///
/// # Arguments
///
/// * `file_path` - The path to the folder to be opened.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `FileEntry` objects if successful,
/// or an error message if there was an issue reading the directory.
#[tauri::command]
pub fn open_folder(folder_path: &str) -> Result<Vec<FileEntry>, String> {
    let before: Instant = Instant::now();

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
        println!("Elapsed time: {:.2?}", before.elapsed());
        Ok(items)
    } else {
        return Err(format!("Invalid path or not a directory: {}", folder_path));
    }
}
