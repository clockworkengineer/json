pub mod json_lib;

/// Returns the current version of the JSON library
// pub use json_lib::misc::get_version as version;

pub use json_lib::file::file::Format as Format;
pub use json_lib::file::file::detect_format as detect_format;
pub use json_lib::file::file::read_file_to_string as read_file_to_string;
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
