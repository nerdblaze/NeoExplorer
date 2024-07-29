#![allow(unused_assignments, unreachable_patterns, unused_variables, dead_code)]

use crate::core::search_engine::{file_exists, CHUNK_SIZE, MFT_RECORD_SIZE};
use lazy_static::lazy_static;
use rayon::prelude::*;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self};
use std::io::{self, Write};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::time::Instant;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{
    CreateFileW,  GetLogicalDrives,
    ReadFile, SetFilePointerEx, OPEN_EXISTING,
};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winbase::FILE_BEGIN;
use winapi::um::winioctl::{FSCTL_GET_NTFS_VOLUME_DATA, NTFS_VOLUME_DATA_BUFFER};
use winapi::um::winnt::{
    FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, HANDLE, LARGE_INTEGER,
};

use super::{ BootSector, FileAttributes, MFTEntry, BOOT_SECTOR_SIZE};

lazy_static! {
    static ref MEM_CONN: Mutex<Connection> = Mutex::new(Connection::open_in_memory().unwrap());
}

fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mft_entries(
            mft_record_number INTEGER,
            file_name TEXT,
            folder_path TEXT,
            file_size INTEGER,
            file_modification_time INTEGER,
            parent_directory INTEGER,
            file_attributes INTEGER,
            mft_sequence INTEGER,
            hard_link_count INTEGER,
            log_file_sequence INTEGER,
            file_creation_time INTEGER,
            file_access_time INTEGER
        )",
        [],
    )?;
    Ok(())
}

fn insert_entries(conn: &Connection, entries: &[MFTEntry]) -> Result<()> {
    conn.execute("DELETE FROM mft_entries", [])?;
    let mut sql = String::from("INSERT INTO mft_entries (mft_record_number, file_name, folder_path, file_size, file_modification_time, parent_directory, file_attributes, mft_sequence, hard_link_count, log_file_sequence, file_creation_time, file_access_time) VALUES ");
    let placeholders: Vec<String> = entries
        .par_iter()
        .map(|entry| {
            format!(
                "({}, '{}', '{}', {}, {}, {}, {}, {}, {}, {}, {}, {})",
                entry.mft_record_number,
                entry.file_name,
                entry.folder_path,
                entry.file_size,
                entry.file_modification_time,
                entry.parent_directory,
                entry.file_attributes.to_u32(),
                entry.mft_sequence,
                entry.hard_link_count,
                entry.log_file_sequence,
                entry.file_creation_time,
                entry.file_access_time,
            )
        })
        .collect();

    sql.push_str(&placeholders.join(", "));
    let mut stmt = conn.prepare(&sql)?;

    // let mut file = File::create("target/test.sql").unwrap();
    // file.write_all(sql.as_bytes()).unwrap();
    // file.flush().unwrap();
    let _ = stmt.execute([]);

    Ok(())
}

