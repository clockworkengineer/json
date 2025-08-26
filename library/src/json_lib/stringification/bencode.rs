use crate::json_lib::nodes::node::*;
use crate::json_lib::io::traits::IDestination;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::None => destination.add_bytes(""),
        Node::Boolean(value) => destination.add_bytes(if *value { "i1e" } else { "i0e" }),
        Node::Number(value) => match value {
            Number::Integer(n) => destination.add_bytes(&format!("i{}e", n)),
            Number::UInteger(n) => destination.add_bytes(&format!("i{}e", n)),
            Number::Float(f) => destination.add_bytes(&format!("i{}e", f.round() as i64)),
            Number::Byte(b) => destination.add_bytes(&format!("i{}e", b)),
            Number::Int32(i) => destination.add_bytes(&format!("i{}e", i)),
            Number::UInt32(u) => destination.add_bytes(&format!("i{}e", u)),
            #[allow(unreachable_patterns)]
            _ => destination.add_bytes(&format!("i{:?}e", value)),
        },
        Node::Str(value) => {
            destination.add_bytes(&format!("{}:", value.len()));
            destination.add_bytes(value);
        },
        Node::Array(items) => {
            destination.add_bytes("l");
            for item in items {
                stringify(item, destination);
            }
            destination.add_bytes("e");
        },
        Node::Object(entries) => {
            destination.add_bytes("d");
            let mut sorted_entries: Vec<_> = entries.iter().collect();
            sorted_entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
            for (key, value) in sorted_entries {
                stringify(&Node::Str(key.clone()), destination);
                stringify(value, destination);
            }
            destination.add_bytes("e");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::json_lib::io::destinations::buffer::Buffer;

    #[test]
    fn test_stringify_none() {
        let mut dest = Buffer::new();
        stringify(&Node::None, &mut dest);
        assert_eq!(dest.to_string(), "");
    }

    #[test]
    fn test_stringify_boolean() {
        let mut dest = Buffer::new();
        stringify(&Node::Boolean(true), &mut dest);
        assert_eq!(dest.to_string(), "i1e");

        let mut dest = Buffer::new();
        stringify(&Node::Boolean(false), &mut dest);
        assert_eq!(dest.to_string(), "i0e");
    }

    #[test]
    fn test_stringify_numbers() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Integer(-42)), &mut dest);
        assert_eq!(dest.to_string(), "i-42e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::UInteger(42)), &mut dest);
        assert_eq!(dest.to_string(), "i42e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Float(42.7)), &mut dest);
        assert_eq!(dest.to_string(), "i43e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Byte(255)), &mut dest);
        assert_eq!(dest.to_string(), "i255e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::Int32(-2147483648)), &mut dest);
        assert_eq!(dest.to_string(), "i-2147483648e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Number::UInt32(4294967295)), &mut dest);
        assert_eq!(dest.to_string(), "i4294967295e");
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("test".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "4:test");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![
            Node::Number(Number::Integer(1)),
            Node::Str("test".to_string()),
        ]), &mut dest);
        assert_eq!(dest.to_string(), "li1e4:teste");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("b".to_string(), Node::Str("value1".to_string()));
        map.insert("a".to_string(), Node::Str("value2".to_string()));
        stringify(&Node::Object(map), &mut dest);
        assert_eq!(dest.to_string(), "d1:a6:value21:b6:value1e");
    }
}
