use super::MFTEntry;
use rusqlite::{Connection, Result};
use rayon::prelude::*;

pub fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mft_entries(
            file_name TEXT,
            folder_path TEXT,
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

pub fn insert_entries(conn: &Connection, entries: &[MFTEntry]) -> Result<()> {
    let _ = conn.execute("DELETE FROM mft_entries", []);
    let mut sql = String::from("INSERT INTO mft_entries (file_name, folder_path, file_size, file_modification_time, file_creation_time, file_access_time, file_attributes) VALUES ");
    let placeholders: Vec<String> = entries
        .par_iter()
        .map(|entry| {
            format!(
                "('{}', '{}', {}, {}, {}, {}, {})",
                entry.file_name,
                entry.folder_path,
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
