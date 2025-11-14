//! JSON Patch (RFC 6902) implementation
//!
//! Provides operations to modify JSON documents: add, remove, replace, move, copy, test

use crate::nodes::node::Node;

#[cfg(feature = "json-pointer")]
use crate::nodes::json_pointer;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

/// JSON Patch operation
#[derive(Debug, Clone, PartialEq)]
pub enum PatchOp {
    Add { path: String, value: Node },
    Remove { path: String },
    Replace { path: String, value: Node },
    Move { from: String, path: String },
    Copy { from: String, path: String },
    Test { path: String, value: Node },
}

/// Error type for patch operations
#[derive(Debug, Clone, PartialEq)]
pub struct PatchError {
    pub message: String,
}

impl PatchError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { message: msg.into() }
    }
}

impl From<String> for PatchError {
    fn from(s: String) -> Self {
        PatchError::new(s)
    }
}

/// Apply a single patch operation to a node
#[cfg(feature = "json-pointer")]
pub fn apply_operation(node: &mut Node, op: &PatchOp) -> Result<(), PatchError> {
    match op {
        PatchOp::Add { path, value } => {
            add_value(node, path, value.clone())
        }
        PatchOp::Remove { path } => {
            remove_value(node, path)
        }
        PatchOp::Replace { path, value } => {
            replace_value(node, path, value.clone())
        }
        PatchOp::Move { from, path } => {
            move_value(node, from, path)
        }
        PatchOp::Copy { from, path } => {
            copy_value(node, from, path)
        }
        PatchOp::Test { path, value } => {
            test_value(node, path, value)
        }
    }
}

/// Apply multiple patch operations in sequence
#[cfg(feature = "json-pointer")]
pub fn apply_patch(node: &mut Node, operations: &[PatchOp]) -> Result<(), PatchError> {
    for op in operations {
        apply_operation(node, op)?;
    }
    Ok(())
}

#[cfg(feature = "json-pointer")]
fn add_value(node: &mut Node, path: &str, value: Node) -> Result<(), PatchError> {
    if path == "" {
        *node = value;
        return Ok(());
    }

    // Split path to get parent and key
    let parts: Vec<&str> = path.rsplitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(PatchError::new("Invalid path"));
    }

    let key = parts[0];
    let parent_path = parts[1];

    // Get parent node
    let parent = if parent_path.is_empty() {
        node
    } else {
        json_pointer::get_mut(node, parent_path)
            .ok_or_else(|| PatchError::new("Parent path not found"))?
    };

    // Add to parent
    if let Some(obj) = parent.as_object_mut() {
        obj.insert(key.to_string(), value);
        Ok(())
    } else if let Some(arr) = parent.as_array_mut() {
        if key == "-" {
            arr.push(value);
            Ok(())
        } else if let Ok(idx) = key.parse::<usize>() {
            if idx <= arr.len() {
                arr.insert(idx, value);
                Ok(())
            } else {
                Err(PatchError::new("Array index out of bounds"))
            }
        } else {
            Err(PatchError::new("Invalid array index"))
        }
    } else {
        Err(PatchError::new("Parent is not an object or array"))
    }
}

#[cfg(feature = "json-pointer")]
fn remove_value(node: &mut Node, path: &str) -> Result<(), PatchError> {
    if path == "" {
        return Err(PatchError::new("Cannot remove root"));
    }

    match json_pointer::remove(node, path) {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(PatchError::new("Path not found")),
        Err(e) => Err(PatchError::new(&format!("Invalid path: {}", e))),
    }
}

#[cfg(feature = "json-pointer")]
fn replace_value(node: &mut Node, path: &str, value: Node) -> Result<(), PatchError> {
    if path == "" {
        *node = value;
        return Ok(());
    }

    let target = json_pointer::get_mut(node, path)
        .ok_or_else(|| PatchError::new("Path not found"))?;
    *target = value;
    Ok(())
}

#[cfg(feature = "json-pointer")]
fn move_value(node: &mut Node, from: &str, to: &str) -> Result<(), PatchError> {
    // Get the value to move
    let value = json_pointer::get(node, from)
        .ok_or_else(|| PatchError::new("Source path not found"))?
        .clone();
    
    // Remove from source
    match json_pointer::remove(node, from) {
        Ok(Some(_)) => {},
        Ok(None) => return Err(PatchError::new("Failed to remove from source")),
        Err(e) => return Err(PatchError::new(&format!("Invalid path: {}", e))),
    }
    
    // Add to destination
    add_value(node, to, value)
}

