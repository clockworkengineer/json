use crate::json_lib::nodes::node::Node;
use crate::json_lib::nodes::node::Number;
use std::collections::HashMap;
use crate::json_lib::io::traits::ISource;
// use crate::json_lib::error::messages::*;

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
        Some(c) => Err(format!("Unexpected character: {}", c)),
        None => Err("Empty input".to_string())
    }
}

fn skip_whitespace(source: &mut dyn ISource) {
    while let Some(c) = source.current() {
        if !c.is_whitespace() {
            break;
        }
        source.next();
    }
}

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
            _ => return Err("Object key must be a string".to_string())
        };

        skip_whitespace(source);

        // Check for colon
        match source.current() {
            Some(':') => source.next(),
            _ => return Err("Expected ':' after object key".to_string())
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
            _ => return Err("Expected ',' or '}' in object".to_string())
        }
    }

    Ok(Node::Object(map))
}

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
            _ => return Err("Expected ',' or ']' in array".to_string())
        }
    }

    Ok(Node::Array(vec))
}

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
                    _ => return Err("Invalid escape sequence".to_string())
                }
                source.next();
            }
            _ => {
                s.push(c);
                source.next();
            }
        }
    }

    Err("Unterminated string".to_string())
}

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
                    return Err("Multiple decimal points in number".to_string());
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
            Ok(n) => Ok(Node::Number(Number::Float(n))),
            Err(_) => Err("Invalid float number".to_string())
        }
    } else {
        match num_str.parse::<i64>() {
            Ok(n) => Ok(Node::Number(Number::Integer(n))),
            Err(_) => Err("Invalid integer number".to_string())
        }
    }
}

fn parse_true(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 't'
    for c in ['r', 'u', 'e'] {
        if source.current() != Some(c) {
            return Err("Expected 'true'".to_string());
        }
        source.next();
    }
    Ok(Node::Boolean(true))
}

fn parse_false(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 'f'
    for c in ['a', 'l', 's', 'e'] {
        if source.current() != Some(c) {
            return Err("Expected 'false'".to_string());
        }
        source.next();
    }
    Ok(Node::Boolean(false))
}

fn parse_null(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // Skip 'n'
    for c in ['u', 'l', 'l'] {
        if source.current() != Some(c) {
            return Err("Expected 'null'".to_string());
        }
        source.next();
    }
    Ok(Node::None)
}