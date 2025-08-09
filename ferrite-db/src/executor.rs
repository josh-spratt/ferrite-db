use crate::catalog::{Catalog, Column, TableSchema};
use crate::storage::{Row, insert_row, read_all_rows};
use sqlparser::ast::*;
use std::sync::Mutex; // Used for thread-safe, shared mutable data.

// `lazy_static!` lets us create static variables that require code to initialize them.
// In Rust, normal `static` variables must be constant values, but here we want
// to create something like a global variable that's computed at runtime.
lazy_static::lazy_static! {
    // `CATALOG` is our in-memory "database catalog" — the list of all tables & their schemas.
    // We wrap it in `Mutex` so that multiple threads can safely read/write it without data races.
    static ref CATALOG: Mutex<Catalog> = Mutex::new(Catalog::new());
}

// This function takes a `Statement` (from sqlparser) and executes it.
// The Statement represents a parsed SQL command (e.g., CREATE TABLE, INSERT, SELECT).
pub fn execute(stmt: Statement) {
    // `match` works like a switch-case in other languages, but it's more powerful.
    // It lets us check which kind of Statement we have and run different code for each.
    match stmt {

        // ---------- CREATE TABLE ----------
        Statement::CreateTable { name, columns, .. } => {
            // Build a TableSchema from the parsed SQL data
            let schema = TableSchema {
                name: name.to_string(), // Convert name to a Rust String
                columns: columns
                    .iter() // Loop through all parsed column definitions
                    .map(|c| Column {
                        name: c.name.to_string(),         // Column name as String
                        data_type: c.data_type.to_string(), // Column type as String
                    })
                    .collect(), // Collect all mapped columns into a Vec<Column>
            };

            // Lock the global catalog, add the new table schema, then unlock automatically
            CATALOG.lock().unwrap().create_table(schema);
            println!("Table created: {}", name);
        }

        // ---------- INSERT INTO ... VALUES ----------
        Statement::Insert {
            table_name, source, ..
        } => {
            // `source` is the data we want to insert (VALUES or SELECT ...).
            // For now, we only support INSERT with VALUES.
            let source = match source.as_ref() {
                Some(q) => q,
                None => {
                    println!("INSERT without source not supported");
                    return; // Exit the function early
                }
            };

            // `source.body` is a Box<SetExpr>, so `&*` dereferences it.
            // We check if it's specifically `SetExpr::Values(...)`.
            let values_rows = match &*source.body {
                SetExpr::Values(values) => &values.rows,
                _ => panic!("Only VALUES supported in INSERT"),
            };

            // Loop through each row of VALUES (...), (...), ...
            for row_exprs in values_rows {
                let row = Row {
                    // Convert each SQL expression into a String
                    values: row_exprs.iter().map(|e| format!("{}", e)).collect(),
                };
                // Insert the row into storage for this table
                insert_row(&table_name.to_string(), row);
            }

            println!("Inserted into {}", table_name);
        }

        // ---------- SELECT ----------
        Statement::Query(q) => {
            // For now, only SELECT from a single table is supported.
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

            // Read all rows from the given table and print them.
            let rows = read_all_rows(&table_name);
            for row in rows {
                println!("{:?}", row.values); // {:?} prints debug format
            }
        }

        // ---------- Everything else ----------
        _ => println!("Statement type not supported yet"),
    }
}
