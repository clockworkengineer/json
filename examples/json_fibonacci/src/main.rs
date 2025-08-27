//! Fibonacci sequence generator using json format for storage.
//! This program maintains a sequence of Fibonacci numbers in a json file,
//! reading the existing sequence and appending the next number on each run.

use json_lib::{FileDestination, FileSource, Node, parse, stringify, Numeric};
use std::path::Path;

/// Reads a Fibonacci sequence from a json-encoded file.
/// If the file doesn't exist, initializes a new sequence starting with [1, 1].
///
/// # Arguments
/// * `file_path` - Path to the json file containing the sequence
///
/// # Returns
/// * `Ok(Node)` - A Node::List containing the sequence
/// * `Err(String)` - Error message if reading or parsing fails
fn read_sequence(file_path: &Path) -> Result<Node, String> {
    // Initialize with the default sequence if the file doesn't exist
    if !file_path.exists() {
        return Ok(Node::Array(vec![Node::Number(Numeric::Integer(1)), Node::Number(Numeric::Integer(1))]));
    }

    // Try to open and parse the existing file
    match FileSource::new(&file_path.to_string_lossy()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::Array(list)) => Ok(Node::Array(list)),
            Ok(_) => Err("Invalid file format: expected a list".to_string()),
            Err(e) => Err(e),
        },
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }

}

/// Adds the next Fibonacci number to the sequence by summing the last two numbers.
/// Uses checked addition to prevent integer overflow.
///
/// # Arguments
/// * `sequence` - Mutable reference to the Node containing the sequence
fn add_next(sequence: &mut Node) {
    // Extract the list of numbers from the Node
    if let Node::Array(items) = sequence {
        if items.len() < 2 {
            return;
        }
        match (&items[items.len() - 2], &items[items.len() - 1]) {
            (Node::Number(Numeric::Integer(a)), Node::Number(Numeric::Integer(b))) => {
                if let Some(sum) = a.checked_add(*b) {
                    items.push(Node::Number(Numeric::Integer(sum)));
                }
            }
            _ => {}
        }
    }

}

/// Saves the Fibonacci sequence to a json-encoded file.
///
/// # Arguments
/// * `file_path` - Path where to save the sequence
/// * `sequence` - The Node containing the sequence to save
///
/// # Returns
/// * `Ok(())` - Write operation succeeded
/// * `Err(String)` - Error message if writing fails
fn write_sequence(file_path: &Path, sequence: &Node) -> Result<(), String> {
    // Create a new file destination, falling back to empty string if the path is invalid
    let  file = FileDestination::new(file_path.to_str().unwrap_or(""));
    match file {
        Ok(mut f) => { stringify(&sequence, &mut f); Ok(()) }
        Err(e) => { Err( e.to_string())}
    }
}

/// Main program entry point.
/// Reads the existing Fibonacci sequence, adds the next number,
/// and saves the updated sequence back to the file.
fn main() {
    // Define the file_path to the sequence file
    let file_path = Path::new("fibonacci.json");
    match read_sequence(file_path) {
        Ok(mut sequence) => {
            add_next(&mut sequence);
            if let Err(e) = write_sequence(file_path, &sequence) {
                eprintln!("Failed to write sequence: {}", e);
                return;
            }
        }
        Err(e) => eprintln!("Failed to read sequence: {}", e)
    }
}
