/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: utilities
 * File Name: file.rs
 * Author: B74Z3
 * Description: This file handles file utility operations etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::fs;

// External Crates

// Internal Modules

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

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    const K: f64 = 1024.0;

    let i = (bytes as f64).log(K).floor() as usize;
    let size = bytes as f64 / K.powi(i as i32);

    format!("{:.2} {}", size, UNITS[i])
}

/******************************************************************************
 * Tests:
 ******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use fs::File;
    use std::{env, path::PathBuf};

    fn cleanup_test_files_and_dirs(temp_dir: &PathBuf) {
        let files_and_dirs = [
            "test_file_exists.txt",
            "test_directory",
        ];

        for file_or_dir in files_and_dirs.iter() {
            let path = temp_dir.join(file_or_dir);

            if path.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
    }

    #[test]
    fn test_file_exists() {
        let temp_dir = env::temp_dir();
        cleanup_test_files_and_dirs(&temp_dir);

        let test_file_path = temp_dir.join("test_file_exists.txt");
        File::create(&test_file_path).expect("Unable to create file");
        assert!(file_exists(test_file_path.to_str().unwrap()));

        let test_file_path = temp_dir.join("test_file_no_exists.txt");
        assert!(!file_exists(test_file_path.to_str().unwrap()));

        let test_dir_path = temp_dir.join("test_directory");
        fs::create_dir(&test_dir_path).expect("Unable to create directory");
        assert!(file_exists(test_dir_path.to_str().unwrap()));

        let test_dir_path = temp_dir.join("test_no_directory");
        assert!(!file_exists(test_dir_path.to_str().unwrap()));

        cleanup_test_files_and_dirs(&temp_dir);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
}
