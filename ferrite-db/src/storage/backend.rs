use crate::catalog::TableSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Row {
    pub values: Vec<String>,
}

pub trait StorageBackend {
    fn insert_row(&self, table_name: &str, row: Row) -> Result<(), String>;
    fn read_all_rows(&self, table_name: &str) -> Result<Vec<Row>, String>;
    fn create_table(&self, schema: &TableSchema) -> Result<(), String>;
}
