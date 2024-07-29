use crate::core::search_engine::to_wide_string;

use super::index_service::list_disks_and_volumes;
use super::FileItem;
use super::WindowsDrives;
use rayon::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::ptr;
use std::time::Instant;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::GetVolumeInformationW;
use winapi::um::{fileapi::GetDiskFreeSpaceExW, winnt::ULARGE_INTEGER};

#[tauri::command]
pub fn list_drives() -> Vec<WindowsDrives> {
    match list_disks_and_volumes() {
        Ok(disks) => disks
            .par_iter()
            .filter_map(|disk| {
                let mut total_bytes: ULARGE_INTEGER = unsafe { mem::zeroed() };
                let mut free_bytes: ULARGE_INTEGER = unsafe { mem::zeroed() };
                let mut volume_name = vec![0u16; MAX_PATH];
                let mut file_system_name = vec![0u16; MAX_PATH];
                let mut volume_serial_number: DWORD = 0;
                let mut max_component_length: DWORD = 0;
                let mut file_system_flags: DWORD = 0;

                let volume_path_wide = to_wide_string(&format!("{}:\\", disk));

                unsafe {
                    if GetDiskFreeSpaceExW(
                        volume_path_wide.as_ptr(),
                        ptr::null_mut(),
                        &mut total_bytes,
                        &mut free_bytes,
                    ) != 0
                    {
                        if GetVolumeInformationW(
                            volume_path_wide.as_ptr(),
                            volume_name.as_mut_ptr(),
                            MAX_PATH as DWORD,
                            &mut volume_serial_number,
                            &mut max_component_length,
                            &mut file_system_flags,
                            file_system_name.as_mut_ptr(),
                            MAX_PATH as DWORD,
                        ) != 0
                        {
                            let volume_name_str = OsString::from_wide(
                                &volume_name[..volume_name
                                    .iter()
                                    .position(|&x| x == 0)
                                    .unwrap_or(volume_name.len())],
                            )
                            .to_string_lossy()
                            .into_owned();

                            let file_system_str = OsString::from_wide(
                                &file_system_name[..file_system_name
                                    .iter()
                                    .position(|&x| x == 0)
                                    .unwrap_or(file_system_name.len())],
                            )
                            .to_string_lossy()
                            .into_owned();

                            let used_space = total_bytes.QuadPart() - free_bytes.QuadPart();
                            Some(WindowsDrives {
                                drive_label: disk.clone(),
                                drive_name: volume_name_str,
                                total_space: *total_bytes.QuadPart(),
                                used_space,
                                file_system: file_system_str,
                                volume_serial_number,
                            })
                        } else {
                            eprintln!("Failed to get volume information for drive: {}", disk);
                            eprintln!("Error Code: {}", GetLastError());
                            None
                        }
                    } else {
                        eprintln!("Failed to get disk space for drive: {}", disk);
                        eprintln!("Error Code: {}", GetLastError());
                        None
                    }
                }
            })
            .collect(),
        Err(e) => {
            eprintln!("Error listing disks and volumes: {}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn open_folder(folder_path: &str) -> Result<Vec<FileItem>, String> {
    let before = Instant::now();
    let mut items: Vec<FileItem> = Vec::new();

    let path = Path::new(folder_path);

    if path.exists() && path.is_dir() {
        // Attempt to read the directory
        let read_dir_result = fs::read_dir(path);

        // Handle potential errors from fs::read_dir
        let entries = match read_dir_result {
            Ok(entries) => entries,
            Err(e) => { return Err(e.to_string());
            }
        };

        // Process directory entries in parallel
        items = entries
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                FileItem {
                    name: path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    is_folder: path.is_dir(),
                }
            })
            .collect();
    } else {
        return Err(format!("Invalid path or not a directory: {}", folder_path));
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(items)
}
