pub mod database_service;
pub mod index_service;
pub mod explorer_service;

use lazy_static::lazy_static;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fs,
    os::windows::ffi::OsStrExt,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref MEM_CONN: Mutex<Connection> = Mutex::new(Connection::open_in_memory().unwrap());
}

const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001;
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x00000004;
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010;
const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x00000020;
const FILE_ATTRIBUTE_DEVICE: u32 = 0x00000040;
const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x00000100;
const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x00000800;
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x00001000;
const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x00004000;
const FILE_ATTRIBUTE_DELETED: u32 = 0x80000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    file_name: String,
    file_path: String,
    file_size: u64,
    file_modification_time: u64,
    file_creation_time: u64,
    file_access_time: u64,
    file_attributes: FileAttributes,
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
        0 | (if self.read_only {
            FILE_ATTRIBUTE_READONLY
        } else {
            0
        }) | (if self.hidden {
            FILE_ATTRIBUTE_HIDDEN
        } else {
            0
        }) | (if self.system {
            FILE_ATTRIBUTE_SYSTEM
        } else {
            0
        }) | (if self.directory {
            FILE_ATTRIBUTE_DIRECTORY
        } else {
            0
        }) | (if self.archive {
            FILE_ATTRIBUTE_ARCHIVE
        } else {
            0
        }) | (if self.device {
            FILE_ATTRIBUTE_DEVICE
        } else {
            0
        }) | (if self.temporary {
            FILE_ATTRIBUTE_TEMPORARY
        } else {
            0
        }) | (if self.compressed {
            FILE_ATTRIBUTE_COMPRESSED
        } else {
            0
        }) | (if self.offline {
            FILE_ATTRIBUTE_OFFLINE
        } else {
            0
        }) | (if self.encrypted {
            FILE_ATTRIBUTE_ENCRYPTED
        } else {
            0
        }) | (if self.deleted {
            FILE_ATTRIBUTE_DELETED
        } else {
            0
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    name: String,
    is_folder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsDrives {
    disk_label: String,
    disk_name: String,
    total_space: u64,
    free_space: u64,
    file_system: String,
    disk_type: String,
    is_removable: bool,
}

pub fn to_wide_string(instr: &str) -> Vec<u16> {
    OsString::from(instr)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
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
