use super::{system_time_to_unix_time, FileAttributes, FileEntry, WindowsDrives};
use crate::core::search_engine::database_service::{
    insert_entries, retrieve_db, setup_database, store_db,
};
use crate::core::search_engine::{file_exists, MEM_CONN};
use rayon::prelude::*;
use rusqlite::Result;
use std::fs::{self, Metadata};
use std::io::{self};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::time::Instant;
use sysinfo::Disks;
use walkdir::{DirEntry, WalkDir};

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

#[tauri::command]
pub fn open_folder(file_path: &str) -> Result<Vec<FileEntry>, String> {
    let before = Instant::now();

    let path = Path::new(file_path);

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
        return Err(format!("Invalid path or not a directory: {}", file_path));
    }

}

fn take_mft_snapshot() -> Vec<FileEntry> {
    let mut file_entries: Vec<FileEntry> = vec![];
    for drive in list_drives() {
        // read_mft_data(&drive.disk_label);

        let volume_path = format!("{}:\\", drive.disk_label);
        // Collect entries first
        let entries: Vec<DirEntry> = WalkDir::new(&volume_path)
            .into_iter()
            .par_bridge()
            .into_par_iter()
            .filter_map(Result::ok)
            .collect();

        // Use rayon to process entries in parallel
        let new_entries: Vec<FileEntry> = entries
            .into_par_iter()
            .filter_map(|entry| {
                if let Ok(metadata) = entry.metadata() {
                    Some(create_index(entry.path(), &metadata).ok())
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

pub fn build_index() {
    let before = Instant::now();

    let filename: &str = "target/index.db";
    let master_file_table: Vec<FileEntry>;

    if !file_exists(filename) {
        master_file_table = take_mft_snapshot();
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
