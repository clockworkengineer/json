//! JSON parser implementation that converts JSON text into Node structures
//! Provides functions for parsing different JSON data types including objects,
//! arrays, strings, numbers, boolean and null values.

use crate::error::messages::*;
use crate::io::traits::ISource;
use crate::nodes::node::Node;
use crate::nodes::node::Numeric;
use crate::parser::config::ParserConfig;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

// Use smallvec for small arrays to reduce heap allocations
use smallvec::SmallVec;

/// Constants used for JSON parsing
/// These define the special characters and starting characters
/// used to identify different JSON elements like objects, arrays,
/// strings, numbers and literals
const OBJECT_START: char = '{';
const OBJECT_END: char = '}';
const ARRAY_START: char = '[';
const ARRAY_END: char = ']';
const QUOTE: char = '"';
const COMMA: char = ',';
const COLON: char = ':';
const BACKSLASH: char = '\\';
const TRUE_START: char = 't';
const FALSE_START: char = 'f';
const NULL_START: char = 'n';
const MINUS: char = '-';
const DECIMAL_POINT: char = '.';
const EXPONENT_LOWER: char = 'e';
const EXPONENT_UPPER: char = 'E';
const PLUS: char = '+';

/// Parses JSON input from a source and returns a Node representation
///
/// # Arguments
/// * `source` - Source implementing ISource trait that provides JSON input
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message if parsing fails
pub fn parse(source: &mut dyn ISource) -> Result<Node, String> {
    parse_with_config(source, &ParserConfig::unlimited())
}

/// Convenience function to parse JSON from a string slice
///
/// # Arguments
/// * `s` - JSON string to parse
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
///
/// # Examples
/// ```
/// use json_lib::parser::default::from_str;
///
/// let node = from_str(r#"{"name": "Alice", "age": 30}"#).unwrap();
/// assert!(node.is_object());
/// ```
pub fn from_str(s: &str) -> Result<Node, String> {
    use crate::io::sources::buffer::Buffer as BufferSource;
    let mut source = BufferSource::new(s.as_bytes());
    parse(&mut source)
}

/// Convenience function to parse JSON from a byte slice
///
/// # Arguments
/// * `bytes` - JSON bytes to parse
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
///
/// # Examples
/// ```
/// use json_lib::parser::default::from_bytes;
///
/// let bytes = br#"{"name": "Alice"}"#;
/// let node = from_bytes(bytes).unwrap();
/// assert!(node.is_object());
/// ```
pub fn from_bytes(bytes: &[u8]) -> Result<Node, String> {
    use crate::io::sources::buffer::Buffer as BufferSource;
    let mut source = BufferSource::new(bytes);
    parse(&mut source)
}

/// Parses JSON input with custom configuration for resource limits
///
/// # Arguments
/// * `source` - Source implementing ISource trait that provides JSON input
/// * `config` - Parser configuration with resource limits
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message if parsing fails
pub fn parse_with_config(source: &mut dyn ISource, config: &ParserConfig) -> Result<Node, String> {
    parse_value(source, config, 0)
}

/// Internal parsing function with depth tracking
fn parse_value(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: usize,
) -> Result<Node, String> {
    // Check depth limit
    if let Some(max_depth) = config.max_depth {
        if depth >= max_depth {
            use arrayvec::ArrayString;
            use core::fmt::Write;
            let mut msg: ArrayString<64> = ArrayString::new();
            let _ = write!(&mut msg, "Maximum nesting depth of {} exceeded", max_depth);
            return Err(msg.to_string());
        }
    }

    skip_whitespace(source);

    match source.current() {
        Some(OBJECT_START) => parse_object_with_config(source, config, depth),
        Some(ARRAY_START) => parse_array_with_config(source, config, depth),
        Some(QUOTE) => parse_string_with_config(source, config),
        Some(TRUE_START) => parse_true(source),
        Some(FALSE_START) => parse_false(source),
        Some(NULL_START) => parse_null(source),
        Some(c) if c.is_digit(10) || c == MINUS => parse_number(source),
        Some(c) => Err(format!("{}{}", ERR_UNEXPECTED_CHAR, c)),
        None => Err(ERR_EMPTY_INPUT.to_string()),
    }
}

/// Advances the source past any whitespace characters
///
/// # Arguments
/// * `source` - Source to read characters from
#[inline]
fn skip_whitespace(source: &mut dyn ISource) {
    while let Some(c) = source.current() {
        if !c.is_whitespace() {
            break;
        }
        source.next();
    }
}

