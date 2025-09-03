
//! Utility library for handling JSON files and related operations.
//! Provides functionality for file system operations specific to JSON files.

use std::fs;
use std::path::Path;

/// Returns a list of JSON file paths from the specified directory.
///
/// # Arguments
///
/// * `file_path` - Path to the directory containing JSON files
///
/// # Returns
///
/// A vector of strings containing paths to all .json files in the directory
pub fn get_json_file_list(file_path: &str) -> Vec<String> {
    // Convert the input path string to a Path
    let files_dir = Path::new(file_path);
    // Create a directory if it doesn't exist
    if !files_dir.exists() {
        fs::create_dir("files").expect("Failed to create files directory");
        return vec![];
    }

    // Read the directory and collect all .json files
    fs::read_dir(files_dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            // Extract entry and convert to the path
            let entry = entry.ok()?;
            let file_path = entry.path();
            // Check if the file has a.json extension
            if file_path.extension()? == "json" {
                Some(file_path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

