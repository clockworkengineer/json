//! YAML string conversion module for Node structures
//! Provides functionality to convert Node types into YAML formatted strings

use crate::nodes::node::*;
use crate::io::traits::IDestination;

/// Converts a Node into a YAML formatted string and writes it to the destination
///
/// # Arguments
/// * `node` - The Node to convert
/// * `destination` - The output destination implementing IDestination
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    stringify_with_indent(node, destination, 0);
    Ok(())
}

/// Converts a Node into a YAML formatted string with proper indentation
///
/// # Arguments
/// * `node` - The Node to convert
/// * `destination` - The output destination implementing IDestination
/// * `indent` - Current indentation level in spaces
fn stringify_with_indent(node: &Node, destination: &mut dyn IDestination, indent: usize) {
    match node {
        // Handle null values
        Node::None => destination.add_bytes("null"),
        // Handle boolean values
        Node::Boolean(value) => destination.add_bytes(if *value { "true" } else { "false" }),
        // Handle different numeric types
        Node::Number(value) => match value {
            Numeric::Integer(n) => destination.add_bytes(&n.to_string()),
            Numeric::UInteger(n) => destination.add_bytes(&n.to_string()),
            Numeric::Float(f) => destination.add_bytes(&f.to_string()),
            Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
            Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
            #[allow(unreachable_patterns)]
            _ => destination.add_bytes(&format!("{:?}", value)),
        },
        // Handle string values with special treatment for multi-line and quoted strings
        Node::Str(value) => {
            if value.contains('\n') || value.contains('"') {
                destination.add_bytes("|\n");
                for line in value.lines() {
                    destination.add_bytes(&" ".repeat(indent + 2));
                    destination.add_bytes(line);
                    destination.add_bytes("\n");
                }
            } else {
                destination.add_bytes(value);
            }
        },
        // Handle arrays with proper YAML list formatting
        Node::Array(items) => {
            if items.is_empty() {
                destination.add_bytes("[]");
                return;
            }
            destination.add_bytes("\n");
            for item in items {
                destination.add_bytes(&" ".repeat(indent));
                destination.add_bytes("- ");
                stringify_with_indent(item, destination, indent + 2);
                destination.add_bytes("\n");
            }
        },
        // Handle objects/maps with proper YAML mapping formatting
        Node::Object(entries) => {
            if entries.is_empty() {
                destination.add_bytes("{}");
                return;
            }
            destination.add_bytes("\n");
            for (key, value) in entries {
                destination.add_bytes(&" ".repeat(indent));
                destination.add_bytes(key);
                destination.add_bytes(": ");
                stringify_with_indent(value, destination, indent + 2);
                destination.add_bytes("\n");
            }
        }
    }
}

#[cfg(test)]
/// Tests for YAML stringification functionality
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::io::destinations::buffer::Buffer;

    #[test]
    fn test_stringify_null() {
        let mut dest = Buffer::new();
        stringify(&Node::None, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "null");
    }

    #[test]
    fn test_stringify_boolean() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(true), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "true");
        dest.clear();
        stringify(&Node::Boolean(false), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "false");
    }

    #[test]
    fn test_stringify_numbers() {
        let mut dest = Buffer::new();
        let test_cases = vec![
            (Node::Number(Numeric::Integer(-42)), "-42"),
            (Node::Number(Numeric::UInteger(42)), "42"),
            (Node::Number(Numeric::Float(42.5)), "42.5"),
            (Node::Number(Numeric::Byte(255)), "255"),
            (Node::Number(Numeric::Int32(-2147483648)), "-2147483648"),
            (Node::Number(Numeric::UInt32(4294967295)), "4294967295"),
        ];
        for (node, expected) in test_cases {
            dest.clear();
            stringify(&node, &mut dest).unwrap();
            assert_eq!(dest.to_string(), expected);
        }
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("simple".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "simple");

        dest.clear();
        stringify(&Node::Str("multi\nline".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "|\n  multi\n  line\n");

        dest.clear();
        stringify(&Node::Str("with \"quotes\"".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "|\n  with \"quotes\"\n");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "[]");

        dest.clear();
        stringify(&Node::Array(vec![
            Node::Number(Numeric::Integer(1)),
            Node::Str("test".to_string()),
        ]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\n- 1\n- test\n");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        stringify(&Node::Object(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{}");

        dest.clear();
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Object(map), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\nkey: value\n");
    }

    #[test]
    fn test_stringify_complex() {
        let mut dest = Buffer::new();
        let mut inner_map = HashMap::new();
        inner_map.insert("nested".to_string(), Node::Array(vec![
            Node::Boolean(true),
            Node::Str("multi\nline".to_string())
        ]));
        let mut outer_map = HashMap::new();
        outer_map.insert("test".to_string(), Node::Object(inner_map));
        stringify(&Node::Object(outer_map), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\ntest: \n  nested: \n    - true\n    - |\n        multi\n        line\n\n\n\n");
    }
    #[test]
    fn test_stringify_empty_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "[]");
    }

    #[test]
    fn test_stringify_nested_arrays() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Array(vec![
                Node::Number(Numeric::Integer(1)),
                Node::Number(Numeric::Integer(2))
            ]),
            Node::Array(vec![
                Node::Number(Numeric::Integer(3)),
                Node::Number(Numeric::Integer(4))
            ])
        ]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\n- \n  - 1\n  - 2\n\n- \n  - 3\n  - 4\n\n");
    }

    #[test]
    fn test_stringify_complex_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("Hello \"world\"\nWith\nMultiple\nLines".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "|\n  Hello \"world\"\n  With\n  Multiple\n  Lines\n");
    }

    #[test]
    fn test_stringify_array_with_empty_object() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Object(HashMap::new()),
            Node::Number(Numeric::Integer(1))
        ]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\n- {}\n- 1\n");
    }
    #[test]
    fn test_stringify_empty_object() {
        let mut dest = Buffer::new();
        stringify(&Node::Object(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{}");
    }
    #[test]
    fn test_stringify_empty_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("".to_string()), &mut dest).unwrap();
    }
    #[test]
    fn test_stringify_empty_number() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::Integer(0)), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "0");
    }
    #[test]
    fn test_stringify_empty_boolean() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(false), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "false");
    }
    #[test]
    fn test_stringify_empty_null() {
        let mut dest = Buffer::new();
        stringify(&Node::None, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "null");
    }
    #[test]
    fn test_stringify_empty_array_with_empty_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Str("".to_string()),
            Node::Number(Numeric::Integer(1))
        ]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\n- \n- 1\n");
    }
    
}
