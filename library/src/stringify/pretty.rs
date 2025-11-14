//! Pretty-printing JSON stringifier module

use crate::io::traits::IDestination;
use crate::nodes::node::*;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

/// Helper function to write an escaped JSON string
#[inline]
fn write_escaped_string(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");

    let bytes = s.as_bytes();
    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let needs_escape = match bytes[i] {
            b'"' | b'\\' | b'\n' | b'\r' | b'\t' => true,
            b if b < 32 => true,
            _ => false,
        };

        if needs_escape {
            if i > start {
                destination.add_bytes(core::str::from_utf8(&bytes[start..i]).unwrap());
            }

            match bytes[i] {
                b'"' => destination.add_bytes("\\\""),
                b'\\' => destination.add_bytes("\\\\"),
                b'\n' => destination.add_bytes("\\n"),
                b'\r' => destination.add_bytes("\\r"),
                b'\t' => destination.add_bytes("\\t"),
                b => destination.add_bytes(&format!("\\u{:04x}", b as u32)),
            }

            i += 1;
            start = i;
        } else {
            i += 1;
        }
    }

    if start < bytes.len() {
        destination.add_bytes(core::str::from_utf8(&bytes[start..]).unwrap());
    }

    destination.add_bytes("\"");
}

/// Serializes a `Node` into pretty-printed JSON
///
/// # Arguments
/// * `node` - The JSON node to serialize
/// * `destination` - The destination to write to
/// * `indent` - The indentation string (e.g., "  " for 2 spaces, "\t" for tabs)
pub fn stringify_pretty(
    node: &Node,
    destination: &mut dyn IDestination,
    indent: &str,
) -> Result<(), String> {
    stringify_pretty_internal(node, destination, indent, 0)
}

fn stringify_pretty_internal(
    node: &Node,
    destination: &mut dyn IDestination,
    indent: &str,
    depth: usize,
) -> Result<(), String> {
    match node {
        Node::None => destination.add_bytes("null"),
        Node::Boolean(value) => destination.add_bytes(if *value { "true" } else { "false" }),
        Node::Number(value) => match value {
            Numeric::Integer(n) => destination.add_bytes(&n.to_string()),
            Numeric::UInteger(n) => destination.add_bytes(&n.to_string()),
            Numeric::Float(f) => destination.add_bytes(&f.to_string()),
            Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
            Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
            Numeric::Int16(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt16(u) => destination.add_bytes(&u.to_string()),
            Numeric::Int8(i) => destination.add_bytes(&i.to_string()),
        },
        Node::Str(value) => write_escaped_string(value, destination),
        Node::Array(items) => {
            if items.is_empty() {
                destination.add_bytes("[]");
            } else {
                destination.add_bytes("[\n");
                for (index, item) in items.iter().enumerate() {
                    // Add indentation
                    for _ in 0..=depth {
                        destination.add_bytes(indent);
                    }
                    stringify_pretty_internal(item, destination, indent, depth + 1)?;
                    if index < items.len() - 1 {
                        destination.add_bytes(",");
                    }
                    destination.add_bytes("\n");
                }
                // Closing bracket indentation
                for _ in 0..depth {
                    destination.add_bytes(indent);
                }
                destination.add_bytes("]");
            }
        }
        Node::Object(map) => {
            if map.is_empty() {
                destination.add_bytes("{}");
            } else {
                destination.add_bytes("{\n");
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort(); // Sort keys for consistent output
                
                for (index, key) in keys.iter().enumerate() {
                    // Add indentation
                    for _ in 0..=depth {
                        destination.add_bytes(indent);
                    }
                    write_escaped_string(key, destination);
                    destination.add_bytes(": ");
                    let value = map.get(*key).unwrap();
                    stringify_pretty_internal(value, destination, indent, depth + 1)?;
                    if index < keys.len() - 1 {
                        destination.add_bytes(",");
                    }
                    destination.add_bytes("\n");
                }
                // Closing brace indentation
                for _ in 0..depth {
                    destination.add_bytes(indent);
                }
                destination.add_bytes("}");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;

    #[cfg(feature = "std")]
    use std::collections::HashMap;

    #[cfg(not(feature = "std"))]
    use alloc::collections::BTreeMap as HashMap;

    #[test]
    fn test_pretty_simple_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("name".to_string(), Node::Str("Alice".to_string()));
        map.insert("age".to_string(), Node::Number(Numeric::Int32(30)));
        
        stringify_pretty(&Node::Object(map), &mut dest, "  ").unwrap();
        let result = dest.to_string();
        
        assert!(result.contains("\"age\": 30"));
        assert!(result.contains("\"name\": \"Alice\""));
    }

    #[test]
    fn test_pretty_nested() {
        let mut dest = Buffer::new();
        let mut inner = HashMap::new();
        inner.insert("city".to_string(), Node::Str("NYC".to_string()));
        
        let mut outer = HashMap::new();
        outer.insert("address".to_string(), Node::Object(inner));
        
        stringify_pretty(&Node::Object(outer), &mut dest, "  ").unwrap();
        let result = dest.to_string();
        
        assert!(result.contains("\"address\": {"));
        assert!(result.contains("    \"city\": \"NYC\""));
    }

    #[test]
    fn test_pretty_array() {
        let mut dest = Buffer::new();
        let array = Node::Array(vec![
            Node::Number(Numeric::Int32(1)),
            Node::Number(Numeric::Int32(2)),
            Node::Number(Numeric::Int32(3)),
        ]);
        
        stringify_pretty(&array, &mut dest, "  ").unwrap();
        let result = dest.to_string();
        
        assert!(result.contains("[\n"));
        assert!(result.contains("  1"));
        assert!(result.contains("]"));
    }
}
