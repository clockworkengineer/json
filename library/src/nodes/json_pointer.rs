//! JSON Pointer implementation (RFC 6901)
//!
//! Provides functionality to navigate, query, and modify JSON structures using JSON Pointer syntax.
//! A JSON Pointer is a string syntax for identifying a specific value within a JSON document.
//!
//! # Examples
//!
//! ```
//! use json_lib::{Node, Numeric};
//! # #[cfg(feature = "std")]
//! use std::collections::HashMap;
//! # #[cfg(not(feature = "std"))]
//! # use alloc::collections::BTreeMap as HashMap;
//!
//! # #[cfg(feature = "alloc")]
//! # {
//! let mut obj = HashMap::new();
//! obj.insert("name".to_string(), Node::Str("Alice".to_string()));
//! let node = Node::Object(obj);
//!
//! // Get a value using JSON Pointer
//! let name = json_lib::nodes::json_pointer::get(&node, "/name");
//! # }
//! ```

use crate::nodes::node::Node;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

/// Gets a value from a Node using a JSON Pointer string (RFC 6901)
///
/// # Arguments
/// * `node` - The root Node to query
/// * `pointer` - JSON Pointer string (e.g., "/users/0/name")
///
/// # Returns
/// * `Some(&Node)` if the pointer resolves to a value
/// * `None` if the pointer cannot be resolved
///
/// # Examples
/// ```
/// use json_lib::{Node, Numeric};
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("foo".to_string(), Node::Array(vec![
///     Node::Str("bar".to_string()),
///     Node::Str("baz".to_string()),
/// ]));
/// let node = Node::Object(obj);
///
/// // Access "/foo/0"
/// let result = json_lib::nodes::json_pointer::get(&node, "/foo/0");
/// assert!(result.is_some());
/// ```
pub fn get<'a>(node: &'a Node, pointer: &str) -> Option<&'a Node> {
    if pointer.is_empty() || pointer == "/" {
        return Some(node);
    }

    if !pointer.starts_with('/') {
        return None;
    }

    let tokens = parse_pointer(pointer);
    let mut current = node;

    for token in tokens {
        current = match current {
            Node::Object(map) => map.get(&token)?,
            Node::Array(arr) => {
                let index = parse_array_index(&token)?;
                arr.get(index)?
            }
            _ => return None,
        };
    }

    Some(current)
}

/// Gets a mutable reference to a value using a JSON Pointer string
///
/// # Arguments
/// * `node` - The root Node to query (mutable)
/// * `pointer` - JSON Pointer string (e.g., "/users/0/name")
///
/// # Returns
/// * `Some(&mut Node)` if the pointer resolves to a value
/// * `None` if the pointer cannot be resolved
pub fn get_mut<'a>(node: &'a mut Node, pointer: &str) -> Option<&'a mut Node> {
    if pointer.is_empty() || pointer == "/" {
        return Some(node);
    }

    if !pointer.starts_with('/') {
        return None;
    }

    let tokens = parse_pointer(pointer);
    let mut current = node;

    for token in tokens {
        current = match current {
            Node::Object(map) => map.get_mut(&token)?,
            Node::Array(arr) => {
                let index = parse_array_index(&token)?;
                arr.get_mut(index)?
            }
            _ => return None,
        };
    }

    Some(current)
}