/// Parses a JSON object starting with '{' and returns Node::Object
/// Handles nested key-value pairs separated by commas
///
/// # Arguments
/// * `source` - Source to read characters from
///
/// # Returns
/// * `Result<Node, String>` - Object Node or error message
#[allow(dead_code)]
fn parse_object(source: &mut dyn ISource) -> Result<Node, String> {
    parse_object_with_config(source, &ParserConfig::unlimited(), 0)
}

/// Parses a JSON object with configuration and depth tracking
fn parse_object_with_config(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: usize,
) -> Result<Node, String> {
    // Use SmallVec for small objects to reduce heap allocations
    let mut pairs: SmallVec<[(String, Node); 8]> = SmallVec::new();
    source.next(); // Skip '{'

    skip_whitespace(source);

    if let Some(OBJECT_END) = source.current() {
        source.next();
        // Build HashMap from pairs (empty)
        return Ok(Node::Object(HashMap::new()));
    }

    loop {
        // Check object size limit
        if let Some(max_size) = config.max_object_size {
            if pairs.len() >= max_size {
                use arrayvec::ArrayString;
                use core::fmt::Write;
                let mut msg: ArrayString<64> = ArrayString::new();
                let _ = write!(&mut msg, "Maximum object size of {} exceeded", max_size);
                return Err(msg.to_string());
            }
        }

        skip_whitespace(source);

        // Parse key
        let key = match parse_string_with_config(source, config)? {
            Node::Str(s) => s,
            _ => return Err(ERR_OBJECT_KEY.to_string()),
        };

        skip_whitespace(source);

        // Check for colon
        match source.current() {
            Some(COLON) => source.next(),
            _ => return Err(ERR_EXPECT_COLON.to_string()),
        }

        skip_whitespace(source);

        // Parse value with incremented depth
        let value = parse_value(source, config, depth + 1)?;
        pairs.push((key, value));

        skip_whitespace(source);

        match source.current() {
            Some(COMMA) => {
                source.next();
                continue;
            }
            Some(OBJECT_END) => {
                source.next();
                break;
            }
            _ => return Err(ERR_EXPECT_OBJECT_END.to_string()),
        }
    }

    // Build HashMap from pairs
    // Use a higher initial capacity to reduce rehashing (Rust's default load factor is 0.75)
    let mut map = HashMap::with_capacity((pairs.len() * 4) / 3 + 1);
    for (k, v) in pairs {
        map.insert(k, v);
    }
    Ok(Node::Object(map))
}

/// Parses a JSON array starting with '[' and returns Node::Array
/// Handles comma-separated values of any valid JSON type
///
/// # Arguments
/// * `source` - Source to read characters from
///
/// # Returns
/// * `Result<Node, String>` - Array Node or error message
#[allow(dead_code)]
fn parse_array(source: &mut dyn ISource) -> Result<Node, String> {
    parse_array_with_config(source, &ParserConfig::unlimited(), 0)
}

/// Parses a JSON array with configuration and depth tracking
fn parse_array_with_config(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: usize,
) -> Result<Node, String> {
    // Use SmallVec for small arrays to reduce heap allocations
    let mut vec: SmallVec<[Node; 8]> = SmallVec::new();
    source.next(); // Skip '['

    skip_whitespace(source);

    if let Some(ARRAY_END) = source.current() {
        source.next();
        return Ok(Node::Array(vec.into_vec()));
    }

    loop {
        // Check array size limit
        if let Some(max_size) = config.max_array_size {
            if vec.len() >= max_size {
                use arrayvec::ArrayString;
                use core::fmt::Write;
                let mut msg: ArrayString<64> = ArrayString::new();
                let _ = write!(&mut msg, "Maximum array size of {} exceeded", max_size);
                return Err(msg.to_string());
            }
        }

        // Parse value with incremented depth
        vec.push(parse_value(source, config, depth + 1)?);

        skip_whitespace(source);

        match source.current() {
            Some(COMMA) => {
                source.next();
                continue;
            }
            Some(ARRAY_END) => {
                source.next();
                break;
            }
            _ => return Err(ERR_EXPECT_ARRAY_END.to_string()),
        }
    }

    // Convert SmallVec to Vec for Node::Array
    Ok(Node::Array(vec.into_vec()))
}

