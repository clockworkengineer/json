use std::path::Path;
use json_lib::{FileSource, parse, FileDestination, to_bencode};
use json_utility_lib::get_json_file_list;

/// Processes a single JSON file by converting it to bencode format
///
/// # Arguments
/// * `file_path` - Path to the JSON file to be processed
///
/// # Returns
/// * `Result<(), String>` - Ok(()) if successful, Err with an error message if failed
fn process_json_file(file_path: &str) -> Result<(), String> {
    // Create a file source for reading JSON data
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;

    // Parse the JSON content into an abstract syntax tree
    let node = parse(&mut source).map_err(|e| e.to_string())?;

    // Create a destination file with .bencode extension for output
    let mut destination = FileDestination::new(
        Path::new(file_path)
            .with_extension("bencode")
            .to_string_lossy()
            .as_ref()
    ).map_err(|e| e.to_string())?;

    // Convert and write the parsed JSON to bencode format
    to_bencode(&node, &mut destination);
    Ok(())
}

fn main() {
    // Get a list of JSON files from the "files" directory
    let json_files = get_json_file_list("files");

    // Process each JSON file and convert it to bencode format
    for file_path in json_files {
        match process_json_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}