/// Sets a value in a Node using a JSON Pointer string
/// Creates intermediate objects/arrays as needed
///
/// # Arguments
/// * `node` - The root Node to modify
/// * `pointer` - JSON Pointer string (e.g., "/users/0/name")
/// * `value` - The value to set
///
/// # Returns
/// * `Ok(())` if the value was set successfully
/// * `Err(String)` if the pointer is invalid or cannot be created
pub fn set(node: &mut Node, pointer: &str, value: Node) -> Result<(), String> {
    if pointer.is_empty() || pointer == "/" {
        *node = value;
        return Ok(());
    }

    if !pointer.starts_with('/') {
        return Err("JSON Pointer must start with '/'".to_string());
    }

    let tokens = parse_pointer(pointer);
    if tokens.is_empty() {
        return Err("Invalid JSON Pointer".to_string());
    }

    let (parent_tokens, last_token) = tokens.split_at(tokens.len() - 1);
    let last_token = &last_token[0];

    // Navigate to parent, creating structure as needed
    let mut current = node;
    for token in parent_tokens {
        // Determine if next level should be array or object
        let is_next_array = token.parse::<usize>().is_ok();

        current = match current {
            Node::Object(map) => map.entry(token.clone()).or_insert_with(|| {
                if is_next_array {
                    Node::Array(Vec::new())
                } else {
                    Node::Object(HashMap::new())
                }
            }),
            Node::Array(arr) => {
                let index = parse_array_index(token)
                    .ok_or_else(|| format!("Invalid array index: {}", token))?;
                if index >= arr.len() {
                    arr.resize(index + 1, Node::None);
                }
                &mut arr[index]
            }
            _ => return Err("Cannot navigate through non-object/non-array".to_string()),
        };
    }

    // Set the final value
    match current {
        Node::Object(map) => {
            map.insert(last_token.clone(), value);
        }
        Node::Array(arr) => {
            let index = parse_array_index(last_token)
                .ok_or_else(|| format!("Invalid array index: {}", last_token))?;
            if index >= arr.len() {
                arr.resize(index + 1, Node::None);
            }
            arr[index] = value;
        }
        _ => return Err("Parent must be an object or array".to_string()),
    }

    Ok(())
}

/// Removes a value from a Node using a JSON Pointer string
///
/// # Arguments
/// * `node` - The root Node to modify
/// * `pointer` - JSON Pointer string (e.g., "/users/0")
///
/// # Returns
/// * `Ok(Some(Node))` if a value was removed
/// * `Ok(None)` if the pointer didn't point to anything
/// * `Err(String)` if the pointer is invalid
pub fn remove(node: &mut Node, pointer: &str) -> Result<Option<Node>, String> {
    if pointer.is_empty() || pointer == "/" {
        return Err("Cannot remove root node".to_string());
    }

    if !pointer.starts_with('/') {
        return Err("JSON Pointer must start with '/'".to_string());
    }

    let tokens = parse_pointer(pointer);
    if tokens.is_empty() {
        return Err("Invalid JSON Pointer".to_string());
    }

    let (parent_tokens, last_token) = tokens.split_at(tokens.len() - 1);
    let last_token = &last_token[0];

    // Navigate to parent
    let mut current = node;
    for token in parent_tokens {
        current = match current {
            Node::Object(map) => map.get_mut(token).ok_or("Path not found")?,
            Node::Array(arr) => {
                let index = parse_array_index(token)
                    .ok_or_else(|| format!("Invalid array index: {}", token))?;
                arr.get_mut(index).ok_or("Index out of bounds")?
            }
            _ => return Err("Cannot navigate through non-object/non-array".to_string()),
        };
    }

    // Remove the final value
    match current {
        Node::Object(map) => Ok(map.remove(last_token)),
        Node::Array(arr) => {
            let index = parse_array_index(last_token)
                .ok_or_else(|| format!("Invalid array index: {}", last_token))?;
            if index < arr.len() {
                Ok(Some(arr.remove(index)))
            } else {
                Ok(None)
            }
        }
        _ => Err("Parent must be an object or array".to_string()),
    }
}

/// Parses a JSON Pointer string into individual reference tokens
fn parse_pointer(pointer: &str) -> Vec<String> {
    pointer[1..] // Skip leading '/'
        .split('/')
        .map(|token| {
            // Unescape ~1 to / and ~0 to ~
            token.replace("~1", "/").replace("~0", "~")
        })
        .collect()
}

