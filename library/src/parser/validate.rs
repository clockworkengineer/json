//! JSON validation without allocation
//!
//! Provides fast validation of JSON syntax without building a Node tree.
//! Useful for rejecting invalid data before attempting full parsing.

use crate::io::traits::ISource;
use crate::parser::config::ParserConfig;

const BACKSLASH: char = '\\';
const COMMA: char = ',';
const DECIMAL_POINT: char = '.';
const QUOTE: char = '"';
const COLON: char = ':';
const L_BRACE: char = '{';
const R_BRACE: char = '}';
const L_BRACKET: char = '[';
const R_BRACKET: char = ']';

/// Validates JSON syntax without allocating memory for nodes
///
/// This is faster than full parsing and uses minimal memory.
/// Useful for:
/// - Early rejection of invalid data
/// - Security checks before processing
/// - Bandwidth verification before transmission
///
/// # Arguments
/// * `source` - Source to read JSON from
/// * `config` - Parser configuration for depth/size limits
///
/// # Returns
/// * `Ok(())` if JSON is valid
/// * `Err(String)` with error message if invalid
///
/// # Examples
/// ```
/// use json_lib::{validate_json, BufferSource, ParserConfig};
///
/// let json = br#"{"valid": true}"#;
/// let mut source = BufferSource::new(json);
/// let config = ParserConfig::new();
///
/// assert!(validate_json(&mut source, &config).is_ok());
/// ```
pub fn validate_json(source: &mut dyn ISource, config: &ParserConfig) -> Result<(), String> {
    let mut depth = 0;
    validate_value(source, config, &mut depth)
}

fn validate_value(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: &mut usize,
) -> Result<(), String> {
    skip_whitespace(source);

    match source.current() {
        Some(L_BRACE) => validate_object(source, config, depth),
        Some(L_BRACKET) => validate_array(source, config, depth),
        Some(QUOTE) => validate_string(source, config),
        Some('t') | Some('f') => validate_boolean(source),
        Some('n') => validate_null(source),
        Some('-') | Some('0'..='9') => validate_number(source),
        Some(c) => Err(format!("Unexpected character: {}", c)),
        None => Err("Unexpected end of input".to_string()),
    }
}

fn validate_object(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: &mut usize,
) -> Result<(), String> {
    *depth += 1;
    if let Some(max_depth) = config.max_depth {
        if *depth > max_depth {
            return Err(format!("Maximum nesting depth of {} exceeded", max_depth));
        }
    }

    source.next(); // skip {
    skip_whitespace(source);

    let mut size = 0;

    // Empty object
    if source.current() == Some(R_BRACE) {
        source.next();
        *depth -= 1;
        return Ok(());
    }

    loop {
        size += 1;
        if let Some(max_size) = config.max_object_size {
            if size > max_size {
                return Err(format!("Maximum object size of {} exceeded", max_size));
            }
        }

        // Expect key (string)
        skip_whitespace(source);
        if source.current() != Some(QUOTE) {
            return Err("Expected string key in object".to_string());
        }
        validate_string(source, config)?;

        // Expect colon
        skip_whitespace(source);
        if source.current() != Some(COLON) {
            return Err("Expected ':' after object key".to_string());
        }
        source.next();

        // Expect value
        validate_value(source, config, depth)?;

        // Check for comma or end
        skip_whitespace(source);
        match source.current() {
            Some(COMMA) => {
                source.next();
                skip_whitespace(source);
                // Check for trailing comma
                if source.current() == Some(R_BRACE) {
                    return Err("Trailing comma in object".to_string());
                }
            }
            Some(R_BRACE) => {
                source.next();
                *depth -= 1;
                return Ok(());
            }
            _ => return Err("Expected ',' or '}' in object".to_string()),
        }
    }
}

fn validate_array(
    source: &mut dyn ISource,
    config: &ParserConfig,
    depth: &mut usize,
) -> Result<(), String> {
    *depth += 1;
    if let Some(max_depth) = config.max_depth {
        if *depth > max_depth {
            return Err(format!("Maximum nesting depth of {} exceeded", max_depth));
        }
    }

    source.next(); // skip [
    skip_whitespace(source);

    let mut size = 0;

    // Empty array
    if source.current() == Some(R_BRACKET) {
        source.next();
        *depth -= 1;
        return Ok(());
    }

    loop {
        size += 1;
        if let Some(max_size) = config.max_array_size {
            if size > max_size {
                return Err(format!("Maximum array size of {} exceeded", max_size));
            }
        }

        validate_value(source, config, depth)?;

        skip_whitespace(source);
        match source.current() {
            Some(COMMA) => {
                source.next();
                skip_whitespace(source);
                // Check for trailing comma
                if source.current() == Some(R_BRACKET) {
                    return Err("Trailing comma in array".to_string());
                }
            }
            Some(R_BRACKET) => {
                source.next();
                *depth -= 1;
                return Ok(());
            }
            _ => return Err("Expected ',' or ']' in array".to_string()),
        }
    }
}

