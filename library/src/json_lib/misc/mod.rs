use crate::json_lib::io::traits::IDestination;
use crate::Node;
use crate::json_lib::nodes::node::Numeric;

/// Returns the current version of the package as specified in Cargo.toml.
/// Uses CARGO_PKG_VERSION environment variable that is set during compilation
/// from the version field in Cargo.toml.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
pub fn print(node: &Node, destination: &mut dyn IDestination, indent: usize) {
    match node {
        Node::Boolean(b) => destination.add_bytes(if *b { "true" } else { "false" }),
        Node::Number(n) => match n {
            Numeric::Integer(i) => destination.add_bytes(&i.to_string()),
            Numeric::Float(f) => destination.add_bytes(&f.to_string()),
            Numeric::UInteger(u) => destination.add_bytes(&u.to_string()),
            Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
            Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
            Numeric::Int16(i) => destination.add_bytes(&i.to_string()),
            Numeric::UInt16(u) => destination.add_bytes(&u.to_string()),
            Numeric::Int8(i) => destination.add_bytes(&i.to_string()),
        },
        Node::Str(s) => {
            destination.add_byte(b'"');
            destination.add_bytes(s);
            destination.add_byte(b'"');
        }
        Node::Array(arr) => {
            destination.add_byte(b'[');
            if !arr.is_empty() {
                destination.add_byte(b'\n');
                for (i, item) in arr.iter().enumerate() {
                    destination.add_bytes(&" ".repeat(indent + 2));
                    print(item, destination, indent + 2);
                    if i < arr.len() - 1 {
                        destination.add_bytes(",\n");
                    }
                }
                destination.add_byte(b'\n');
                destination.add_bytes(&" ".repeat(indent));
            }
            destination.add_byte(b']');
        }
        Node::Object(obj) => {
            destination.add_byte(b'{');
            if !obj.is_empty() {
                destination.add_byte(b'\n');
                let mut entries: Vec<_> = obj.iter().collect();
                entries.sort_by(|a, b| a.0.cmp(b.0));
                for (i, (key, value)) in entries.iter().enumerate() {
                    destination.add_bytes(&" ".repeat(indent + 2));
                    destination.add_byte(b'"');
                    destination.add_bytes(key);
                    destination.add_bytes("\": ");
                    print(value, destination, indent + 2);
                    if i < entries.len() - 1 {
                        destination.add_bytes(",\n");
                    }
                }
                destination.add_byte(b'\n');
                destination.add_bytes(&" ".repeat(indent));
            }
            destination.add_byte(b'}');
        }
        Node::None => destination.add_bytes("null"),
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
        print(&Node::Boolean(true), &mut dest, 0);
        assert_eq!(dest.to_string(), "true");
    }

    #[test]
    fn test_print_numeric() {
        let mut dest = BufferDestination::new();
        print(&Node::Number(Numeric::Integer(42)), &mut dest, 0);
        assert_eq!(dest.to_string(), "42");
    }

    #[test]
    fn test_print_string() {
        let mut dest = BufferDestination::new();
        print(&Node::Str("hello".to_string()), &mut dest, 0);
        assert_eq!(dest.to_string(), "\"hello\"");
    }

    #[test]
    fn test_print_array() {
        let mut dest = BufferDestination::new();
        print(&Node::Array(vec![Node::Boolean(true), Node::Number(Numeric::Integer(1))]), &mut dest, 0);
        assert_eq!(dest.to_string(), "[\n  true,\n  1\n]");
    }

    #[test]
    fn test_print_object() {
        let mut dest = BufferDestination::new();
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        let hashmap: std::collections::HashMap<String, Node> = map.into_iter().collect();
        print(&Node::Object(hashmap), &mut dest, 0);
        assert_eq!(dest.to_string(), "{\n  \"key\": \"value\"\n}");
    }

    #[test]
    fn test_print_null() {
        let mut dest = BufferDestination::new();
        print(&Node::None, &mut dest, 0);
        assert_eq!(dest.to_string(), "null");
    }
}