use super::{Row, StorageBackend};
use crate::catalog::TableSchema;
use bincode;
use std::fs::OpenOptions;
use std::io::{Cursor, Write};

pub struct FileStorageBackend;

impl FileStorageBackend {
    pub fn new() -> Self {
        FileStorageBackend
    }
}

impl StorageBackend for FileStorageBackend {
    fn insert_row(&self, table_name: &str, row: Row) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}.tbl", table_name))
            .map_err(|e| e.to_string())?;

        let encoded = bincode::serialize(&row).map_err(|e| e.to_string())?;
        file.write_all(&encoded).map_err(|e| e.to_string())
    }

    fn read_all_rows(&self, table_name: &str) -> Result<Vec<Row>, String> {
        let path = format!("{}.tbl", table_name);
        let bytes = std::fs::read(path).unwrap_or_default();
        let mut rows = Vec::new();
        let mut cursor = Cursor::new(&bytes);

        while (cursor.position() as usize) < bytes.len() {
            let row: Row = bincode::deserialize_from(&mut cursor).map_err(|e| e.to_string())?;
            rows.push(row);
        }
        Ok(rows)
    }

    fn create_table(&self, _schema: &TableSchema) -> Result<(), String> {
        // For now, creating a table just means ensuring the file exists.
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("{}.tbl", _schema.name))
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
