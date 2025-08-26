

use crate::json_lib::nodes::node::*;
use crate::json_lib::io::traits::IDestination;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::None => destination.add_bytes("null"),
        Node::Boolean(value) => destination.add_bytes(if *value { "true" } else { "false" }),
        Node::Number(value) => match value {
            Number::Integer(n) => destination.add_bytes(&n.to_string()),
            Number::UInteger(n) => destination.add_bytes(&n.to_string()),
            Number::Float(f) => destination.add_bytes(&f.to_string()),
            Number::Byte(b) => destination.add_bytes(&b.to_string()),
            Number::Int32(i) => destination.add_bytes(&i.to_string()),
            Number::UInt32(u) => destination.add_bytes(&u.to_string()),
            // If there are any other variants, add them here
            #[allow(unreachable_patterns)]
            _ => destination.add_bytes(&format!("{:?}", value)),
        },
        Node::Str(value) => {
            destination.add_bytes("\"");
            for c in value.chars() {
                match c {
                    '"' => destination.add_bytes("\\\""),
                    '\\' => destination.add_bytes("\\\\"),
                    '\n' => destination.add_bytes("\\n"),
                    '\r' => destination.add_bytes("\\r"),
                    '\t' => destination.add_bytes("\\t"),
                    c if c.is_control() => destination.add_bytes(&format!("\\u{:04x}", c as u32)),
                    c => destination.add_bytes(&c.to_string()),
                }
            }
            destination.add_bytes("\"");
        }
        Node::Array(items) => {
            destination.add_bytes("[");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    destination.add_bytes(",");
                }
                stringify(item, destination);
            }
            destination.add_bytes("]");
        }
        Node::Object(entries) => {
            destination.add_bytes("{");
            for (index, (key, value)) in entries.iter().enumerate() {
                if index > 0 {
                    destination.add_bytes(",");
                }
                stringify(&Node::Str(key.clone()), destination);
                destination.add_bytes(":");
                stringify(value, destination);
            }
            destination.add_bytes("}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::json_lib::io::destinations::buffer::Buffer;
    
    #[test]
    fn test_stringify_null() {
        let mut dest = Buffer::new();
        stringify(&Node::None, &mut dest);
        assert_eq!(dest.to_string(), "null");
    }

    #[test]
    fn test_stringify_boolean() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(true), &mut dest);
        assert_eq!(dest.to_string(), "true");
    }

    #[test]
    fn test_stringify_number() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Float(42.5)), &mut dest);
        assert_eq!(dest.to_string(), "42.5");
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("Hello\n\"World\"".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "\"Hello\\n\\\"World\\\"\"");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Number(Number::Float(1.0)),
            Node::Str("test".to_string()),
        ]), &mut dest);
        assert_eq!(dest.to_string(), "[1,\"test\"]");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Object(map), &mut dest);
        assert_eq!(dest.to_string(), "{\"key\":\"value\"}");
    }
}