//! Miscellaneous utility functions for JSON processing
//! Contains functionality for version information and formatted JSON printing

use crate::json_lib::io::traits::{IDestination, ISource};
use crate::Node;
use crate::json_lib::nodes::node::Numeric;

/// Returns the current version of the package as specified in Cargo.toml.
/// Uses CARGO_PKG_VERSION environment variable that is set during compilation
/// from the version field in Cargo.toml.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
/// Prints a JSON node to the specified destination with formatting
///
/// # Arguments
/// * `node` - The JSON node to print
/// * `destination` - The output destination implementing IDestination
/// * `indent` - Number of spaces to use for each indentation level
/// * `current_indent` - Current indentation level in spaces
pub fn print(node: &Node, destination: &mut dyn IDestination, indent: usize, current_indent: usize) {
    match node {
        // Handle boolean values (true/false)
        Node::Boolean(value) => destination.add_bytes(&value.to_string()),
        // Handle various numeric types
        Node::Number(value) => match value {
            Numeric::Integer(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInteger(u) => destination.add_bytes(&u.to_string()),
            Numeric::Float(f) => destination.add_bytes(&f.to_string()),
            Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
            Numeric::Int8(i) => destination.add_bytes(&i.to_string()),
            Numeric::Int16(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt16(u) => destination.add_bytes(&u.to_string()),
            Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
        },
        // Handle string values with proper JSON escaping
        Node::Str(value) => destination.add_bytes(&format!("\"{}\"", value)),
        // Handle null values
        Node::None => destination.add_bytes("null"),
        // Handle arrays with proper indentation and formatting
        Node::Array(array) => {
            destination.add_bytes("[\n");
            for (i, item) in array.iter().enumerate() {
                destination.add_bytes(&" ".repeat(current_indent + indent));
                print(item, destination, indent, current_indent + indent);
                if i < array.len() - 1 {
                    destination.add_bytes(",");
                }
                destination.add_bytes("\n");
            }
            destination.add_bytes(&" ".repeat(current_indent));
            destination.add_bytes("]");
        }
        // Handle objects/maps with proper indentation and key-value formatting
        Node::Object(map) => {
            destination.add_bytes("{\n");
            for (i, (key, value)) in map.iter().enumerate() {
                destination.add_bytes(&" ".repeat(current_indent + indent));
                destination.add_bytes(&format!("\"{}\"", key));
                destination.add_bytes(": ");
                print(value, destination, indent, current_indent + indent);
                if i < map.len() - 1 {
                    destination.add_bytes(",");
                }
                destination.add_bytes("\n");
            }
            destination.add_bytes(&" ".repeat(current_indent));
            destination.add_bytes("}");
        }
    }
}

/// Strips whitespace from JSON while preserving string content.
/// Copies non-whitespace characters from source to destination, handling string literals specially.
pub fn strip(source: &mut dyn ISource, destination: &mut dyn IDestination) {
    while source.more() {
        if let Some(c) = source.current() {
            // Skip whitespace characters outside of strings
            if !(c == ' ' || c == '\t' || c == '\n' || c == '\r') {
                destination.add_byte(c as u8);

                // Handle string literals especially to preserve their whitespace
                if source.current() == Some('"') {
                    source.next();
                    // Copy characters until closing quote
                    while source.more() && source.current() != Some('"') {
                        // Handle escaped characters
                        if source.current() == Some('\\') {
                            destination.add_bytes(&source.current().unwrap().to_string());
                            source.next();
                        }
                        destination.add_byte(source.current().unwrap() as u8);
                        source.next();
                    }
                    destination.add_bytes(&source.current().unwrap().to_string());
                }
            }
        }
        source.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferDestination;
    use crate::Node;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_version_env() {
        assert_eq!(get_version(), "0.1.0");
    }

    #[test]
    fn test_print_boolean() {
        let mut dest = BufferDestination::new();
        print(&Node::Boolean(true), &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "true");
    }

    #[test]
    fn test_print_numeric() {
        let mut dest = BufferDestination::new();
        print(&Node::Number(Numeric::Integer(42)), &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "42");
    }

    #[test]
    fn test_print_string() {
        let mut dest = BufferDestination::new();
        print(&Node::Str("hello".to_string()), &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "\"hello\"");
    }

    #[test]
    fn test_print_array() {
        let mut dest = BufferDestination::new();
        print(&Node::Array(vec![Node::Boolean(true), Node::Number(Numeric::Integer(1))]), &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "[\ntrue,\n1\n]");
    }

    #[test]
    fn test_print_object() {
        let mut dest = BufferDestination::new();
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        let hashmap: std::collections::HashMap<String, Node> = map.into_iter().collect();
        print(&Node::Object(hashmap), &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "{\n\"key\": \"value\"\n}");
    }

    #[test]
    fn test_print_null() {
        let mut dest = BufferDestination::new();
        print(&Node::None, &mut dest, 0, 0);
        assert_eq!(dest.to_string(), "null");
    }

    #[test]
    fn test_strip_basic_object() {
        let json = "{ \"name\" :  \"value\" }";
        let mut source = crate::BufferSource::new(json.as_bytes());
        let mut dest = BufferDestination::new();
        strip(&mut source, &mut dest);
        assert_eq!(dest.to_string(), "{\"name\":\"value\"}");
    }

    #[test]
    fn test_strip_string_content() {
        let json = "{ \"text\" : \"  keep  spaces  \" }";
        let mut source = crate::BufferSource::new(json.as_bytes());
        let mut dest = BufferDestination::new();
        strip(&mut source, &mut dest);
        assert_eq!(dest.to_string(), "{\"text\":\"  keep  spaces  \"}");
    }

    #[test]
    fn test_strip_nested_structure() {
        let json = "{\n \"array\": [\n  1,\n  2\n ]\n}";
        let mut source = crate::BufferSource::new(json.as_bytes());
        let mut dest = BufferDestination::new();
        strip(&mut source, &mut dest);
        assert_eq!(dest.to_string(), "{\"array\":[1,2]}");
    }
}