/// Parses a JSON string with support for escape sequences
/// Handles standard escapes and Unicode escape sequences
///
/// # Arguments
/// * `source` - Source to read characters from
///
/// # Returns
/// * `Result<Node, String>` - String Node or error message
#[allow(dead_code)]
fn parse_string(source: &mut dyn ISource) -> Result<Node, String> {
    parse_string_with_config(source, &ParserConfig::unlimited())
}

/// Parses a JSON string with configuration for length limits
fn parse_string_with_config(
    source: &mut dyn ISource,
    config: &ParserConfig,
) -> Result<Node, String> {
    // Use SmallVec for short strings to reduce heap allocations
    use smallvec::SmallVec;
    let capacity = config.max_string_length.unwrap_or(64).min(32);
    let mut buf: SmallVec<[u8; 32]> = SmallVec::with_capacity(capacity);
    source.next(); // Skip opening quote

    while let Some(c) = source.current() {
        // Check string length limit
        if let Some(max_len) = config.max_string_length {
            if buf.len() >= max_len {
                use arrayvec::ArrayString;
                use core::fmt::Write;
                let mut msg: ArrayString<64> = ArrayString::new();
                let _ = write!(&mut msg, "Maximum string length of {} bytes exceeded", max_len);
                return Err(msg.to_string());
            }
        }

        match c {
            QUOTE => {
                source.next();
                // Convert SmallVec<u8> to String
                return Ok(Node::Str(unsafe { String::from_utf8_unchecked(buf.into_vec()) }));
            }
            BACKSLASH => {
                source.next();
                match source.current() {
                    Some('"') => buf.push(b'"'),
                    Some('\\') => buf.push(b'\\'),
                    Some('/') => buf.push(b'/'),
                    Some('b') => buf.push(b'\x08'),
                    Some('f') => buf.push(b'\x0c'),
                    Some('n') => buf.push(b'\n'),
                    Some('r') => buf.push(b'\r'),
                    Some('t') => buf.push(b'\t'),
                    Some('u') => {
                        source.next();
                        // Use SmallVec for hex buffer
                        let mut hex: SmallVec<[u8; 4]> = SmallVec::with_capacity(4);
                        for _ in 0..4 {
                            match source.current() {
                                Some(d) if d.is_ascii_hexdigit() => {
                                    hex.push(d as u8);
                                    source.next();
                                }
                                _ => return Err(ERR_INVALID_ESCAPE.to_string()),
                            }
                        }
                        if let Ok(hex_str) = core::str::from_utf8(&hex) {
                            if let Ok(code) = u32::from_str_radix(hex_str, 16) {
                                if let Some(ch) = char::from_u32(code) {
                                    let mut utf8_buf = [0u8; 4];
                                    let encoded = ch.encode_utf8(&mut utf8_buf);
                                    buf.extend_from_slice(encoded.as_bytes());
                                } else {
                                    return Err(ERR_INVALID_ESCAPE.to_string());
                                }
                            } else {
                                return Err(ERR_INVALID_ESCAPE.to_string());
                            }
                        } else {
                            return Err(ERR_INVALID_ESCAPE.to_string());
                        }
                        continue;
                    }
                    _ => return Err(ERR_INVALID_ESCAPE.to_string()),
                }
                source.next();
            }
            _ => {
                // Push UTF-8 bytes for char
                let mut utf8_buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut utf8_buf);
                buf.extend_from_slice(encoded.as_bytes());
                source.next();
            }
        }
    }

    Err(ERR_UNTERMINATED_STRING.to_string())
}

/// Parses JSON numbers including integers, floats and scientific notation
/// Supports negative numbers and exponential notation
///
/// # Arguments
/// * `source` - Source to read characters from
///
/// # Returns
/// * `Result<Node, String>` - Number Node or error message
fn parse_number(source: &mut dyn ISource) -> Result<Node, String> {
    let mut num_str = String::new();
    let mut is_float = false;

    // Handle negative numbers
    if source.current() == Some(MINUS) {
        num_str.push(MINUS);
        source.next();
    }

    while let Some(c) = source.current() {
        match c {
            '0'..='9' => {
                num_str.push(c);
                source.next();
            }
            DECIMAL_POINT => {
                if is_float {
                    return Err(ERR_MULTIPLE_DECIMAL.to_string());
                }
                is_float = true;
                num_str.push(c);
                source.next();
            }
            EXPONENT_LOWER | EXPONENT_UPPER => {
                is_float = true;
                num_str.push(c);
                source.next();

                if let Some(sign) = source.current() {
                    if sign == PLUS || sign == MINUS {
                        num_str.push(sign);
                        source.next();
                    }
                }
            }
            _ => break,
        }
    }

    if is_float {
        match num_str.parse::<f64>() {
            Ok(n) => Ok(Node::Number(Numeric::Float(n))),
            Err(_) => Err(ERR_INVALID_FLOAT.to_string()),
        }
    } else {
        match num_str.parse::<i64>() {
            Ok(n) => Ok(Node::Number(Numeric::Integer(n))),
            Err(_) => Err(ERR_INVALID_INTEGER.to_string()),
        }
    }
}