// Function to check if a character is valid in a Windows file name
fn is_valid_char(c: char) -> bool {
    // Valid characters are letters, digits, space, and a limited set of special characters
    matches!(
        c,
        'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '+' | ',' | '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_' | '`' | '{' | '|' | '}' | '~'
    ) && !c.is_control()
}

// Function to clean the file name
fn clean_file_name(file_name: &str) -> String {
    file_name.chars().filter(|&c| is_valid_char(c)).collect()
}

// Function to extract and clean the file name
fn extract_and_clean_file_name(record: &[u8], attr_offset: usize) -> String {
    let name_length = record[attr_offset + 64] as usize;
    let name_offset = attr_offset + 66;

    if name_length > 0 {
        let name_bytes_len = name_length * 2;
        let name_bytes = &record[name_offset..name_offset + name_bytes_len];

        let name_wide: Vec<u16> = name_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        // Convert the wide character vector to a String
        let file_name = OsString::from_wide(&name_wide)
            .to_string_lossy()
            .into_owned();

        // Clean the file name by removing invalid characters
        clean_file_name(&file_name)
    } else {
        String::new()
    }
}

fn parse_mft_record(record: &Vec<u8>) -> Option<MFTEntry> {

    // Check the record signature
    if &record[0..4] != b"FILE" {
        return None;
    }


    let mut mft_record = MFTEntry::new();
    mft_record.log_file_sequence = u64::from_le_bytes(record[8..16].try_into().unwrap());
    mft_record.mft_sequence = u16::from_le_bytes(record[16..18].try_into().unwrap());
    mft_record.hard_link_count = u16::from_le_bytes(record[18..20].try_into().unwrap());
    mft_record.file_attributes.deleted = (u16::from_le_bytes(record[22..24].try_into().unwrap()) & 1) == 0;
    mft_record.mft_record_number = u32::from_le_bytes(record[44..48].try_into().unwrap());

    let mut offset = u16::from_le_bytes(record[20..22].try_into().unwrap()) as usize;

    while offset < record.len() {
        let attr_type = u32::from_le_bytes(record[offset..offset + 4].try_into().unwrap());
        let attr_len = u32::from_le_bytes(record[offset + 4..offset + 8].try_into().unwrap());

        if attr_type == 0xFFFFFFFF {
            break;
        }
        let attr_offset = offset + 0x18;
        match attr_type {
            0x30 => {
                mft_record.parent_directory = u64::from_le_bytes(record[attr_offset..attr_offset + 8].try_into().unwrap()) & 0x0000_FFFF_FFFF_FFFF;
                mft_record.file_creation_time = u64::from_le_bytes(record[attr_offset + 8..attr_offset + 16].try_into().unwrap());
                mft_record.file_modification_time = u64::from_le_bytes(record[attr_offset + 16..attr_offset + 24].try_into().unwrap());
                mft_record.file_access_time = u64::from_le_bytes(record[attr_offset + 32..attr_offset + 40].try_into().unwrap());
                mft_record.file_attributes = FileAttributes::from_u32(u32::from_le_bytes(record[attr_offset + 56..attr_offset + 60].try_into().unwrap()));
                mft_record.file_name = extract_and_clean_file_name(record, attr_offset);
            }
            0x80 => {
                mft_record.file_size = u64::from_le_bytes(record[attr_offset + 16..attr_offset + 24].try_into().unwrap());
            }
            _ => {}
        }

        offset += attr_len as usize;
    }

    Some(mft_record)
}

fn read_boot_sector(handle: HANDLE) -> Result<BootSector, io::Error> {
    let mut buffer = [0u8; BOOT_SECTOR_SIZE];
    let mut bytes_read: DWORD = 0;

    let result = unsafe {
        ReadFile(
            handle,
            buffer.as_mut_ptr() as *mut _,
            buffer.len() as DWORD,
            &mut bytes_read,
            null_mut(),
        )
    };

    if result == 0 {
        unsafe { CloseHandle(handle) };
        return Err(io::Error::last_os_error());
    }

    if bytes_read != BOOT_SECTOR_SIZE as DWORD {
        unsafe { CloseHandle(handle) };
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to read boot sector",
        ));
    }

    let boot_sector = unsafe { *(buffer.as_ptr() as *const BootSector) };

    Ok(boot_sector)
}

pub fn list_disks_and_volumes() -> Result<Vec<String>, io::Error> {
    // Get the bitmask representing the drives
    let drive_mask: DWORD = unsafe { GetLogicalDrives() };

    if drive_mask == 0 {
        return Err(io::Error::last_os_error());
    }

    let mut drive_letters = Vec::new();

    // Iterate through each bit in the bitmask
    for i in 0..26 {
        let bit = 1 << i;
        if drive_mask & bit != 0 {
            // Convert bit index to a drive letter
            let drive_letter = (b'A' + i as u8) as char;
            drive_letters.push(format!("{}", drive_letter));
        }
    }

    // Sort the drive letters for consistency
    drive_letters.sort();

    Ok(drive_letters)
}

fn open_volume(volume_path: &str) -> io::Result<HANDLE> {
    let volume_path_wide: Vec<u16> = OsString::from(volume_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let handle: HANDLE = unsafe {
        CreateFileW(
            volume_path_wide.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            null_mut(),
            OPEN_EXISTING,
            0,
            null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    Ok(handle)
}

fn read_mft_data(volume_letter: &str) -> io::Result<Vec<MFTEntry>> {
    // Buffer for MFT records
    let mut entries = Vec::new();

    let volume_path = format!("\\\\.\\{}:", volume_letter);
    let handle = open_volume(&volume_path)?;

    let mut buffer = vec![0u8; BOOT_SECTOR_SIZE];
    let mut bytes_read: DWORD = 0;

    let success = unsafe {
        DeviceIoControl(
            handle,
            FSCTL_GET_NTFS_VOLUME_DATA,
            null_mut(),
            0,
            buffer.as_mut_ptr() as *mut _,
            BOOT_SECTOR_SIZE as DWORD,
            &mut bytes_read,
            null_mut(),
        )
    };

    if success == 0 {
        unsafe { CloseHandle(handle) };
        return Err(io::Error::last_os_error());
    }

    let volume_data = unsafe { &*(buffer.as_ptr() as *const NTFS_VOLUME_DATA_BUFFER) };

    let mft_start_cluster = unsafe { *volume_data.MftStartLcn.QuadPart() } as u64;
    let mft_start_byte = mft_start_cluster * volume_data.BytesPerCluster as u64;
    let mft_size = unsafe { *volume_data.MftValidDataLength.QuadPart() } as u64;

    // println!(
    //     "$MFT Info: \n\tStart Location Cluster: {mft_start_cluster} \n\tStart Location in Bytes: {mft_start_byte}\n\tSize: {mft_size}"
    // );

    let mut distance_to_move = unsafe { std::mem::zeroed::<LARGE_INTEGER>() };
    *unsafe { distance_to_move.QuadPart_mut() } = mft_start_byte as i64;

    if unsafe { SetFilePointerEx(handle, distance_to_move, null_mut(), FILE_BEGIN) } == FALSE {
        let error_code = unsafe { GetLastError() };
        println!("SetFilePointerEx failed with error code: {}", error_code);
        unsafe { CloseHandle(handle) };
        return Err(io::Error::from_raw_os_error(error_code as i32));
    }

    let total_records = (mft_size / MFT_RECORD_SIZE as u64) as usize;

    // println!("Expected Entries: {total_records}");

    // Read MFT records in chunks
    let chunk_count = (total_records / (CHUNK_SIZE / MFT_RECORD_SIZE)) + 1;
    let mut chunk_buffer: Vec<u8> = vec![0u8; CHUNK_SIZE];

    // Using a thread pool to process records in parallel
    for _ in 0..chunk_count {
        let result = unsafe {
            ReadFile(
                handle,
                chunk_buffer.as_mut_ptr() as *mut _,
                CHUNK_SIZE as DWORD,
                &mut bytes_read,
                null_mut(),
            )
        };

        if result == 0 || bytes_read == 0 as DWORD {
            unsafe { CloseHandle(handle) };
            return Err(io::Error::last_os_error());
        }
        let record_entries: Vec<MFTEntry> = chunk_buffer
            .chunks(MFT_RECORD_SIZE)
            .par_bridge()
            .into_par_iter() //TODO[Remove before commit]
            .filter_map(|record| parse_mft_record(&record.to_vec()))
            .collect();
        entries.extend(record_entries);
    }
    println!("Total Entries {}:\\ => {}", volume_letter, entries.len());
    unsafe { CloseHandle(handle) };

    Ok(entries)
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
        let mut new_entries: Vec<MFTEntry> = match read_mft_data(&disk) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Failed to read MFT data from path {}: {}", disk, e);
                continue; // Skip this disk if reading fails
            }
        };

        // Create a HashMap for quick lookups by MFT record number
        let entry_map: HashMap<u64, MFTEntry> = new_entries
            .iter()
            .cloned()
            .map(|entry| (entry.mft_record_number as u64, entry))
            .collect();

        // Build folder paths for each entry
        for entry in &mut new_entries {
            entry.folder_path = format!("{}:/{}",disk,build_folder_path(&entry_map, entry.mft_record_number as u64));
        }

        mft_entries.extend(new_entries);
    }

    mft_entries
}

// Helper function to build the folder path
fn build_folder_path(entries: &HashMap<u64, MFTEntry>, mut entry_id: u64) -> String {
    let mut path_parts = vec![];
    while let Some(entry) = entries.get(&entry_id) {
        if entry.file_name.is_empty() {
            break;
        }
        path_parts.push(entry.file_name.clone());
        entry_id = entry.parent_directory; // Move to parent directory
        if entry_id == 5 { // Assuming 5 is the root or invalid parent
            break;
        }
    }
    path_parts.reverse(); // To get the path in the correct order
    path_parts.join("/")
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
        // println!("Total Table size across disk: {}", mft_entries.len());
        let _ = store_db(&mft_entries, filename);
    } else {
        mft_entries = retrieve_db(filename).unwrap();
    }

    // let mem_conn = Connection::open_in_memory().unwrap();
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

    let query = "SELECT mft_record_number, file_name, folder_path, file_size, file_modification_time, parent_directory, file_attributes, mft_sequence, hard_link_count, log_file_sequence, file_creation_time, file_access_time FROM mft_entries WHERE folder_path LIKE ? and file_size > 0 and parent_directory > 0 LIMIT ? OFFSET ?";

    let mut stmt = mem_conn.prepare(query).unwrap();

    let rows = stmt.query_map(
        params![format!("%{}%", search_term), page_size, offset],
        |row| {
            Ok(MFTEntry {
                mft_record_number: row.get(0)?,
                file_name: row.get(1)?,
                folder_path: row.get(2)?,
                file_size: row.get(3)?,
                file_modification_time: row.get(4)?,
                parent_directory: row.get(5)?,
                file_attributes: FileAttributes::from_u32(row.get(6)?),
                mft_sequence: row.get(7)?,
                hard_link_count: row.get(8)?,
                log_file_sequence: row.get(9)?,
                file_creation_time: row.get(10)?,
                file_access_time: row.get(11)?,
            })
        },
    ).unwrap();

    let results: Vec<MFTEntry> = rows.filter_map(Result::ok).collect();

    println!("Took time: {:.2?}", before.elapsed());

    results
}
