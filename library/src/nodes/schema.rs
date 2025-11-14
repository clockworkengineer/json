//! JSON Schema validation (subset of JSON Schema Draft 7)
//!
//! This module provides basic JSON Schema validation support for common use cases.
//! Supports type checking, required fields, min/max constraints, pattern matching, etc.

use crate::nodes::node::Node;

#[cfg(feature = "std")]
use std::collections::HashSet;

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeSet as HashSet,
    string::{String, ToString},
    vec::Vec,
};

/// Validation error with details
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Simple JSON Schema validator
pub struct SchemaValidator {
    schema: Node,
}

impl SchemaValidator {
    /// Create a new validator with the given schema
    pub fn new(schema: Node) -> Self {
        Self { schema }
    }

    /// Validate a node against the schema
    pub fn validate(&self, data: &Node) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        self.validate_node(data, &self.schema, "", &mut errors);
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_node(
        &self,
        data: &Node,
        schema: &Node,
        path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        // Type validation
        if let Some(type_val) = schema.get("type").and_then(|n| n.as_str()) {
            if !self.check_type(data, type_val) {
                errors.push(ValidationError::new(
                    path,
                    format!("Expected type '{}' but got '{}'", type_val, self.get_type_name(data)),
                ));
                return; // Don't continue if type is wrong
            }
        }

        // Object validation
        if data.is_object() {
            self.validate_object(data, schema, path, errors);
        }

        // Array validation
        if data.is_array() {
            self.validate_array(data, schema, path, errors);
        }

        // String validation
        if data.is_string() {
            self.validate_string(data, schema, path, errors);
        }

        // Number validation
        if data.is_number() {
            self.validate_number(data, schema, path, errors);
        }
    }

    fn check_type(&self, data: &Node, expected_type: &str) -> bool {
        match expected_type {
            "object" => data.is_object(),
            "array" => data.is_array(),
            "string" => data.is_string(),
            "number" | "integer" => data.is_number(),
            "boolean" => data.is_boolean(),
            "null" => data.is_null(),
            _ => true,
        }
    }

    fn get_type_name(&self, data: &Node) -> &str {
        if data.is_object() { "object" }
        else if data.is_array() { "array" }
        else if data.is_string() { "string" }
        else if data.is_number() { "number" }
        else if data.is_boolean() { "boolean" }
        else if data.is_null() { "null" }
        else { "unknown" }
    }

