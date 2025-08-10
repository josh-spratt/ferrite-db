use crate::catalog::{Catalog, Column, TableSchema};
use crate::storage::{Row, StorageBackend};
use sqlparser::ast::*;
use std::sync::{Arc, Mutex};

pub struct Executor<B: StorageBackend + Send + Sync + 'static> {
    catalog: Arc<Mutex<Catalog>>,
    backend: Arc<B>,
}

impl<B: StorageBackend + Send + Sync + 'static> Executor<B> {
    pub fn new(catalog: Arc<Mutex<Catalog>>, backend: Arc<B>) -> Self {
        Executor { catalog, backend }
    }

    pub fn execute(&self, stmt: Statement) {
        match stmt {
            Statement::CreateTable { name, columns, .. } => {
                let schema = TableSchema {
                    name: name.to_string(),
                    columns: columns
                        .iter()
                        .map(|c| Column {
                            name: c.name.to_string(),
                            data_type: c.data_type.to_string(),
                        })
                        .collect(),
                };

                self.catalog.lock().unwrap().create_table(schema.clone());
                self.backend.create_table(&schema).unwrap();
                println!("Table created: {}", name);
            }
            Statement::Insert { table_name, source, .. } => {
                let source = source.as_ref().expect("INSERT must have source");
                let values_rows = match &*source.body {
                    SetExpr::Values(values) => &values.rows,
                    _ => panic!("Only VALUES supported in INSERT"),
                };

                for row_exprs in values_rows {
                    let row = Row {
                        values: row_exprs.iter().map(|e| format!("{}", e)).collect(),
                    };
                    self.backend.insert_row(&table_name.to_string(), row).unwrap();
                }
                println!("Inserted into {}", table_name);
            }
            Statement::Query(q) => {
                let table_name = match &*q.body {
                    SetExpr::Select(select) => {
                        if let Some(TableWithJoins {
                            relation: TableFactor::Table { name, .. },
                            ..
                        }) = select.from.first()
                        {
                            name.to_string()
                        } else {
                            panic!("Only simple SELECTs supported");
                        }
                    }
                    _ => panic!("Only SELECT supported"),
                };

                let rows = self.backend.read_all_rows(&table_name).unwrap();
                for row in rows {
                    println!("{:?}", row.values);
                }
            }
            _ => println!("Statement type not supported yet"),
        }
    }
}
