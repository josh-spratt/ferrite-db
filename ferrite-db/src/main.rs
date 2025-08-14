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

    // -----------------------------------------------------------------
    //  Create shared state
    // -----------------------------------------------------------------
    // Wrap the catalog in `Mutex` so only one thread can modify it at a
    // time, and we wrap that in `Arc` (atomic reference counted)
    // so it can be shared across threads safely.
    // In this single‑threaded example the `Arc`/`Mutex` isn't strictly
    // necessary, but it's a common pattern for code that may become
    // multi‑threaded later.
    let catalog = Arc::new(Mutex::new(
    Catalog::load_from_file("catalog.json").unwrap_or_else(|err| {
        eprintln!("Warning: could not load catalog: {}", err);
        Catalog::new()
        })
    ));


    // `FileStorageBackend` is the part of the system that talks to
    // disk.  It too is wrapped in `Arc` for shared ownership.
    let backend = Arc::new(FileStorageBackend::new());

    // Build the `Executor`.  It will be responsible for taking a
    // parsed SQL statement and turning it into actions (e.g. reading
    // from the catalog, accessing the storage, etc.).
    // We pass it clones of the `Arc`s.  `Arc::clone()` simply
    // increments the reference count; no deep copy is performed.
    let executor = Executor::new(catalog.clone(), backend);

    // -----------------------------------------------------------------
    //  REPL loop: read‑evaluate‑print loop
    // -----------------------------------------------------------------
    loop {
        print!("> ");
        // Flush stdout immediately so the prompt appears before
        // the program blocks waiting for input.
        io::stdout().flush().unwrap();

        // Read a line of input from the user into a mutable String.
        let mut input = String::new();
        // If reading fails (e.g. EOF), break out of the loop.
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        // -------------------------------------------------------------
        //  Parse the SQL that the user typed
        // -------------------------------------------------------------
        // `parse_sql` returns a `Result`.  On success we get a
        // vector of statements (`stmts`).  On failure we get an error.
        let stmts = match parse_sql(&input) {
            Ok(stmts) => stmts, // parsing succeeded
            Err(e) => {
                // Show the error to the user and start the next loop
                // iteration without executing anything.
                eprintln!("Parse error: {}", e);
                continue;
            }
        };

        // -------------------------------------------------------------
        //  Execute each parsed statement
        // -------------------------------------------------------------
        for stmt in stmts {
            executor.execute(stmt);
        }
    }
}
