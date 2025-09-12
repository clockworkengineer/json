use std::path::Path;
// Import the necessary types and functions from custom libraries
use json_lib::{FileSource, parse, FileDestination, to_yaml};
use json_utility_lib::get_json_file_list;

/// Processes a single JSON file by converting it to YAML format
///
/// # Arguments
///
/// * `file_path` - Path to the JSON file to be converted
///
/// # Returns
///
/// * `Result<(), String>` - Ok(()) on successful conversion, Err with an error message on failure
fn process_json_file(file_path: &str) -> Result<(), String> {
    // Create a file source for reading JSON data
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;

    // Parse the JSON content into an abstract syntax tree
    let node = parse(&mut source).map_err(|e| e.to_string())?;

    // Create a file destination for the YAML output
    // Changes the file extension from .json to .yaml
    let mut destination = FileDestination::new(
        Path::new(file_path)
            .with_extension("yaml")
            .to_string_lossy()
            .as_ref()
    ).map_err(|e| e.to_string())?;

    // Convert and write the parsed JSON to YAML format
    to_yaml(&node, &mut destination).unwrap();
    Ok(())
}

fn main() {
    // Get a list of all JSON files in the "files" directory
    let json_files = get_json_file_list("files");

    // Process each JSON file in the list
    for file_path in json_files {
        match process_json_file(&file_path) {
            // Print a success or error message for each file
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}