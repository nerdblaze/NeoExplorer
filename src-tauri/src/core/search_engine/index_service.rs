/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: search_engine
 * File Name: index_service.rs
 * Author: B74Z3
 * Description: This file provides functionality to interact with file systems,
 *              including listing drives, and building an index of file entries.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/
// Standard Libraries
use std::fs::Metadata;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::time::Instant;

// External Crates
use jwalk::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use sysinfo::Disks;

// Internal Modules
use crate::core::{
    search_engine::database_service::{insert_entries, retrieve_db, setup_database, store_db},
    FileAttributes, FileEntry, WindowsDrives, MEM_CONN,
};
use crate::utilities::{file::file_exists, time::system_time_to_unix_time};

/******************************************************************************
 * Constants:
 ******************************************************************************/

const DB_FILENAME: &str = "target/index.db";

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

/******************************************************************************
 * Implementations:
 ******************************************************************************/

/******************************************************************************
* Functions:
******************************************************************************/
/// Creates a `FileEntry` object from the given path and metadata.
///
/// # Arguments
///
/// * `path` - The path of the file or directory.
/// * `metadata` - The metadata associated with the file or directory.
///
/// # Returns
///
/// Returns a result containing the `FileEntry` if successful, or an `io::Error` if
/// there was a problem accessing the file.
pub fn create_index(path: &Path, metadata: &Metadata) -> io::Result<FileEntry> {
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
        .replace("'", "");
    let file_path = path.to_string_lossy().into_owned().replace("'", "");

    let file_size = metadata.len();
    let file_modification_time = metadata.modified().map_or(0, system_time_to_unix_time);
    let file_creation_time = metadata.created().map_or(0, system_time_to_unix_time);
    let file_access_time = metadata.accessed().map_or(0, system_time_to_unix_time);
    let file_attributes = FileAttributes::from_u32(metadata.file_attributes());
    Ok(FileEntry {
        file_name,
        file_path,
        file_size,
        file_modification_time,
        file_creation_time,
        file_access_time,
        file_attributes,
    })
}

/// Lists all drives on the system.
///
/// # Returns
///
/// Returns a vector of `WindowsDrives` representing each drive's label, name,
/// total space, free space, file system, type, and whether it is removable.
#[tauri::command]
pub fn list_drives() -> Vec<WindowsDrives> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .list()
        .par_iter()
        .map(|disk| {
            let disk_label = disk.mount_point().to_str().unwrap_or("")[0..1].to_string();
            let disk_name = disk.name().to_str().unwrap_or("").to_string();
            let total_space = disk.total_space();
            let free_space = disk.available_space();
            let file_system = disk.file_system().to_str().unwrap_or("").to_string();
            let disk_type = disk.kind().to_string();
            let is_removable = disk.is_removable();

            WindowsDrives {
                disk_label,
                disk_name,
                total_space,
                free_space,
                file_system,
                disk_type,
                is_removable,
            }
        })
        .collect()
}

/// Takes a snapshot of the MFT (Master File Table) for all drives.
///
/// # Returns
///
/// Returns a vector of `FileEntry` objects representing the files on all drives.
fn take_storage_snapshot() -> Vec<FileEntry> {
    let mut file_entries: Vec<FileEntry> = vec![];

    for drive in list_drives() {
        if drive.is_removable {
            continue;
        }
        let volume_path = format!("{}:\\", drive.disk_label);
        // Collect entries first
        let new_entries: Vec<FileEntry> = WalkDir::new(&volume_path)
            .into_iter()
            .par_bridge()
            .filter_map(|dir_entry_result| {
                let dir_entry = dir_entry_result.ok()?;
                let path = dir_entry.path();
                if let Ok(metadata) = dir_entry.metadata() {
                    Some(create_index(&path, &metadata).ok())
                } else {
                    None
                }
            })
            .filter_map(|entry| entry)
            .collect();

        file_entries.extend(new_entries);
    }
    file_entries
}

/// Builds an index of file entries, either by taking a new MFT snapshot or
/// retrieving from an existing database.
///
/// # Returns
///
/// This function does not return a value but prints the total number of MFT entries
/// and the elapsed time.
pub fn build_index() {
    let before = Instant::now();

    let filename: &str = DB_FILENAME;
    let master_file_table: Vec<FileEntry>;

    if !file_exists(filename) {
        master_file_table = take_storage_snapshot();
        let _ = store_db(&master_file_table, filename);
    } else {
        master_file_table = retrieve_db(filename).unwrap();
    }

    let mem_conn = MEM_CONN.lock().unwrap();

    let _ = setup_database(&mem_conn);
    let _ = insert_entries(&mem_conn, &master_file_table);

    drop(mem_conn);

    println!("Total MFT Entries retrieved: {}", master_file_table.len());
    println!("Elapsed time: {:.2?}", before.elapsed());
}
