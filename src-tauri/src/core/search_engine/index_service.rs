use rayon::prelude::*;
use rusqlite::{params, Result};
use std::fs::{self, Metadata};
use std::io::{self, Write};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

use crate::core::search_engine::database_service::{insert_entries, setup_database};
use crate::core::search_engine::{file_exists, MEM_CONN};

use super::{system_time_to_unix_time, FileAttributes, MFTEntry};

pub fn list_disks_and_volumes() -> Result<Vec<String>, io::Error> {
    // Collect drive letters from 'A' to 'Z' that exist as directories
    let drive_letters: Vec<String> = (b'A'..=b'Z')
        .filter_map(|letter| {
            let drive = format!("{}:\\", letter as char);
            let path = Path::new(&drive);
            if path.exists() && path.is_dir() {
                Some(format!("{}", letter as char))
            } else {
                None
            }
        })
        .collect();

    // Return sorted drive letters
    Ok(drive_letters)
}
fn read_mft_data(volume_letter: &str) -> Vec<MFTEntry> {
    let volume_path = format!("{}:\\", volume_letter);

    // Collect entries first
    let entries: Vec<DirEntry> = WalkDir::new(&volume_path)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    // Use rayon to process entries in parallel
    entries
        .into_par_iter()
        .filter_map(|entry| {
            if let Ok(metadata) = entry.metadata() {
                Some(create_index(entry.path(), &metadata).ok())
            } else {
                None
            }
        })
        .filter_map(|entry| entry)
        .collect()
}

fn create_index(path: &Path, metadata: &Metadata) -> io::Result<MFTEntry> {
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
        .replace("'", "");
    let folder_path = path.to_string_lossy().into_owned().replace("'", "");

    let file_size = metadata.len();
    let file_modification_time = metadata.modified().map_or(0, system_time_to_unix_time);
    let file_creation_time = metadata.created().map_or(0, system_time_to_unix_time);
    let file_access_time = metadata.accessed().map_or(0, system_time_to_unix_time);
    let file_attributes = FileAttributes::from_u32(metadata.file_attributes());
    Ok(MFTEntry {
        file_name,
        folder_path,
        file_size,
        file_modification_time,
        file_creation_time,
        file_access_time,
        file_attributes,
    })
}

fn take_mft_snapshot() -> Vec<MFTEntry> {
    let mut mft_entries: Vec<MFTEntry> = vec![];

    // Safely list all disks and volumes
    let disks = match list_disks_and_volumes() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to list disks and volumes: {}", e);
            return mft_entries; // Return empty if listing fails
        }
    };

    for disk in disks {
        let new_entries: Vec<MFTEntry> = read_mft_data(&disk);
        mft_entries.extend(new_entries);
    }

    mft_entries
}

fn store_db(entries: &Vec<MFTEntry>, filename: &str) -> io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(entries).unwrap();
    let mut file = fs::File::create(filename)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn retrieve_db(filename: &str) -> io::Result<Vec<MFTEntry>> {
    let encoded: Vec<u8> = std::fs::read(filename)?;
    let users: Vec<MFTEntry> = bincode::deserialize(&encoded).unwrap();
    Ok(users)
}

pub fn build_index() {
    let before = Instant::now();

    let filename = "target/index.db";
    let mut mft_entries: Vec<MFTEntry> = vec![];

    if !file_exists(&filename) {
        mft_entries = take_mft_snapshot();
        let _ = store_db(&mft_entries, filename);
    } else {
        mft_entries = retrieve_db(filename).unwrap();
    }

    let mem_conn = MEM_CONN.lock().unwrap();

    let _ = setup_database(&mem_conn);
    let _ = insert_entries(&mem_conn, &mft_entries);
    println!("Total MFT Entries retrieved: {}", mft_entries.len());
    println!("Elapsed time: {:.2?}", before.elapsed());
}

// This function takes search_term as input and return a list of file name, folder path, modification time as output the length of list depends on page, page_size
#[tauri::command]
pub fn search_system(
    search_term: &str,
    page: Option<u32>,
    page_size: Option<u32>,
) -> Vec<MFTEntry> {
    let before = Instant::now();
    let mem_conn = MEM_CONN.lock().unwrap();

    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(1000);
    let offset = (page - 1) * page_size;

    let query = "SELECT file_name, folder_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes FROM mft_entries WHERE folder_path LIKE ? and file_size > 0 LIMIT ? OFFSET ?";

    let mut stmt = mem_conn.prepare(query).unwrap();

    let rows = stmt
        .query_map(
            params![format!("%{}%", search_term), page_size, offset],
            |row| {
                Ok(MFTEntry {
                    file_name: row.get(0)?,
                    folder_path: row.get(1)?,
                    file_size: row.get(2)?,
                    file_modification_time: row.get(3)?,
                    file_creation_time: row.get(4)?,
                    file_access_time: row.get(5)?,
                    file_attributes: FileAttributes::from_u32(row.get(6)?),
                })
            },
        )
        .unwrap();

    let results: Vec<MFTEntry> = rows.filter_map(Result::ok).collect();

    println!("Took time: {:.2?}", before.elapsed());

    results
}
