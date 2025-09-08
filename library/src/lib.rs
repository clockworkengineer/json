//! json_lib - A lightweight, modular JSON toolkit for Rust
//!
//! This library provides a flexible JSON implementation with:
//! - Core Node type for representing JSON structures
//! - Parser to build Node trees from streams 
//! - Multiple format serializers (JSON, YAML, XML, Bencode)
//! - File and buffer I/O abstractions
//! - Pretty-printing utilities
//! - Unicode-aware file handling
//!
//! Minimum supported Rust version: 1.88.0

/// Module providing input/output operations for reading and writing JSON data
pub mod io;
/// Module containing JSON data structure definitions and node types
pub mod nodes;
/// Module implementing JSON parsing and value extraction
pub mod parser;
/// Module defining error types and handling for JSON operations,
/// including custom error types, error conversion implementations,
/// and utility functions for error handling
pub mod error;
/// Module for converting JSON structures to formatted strings
pub mod stringify;
/// Module handling JSON file reading and writing operations
pub mod file;
/// Module containing utility functions and helpers for JSON processing
pub mod misc;

///
/// JSON_lib API
///

/// Returns the current version of the JSON library
pub use misc::get_version as version;
/// Strip whitespace from a string.
pub use misc::strip as strip_whitespace;
/// Prints a formatted string to the destination.
pub use misc::print as print;
/// This enum represents different Unicode text file formats with their corresponding byte order marks (BOM)
pub use file::file::Format as Format;
/// This function detects the Unicode format of a text file by examining its byte order mark (BOM)
pub use file::file::detect_format as detect_format;
/// This function reads a text file and returns its content as a String, handling different Unicode formats
pub use file::file::read_file_to_string as read_file_to_string;
/// This function writes a string to a file in the specified Unicode format
pub use file::file::write_file_from_string as write_file_from_string;

/// Source implementation for reading JSON data from a memory buffer
pub use io::sources::buffer::Buffer as BufferSource;
/// Destination implementation for writing JSON data to a memory buffer
pub use io::destinations::buffer::Buffer as BufferDestination;
/// Source implementation for reading JSON data from a file
pub use io::sources::file::File as FileSource;
/// Destination implementation for writing JSON data to a file
pub use io::destinations::file::File as FileDestination;

/// Core data structure representing a JSON node and numerical node in the parsed tree
pub use nodes::node::Node as Node;
pub use nodes::node::Numeric as Numeric;

/// Converts a Node tree back to JSON format
pub use stringify::default::stringify as stringify;
/// Parses json data into a Node tree structure
pub use parser::default::parse as parse;
/// Converts a Node tree to JSON format
pub use stringify::bencode::stringify as to_bencode;
/// Converts a Node tree to YAML format
pub use stringify::yaml::stringify as to_yaml;
/// Converts a Node tree to XML format
pub use stringify::xml::stringify as to_xml;
