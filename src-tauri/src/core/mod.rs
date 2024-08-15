/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: core
 * File Name: mod.rs
 * Author: B74Z3
 * Description: This module handles all core application operations
 ******************************************************************************/

pub mod explorer_engine;
pub mod search_engine;

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::sync::{Arc, Mutex};

// External Crates
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// Internal Modules

/******************************************************************************
 * Constants:
******************************************************************************/
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

lazy_static! {
    static ref INDEX_DB: Arc<Mutex<Vec<FileEntry>>> = Arc::new(Mutex::new(Vec::new()));
}

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
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

/******************************************************************************
 * Implementations:
 ******************************************************************************/

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
}

/******************************************************************************
 * Functions:
 ******************************************************************************/