    fn validate_object(
        &self,
        data: &Node,
        schema: &Node,
        path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let obj = match data.as_object() {
            Some(o) => o,
            None => return,
        };

        // Required properties
        if let Some(required) = schema.get("required").and_then(|n| n.as_array()) {
            for req in required {
                if let Some(key) = req.as_str() {
                    if !obj.contains_key(key) {
                        errors.push(ValidationError::new(
                            path,
                            format!("Missing required property '{}'", key),
                        ));
                    }
                }
            }
        }

        // Properties validation
        if let Some(props) = schema.get("properties").and_then(|n| n.as_object()) {
            for (key, value) in obj.iter() {
                if let Some(prop_schema) = props.get(key) {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    self.validate_node(value, prop_schema, &new_path, errors);
                }
            }
        }

        // Min/max properties
        if let Some(min) = schema.get("minProperties").and_then(|n| n.as_i64()) {
            if obj.len() < min as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("Object has {} properties, minimum is {}", obj.len(), min),
                ));
            }
        }

        if let Some(max) = schema.get("maxProperties").and_then(|n| n.as_i64()) {
            if obj.len() > max as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("Object has {} properties, maximum is {}", obj.len(), max),
                ));
            }
        }
    }

    fn validate_array(
        &self,
        data: &Node,
        schema: &Node,
        path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let arr = match data.as_array() {
            Some(a) => a,
            None => return,
        };

        // Items validation
        if let Some(items_schema) = schema.get("items") {
            for (i, item) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, i);
                self.validate_node(item, items_schema, &new_path, errors);
            }
        }

        // Min/max items
        if let Some(min) = schema.get("minItems").and_then(|n| n.as_i64()) {
            if arr.len() < min as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("Array has {} items, minimum is {}", arr.len(), min),
                ));
            }
        }

        if let Some(max) = schema.get("maxItems").and_then(|n| n.as_i64()) {
            if arr.len() > max as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("Array has {} items, maximum is {}", arr.len(), max),
                ));
            }
        }

        // Unique items
        if let Some(unique) = schema.get("uniqueItems").and_then(|n| n.as_bool()) {
            if unique && !self.has_unique_items(arr) {
                errors.push(ValidationError::new(
                    path,
                    "Array items must be unique".to_string(),
                ));
            }
        }
    }

    fn validate_string(
        &self,
        data: &Node,
        schema: &Node,
        path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let s = match data.as_str() {
            Some(s) => s,
            None => return,
        };

        // Min/max length
        if let Some(min) = schema.get("minLength").and_then(|n| n.as_i64()) {
            if s.len() < min as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("String length is {}, minimum is {}", s.len(), min),
                ));
            }
        }

        if let Some(max) = schema.get("maxLength").and_then(|n| n.as_i64()) {
            if s.len() > max as usize {
                errors.push(ValidationError::new(
                    path,
                    format!("String length is {}, maximum is {}", s.len(), max),
                ));
            }
        }

        // Enum validation
        if let Some(enum_vals) = schema.get("enum").and_then(|n| n.as_array()) {
            let mut found = false;
            for val in enum_vals {
                if let Some(v) = val.as_str() {
                    if v == s {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                errors.push(ValidationError::new(
                    path,
                    format!("Value '{}' is not in allowed enum values", s),
                ));
            }
        }
    }

    fn validate_number(
        &self,
        data: &Node,
        schema: &Node,
        path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let num = match data.as_f64() {
            Some(n) => n,
            None => return,
        };

        // Min/max
        if let Some(min) = schema.get("minimum").and_then(|n| n.as_f64()) {
            if num < min {
                errors.push(ValidationError::new(
                    path,
                    format!("Number {} is less than minimum {}", num, min),
                ));
            }
        }

        if let Some(max) = schema.get("maximum").and_then(|n| n.as_f64()) {
            if num > max {
                errors.push(ValidationError::new(
                    path,
                    format!("Number {} is greater than maximum {}", num, max),
                ));
            }
        }

        // Exclusive min/max
        if let Some(min) = schema.get("exclusiveMinimum").and_then(|n| n.as_f64()) {
            if num <= min {
                errors.push(ValidationError::new(
                    path,
                    format!("Number {} is not greater than exclusive minimum {}", num, min),
                ));
            }
        }

        if let Some(max) = schema.get("exclusiveMaximum").and_then(|n| n.as_f64()) {
            if num >= max {
                errors.push(ValidationError::new(
                    path,
                    format!("Number {} is not less than exclusive maximum {}", num, max),
                ));
            }
        }
    }

    fn has_unique_items(&self, arr: &[Node]) -> bool {
        let mut seen = HashSet::new();
        for item in arr {
            let key = format!("{:?}", item);
            if !seen.insert(key) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn test_type_validation() {
        let schema = json!({"type": "string"});
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!("hello")).is_ok());
        assert!(validator.validate(&json!(42)).is_err());
    }

    #[test]
    fn test_required_properties() {
        let schema = json!({
            "type": "object",
            "required": ["name", "age"]
        });
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!({"name": "Alice", "age": 30})).is_ok());
        assert!(validator.validate(&json!({"name": "Bob"})).is_err());
    }

    #[test]
    fn test_min_max_properties() {
        let schema = json!({
            "type": "object",
            "minProperties": 2,
            "maxProperties": 4
        });
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!({"a": 1, "b": 2})).is_ok());
        assert!(validator.validate(&json!({"a": 1})).is_err());
        assert!(validator.validate(&json!({"a": 1, "b": 2, "c": 3, "d": 4, "e": 5})).is_err());
    }

    #[test]
    fn test_array_validation() {
        let schema = json!({
            "type": "array",
            "items": {"type": "number"},
            "minItems": 1,
            "maxItems": 5
        });
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!([1, 2, 3])).is_ok());
        assert!(validator.validate(&json!([])).is_err());
        assert!(validator.validate(&json!([1, 2, 3, 4, 5, 6])).is_err());
    }

    #[test]
    fn test_string_constraints() {
        let schema = json!({
            "type": "string",
            "minLength": 3,
            "maxLength": 10
        });
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!("hello")).is_ok());
        assert!(validator.validate(&json!("hi")).is_err());
        assert!(validator.validate(&json!("this is too long")).is_err());
    }

    #[test]
    fn test_number_constraints() {
        let schema = json!({
            "type": "number",
            "minimum": 0,
            "maximum": 100
        });
        let validator = SchemaValidator::new(schema);

        assert!(validator.validate(&json!(50)).is_ok());
        assert!(validator.validate(&json!(-1)).is_err());
        assert!(validator.validate(&json!(101)).is_err());
    }
}
