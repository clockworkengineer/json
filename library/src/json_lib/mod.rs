//! JSON processing library providing parsing, manipulation, and serialization capabilities
//! for working with JSON data in Rust

/// Input/output operations for reading and writing JSON data
pub mod io;
/// Core JSON data structures and node type definitions
pub mod nodes;
/// JSON parsing functionality for converting text to structured data
pub mod parser;
/// Error types and handling for JSON operations
pub mod error;
/// JSON serialization for converting data structures to JSON text
pub mod stringify;
/// File system operations for reading and writing JSON files
pub mod file;
/// Miscellaneous utility functions for JSON processing
pub mod misc;