fn validate_string(source: &mut dyn ISource, config: &ParserConfig) -> Result<(), String> {
    source.next(); // skip opening quote
    let mut length = 0;

    while let Some(c) = source.current() {
        if let Some(max_len) = config.max_string_length {
            if length >= max_len {
                return Err(format!(
                    "Maximum string length of {} bytes exceeded",
                    max_len
                ));
            }
        }

        match c {
            QUOTE => {
                source.next();
                return Ok(());
            }
            BACKSLASH => {
                source.next();
                match source.current() {
                    Some('"') | Some('\\') | Some('/') | Some('b') | Some('f') | Some('n')
                    | Some('r') | Some('t') => {
                        source.next();
                        length += 1;
                    }
                    Some('u') => {
                        // Validate unicode escape
                        source.next();
                        for _ in 0..4 {
                            match source.current() {
                                Some(d) if d.is_ascii_hexdigit() => source.next(),
                                _ => return Err("Invalid unicode escape".to_string()),
                            }
                        }
                        length += 1;
                    }
                    _ => return Err("Invalid escape sequence".to_string()),
                }
            }
            _ => {
                source.next();
                length += 1;
            }
        }
    }

    Err("Unterminated string".to_string())
}

fn validate_number(source: &mut dyn ISource) -> Result<(), String> {
    // Handle negative
    if source.current() == Some('-') {
        source.next();
    }

    // Require at least one digit
    if !matches!(source.current(), Some('0'..='9')) {
        return Err("Expected digit in number".to_string());
    }

    // Integer part
    if source.current() == Some('0') {
        source.next();
        // Leading zero must be followed by . or e/E or end
        match source.current() {
            Some('.') | Some('e') | Some('E') | None => {}
            Some(c) if c.is_whitespace() || c == ',' || c == ']' || c == '}' => {}
            _ => return Err("Invalid number: leading zero".to_string()),
        }
    } else {
        while matches!(source.current(), Some('0'..='9')) {
            source.next();
        }
    }

    // Fractional part
    if source.current() == Some(DECIMAL_POINT) {
        source.next();
        if !matches!(source.current(), Some('0'..='9')) {
            return Err("Expected digit after decimal point".to_string());
        }
        while matches!(source.current(), Some('0'..='9')) {
            source.next();
        }
    }

    // Exponent part
    if matches!(source.current(), Some('e') | Some('E')) {
        source.next();
        if matches!(source.current(), Some('+') | Some('-')) {
            source.next();
        }
        if !matches!(source.current(), Some('0'..='9')) {
            return Err("Expected digit in exponent".to_string());
        }
        while matches!(source.current(), Some('0'..='9')) {
            source.next();
        }
    }

    Ok(())
}

fn validate_boolean(source: &mut dyn ISource) -> Result<(), String> {
    match source.current() {
        Some('t') => {
            for expected in ['t', 'r', 'u', 'e'] {
                if source.current() != Some(expected) {
                    return Err("Invalid boolean: expected 'true'".to_string());
                }
                source.next();
            }
            Ok(())
        }
        Some('f') => {
            for expected in ['f', 'a', 'l', 's', 'e'] {
                if source.current() != Some(expected) {
                    return Err("Invalid boolean: expected 'false'".to_string());
                }
                source.next();
            }
            Ok(())
        }
        _ => Err("Invalid boolean".to_string()),
    }
}

fn validate_null(source: &mut dyn ISource) -> Result<(), String> {
    for expected in ['n', 'u', 'l', 'l'] {
        if source.current() != Some(expected) {
            return Err("Invalid null value".to_string());
        }
        source.next();
    }
    Ok(())
}

fn skip_whitespace(source: &mut dyn ISource) {
    while let Some(c) = source.current() {
        if c.is_whitespace() {
            source.next();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::sources::buffer::Buffer;

    #[test]
    fn test_validate_simple_object() {
        let json = br#"{"key": "value"}"#;
        let mut source = Buffer::new(json);
        let config = ParserConfig::new();

        assert!(validate_json(&mut source, &config).is_ok());
    }

    #[test]
    fn test_validate_nested() {
        let json = br#"{"a": {"b": {"c": 123}}}"#;
        let mut source = Buffer::new(json);
        let config = ParserConfig::new();

        assert!(validate_json(&mut source, &config).is_ok());
    }

    #[test]
    fn test_validate_array() {
        let json = br#"[1, 2, 3, "test", true, null]"#;
        let mut source = Buffer::new(json);
        let config = ParserConfig::new();

        assert!(validate_json(&mut source, &config).is_ok());
    }

    #[test]
    fn test_validate_invalid_json() {
        let json = br#"{"key": }"#;
        let mut source = Buffer::new(json);
        let config = ParserConfig::new();

        assert!(validate_json(&mut source, &config).is_err());
    }

    #[test]
    fn test_validate_depth_limit() {
        let json = br#"{"a":{"b":{"c":{"d":1}}}}"#;
        let mut source = Buffer::new(json);
        let config = ParserConfig::new().with_max_depth(Some(3));

        assert!(validate_json(&mut source, &config).is_err());
    }

    #[test]
    fn test_validate_numbers() {
        let test_cases: Vec<(&[u8], bool)> = vec![
            (b"123", true),
            (b"-456", true),
            (b"3.14", true),
            (b"1e10", true),
            (b"2.5e-3", true),
            (b"01", false), // Leading zero
            (b"-.5", false), // No digit before decimal
        ];

        for (json, should_pass) in test_cases {
            let mut source = Buffer::new(json);
            let config = ParserConfig::new();
            let result = validate_json(&mut source, &config);

            if should_pass {
                assert!(result.is_ok(), "Expected {:?} to be valid", json);
            } else {
                assert!(result.is_err(), "Expected {:?} to be invalid", json);
            }
        }
    }
}
