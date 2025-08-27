pub mod json_lib;

/// Returns the current version of the json library
// pub use json_lib::misc::get_version as version;
// /// Reads and parses a json-encoded file from disk
// pub use json_lib::misc::read_json_file as read_file;
// /// Writes json-encoded data to a file on disk
// pub use json_lib::misc::write_json_file as write_file;

/// Source implementation for reading json data from a memory buffer
pub use json_lib::io::sources::buffer::Buffer as BufferSource;
/// Destination implementation for writing json data to a memory buffer
pub use json_lib::io::destinations::buffer::Buffer as BufferDestination;
/// Source implementation for reading json data from a file
pub use json_lib::io::sources::file::File as FileSource;
/// Destination implementation for writing json data to a file
pub use json_lib::io::destinations::file::File as FileDestination;

/// Core data structure representing a json node in the parsed tree
pub use json_lib::nodes::node::Node as Node;
pub use json_lib::nodes::node::Numeric as Numeric;

/// Converts a Node tree back to json format
pub use json_lib::stringify::default::stringify as stringify;
/// Parses json data into a Node tree structure
pub use json_lib::parser::default::parse as parse;
/// Converts a Node tree to JSON format
pub use json_lib::stringify::default::stringify as to_json;
/// Converts a Node tree to YAML format
pub use json_lib::stringify::yaml::stringify as to_yaml;
/// Converts a Node tree to XML format
pub use json_lib::stringify::xml::stringify as to_xml;
