// Import the necessary types and functions from json_lib and json_utility_lib
use json_lib::{parse, to_toml, FileDestination, FileSource};
use json_utility_lib::get_json_file_list;
use std::fs;
use std::path::Path;

/// Processes a single JSON file by converting it to TOML format.
///
/// # Arguments
/// * `file_path` - Path to the JSON file to be converted
///
/// # Returns
/// * `Result<(), String>` - Ok(()) if successful, Err with an error message if failed
fn process_json_file(file_path: &str) -> Result<(), String> {
    // Create a new file source for reading JSON data
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;

    // Parse the JSON content into a Node structure
    let node = parse(&mut source).map_err(|e| e.to_string())?;

    // Create a new file destination with .xml extension for the output
    let mut destination = FileDestination::new(
        Path::new(file_path)
            .with_extension("toml")
            .to_string_lossy()
            .as_ref()
    ).map_err(|e| e.to_string())?;

    // Convert the parsed JSON node to TOML format and write to destination
    to_toml(&node, &mut destination)?;
    Ok(())
}

fn main() {
    // Get a list of all JSON files in the "files" directory
    let json_files = get_json_file_list("files");

    // Process each JSON file in the list
    for file_path in json_files {
        // Attempt to convert each file and handle any errors
        match process_json_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => {
                eprintln!("Failed to convert {}: {}", file_path, e);
                let toml_file = file_path.replace(".json", ".toml");
                fs::remove_file(toml_file).unwrap();
            }
        }
    }
}