fn parse_true(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 't'
    for c in ['r', 'u', 'e'] {
        if source.current() != Some(c) {
            return Err(ERR_EXPECT_TRUE.to_string());
        }
        source.next();
    }
    Ok(Node::Boolean(true))
}

fn parse_false(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 'f'
    for c in ['a', 'l', 's', 'e'] {
        if source.current() != Some(c) {
            return Err(ERR_EXPECT_FALSE.to_string());
        }
        source.next();
    }
    Ok(Node::Boolean(false))
}

fn parse_null(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 'n'
    for c in ['u', 'l', 'l'] {
        if source.current() != Some(c) {
            return Err(ERR_EXPECT_NULL.to_string());
        }
        source.next();
    }
    Ok(Node::None)
}

#[cfg(test)]
/// Test module for JSON parser functionality
/// Includes tests for all JSON data types and error conditions
mod tests {
    use super::*;
    use crate::io::sources::buffer::Buffer;

    #[test]
    fn test_parse_null() {
        let mut source = Buffer::new(b"null");
        assert!(matches!(parse(&mut source), Ok(Node::None)));
    }

    #[test]
    fn test_parse_true() {
        let mut source = Buffer::new(b"true");
        assert!(matches!(parse(&mut source), Ok(Node::Boolean(true))));
    }

    #[test]
    fn test_parse_false() {
        let mut source = Buffer::new(b"false");
        assert!(matches!(parse(&mut source), Ok(Node::Boolean(false))));
    }

    #[test]
    fn test_parse_string() {
        let mut source = Buffer::new(b"\"hello\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "hello"));
    }

    #[test]
    fn test_parse_escaped_string() {
        let mut source = Buffer::new(b"\"hello\\\"world\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "hello\"world"));
    }

    #[test]
    fn test_parse_number_integer() {
        let mut source = Buffer::new(b"123");
        assert!(matches!(
            parse(&mut source),
            Ok(Node::Number(Numeric::Integer(123)))
        ));
    }

