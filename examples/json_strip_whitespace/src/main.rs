use std::path::Path;
use json_lib::{FileSource, FileDestination, strip_whitespace};
use json_utility_lib::get_json_file_list;

/// Processes a single JSON file by reading, parsing, and pretty-printing it
///
/// # Arguments
///
/// * `file_path` - Path to the input JSON file
///
/// # Returns
///
/// * `Result<(), String>` - Ok(()) on success, Err with an error message on failure
fn process_json_file(file_path: &str) -> Result<(), String> {
    // Create a source reader for the JSON file
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;

    // Create a destination file with "_pp" suffix for pretty-printed output
    let mut destination = FileDestination::new(
        Path::new(file_path)
            .with_extension("json_stripped")
            .to_string_lossy()
            .as_ref()
    ).map_err(|e| e.to_string())?;

    // Pretty print the JSON with 4-space indentation
    strip_whitespace(&mut source, &mut destination);
    Ok(())
}

fn main() {
    // Get a list of JSON files from the "files" directory
    let json_files = get_json_file_list("files");

    // Process each JSON file
    for file_path in json_files {
        match process_json_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}
