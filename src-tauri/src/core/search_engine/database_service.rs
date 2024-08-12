/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: search_engine
 * File Name: database_service.rs
 * Author: B74Z3
 * Description: This file provides functionality to perform database operations
 *              related to searching, indexing etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::{
    ffi::OsStr, fs::{self}, io::{self, Write}, path::Path, time::Instant
};

// External Crates
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rusqlite::{params, Connection, Result, Statement};

// Internal Modules
use crate::core::{FileAttributes, FileEntry, MEM_CONN};

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
pub fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS master_file_table(
            file_path TEXT,
            file_size INTEGER,
            file_modification_time INTEGER,
            file_creation_time INTEGER,
            file_access_time INTEGER,
            file_attributes INTEGER
        );",
        [],
    )?;

    Ok(())
}

pub fn insert_entries(conn: &Connection, entries: &[FileEntry]) -> Result<()> {
    let _ = conn.execute("DELETE FROM master_file_table", []);
    let chunk_size = 4096;
    let mut row_count = 0;
    for chunk in entries.chunks(chunk_size) {
        let mut sql: String = String::from("INSERT INTO master_file_table (file_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes) VALUES ");
        let placeholders: Vec<String> = chunk
            .par_iter()
            .map(|entry: &FileEntry| {
                format!(
                    "('{}', {}, {}, {}, {}, {})",
                    entry.file_path,
                    entry.file_size,
                    entry.file_modification_time,
                    entry.file_creation_time,
                    entry.file_access_time,
                    entry.file_attributes.to_u32(),
                )
            })
            .collect();
        sql.push_str(&placeholders.join(", "));
        let mut stmt: Statement = conn.prepare(&sql)?;
        row_count = row_count + stmt.execute([])?;
    }
    println!("Total DB Entries: {}", row_count);

    Ok(())
}

pub fn store_db(entries: &Vec<FileEntry>, filename: &str) -> io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(entries).unwrap();
    let mut file = fs::File::create(filename)?;
    file.write_all(&encoded)?;
    Ok(())
}

pub fn retrieve_db(filename: &str) -> io::Result<Vec<FileEntry>> {
    let encoded: Vec<u8> = std::fs::read(filename)?;
    let users: Vec<FileEntry> = bincode::deserialize(&encoded).unwrap();
    Ok(users)
}

// This function takes search_term as input and return a list of file name, folder path, modification time as output the length of list depends on page, page_size
#[tauri::command]
pub fn search_system(
    search_term: &str,
    page: Option<u64>,
    page_size: Option<u64>,
) -> Vec<FileEntry> {
    let before = Instant::now();
    if search_term.is_empty() {
        return vec![];
    }
    let mem_conn = MEM_CONN.lock().unwrap();

    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(1000);
    let offset = (page - 1) * page_size;
    let query = "SELECT file_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes FROM master_file_table WHERE file_path LIKE ? and file_size > 0 LIMIT ? OFFSET ?";

    let results: Vec<FileEntry> = match mem_conn.prepare(query) {
        Ok(mut stmt) => {
            let rows: Vec<FileEntry> = stmt
                .query_map(
                    params![format!("%{}%", search_term), page_size, offset],
                    |row| {
                        let file_path: String = row.get(0)?;
                        let file_name = Path::new(&file_path)
                            .file_name()
                            .unwrap_or_else(|| OsStr::new(""))
                            .to_string_lossy()
                            .into_owned();
                        Ok(FileEntry {
                            file_name,
                            file_path,
                            file_size: row.get(1)?,
                            file_modification_time: row.get(2)?,
                            file_creation_time: row.get(3)?,
                            file_access_time: row.get(4)?,
                            file_attributes: FileAttributes::from_u32(row.get(5)?),
                        })
                    },
                )
                .unwrap()
                .filter_map(Result::ok)
                .collect();
            println!("Search time: {:.2?}", before.elapsed());
            return rows;
        }
        Err(_) => Vec::new(),
    };

    results
}
