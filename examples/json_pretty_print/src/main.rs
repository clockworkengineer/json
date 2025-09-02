use std::path::Path;
// Import required functionality from json_lib and json_utility_lib
use json_lib::{FileSource, parse, FileDestination};  // Removed unused 'to_bencode' import
use json_lib::json_lib::misc::print;
use json_utility_lib::get_json_file_list;

/// Processes a single JSON file by reading, parsing, and pretty-printing it
///
/// # Arguments
///
/// * `file_path` - Path to the input JSON file
///
/// # Returns
///
/// * `Result<(), String>` - Ok(()) on success, Err with error message on failure
fn process_json_file(file_path: &str) -> Result<(), String> {
    // Create a source reader for the JSON file
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;

    // Parse the JSON data into an in-memory node structure
    let node = parse(&mut source).map_err(|e| e.to_string())?;

    // Create a destination file with "_pp" suffix for pretty-printed output
    let mut destination = FileDestination::new(
        Path::new(file_path)
            .with_extension("json_pp")
            .to_string_lossy()
            .as_ref()
    ).map_err(|e| e.to_string())?;

    // Pretty print the JSON with 4-space indentation
    print(&node, &mut destination, 4, 0);
    Ok(())
}

fn main() {
    // Get list of JSON files from the "files" directory
    let json_files = get_json_file_list("files");

    // Process each JSON file
    for file_path in json_files {
        match process_json_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}