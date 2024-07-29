pub mod index_service;
pub mod windows_service;
pub mod database_service;

use std::{ffi::OsString, fs, os::windows::ffi::OsStrExt, sync::Mutex, time::{SystemTime, UNIX_EPOCH}};
use lazy_static::lazy_static;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_COMPRESSED, FILE_ATTRIBUTE_DEVICE,
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN,
    FILE_ATTRIBUTE_OFFLINE, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
    FILE_ATTRIBUTE_TEMPORARY,
};

const FILE_ATTRIBUTE_DELETED: u32 = 0x8000_0000;

lazy_static! {
    static ref MEM_CONN: Mutex<Connection> = Mutex::new(Connection::open_in_memory().unwrap());
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFTEntry {
    file_name: String,
    folder_path: String,
    file_size: u64,
    file_modification_time: u64,
    file_creation_time: u64,
    file_access_time: u64,
    file_attributes: FileAttributes,
}

impl MFTEntry {
    fn new() -> Self {
        Self {
            file_name: String::new(),
            folder_path: String::new(),
            file_size: 0,
            file_modification_time: 0,
            file_creation_time: 0,
            file_access_time: 0,
            file_attributes: FileAttributes::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
    device: bool,
    temporary: bool,
    compressed: bool,
    offline: bool,
    encrypted: bool,
    deleted: bool,
}

impl FileAttributes {
    fn new() -> Self {
        Self {
            read_only: false,
            hidden: false,
            system: false,
            directory: false,
            archive: false,
            device: false,
            temporary: false,
            compressed: false,
            offline: false,
            encrypted: false,
            deleted: false,
        }
    }

    fn from_u32(attributes: u32) -> Self {
        Self {
            read_only: attributes & FILE_ATTRIBUTE_READONLY != 0,
            hidden: attributes & FILE_ATTRIBUTE_HIDDEN != 0,
            system: attributes & FILE_ATTRIBUTE_SYSTEM != 0,
            directory: attributes & FILE_ATTRIBUTE_DIRECTORY != 0,
            archive: attributes & FILE_ATTRIBUTE_ARCHIVE != 0,
            device: attributes & FILE_ATTRIBUTE_DEVICE != 0,
            temporary: attributes & FILE_ATTRIBUTE_TEMPORARY != 0,
            compressed: attributes & FILE_ATTRIBUTE_COMPRESSED != 0,
            offline: attributes & FILE_ATTRIBUTE_OFFLINE != 0,
            encrypted: attributes & FILE_ATTRIBUTE_ENCRYPTED != 0,
            deleted: attributes & FILE_ATTRIBUTE_DELETED != 0,
        }
    }
    fn to_u32(&self) -> u32 {
        let mut attributes = 0u32;
        attributes |= if self.read_only {
            FILE_ATTRIBUTE_READONLY
        } else {
            0
        };
        attributes |= if self.hidden {
            FILE_ATTRIBUTE_HIDDEN
        } else {
            0
        };
        attributes |= if self.system {
            FILE_ATTRIBUTE_SYSTEM
        } else {
            0
        };
        attributes |= if self.directory {
            FILE_ATTRIBUTE_DIRECTORY
        } else {
            0
        };
        attributes |= if self.archive {
            FILE_ATTRIBUTE_ARCHIVE
        } else {
            0
        };
        attributes |= if self.device {
            FILE_ATTRIBUTE_DEVICE
        } else {
            0
        };
        attributes |= if self.temporary {
            FILE_ATTRIBUTE_TEMPORARY
        } else {
            0
        };
        attributes |= if self.compressed {
            FILE_ATTRIBUTE_COMPRESSED
        } else {
            0
        };
        attributes |= if self.offline {
            FILE_ATTRIBUTE_OFFLINE
        } else {
            0
        };
        attributes |= if self.encrypted {
            FILE_ATTRIBUTE_ENCRYPTED
        } else {
            0
        };
        attributes |= if self.deleted {
            FILE_ATTRIBUTE_DELETED
        } else {
            0
        };
        attributes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    name: String,
    is_folder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsDrives {
    drive_label: String,
    drive_name: String,
    total_space: u64,
    used_space: u64,
    file_system: String,
    volume_serial_number: u32,
}

pub fn to_wide_string(instr: &str)->Vec<u16>{
    OsString::from(instr).encode_wide().chain(std::iter::once(0)).collect()
}

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn system_time_to_unix_time(system_time: SystemTime) -> u64 {
    system_time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

