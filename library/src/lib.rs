pub mod json_lib;

/// Returns the current version of the JSON library
pub use json_lib::misc::get_version as version;
pub use json_lib::misc::strip as strip_whitespace;
/// This enum represents different Unicode text file formats with their corresponding byte order marks (BOM)
pub use json_lib::file::file::Format as Format;
/// This function detects the Unicode format of a text file by examining its byte order mark (BOM)
pub use json_lib::file::file::detect_format as detect_format;
/// This function reads a text file and returns its content as a String, handling different Unicode formats
pub use json_lib::file::file::read_file_to_string as read_file_to_string;
/// This function writes a string to a file in the specified Unicode format
pub use json_lib::file::file::write_file_from_string as write_file_from_string;

/// Source implementation for reading JSON data from a memory buffer
pub use json_lib::io::sources::buffer::Buffer as BufferSource;
/// Destination implementation for writing JSON data to a memory buffer
pub use json_lib::io::destinations::buffer::Buffer as BufferDestination;
/// Source implementation for reading JSON data from a file
pub use json_lib::io::sources::file::File as FileSource;
/// Destination implementation for writing JSON data to a file
pub use json_lib::io::destinations::file::File as FileDestination;

/// Core data structure representing a JSON node and numerical node in the parsed tree
pub use json_lib::nodes::node::Node as Node;
pub use json_lib::nodes::node::Numeric as Numeric;

/// Converts a Node tree back to JSON format
pub use json_lib::stringify::default::stringify as stringify;
/// Parses json data into a Node tree structure
pub use json_lib::parser::default::parse as parse;
/// Converts a Node tree to JSON format
pub use json_lib::stringify::bencode::stringify as to_bencode;
/// Converts a Node tree to YAML format
pub use json_lib::stringify::yaml::stringify as to_yaml;
/// Converts a Node tree to XML format
pub use json_lib::stringify::xml::stringify as to_xml;
