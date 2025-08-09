use std::collections::HashMap;

// Represents a single column in a table.
// `name` is the column name, and `data_type` is a simple string ("INT" or "TEXT").
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String, // Keeping this simple only "INT" or "TEXT" for now
}

// Schema for a table: its name and the list of columns it contains.
#[derive(Debug)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

// In-memory catalog that holds all table schemas, indexed by table name.
pub struct Catalog {
    pub tables: HashMap<String, TableSchema>,
}

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
}
