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

/// Module defining error types and handling for JSON operations.
pub mod error;
/// Module handling JSON file reading and writing operations
pub mod file;
/// Module providing input/output operations for reading and writing JSON data
pub mod io;
/// Module containing utility functions and helpers for JSON processing
pub mod misc;
/// Module containing JSON data structure definitions and node types
pub mod nodes;
/// Module implementing JSON parsing and value extraction
pub mod parser;
/// Module for converting JSON structures to formatted strings
pub mod stringify;

/// Integration tests module
mod integration_tests;

/// This enum represents different Unicode text file formats with their corresponding byte order marks (BOM)
pub use file::file::Format;
/// This function detects the Unicode format of a text file by examining its byte order mark (BOM)
pub use file::file::detect_format;
/// This function reads a text file and returns its content as a String, handling different Unicode formats
pub use file::file::read_file_to_string;
/// This function writes a string to a file in the specified Unicode format
pub use file::file::write_file_from_string;
///
/// JSON_lib API
///

/// Returns the current version of the JSON library
pub use misc::get_version as version;
/// Prints a formatted string to the destination.
pub use misc::print;
/// Strip whitespace from a string.
pub use misc::strip as strip_whitespace;

/// Destination implementation for writing JSON data to a memory buffer
pub use io::destinations::buffer::Buffer as BufferDestination;
/// Destination implementation for writing JSON data to a file
pub use io::destinations::file::File as FileDestination;
/// Source implementation for reading JSON data from a memory buffer
pub use io::sources::buffer::Buffer as BufferSource;
/// Source implementation for reading JSON data from a file
pub use io::sources::file::File as FileSource;
/// Core data structure representing a JSON node and numerical node in the parsed tree
pub use nodes::node::Node;
/// Core data structure representing a numeric value node in the parsed tree
pub use nodes::node::Numeric;
/// Parses json data into a Node tree structure
pub use parser::default::parse;
/// Converts a Node tree to JSON format
pub use stringify::bencode::stringify as to_bencode;
/// Converts a Node tree back to JSON format
pub use stringify::default::stringify;
/// Converts a Node tree to TOML format
pub use stringify::toml::stringify as to_toml;
/// Converts a Node tree to XML format
pub use stringify::xml::stringify as to_xml;
/// Converts a Node tree to YAML format
pub use stringify::yaml::stringify as to_yaml;

// JSON Pointer support (RFC 6901)
/// Gets a value from a Node using a JSON Pointer string
pub use nodes::json_pointer::get as pointer_get;
/// Gets a mutable reference to a value using a JSON Pointer string
pub use nodes::json_pointer::get_mut as pointer_get_mut;
/// Removes a value from a Node using a JSON Pointer string
pub use nodes::json_pointer::remove as pointer_remove;
/// Sets a value in a Node using a JSON Pointer string
pub use nodes::json_pointer::set as pointer_set;