#[cfg(feature = "json-pointer")]
fn copy_value(node: &mut Node, from: &str, to: &str) -> Result<(), PatchError> {
    let value = json_pointer::get(node, from)
        .ok_or_else(|| PatchError::new("Source path not found"))?
        .clone();
    
    add_value(node, to, value)
}

#[cfg(feature = "json-pointer")]
fn test_value(node: &Node, path: &str, expected: &Node) -> Result<(), PatchError> {
    let actual = json_pointer::get(node, path)
        .ok_or_else(|| PatchError::new("Path not found"))?;
    
    if actual == expected {
        Ok(())
    } else {
        Err(PatchError::new(format!(
            "Test failed: expected {:?}, got {:?}",
            expected, actual
        )))
    }
}

/// Parse a JSON Patch document (array of operations)
#[cfg(feature = "json-pointer")]
pub fn parse_patch(patch_doc: &Node) -> Result<Vec<PatchOp>, PatchError> {
    let arr = patch_doc.as_array()
        .ok_or_else(|| PatchError::new("Patch document must be an array"))?;
    
    let mut operations = Vec::new();
    
    for item in arr {
        let obj = item.as_object()
            .ok_or_else(|| PatchError::new("Patch operation must be an object"))?;
        
        let op_type = obj.get("op")
            .and_then(|n| n.as_str())
            .ok_or_else(|| PatchError::new("Missing 'op' field"))?;
        
        let op = match op_type {
            "add" => {
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                let value = obj.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field"))?;
                PatchOp::Add {
                    path: path.to_string(),
                    value: value.clone(),
                }
            }
            "remove" => {
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                PatchOp::Remove {
                    path: path.to_string(),
                }
            }
            "replace" => {
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                let value = obj.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field"))?;
                PatchOp::Replace {
                    path: path.to_string(),
                    value: value.clone(),
                }
            }
            "move" => {
                let from = obj.get("from")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'from' field"))?;
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                PatchOp::Move {
                    from: from.to_string(),
                    path: path.to_string(),
                }
            }
            "copy" => {
                let from = obj.get("from")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'from' field"))?;
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                PatchOp::Copy {
                    from: from.to_string(),
                    path: path.to_string(),
                }
            }
            "test" => {
                let path = obj.get("path")
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| PatchError::new("Missing 'path' field"))?;
                let value = obj.get("value")
                    .ok_or_else(|| PatchError::new("Missing 'value' field"))?;
                PatchOp::Test {
                    path: path.to_string(),
                    value: value.clone(),
                }
            }
            _ => return Err(PatchError::new(format!("Unknown operation: {}", op_type))),
        };
        
        operations.push(op);
    }
    
    Ok(operations)
}

#[cfg(test)]
#[cfg(feature = "json-pointer")]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn test_add_operation() {
        let mut doc = json!({"a": 1});
        let op = PatchOp::Add {
            path: "/b".to_string(),
            value: json!(2),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert_eq!(doc["b"], json!(2));
    }

    #[test]
    fn test_remove_operation() {
        let mut doc = json!({"a": 1, "b": 2});
        let op = PatchOp::Remove {
            path: "/b".to_string(),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert!(doc["b"].is_null());
    }

    #[test]
    fn test_replace_operation() {
        let mut doc = json!({"a": 1});
        let op = PatchOp::Replace {
            path: "/a".to_string(),
            value: json!(2),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert_eq!(doc["a"], json!(2));
    }

    #[test]
    fn test_move_operation() {
        let mut doc = json!({"a": 1, "b": 2});
        let op = PatchOp::Move {
            from: "/a".to_string(),
            path: "/c".to_string(),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert!(doc["a"].is_null());
        assert_eq!(doc["c"], json!(1));
    }

    #[test]
    fn test_copy_operation() {
        let mut doc = json!({"a": 1});
        let op = PatchOp::Copy {
            from: "/a".to_string(),
            path: "/b".to_string(),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert_eq!(doc["a"], json!(1));
        assert_eq!(doc["b"], json!(1));
    }

    #[test]
    fn test_test_operation() {
        let doc = json!({"a": 1});
        let op = PatchOp::Test {
            path: "/a".to_string(),
            value: json!(1),
        };
        
        assert!(apply_operation(&mut doc.clone(), &op).is_ok());
        
        let op_fail = PatchOp::Test {
            path: "/a".to_string(),
            value: json!(2),
        };
        assert!(apply_operation(&mut doc.clone(), &op_fail).is_err());
    }

    #[test]
    fn test_array_operations() {
        let mut doc = json!([1, 2, 3]);
        let op = PatchOp::Add {
            path: "/-".to_string(),
            value: json!(4),
        };
        
        assert!(apply_operation(&mut doc, &op).is_ok());
        assert_eq!(doc[3], json!(4));
    }
}
