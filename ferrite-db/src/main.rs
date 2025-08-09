mod catalog;
mod executor;
mod parser;
mod storage;

use std::io::{self, Write};

fn main() {
    println!("Welcome to ferrite-db!");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let stmts = match parser::parse_sql(&input) {
            Ok(stmts) => stmts,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                continue;
            }
        };

        for stmt in stmts {
            executor::execute(stmt);
        }
    }
}
