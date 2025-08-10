mod catalog;
mod executor;
mod parser;
mod storage;

use catalog::Catalog;
use executor::Executor;
use parser::parse_sql;
use storage::FileStorageBackend;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};

fn main() {
    println!("Welcome to ferrite-db!");

    let catalog = Arc::new(Mutex::new(Catalog::new()));
    let backend = Arc::new(FileStorageBackend::new());
    let executor = Executor::new(catalog.clone(), backend);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let stmts = match parse_sql(&input) {
            Ok(stmts) => stmts,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                continue;
            }
        };

        for stmt in stmts {
            executor.execute(stmt);
        }
    }
}
