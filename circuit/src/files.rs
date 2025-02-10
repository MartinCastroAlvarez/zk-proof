use std::fs::File;
use std::io::{Read, Result};

/// Helper: reads a file fully into a `String`.
pub fn read_file_to_string(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
