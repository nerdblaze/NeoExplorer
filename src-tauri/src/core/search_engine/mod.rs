pub mod index_service;
pub mod windows_service;

use std::{ffi::OsString, fs, os::windows::ffi::OsStrExt};

use serde::{Deserialize, Serialize};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_COMPRESSED, FILE_ATTRIBUTE_DEVICE,
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN,
    FILE_ATTRIBUTE_OFFLINE, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
    FILE_ATTRIBUTE_TEMPORARY,
};

const BOOT_SECTOR_SIZE: usize = 512;
const MFT_RECORD_SIZE: usize = 1024;
const CHUNK_SIZE: usize = 4 * 1024 * 1024; // Read in chunks of 1MB
const FILE_ATTRIBUTE_DELETED: u32 = 0x8000_0000;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct BPB {
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    unused_1: [u8; 3],
    unused_2: u16,
    media_descriptor: u8,
    unused3: u16,
    sectors_per_track: u16,
    number_of_heads: u16,
    hidden_sectors: u32,
    unused_3: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct ExBPB {
    unused_4: u32,
    total_sectors: u64,
    mft_lcn: u64,
    mft_mirror_lcn: u64,
    clusters_per_mft_record: u32,
    clusters_per_index_buffer: u8,
    unused_5: [u8; 3],
    volume_serial_number: u64,
    checksum: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct BootSector {
    jump: [u8; 3],
    oem_id: [u8; 8],
    bpb: BPB,            //[u8; 25],
    extended_bpb: ExBPB, //[u8; 48],
    boot_code: [u8; 426],
    signature: [u8; 2],
}

#[derive(Debug)]
struct Volume {
    name: String,
    paths: Vec<String>,
}

#[derive(Debug)]
pub struct Disk {
    name: String,
    volumes: Vec<Volume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MFTEntry {
    mft_record_number: u32,
    file_name: String,
    folder_path: String,
    file_size: u64,
    file_modification_time: u64,
    parent_directory: u64,
    file_attributes: FileAttributes,
    mft_sequence: u16,
    hard_link_count: u16,
    log_file_sequence: u64,
    file_creation_time: u64,
    file_access_time: u64,
}

impl MFTEntry {
    fn new() -> Self {
        Self {
            mft_record_number: 0,
            file_name: String::new(),
            folder_path: String::new(),
            file_size: 0,
            file_modification_time: 0,
            parent_directory: 0,
            file_attributes: FileAttributes::new(),
            mft_sequence: 0,
            hard_link_count: 0,
            log_file_sequence: 0,
            file_creation_time: 0,
            file_access_time: 0,
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
impl WindowsDrives {
    fn new() -> Self {
        Self {
            drive_label: String::new(),
            drive_name: String::new(),
            total_space: 0,
            used_space: 0,
            file_system: String::new(),
            volume_serial_number: 0,
        }
    }
}

pub fn to_wide_string(instr: &str)->Vec<u16>{
    OsString::from(instr).encode_wide().chain(std::iter::once(0)).collect()
}

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}