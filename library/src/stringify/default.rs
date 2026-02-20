// Use itoa/dtoa for fast, allocation-free number formatting
/// JSON stringifier module.
///
/// Provides a function to serialize a `Node` (representing a JSON value) into a string
/// and write it to a destination implementing `IDestination`. Handles all JSON value types,
/// including strings (with proper escaping), numbers, booleans, arrays, objects, and nulls.
use crate::io::traits::IDestination;
use crate::nodes::node::*;
use dtoa;
use itoa;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

// Use smallvec for small arrays to reduce heap allocations
use smallvec::SmallVec;

/// Helper function to write an escaped JSON string directly to destination
/// Optimized to batch write unescaped characters
#[inline]
fn write_escaped_string(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");

    let bytes = s.as_bytes();
    let mut start = 0;
    let mut i = 0;

    while i < bytes.len() {
        let needs_escape = match bytes[i] {
            b'"' | b'\\' | b'\n' | b'\r' | b'\t' => true,
            b if b < 32 => true, // Control characters
            _ => false,
        };

        if needs_escape {
            // Write accumulated unescaped bytes
            if i > start {
                destination.add_bytes(core::str::from_utf8(&bytes[start..i]).unwrap());
            }

            // Write escape sequence
            match bytes[i] {
                b'"' => destination.add_bytes("\\\""),
                b'\\' => destination.add_bytes("\\\\"),
                b'\n' => destination.add_bytes("\\n"),
                b'\r' => destination.add_bytes("\\r"),
                b'\t' => destination.add_bytes("\\t"),
                b => {
                    // Manual formatting for \uXXXX
                    let b = b as u32;
                    let mut buf = [b'\\', b'u', b'0', b'0', b'0', b'0'];
                    // Write hex digits
                    for j in (2..6).rev() {
                        let digit = (b >> (4 * (5 - j))) & 0xF;
                        buf[j] = match digit {
                            0..=9 => b'0' + digit as u8,
                            10..=15 => b'a' + (digit as u8 - 10),
                            _ => b'?',
                        };
                    }
                    destination.add_bytes(core::str::from_utf8(&buf).unwrap());
                }
            }

            i += 1;
            start = i;
        } else {
            i += 1;
        }
    }

    // Write any remaining unescaped bytes
    if start < bytes.len() {
        destination.add_bytes(core::str::from_utf8(&bytes[start..]).unwrap());
    }

    destination.add_bytes("\"");
}

/// Serializes a `Node` into JSON and writes it to the given destination.
///
/// # Arguments
///
/// * `node` - The JSON node to serialize.
/// * `destination` - The destination to write the JSON string to.

pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::None => destination.add_bytes("null"),
        Node::Boolean(value) => destination.add_bytes(if *value { "true" } else { "false" }),
        Node::Number(value) => {
            let mut buf = itoa::Buffer::new();
            let mut fbuf = dtoa::Buffer::new();
            match value {
                Numeric::Integer(n) => destination.add_bytes(buf.format(*n)),
                Numeric::UInteger(n) => destination.add_bytes(buf.format(*n)),
                Numeric::Float(f) => destination.add_bytes(fbuf.format(*f)),
                Numeric::Byte(b) => destination.add_bytes(buf.format(*b)),
                Numeric::Int32(i) => destination.add_bytes(buf.format(*i)),
                Numeric::UInt32(u) => destination.add_bytes(buf.format(*u)),
                #[allow(unreachable_patterns)]
                _ => destination.add_bytes("null"), // fallback for unknown numeric type
            }
        }
        Node::Str(value) => write_escaped_string(value, destination),
        Node::Array(items) => {
            destination.add_bytes("[");
            // Use SmallVec for small arrays to reduce heap allocations
            let mut temp: SmallVec<[usize; 8]> = SmallVec::new();
            temp.extend(0..items.len());
            for (_index, item) in temp.iter().enumerate() {
                if *item > 0 {
                    destination.add_bytes(",");
                }
                stringify(&items[*item], destination)?;
            }
            destination.add_bytes("]");
        }
        Node::Object(entries) => {
            destination.add_bytes("{");
            // Use SmallVec for small objects to reduce heap allocations
            let mut temp: SmallVec<[usize; 8]> = SmallVec::new();
            temp.extend(0..entries.len());
            for (_index, idx) in temp.iter().enumerate() {
                if *idx > 0 {
                    destination.add_bytes(",");
                }
                let (key, value) = entries.iter().nth(*idx).unwrap();
                write_escaped_string(key, destination);
                destination.add_bytes(":");
                stringify(value, destination)?;
            }
            destination.add_bytes("}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;
    use std::collections::HashMap;

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
    }

    #[test]
    fn test_stringify_number() {
        let mut dest = Buffer::new();
        stringify(&Node::Number(Numeric::Float(42.5)), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "42.5");
    }

    #[test]
    fn test_stringify_string() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("Hello\n\"World\"".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\"Hello\\n\\\"World\\\"\"");
    }

    #[test]
    fn test_stringify_array() {
        let mut dest = Buffer::new();
        stringify(
            &Node::Array(vec![
                Node::Number(Numeric::Float(1.0)),
                Node::Str("test".to_string()),
            ]),
            &mut dest,
        )
        .unwrap();
        assert_eq!(dest.to_string(), "[1.0,\"test\"]");
    }

    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Object(map), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{\"key\":\"value\"}");
    }

    #[test]
    fn test_stringify_nested_objects() {
        let mut dest = Buffer::new();
        let mut inner_map = HashMap::new();
        inner_map.insert("inner_key".to_string(), Node::Number(Numeric::Integer(42)));
        let mut outer_map = HashMap::new();
        outer_map.insert("outer_key".to_string(), Node::Object(inner_map));
        stringify(&Node::Object(outer_map), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{\"outer_key\":{\"inner_key\":42}}");
    }

    #[test]
    fn test_stringify_mixed_array() {
        let mut dest = Buffer::new();
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), Node::Boolean(true));
        let array = Node::Array(vec![
            Node::Number(Numeric::Float(1.5)),
            Node::Array(vec![Node::Str("nested".to_string())]),
            Node::Object(obj),
            Node::None,
        ]);
        stringify(&array, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "[1.5,[\"nested\"],{\"key\":true},null]");
    }

    #[test]
    fn test_stringify_special_characters() {
        let mut dest = Buffer::new();
        let mut map = HashMap::new();
        map.insert(
            "special\t\n".to_string(),
            Node::Str("value\u{0001}".to_string()),
        );
        stringify(&Node::Object(map), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{\"special\\t\\n\":\"value\\u0001\"}");
    }

    #[test]
    fn test_stringify_number_formats() {
        let mut dest = Buffer::new();
        let array = Node::Array(vec![
            Node::Number(Numeric::Integer(-42)),
            Node::Number(Numeric::UInteger(42)),
            Node::Number(Numeric::Float(42.42)),
            Node::Number(Numeric::Byte(255)),
            Node::Number(Numeric::Int32(-2147483648)),
            Node::Number(Numeric::UInt32(4294967295)),
        ]);
        stringify(&array, &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "[-42,42,42.42,255,-2147483648,4294967295]"
        );
    }

    #[test]
    fn test_stringify_empty_array() {
        let mut dest = Buffer::new();
        stringify(&Node::Array(vec![]), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "[]");
    }

    #[test]
    fn test_stringify_empty_object() {
        let mut dest = Buffer::new();
        stringify(&Node::Object(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{}");
    }

    #[test]
    fn test_stringify_control_chars() {
        let mut dest = Buffer::new();
        stringify(&Node::Str("\u{0000}\u{001F}".to_string()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\"\\u0000\\u001f\"");
    }
}
