//! Implementation of bencode serialization format.
//! Bencode is the encoding used by the peer-to-peer file sharing system BitTorrent
//! for storing and transmitting loosely structured data.

use crate::nodes::node::*;
use crate::io::traits::IDestination;

/// Converts a Node into its bencode string representation and writes it to the destination.
///
/// # Arguments
/// * `node` - The Node to serialize
/// * `destination` - The output destination implementing IDestination trait
///
/// The bencode format specifies the following encoding rules:
/// - Strings are encoded as <length>: <contents>
/// - Integers are encoded as i<number>e
/// - Lists are encoded as l<bencoded elements>e
/// - Dictionaries are encoded as d<bencoded strings> <bencoded elements>e
pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::None => destination.add_bytes(""),
        Node::Boolean(value) => destination.add_bytes(if *value { "i1e" } else { "i0e" }),
        Node::Number(value) => match value {
            Numeric::Integer(n) => destination.add_bytes(&format!("i{}e", n)),
            Numeric::UInteger(n) => destination.add_bytes(&format!("i{}e", n)),
            Numeric::Float(f) => destination.add_bytes(&format!("i{}e", f.round() as i64)),
            Numeric::Byte(b) => destination.add_bytes(&format!("i{}e", b)),
            Numeric::Int32(i) => destination.add_bytes(&format!("i{}e", i)),
            Numeric::UInt32(u) => destination.add_bytes(&format!("i{}e", u)),
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
/// Test module for verifying bencode serialization functionality.
/// Contains tests for all Node types and their bencode representations.
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::io::destinations::buffer::Buffer;

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
        stringify(&Node::Number(Numeric::Integer(-42)), &mut dest);
        assert_eq!(dest.to_string(), "i-42e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::UInteger(42)), &mut dest);
        assert_eq!(dest.to_string(), "i42e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::Float(42.7)), &mut dest);
        assert_eq!(dest.to_string(), "i43e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::Byte(255)), &mut dest);
        assert_eq!(dest.to_string(), "i255e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::Int32(-2147483648)), &mut dest);
        assert_eq!(dest.to_string(), "i-2147483648e");

        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::UInt32(4294967295)), &mut dest);
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
            Node::Number(Numeric::Integer(1)),
            Node::Str("test".to_string()),
        ]), &mut dest);
        assert_eq!(dest.to_string(), "li1e4:teste");

        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![]), &mut dest);
        assert_eq!(dest.to_string(), "le");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("b".to_string(), Node::Str("value1".to_string()));
        map.insert("a".to_string(), Node::Str("value2".to_string()));
        stringify(&Node::Object(map), &mut dest);
        assert_eq!(dest.to_string(), "d1:a6:value21:b6:value1e");

        let mut dest = Buffer::new();
        stringify(&Node::Object(HashMap::new()), &mut dest);
        assert_eq!(dest.to_string(), "de");
    }

    #[test]
    fn test_stringify_empty_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("".to_string()), &mut dest);
        assert_eq!(dest.to_string(), "0:");
    }

    #[test]
    fn test_stringify_nested_empty() {
        let mut dest = Buffer::new();
        let  inner_map = HashMap::new();
        let mut outer_map = HashMap::new();
        outer_map.insert("empty_object".to_string(), Node::Object(inner_map));
        outer_map.insert("empty_array".to_string(), Node::Array(vec![]));
        outer_map.insert("empty_string".to_string(), Node::Str("".to_string()));
        stringify(&Node::Object(outer_map), &mut dest);
        assert_eq!(dest.to_string(), "d11:empty_arrayle12:empty_objectde12:empty_string0:e");
    }
    #[test]
    fn test_stringify_nested_object() {
        let mut inner_map = HashMap::new();
        inner_map.insert("a".to_string(), Node::Str("value1".to_string()));
        inner_map.insert("b".to_string(), Node::Str("value2".to_string()));
        let mut outer_map = HashMap::new();
        outer_map.insert("inner".to_string(), Node::Object(inner_map));
    }

    
}