/// Parses a string as an array index
fn parse_array_index(token: &str) -> Option<usize> {
    // "-" means append to end (not supported for get/remove)
    if token == "-" {
        return None;
    }
    token.parse::<usize>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::node::Numeric;
    use std::collections::HashMap;

    fn create_test_node() -> Node {
        let mut inner_obj = HashMap::new();
        inner_obj.insert("baz".to_string(), Node::Number(Numeric::Integer(42)));

        let mut obj = HashMap::new();
        obj.insert(
            "foo".to_string(),
            Node::Array(vec![Node::Str("bar".to_string()), Node::Object(inner_obj)]),
        );
        obj.insert("".to_string(), Node::Number(Numeric::Integer(0)));
        obj.insert("a/b".to_string(), Node::Number(Numeric::Integer(1)));
        obj.insert("c%d".to_string(), Node::Number(Numeric::Integer(2)));
        obj.insert("e^f".to_string(), Node::Number(Numeric::Integer(3)));
        obj.insert("g|h".to_string(), Node::Number(Numeric::Integer(4)));
        obj.insert("i\\j".to_string(), Node::Number(Numeric::Integer(5)));
        obj.insert("k\"l".to_string(), Node::Number(Numeric::Integer(6)));
        obj.insert(" ".to_string(), Node::Number(Numeric::Integer(7)));
        obj.insert("m~n".to_string(), Node::Number(Numeric::Integer(8)));

        Node::Object(obj)
    }

    #[test]
    fn test_get_root() {
        let node = create_test_node();
        assert!(get(&node, "").is_some());
        assert!(get(&node, "/").is_some());
    }

    #[test]
    fn test_get_simple_key() {
        let node = create_test_node();
        let result = get(&node, "/foo");
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), Node::Array(_)));
    }

    #[test]
    fn test_get_array_element() {
        let node = create_test_node();
        let result = get(&node, "/foo/0");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), &Node::Str("bar".to_string()));
    }

    #[test]
    fn test_get_nested() {
        let mut inner = HashMap::new();
        inner.insert("baz".to_string(), Node::Number(Numeric::Integer(42)));
        let node = Node::Object(inner);

        let result = get(&node, "/baz");
        assert!(result.is_some());
    }

    #[test]
    fn test_get_nonexistent() {
        let node = create_test_node();
        assert!(get(&node, "/nonexistent").is_none());
        assert!(get(&node, "/foo/99").is_none());
    }

    #[test]
    fn test_get_empty_string_key() {
        let node = create_test_node();
        let result = get(&node, "/");
        assert!(result.is_some());
    }

    #[test]
    fn test_get_special_chars() {
        let node = create_test_node();
        assert!(get(&node, "/a~1b").is_some()); // a/b with ~1 escape
        assert!(get(&node, "/m~0n").is_some()); // m~n with ~0 escape
    }

    #[test]
    fn test_set_new_key() {
        let mut node = Node::Object(HashMap::new());
        set(&mut node, "/foo", Node::Str("bar".to_string())).unwrap();

        let result = get(&node, "/foo");
        assert_eq!(result, Some(&Node::Str("bar".to_string())));
    }

    #[test]
    fn test_set_nested() {
        let mut node = Node::Object(HashMap::new());
        set(&mut node, "/a/b/c", Node::Number(Numeric::Integer(123))).unwrap();

        let result = get(&node, "/a/b/c");
        assert!(result.is_some());
    }

    #[test]
    fn test_remove_key() {
        let mut node = create_test_node();
        let removed = remove(&mut node, "/foo").unwrap();
        assert!(removed.is_some());
        assert!(get(&node, "/foo").is_none());
    }

    #[test]
    fn test_remove_array_element() {
        let mut node = Node::Object({
            let mut map = HashMap::new();
            map.insert(
                "arr".to_string(),
                Node::Array(vec![
                    Node::Number(Numeric::Integer(1)),
                    Node::Number(Numeric::Integer(2)),
                    Node::Number(Numeric::Integer(3)),
                ]),
            );
            map
        });

        let removed = remove(&mut node, "/arr/1").unwrap();
        assert_eq!(removed, Some(Node::Number(Numeric::Integer(2))));

        // Array should now be [1, 3]
        if let Some(Node::Array(arr)) = get(&node, "/arr") {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_get_mut() {
        let mut node = create_test_node();
        if let Some(value) = get_mut(&mut node, "/foo/0") {
            *value = Node::Str("modified".to_string());
        }

        assert_eq!(
            get(&node, "/foo/0"),
            Some(&Node::Str("modified".to_string()))
        );
    }

    #[test]
    fn test_invalid_pointer() {
        let node = create_test_node();
        assert!(get(&node, "invalid").is_none());
    }
}
