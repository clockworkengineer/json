use crate::json_lib::nodes::node::*;
use crate::json_lib::io::traits::IDestination;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    stringify_with_indent(node, destination, 0);
}

fn stringify_with_indent(node: &Node, destination: &mut dyn IDestination, indent: usize) {
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
            #[allow(unreachable_patterns)]
            _ => destination.add_bytes(&format!("{:?}", value)),
        },
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
        dest.clear();
        stringify(&Node::Boolean(false), &mut dest);
        assert_eq!(dest.to_string(), "false");
    }

    #[test]
    fn test_stringify_numbers() {
        let mut dest = Buffer::new();
        let test_cases = vec![
            (Node::Number(Number::Integer(-42)), "-42"),
            (Node::Number(Number::UInteger(42)), "42"),
            (Node::Number(Number::Float(42.5)), "42.5"),
            (Node::Number(Number::Byte(255)), "255"),
            (Node::Number(Number::Int32(-2147483648)), "-2147483648"),
            (Node::Number(Number::UInt32(4294967295)), "4294967295"),
        ];
        for (node, expected) in test_cases {
            dest.clear();
            stringify(&node, &mut dest);
            assert_eq!(dest.to_string(), expected);
        }
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("simple".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "simple");

        dest.clear();
        stringify(&Node::Str("multi\nline".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "|\n  multi\n  line\n");

        dest.clear();
        stringify(&Node::Str("with \"quotes\"".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "|\n  with \"quotes\"\n");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![]), &mut dest);
        assert_eq!(dest.to_string(), "[]");

        dest.clear();
        stringify(&Node::Array(vec![
            Node::Number(Number::Integer(1)),
            Node::Str("test".to_string()),
        ]), &mut dest);
        assert_eq!(dest.to_string(), "\n- 1\n- test\n");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        stringify(&Node::Object(HashMap::new()), &mut dest);
        assert_eq!(dest.to_string(), "{}");

        dest.clear();
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Object(map), &mut dest);
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
        stringify(&Node::Object(outer_map), &mut dest);
        assert_eq!(dest.to_string(), "\ntest: \n  nested: \n    - true\n    - |\n        multi\n        line\n\n\n\n");
    }
}
