use crate::io::traits::IDestination;
use crate::{Node, Numeric};

/// Writes a TOML value to the given destination.
///
/// This function serializes a `Node` (representing a TOML value) and writes its TOML representation
/// to the provided `IDestination`. It handles all TOML value types, including strings, numbers,
/// booleans, arrays, and nulls. Objects are handled separately and are not written by this function.
///
/// # Arguments
/// * `value` - The TOML node to serialize.
/// * `destination` - The output destination implementing `IDestination`.
/// 
fn write_value(value: &Node, destination: &mut dyn IDestination) {
    match value {
        Node::Str(s) => {
            destination.add_bytes("\"");
            destination.add_bytes(s);
            destination.add_bytes("\"");
        }
        Node::Number(value) => match value {
            // Handles signed integer values
            Numeric::Integer(n) => destination.add_bytes(&n.to_string()),
            // Handles unsigned integer values
            Numeric::UInteger(n) => destination.add_bytes(&n.to_string()),
            // Handles floating point numbers
            Numeric::Float(f) => destination.add_bytes(&f.to_string()),
            // Handles 8-bit unsigned values (0-255)
            Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
            // Handles 32-bit signed integers (-2^31 to 2^31-1)
            Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
            // Handles 32-bit unsigned integers (0 to 2^32-1)
            Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
            // Fallback for any future numeric variants
            // If there are any other variants, add them here
            #[allow(unreachable_patterns)]
            _ => destination.add_bytes(&format!("{:?}", value)),
        },
        Node::Boolean(b) => destination.add_bytes(&*b.to_string()),
        Node::None => destination.add_bytes("null"),
        Node::Array(arr) => {
            destination.add_bytes("[");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    destination.add_bytes(", ");
                }
                write_value(item, destination);
            }
            destination.add_bytes("]");
        }
        Node::Object(_) => destination.add_bytes(""), // Handled separately
    }
}

/// Writes a TOML table (object) to the given destination.
///
/// This function serializes a `Node::Object` (representing a TOML table) and writes its TOML
/// representation to the provided `IDestination`. Nested objects are handled recursively,
/// with section headers generated for each nested table.
///
/// # Arguments
/// * `prefix` - The current key prefix for nested tables (empty for root).
/// * `obj` - The TOML node expected to be an object.
/// * `destination` - The output destination implementing `IDestination`.
fn write_table(prefix: &str, obj: &Node, destination: &mut dyn IDestination) {
    if let Node::Object(map) = obj {
        for (key, value) in map {
            match value {
                Node::Object(_) => {
                    let new_prefix = if prefix.is_empty() {
                        key.to_string()
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    destination.add_bytes(&format!("\n[{}]\n", new_prefix));
                    write_table(&new_prefix, value, destination);
                }
                _ => {
                    destination.add_bytes(&format!("{} = ", key));
                    write_value(value, destination);
                    destination.add_bytes("\n");
                }
            }
        }
    }
}

/// Converts a JSON node to TOML format and writes it to the destination
///
/// # Arguments
/// * `node` - The TOML node to convert
/// * `destination` - The output destination implementing IDestination
///
/// # Returns
/// * `Result<(), TomlError>` - Ok if successful, Err with message if root is not an object
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    if let Node::Object(_) = node {
        write_table("", node, destination);
        Ok(())
    } else {
        Err("Root node must be an object".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferDestination;
    use std::collections::{HashMap};

    #[test]
    fn test_stringify_array() {
        let mut dest = BufferDestination::new();
        let result = stringify(&Node::Array(vec![
            Node::Str("a".to_string()),
            Node::Number(crate::nodes::node::Numeric::Float(1.0)),
            Node::Boolean(true)
        ]), &mut dest);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Root node must be an object");
    }

    #[test]
    fn test_stringify_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        let mut dest = BufferDestination::new();
        let hashmap = map.into_iter().collect::<std::collections::HashMap<_, _>>();
        let _ = stringify(&Node::Object(hashmap), &mut dest);
        assert_eq!(dest.to_string(), "key = \"value\"\n");
    }

    #[test]
    fn test_stringify_non_object_root() {
        let mut dest = BufferDestination::new();
        let result = stringify(&Node::Str("test".to_string()), &mut dest);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Root node must be an object");
    }

    #[test]
    fn test_stringify_nested_object() {
        let mut inner = HashMap::new();
        inner.insert("inner_key".to_string(), Node::Str("value".to_string()));
        let inner_hashmap = inner.into_iter().collect::<std::collections::HashMap<_, _>>();
        let mut outer = HashMap::new();
        outer.insert("outer".to_string(), Node::Object(inner_hashmap));
        let outer_hashmap = outer.into_iter().collect::<std::collections::HashMap<_, _>>();
        let mut dest = BufferDestination::new();
        stringify(&Node::Object(outer_hashmap), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\n[outer]\ninner_key = \"value\"\n");
    }
}