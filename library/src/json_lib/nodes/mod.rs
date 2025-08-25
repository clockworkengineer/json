/// Module implementing JSON data structure types and operations.
///
/// This module provides the core functionality for working with JSON format:
/// * Parsing raw JSON data into structured representations
/// * Manipulating JSON data structures in memory
/// * Serializing JSON structures back to their encoded form
///
/// Supports all JSON data types:
/// * Byte strings (length-prefixed)
/// * Integers
/// * Lists (ordered sequences)
/// * Dictionaries (key-value pairs)
pub mod node;