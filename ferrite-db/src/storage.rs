use bincode;
use serde::{Deserialize, Serialize}; // Serde traits to make our structs serializable
use std::fs::OpenOptions; // Allows us to open a file with custom options (e.g., create, append)
use std::io::Cursor;
use std::io::{Seek, SeekFrom, Write}; // Traits for writing to files (and seeking, if needed) // Binary encoding library used to serialize/deserialize the Row struct

// A simple row in our table. It holds a vector of strings.
// The #[derive(...)] automatically implements serialization, deserialization, and debugging output.
#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
    pub values: Vec<String>,
}

// Inserts a new row into the table file.
// `table_name` is the name of the table (used as the file name without extension).
// `row` is the data we want to write.
pub fn insert_row(table_name: &str, row: Row) {
    // Open the file for appending. If it doesn't exist, create it.
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open in append mode so we don't overwrite existing data
        .open(format!("{}.tbl", table_name)) // Build the file name from the table name
        .unwrap(); // In production code we would handle the error instead of unwrapping

    // Convert the Row struct into a binary format that can be written to the file.
    let encoded = bincode::serialize(&row).unwrap();

    // Write the encoded bytes to the file.
    file.write_all(&encoded).unwrap();
}

// Reads every Row from the table file into memory and returns them in a Vec<Row>.
pub fn read_all_rows(table_name: &str) -> Vec<Row> {
    // Build the file name from the table name, e.g., "users" -> "users.tbl"
    let path = format!("{}.tbl", table_name);

    // Read the entire file into memory as raw bytes.
    // If the file doesn't exist, `unwrap_or_default()` gives us an empty Vec<u8>.
    let bytes = std::fs::read(path).unwrap_or_default();

    // This will hold all the rows we successfully read.
    let mut rows = Vec::new();

    // A Cursor lets us treat a &[u8] (slice of bytes) like a file/stream
    // that we can "seek" through. It keeps track of our current position.
    let mut cursor = Cursor::new(&bytes);

    // Keep reading until we've reached the end of the byte array.
    while (cursor.position() as usize) < bytes.len() {
        // Deserialize (decode) a single Row starting at the current cursor position.
        // `deserialize_from` automatically moves the cursor forward
        // by the number of bytes it consumed for this Row.
        let row: Row = bincode::deserialize_from(&mut cursor).unwrap();

        // Add the Row to our growing collection.
        rows.push(row);
    }

    // Return the list of all decoded rows.
    rows
}
