//! JSON Merge Patch (RFC 7386) implementation
//!
//! Simpler alternative to JSON Patch for partial updates.
//! Uses JSON structure itself to describe the changes.

use crate::nodes::node::Node;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;

/// Apply a merge patch to a target document
///
/// # Arguments
/// * `target` - The document to be patched (modified in place)
/// * `patch` - The merge patch to apply
///
/// # Examples
/// ```
/// use json_lib::{json, nodes::merge_patch::merge_patch};
///
/// let mut doc = json!({"a": 1, "b": 2});
/// let patch = json!({"b": 3, "c": 4});
/// merge_patch(&mut doc, &patch);
/// // doc is now {"a": 1, "b": 3, "c": 4}
/// ```
pub fn merge_patch(target: &mut Node, patch: &Node) {
    if !patch.is_object() {
        // If patch is not an object, replace target entirely
        *target = patch.clone();
        return;
    }

    // Ensure target is an object
    if !target.is_object() {
        *target = Node::Object(HashMap::new());
    }

    let target_obj = target.as_object_mut().unwrap();
    let patch_obj = patch.as_object().unwrap();

    for (key, patch_value) in patch_obj.iter() {
        if patch_value.is_null() {
            // Remove key if patch value is null
            target_obj.remove(key);
        } else if patch_value.is_object() {
            // Recursively merge if both are objects
            if let Some(target_value) = target_obj.get_mut(key) {
                if target_value.is_object() {
                    merge_patch(target_value, patch_value);
                } else {
                    *target_value = patch_value.clone();
                }
            } else {
                target_obj.insert(key.clone(), patch_value.clone());
            }
        } else {
            // Replace/add value
            target_obj.insert(key.clone(), patch_value.clone());
        }
    }
}

/// Create a merge patch that transforms source into target
///
/// # Arguments
/// * `source` - The original document
/// * `target` - The desired document
///
/// # Returns
/// A merge patch that, when applied to source, produces target
///
/// # Examples
/// ```
/// use json_lib::{json, nodes::merge_patch::create_merge_patch};
///
/// let source = json!({"a": 1, "b": 2});
/// let target = json!({"a": 1, "c": 3});
/// let patch = create_merge_patch(&source, &target);
/// // patch is {"b": null, "c": 3}
/// ```
pub fn create_merge_patch(source: &Node, target: &Node) -> Node {
    // If target is not an object, the patch is just the target
    if !target.is_object() {
        return target.clone();
    }

    // If source is not an object, the patch is the entire target
    if !source.is_object() {
        return target.clone();
    }

    let source_obj = source.as_object().unwrap();
    let target_obj = target.as_object().unwrap();
    let mut patch = HashMap::new();

    // Add/modify keys
    for (key, target_value) in target_obj.iter() {
        if let Some(source_value) = source_obj.get(key) {
            if source_value != target_value {
                if source_value.is_object() && target_value.is_object() {
                    // Recursively create patch for nested objects
                    let nested_patch = create_merge_patch(source_value, target_value);
                    if !nested_patch.as_object().unwrap().is_empty() {
                        patch.insert(key.clone(), nested_patch);
                    }
                } else {
                    patch.insert(key.clone(), target_value.clone());
                }
            }
        } else {
            // Key exists in target but not source - add it
            patch.insert(key.clone(), target_value.clone());
        }
    }

    // Remove keys that exist in source but not in target
    for key in source_obj.keys() {
        if !target_obj.contains_key(key) {
            patch.insert(key.clone(), Node::None);
        }
    }

    Node::Object(patch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn test_merge_patch_simple() {
        let mut doc = json!({"a": 1, "b": 2});
        let patch = json!({"b": 3, "c": 4});
        merge_patch(&mut doc, &patch);
        
        assert_eq!(doc["a"], json!(1));
        assert_eq!(doc["b"], json!(3));
        assert_eq!(doc["c"], json!(4));
    }

    #[test]
    fn test_merge_patch_remove() {
        let mut doc = json!({"a": 1, "b": 2});
        let patch = json!({"b": null});
        merge_patch(&mut doc, &patch);
        
        assert_eq!(doc["a"], json!(1));
        assert!(doc["b"].is_null());
    }

    #[test]
    fn test_merge_patch_nested() {
        let mut doc = json!({
            "user": {
                "name": "Alice",
                "age": 30
            }
        });
        let patch = json!({
            "user": {
                "age": 31,
                "email": "alice@example.com"
            }
        });
        merge_patch(&mut doc, &patch);
        
        assert_eq!(doc["user"]["name"], json!("Alice"));
        assert_eq!(doc["user"]["age"], json!(31));
        assert_eq!(doc["user"]["email"], json!("alice@example.com"));
    }

    #[test]
    fn test_merge_patch_replace() {
        let mut doc = json!({"a": [1, 2, 3]});
        let patch = json!({"a": [4, 5]});
        merge_patch(&mut doc, &patch);
        
        assert_eq!(doc["a"], json!([4, 5]));
    }

    #[test]
    fn test_create_merge_patch() {
        let source = json!({"a": 1, "b": 2, "c": 3});
        let target = json!({"a": 1, "b": 4, "d": 5});
        let patch = create_merge_patch(&source, &target);
        
        // Patch should update b, add d, and remove c
        assert_eq!(patch["b"], json!(4));
        assert_eq!(patch["d"], json!(5));
        assert!(patch["c"].is_null());
        assert!(patch.get("a").is_none() || patch["a"].is_null());
    }

    #[test]
    fn test_create_merge_patch_nested() {
        let source = json!({
            "user": {
                "name": "Alice",
                "age": 30
            }
        });
        let target = json!({
            "user": {
                "name": "Alice",
                "age": 31
            }
        });
        let patch = create_merge_patch(&source, &target);
        
        assert_eq!(patch["user"]["age"], json!(31));
    }

    #[test]
    fn test_merge_patch_idempotent() {
        let mut doc = json!({"a": 1});
        let patch = json!({"b": 2});
        
        merge_patch(&mut doc, &patch);
        let first_result = doc.clone();
        
        merge_patch(&mut doc, &patch);
        assert_eq!(doc, first_result);
    }
}
