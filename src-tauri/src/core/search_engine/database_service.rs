/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: search_engine
 * File Name: database_service.rs
 * Author: B74Z3
 * Description: This file provides functionality to perform database operations
 *              related to searching, indexing etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::{
    fs::{self},
    io::{self, Write},
    time::Instant,
};

// External Crates
use rayon::{iter::{IntoParallelIterator, ParallelIterator},slice::ParallelSliceMut};

// Internal Modules
use crate::core::{FileEntry, INDEX_DB};

use super::SearchParams;

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
pub fn store_db(entries: &Vec<FileEntry>, filename: &str) -> io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(entries).unwrap();
    let mut file = fs::File::create(filename)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn retrieve_db(filename: &str) -> io::Result<Vec<FileEntry>> {
    let encoded: Vec<u8> = std::fs::read(filename)?;
    let entries: Vec<FileEntry> = bincode::deserialize(&encoded).unwrap();
    Ok(entries)
}

// This function takes search_term as input and return a list of file name, folder path, modification time as output the length of list depends on page, page_size
#[tauri::command]
pub fn search_system(params: SearchParams) -> Vec<FileEntry> {
    let before = Instant::now();
    let binding = INDEX_DB.clone();
    let entries = binding.lock().unwrap();

    // Clone and filter entries based on the search parameters
    let filtered_entries: Vec<FileEntry> = entries
        .clone()
        .into_par_iter()
        .filter(|entry| {
            params.path.as_ref().map_or(true, |path| entry.file_path.to_lowercase().contains(&path.to_lowercase()))
        })
        .collect();

    // Sort entries if a sort parameter is provided
    let sorted_entries: Vec<FileEntry> = if let Some(sort) = params.sort.as_deref() {
        let mut sorted: Vec<FileEntry> = filtered_entries;
        sorted.par_sort_by(|a, b| match sort {
            "file_path" => a.file_path.cmp(&b.file_path),
            "file_size" => a.file_size.cmp(&b.file_size),
            "file_modification_time" => a.file_modification_time.cmp(&b.file_modification_time),
            "file_access_time" => a.file_access_time.cmp(&b.file_access_time),
            "file_creation_time" => a.file_creation_time.cmp(&b.file_creation_time),
            _ => a.file_attributes.directory.cmp(&b.file_attributes.directory) // Default case to avoid panic
        });
        sorted
    } else {
        filtered_entries
    };


    // Reverse order if 'desc' is specified
    let mut final_entries = if params.order.as_deref() == Some("desc") {
        sorted_entries.into_iter().rev().collect()
    } else {
        sorted_entries
    };

    // Limit the number of entries if a limit is specified
    if let Some(limit) = params.limit {
        final_entries.truncate(limit);
    }

    println!("Search time: {:.2?}", before.elapsed());
    final_entries
}
