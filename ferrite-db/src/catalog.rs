use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

// Represents a single column in a table.
// `name` is the column name, and `data_type` is a simple string ("INT" or "TEXT").
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Column {
    pub name: String,
    pub data_type: String, // Keeping this simple only "INT" or "TEXT" for now
}

// Schema for a table: its name and the list of columns it contains.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

// In-memory catalog that holds all table schemas, indexed by table name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub tables: HashMap<String, TableSchema>,
}

#[allow(dead_code)]
impl Catalog {
    // Creates an empty catalog.
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    // Adds a new table schema to the catalog.
    // The schema's name is used as the key in the HashMap.
    pub fn create_table(&mut self, schema: TableSchema) {
        self.tables.insert(schema.name.clone(), schema);
    }

    // Retrieves a reference to a table schema by name, if it exists.
    pub fn get_table(&self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }

    // Saves the catalog to disk.
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let encoded = serde_json::to_string_pretty(&self.tables)
            .map_err(|e| e.to_string())?;
        std::fs::write(path, encoded).map_err(|e| e.to_string())
    }

    // Loads the catalog from disk.
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        if !std::path::Path::new(path).exists() {
            return Ok(Self::new());
        }
        let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let tables: std::collections::HashMap<String, TableSchema> =
            serde_json::from_str(&data).map_err(|e| e.to_string())?;
        Ok(Self { tables })
    }
}
