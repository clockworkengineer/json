//! JSON parser implementation that converts JSON text into Node structures
//! Provides functions for parsing different JSON data types including objects,
//! arrays, strings, numbers, booleans, and null values.

use crate::json_lib::nodes::node::Node;
use crate::json_lib::nodes::node::Numeric;
use std::collections::HashMap;
use crate::json_lib::io::traits::ISource;
use crate::json_lib::error::messages::*;

/// Parses JSON input from a source and returns a Node representation
///
/// # Arguments
/// * `source` - Source implementing ISource trait that provides JSON input
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message if parsing fails
pub fn parse(source: &mut dyn ISource) -> Result<Node, String> {
    skip_whitespace(source);

    match source.current() {
        Some('{') => parse_object(source),
        Some('[') => parse_array(source),
        Some('"') => parse_string(source),
        Some('t') => parse_true(source),
        Some('f') => parse_false(source),
        Some('n') => parse_null(source),
        Some(c) if c.is_digit(10) || c == '-' => parse_number(source),
        Some(c) => Err(format!("{}{}", ERR_UNEXPECTED_CHAR, c)),
        None => Err(ERR_EMPTY_INPUT.to_string())
    }
}

/// Advances the source past any whitespace characters
///
/// # Arguments
/// * `source` - Source to read characters from
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
fn parse_object(source: &mut dyn ISource) -> Result<Node, String> {
    let mut map = HashMap::new();
    source.next(); // Skip '{'

    skip_whitespace(source);

    if let Some('}') = source.current() {
        source.next();
        return Ok(Node::Object(map));
    }

    loop {
        skip_whitespace(source);

        // Parse key
        let key = match parse_string(source)? {
            Node::Str(s) => s,
            _ => return Err(ERR_OBJECT_KEY.to_string())
        };

        skip_whitespace(source);

        // Check for colon
        match source.current() {
            Some(':') => source.next(),
            _ => return Err(ERR_EXPECT_COLON.to_string())
        }

        skip_whitespace(source);

        // Parse value
        let value = parse(source)?;
        map.insert(key, value);

        skip_whitespace(source);

        match source.current() {
            Some(',') => {
                source.next();
                continue;
            }
            Some('}') => {
                source.next();
                break;
            }
            _ => return Err(ERR_EXPECT_OBJECT_END.to_string())
        }
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
fn parse_array(source: &mut dyn ISource) -> Result<Node, String> {
    let mut vec = Vec::new();
    source.next(); // Skip '['

    skip_whitespace(source);

    if let Some(']') = source.current() {
        source.next();
        return Ok(Node::Array(vec));
    }

    loop {
        vec.push(parse(source)?);

        skip_whitespace(source);

        match source.current() {
            Some(',') => {
                source.next();
                continue;
            }
            Some(']') => {
                source.next();
                break;
            }
            _ => return Err(ERR_EXPECT_ARRAY_END.to_string())
        }
    }

    Ok(Node::Array(vec))
}

/// Parses a JSON string with support for escape sequences
/// Handles standard escapes and unicode escape sequences
///
/// # Arguments
/// * `source` - Source to read characters from
///
/// # Returns
/// * `Result<Node, String>` - String Node or error message
fn parse_string(source: &mut dyn ISource) -> Result<Node, String> {
    let mut s = String::new();
    source.next(); // Skip opening quote

    while let Some(c) = source.current() {
        match c {
            '"' => {
                source.next();
                return Ok(Node::Str(s));
            }
            '\\' => {
                source.next();
                match source.current() {
                    Some('"') => s.push('"'),
                    Some('\\') => s.push('\\'),
                    Some('/') => s.push('/'),
                    Some('b') => s.push('\x08'),
                    Some('f') => s.push('\x0c'),
                    Some('n') => s.push('\n'),
                    Some('r') => s.push('\r'),
                    Some('t') => s.push('\t'),
                    Some('u') => {
                        source.next();
                        let mut hex = String::with_capacity(4);
                        for _ in 0..4 {
                            match source.current() {
                                Some(d) if d.is_ascii_hexdigit() => {
                                    hex.push(d);
                                    source.next();
                                }
                                _ => return Err(ERR_INVALID_ESCAPE.to_string())
                            }
                        }
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(ch) = char::from_u32(code) {
                                s.push(ch);
                            } else {
                                return Err(ERR_INVALID_ESCAPE.to_string());
                            }
                        } else {
                            return Err(ERR_INVALID_ESCAPE.to_string());
                        }
                        continue;
                    }
                    _ => return Err(ERR_INVALID_ESCAPE.to_string())
                }
                source.next();
            }
            _ => {
                s.push(c);
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
    if source.current() == Some('-') {
        num_str.push('-');
        source.next();
    }

    while let Some(c) = source.current() {
        match c {
            '0'..='9' => {
                num_str.push(c);
                source.next();
            }
            '.' => {
                if is_float {
                    return Err(ERR_MULTIPLE_DECIMAL.to_string());
                }
                is_float = true;
                num_str.push(c);
                source.next();
            }
            'e' | 'E' => {
                is_float = true;
                num_str.push(c);
                source.next();

                if let Some(sign) = source.current() {
                    if sign == '+' || sign == '-' {
                        num_str.push(sign);
                        source.next();
                    }
                }
            }
            _ => break
        }
    }

    if is_float {
        match num_str.parse::<f64>() {
            Ok(n) => Ok(Node::Number(Numeric::Float(n))),
            Err(_) => Err(ERR_INVALID_FLOAT.to_string())
        }
    } else {
        match num_str.parse::<i64>() {
            Ok(n) => Ok(Node::Number(Numeric::Integer(n))),
            Err(_) => Err(ERR_INVALID_INTEGER.to_string())
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
    use crate::json_lib::io::sources::buffer::Buffer;
    use std::fs;
    use crate::FileSource;

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
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Integer(123)))));
    }

    #[test]
    fn test_parse_number_float() {
        let mut source = Buffer::new(b"123.45");
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 123.45).abs() < f64::EPSILON));
    }

    #[test]
    fn test_parse_array() {
        let mut source = Buffer::new(b"[1,2,3]");
        match parse(&mut source) {
            Ok(Node::Array(arr)) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected array")
        }
    }

    #[test]
    fn test_parse_object() {
        let mut source = Buffer::new(b"{\"key\":\"value\"}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => assert_eq!(obj.len(), 1),
            _ => panic!("Expected object")
        }
    }

    #[test]
    fn test_parse_empty_array() {
        let mut source = Buffer::new(b"[]");
        match parse(&mut source) {
            Ok(Node::Array(arr)) => assert_eq!(arr.len(), 0),
            _ => panic!("Expected empty array")
        }
    }

    #[test]
    fn test_parse_empty_object() {
        let mut source = Buffer::new(b"{}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => assert_eq!(obj.len(), 0),
            _ => panic!("Expected empty object")
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
            _ => panic!("Expected empty object")
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

    fn get_json_file_paths(directory: &str) -> Vec<String> {
        let mut paths = Vec::new();
        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        if let Some(path_str) = path.to_str() {
                            paths.push(path_str.to_string());
                        }
                    }
                }
            }
        }
        paths
    }

    #[test]
    fn test_parse_json_files() {
        let files_dir = "../files";
        let json_files = get_json_file_paths(files_dir);
        for file_path in json_files {
            match FileSource::new(&file_path.to_string()) {
                Ok(mut source) => {
                    let result = parse(&mut source);
                    assert!(result.is_ok(), "Failed to parse {}: {:?}", file_path, result.err());
                },
                Err(e) => panic!("Failed to open {}: {}", file_path, e),
            }


        }
    }

    #[test]
    fn test_parse_negative_number() {
        let mut source = Buffer::new(b"-123");
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Integer(-123)))));

        let mut source = Buffer::new(b"-123.45");
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - -123.45).abs() < f64::EPSILON));
    }

    #[test]
    fn test_parse_scientific_notation() {
        let mut source = Buffer::new(b"1.23e+2");
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 123.0).abs() < f64::EPSILON));

        let mut source = Buffer::new(b"1.23E-2");
        assert!(matches!(parse(&mut source), Ok(Node::Number(Numeric::Float(n))) if (n - 0.0123).abs() < f64::EPSILON));
    }

    #[test]
    fn test_parse_complex_object() {
        let mut source = Buffer::new(b"{\"array\":[1,{\"nested\":true},null],\"string\":\"value\"}");
        match parse(&mut source) {
            Ok(Node::Object(obj)) => {
                assert_eq!(obj.len(), 2);
                assert!(obj.contains_key("array"));
                assert!(obj.contains_key("string"));
            },
            _ => panic!("Expected complex object")
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
}

