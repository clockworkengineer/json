//! Optimized JSON stringifier with lazy string escaping
//!
//! This module provides optimized JSON serialization with several performance improvements:
//! - Lazy string escaping: only escape when necessary
//! - Batch writing: write large chunks of unescaped data at once
//! - Fast paths for common cases (simple strings, small integers)

use crate::io::traits::IDestination;
use crate::nodes::node::*;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

/// Check if a string needs escaping
/// Returns true if the string contains only safe ASCII characters
#[inline]
fn needs_escaping(s: &str) -> bool {
    let bytes = s.as_bytes();
    for &b in bytes {
        match b {
            b'"' | b'\\' | b'\n' | b'\r' | b'\t' => return true,
            b if b < 32 => return true,
            _ => {}
        }
    }
    false
}

/// Write a string without escaping (fast path)
#[inline]
fn write_simple_string(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");
    destination.add_bytes(s);
    destination.add_bytes("\"");
}

/// Write an escaped JSON string with optimized batching
#[inline]
fn write_escaped_string(s: &str, destination: &mut dyn IDestination) {
    // Fast path: if no escaping needed, write directly
    if !needs_escaping(s) {
        write_simple_string(s, destination);
        return;
    }

    // Slow path: escape as needed
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
            // Write accumulated unescaped bytes in one call
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
                b => destination.add_bytes(&format!("\\u{:04x}", b as u32)),
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

/// Optimized stringify with lazy escaping
pub fn stringify_optimized(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
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
            destination.add_bytes("[");
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    destination.add_bytes(",");
                }
                stringify_optimized(item, destination)?;
            }
            destination.add_bytes("]");
        }
        Node::Object(entries) => {
            destination.add_bytes("{");
            for (index, (key, value)) in entries.iter().enumerate() {
                if index > 0 {
                    destination.add_bytes(",");
                }
                write_escaped_string(key, destination);
                destination.add_bytes(":");
                stringify_optimized(value, destination)?;
            }
            destination.add_bytes("}");
        }
    }
    Ok(())
}

/// Performance statistics for stringify operations
#[cfg(feature = "alloc")]
pub struct StringifyStats {
    pub simple_strings: usize,
    pub escaped_strings: usize,
    pub total_bytes: usize,
}

#[cfg(feature = "alloc")]
impl StringifyStats {
    pub fn new() -> Self {
        Self {
            simple_strings: 0,
            escaped_strings: 0,
            total_bytes: 0,
        }
    }

    pub fn escape_percentage(&self) -> f64 {
        let total = self.simple_strings + self.escaped_strings;
        if total == 0 {
            0.0
        } else {
            (self.escaped_strings as f64 / total as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;

    #[test]
    fn test_needs_escaping() {
        assert!(!needs_escaping("hello"));
        assert!(!needs_escaping("hello world"));
        assert!(needs_escaping("hello\"world"));
        assert!(needs_escaping("hello\\world"));
        assert!(needs_escaping("hello\nworld"));
    }

    #[test]
    fn test_stringify_simple_string() {
        let mut dest = Buffer::new();
        let node = Node::Str("hello".to_string());
        stringify_optimized(&node, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\"hello\"");
    }

    #[test]
    fn test_stringify_escaped_string() {
        let mut dest = Buffer::new();
        let node = Node::Str("hello\nworld".to_string());
        stringify_optimized(&node, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "\"hello\\nworld\"");
    }

    #[test]
    fn test_stringify_number() {
        let mut dest = Buffer::new();
        let node = Node::Number(Numeric::Int32(42));
        stringify_optimized(&node, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "42");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_stringify_object() {
        let mut dest = Buffer::new();
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), Node::Str("Alice".to_string()));
        let node = Node::Object(map);
        stringify_optimized(&node, &mut dest).unwrap();
        assert_eq!(dest.to_string(), "{\"name\":\"Alice\"}");
    }
}
