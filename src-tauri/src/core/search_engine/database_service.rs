use std::{
    fs,
    io::{self, Write},
    time::Instant,
};

use crate::core::search_engine::{FileAttributes, MEM_CONN};

use super::FileEntry;
use rayon::prelude::*;
use rusqlite::{params, Connection, Result};

pub fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS master_file_table(
            file_name TEXT,
            file_path TEXT,
            file_size INTEGER,
            file_modification_time INTEGER,
            file_creation_time INTEGER,
            file_access_time INTEGER,
            file_attributes INTEGER
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_entries(conn: &Connection, entries: &[FileEntry]) -> Result<()> {
    let _ = conn.execute("DELETE FROM master_file_table", []);
    let mut sql = String::from("INSERT INTO master_file_table (file_name, file_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes) VALUES ");
    let placeholders: Vec<String> = entries
        .par_iter()
        .map(|entry| {
            format!(
                "('{}', '{}', {}, {}, {}, {}, {})",
                entry.file_name,
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

    let mut stmt = conn.prepare(&sql)?;
    let _ = stmt.execute([])?;
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
    page: Option<u32>,
    page_size: Option<u32>,
) -> Vec<FileEntry> {
    let before = Instant::now();
    let mem_conn = MEM_CONN.lock().unwrap();

    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(1000);
    let offset = (page - 1) * page_size;

    let query = "SELECT file_name, file_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes FROM master_file_table WHERE file_path LIKE ? and file_size > 0 LIMIT ? OFFSET ?";

    let mut stmt = mem_conn.prepare(query).unwrap();

    let rows = stmt
        .query_map(
            params![format!("%{}%", search_term), page_size, offset],
            |row| {
                Ok(FileEntry {
                    file_name: row.get(0)?,
                    file_path: row.get(1)?,
                    file_size: row.get(2)?,
                    file_modification_time: row.get(3)?,
                    file_creation_time: row.get(4)?,
                    file_access_time: row.get(5)?,
                    file_attributes: FileAttributes::from_u32(row.get(6)?),
                })
            },
        )
        .unwrap();

    let results: Vec<FileEntry> = rows.filter_map(Result::ok).collect();

    println!("Took time: {:.2?}", before.elapsed());

    results
}