    #[test]
    fn test_parse_number_float() {
        let mut source = Buffer::new(b"123.45");
        assert!(
            matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 123.45).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn test_parse_array() {
        let mut source = Buffer::new(b"[1,2,3]");
        match parse(&mut source) {
            Ok(Node::Array(arr)) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_object() {
        let mut source = Buffer::new(b"{\"key\":\"value\"}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => assert_eq!(obj.len(), 1),
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_parse_empty_array() {
        let mut source = Buffer::new(b"[]");
        match parse(&mut source) {
            Ok(Node::Array(arr)) => assert_eq!(arr.len(), 0),
            _ => panic!("Expected empty array"),
        }
    }

    #[test]
    fn test_parse_empty_object() {
        let mut source = Buffer::new(b"{}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => assert_eq!(obj.len(), 0),
            _ => panic!("Expected empty object"),
        }
    }

    #[test]
    fn test_invalid_number() {
        let mut source = Buffer::new(b"12.34.56");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_invalid_escape() {
        let mut source = Buffer::new(b"\"\\x\"");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_unterminated_string() {
        let mut source = Buffer::new(b"\"unterminated");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_whitespace() {
        let mut source = Buffer::new(b" \t\n\r{} ");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => assert_eq!(obj.len(), 0),
            _ => panic!("Expected empty object"),
        }
    }

    #[test]
    fn test_unicode_escape() {
        let mut source = Buffer::new(b"\"\\u0048\\u0065\\u006c\\u006c\\u006f\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "Hello"));
    }

    #[test]
    fn test_mixed_unicode_escape() {
        let mut source = Buffer::new(b"\"Hello, \\u0057\\u006f\\u0072\\u006c\\u0064!\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "Hello, World!"));
    }

    #[test]
    fn test_invalid_unicode_escape() {
        let mut source = Buffer::new(b"\"\\u00\"");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_invalid_unicode_hex() {
        let mut source = Buffer::new(b"\"\\u00zz\"");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_parse_negative_number() {
        let mut source = Buffer::new(b"-123");
        assert!(matches!(
            parse(&mut source),
            Ok(Node::Number(Numeric::Integer(-123)))
        ));

        let mut source = Buffer::new(b"-123.45");
        assert!(
            matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - -123.45).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn test_parse_scientific_notation() {
        let mut source = Buffer::new(b"1.23e+2");
        assert!(
            matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 123.0).abs() < f64::EPSILON)
        );

        let mut source = Buffer::new(b"1.23E-2");
        assert!(
            matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 0.0123).abs() < f64::EPSILON)
        );
    }

    #[test]
    fn test_parse_complex_object() {
        let mut source =
            Buffer::new(b"{\"array\":[1,{\"nested\":true},null],\"string\":\"value\"}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => {
                assert_eq!(obj.len(), 2);
                assert!(obj.contains_key("array"));
                assert!(obj.contains_key("string"));
            }
            _ => panic!("Expected complex object"),
        }
    }

    #[test]
    fn test_invalid_syntax() {
        let mut source = Buffer::new(b"{\"key\": value}");
        assert!(parse(&mut source).is_err());

        let mut source = Buffer::new(b"[1,2,]");
        assert!(parse(&mut source).is_err());

        let mut source = Buffer::new(b"{,}");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_string_escapes() {
        let mut source = Buffer::new(b"\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "\"\\/\x08\x0c\n\r\t"));
    }
    #[test]
    fn test_string_unicode_escapes() {
        let mut source = Buffer::new(b"\"\\u0048\\u0065\\u006c\\u006c\\u006f\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "Hello"));
    }
    #[test]
    fn test_string_unicode_escapes_mixed() {
        let mut source = Buffer::new(b"\"Hello, \\u0057\\u006f\\u0072\\u006c\\u0064!\"");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "Hello, World!"));
    }
    #[test]
    fn test_string_unicode_escapes_invalid() {
        let mut source = Buffer::new(b"\"\\u00\"");
        assert!(parse(&mut source).is_err());
    }
    #[test]
    fn test_string_unicode_escapes_invalid_hex() {
        let mut source = Buffer::new(b"\"\\u00zz\"");
        assert!(parse(&mut source).is_err());
    }

    // Configuration tests
    #[test]
    fn test_max_depth_exceeded() {
        let config = ParserConfig::new().with_max_depth(Some(2));
        let mut source = Buffer::new(b"{\"a\":{\"b\":{\"c\":1}}}");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("depth"));
    }

    #[test]
    fn test_max_depth_within_limit() {
        let config = ParserConfig::new().with_max_depth(Some(4));
        let mut source = Buffer::new(b"{\"a\":{\"b\":{\"c\":1}}}");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_string_length_exceeded() {
        let config = ParserConfig::new().with_max_string_length(Some(5));
        let mut source = Buffer::new(b"\"verylongstring\"");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("string length"));
    }

    #[test]
    fn test_max_string_length_within_limit() {
        let config = ParserConfig::new().with_max_string_length(Some(10));
        let mut source = Buffer::new(b"\"short\"");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_array_size_exceeded() {
        let config = ParserConfig::new().with_max_array_size(Some(3));
        let mut source = Buffer::new(b"[1,2,3,4,5]");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("array size"));
    }

    #[test]
    fn test_max_array_size_within_limit() {
        let config = ParserConfig::new().with_max_array_size(Some(5));
        let mut source = Buffer::new(b"[1,2,3]");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_object_size_exceeded() {
        let config = ParserConfig::new().with_max_object_size(Some(2));
        let mut source = Buffer::new(b"{\"a\":1,\"b\":2,\"c\":3}");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("object size"));
    }

    #[test]
    fn test_max_object_size_within_limit() {
        let config = ParserConfig::new().with_max_object_size(Some(5));
        let mut source = Buffer::new(b"{\"a\":1,\"b\":2}");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_strict_config() {
        let config = ParserConfig::strict();
        // Should fail with deeply nested structure
        let mut source = Buffer::new(b"[[[[[[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]]]]]]");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_unlimited_config() {
        let config = ParserConfig::unlimited();
        // Should succeed even with deeply nested structure
        let mut source = Buffer::new(b"[[[[[[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]]]]]]");
        let result = parse_with_config(&mut source, &config);
        assert!(result.is_ok());
    }
}
