//! Embedded utilities module providing common patterns and helpers for embedded systems
//!
//! This module contains helper functions and types specifically designed for embedded
//! systems working with JSON, including sensor data formatting, configuration management,
//! and memory-efficient JSON construction.

use crate::nodes::node::{Node, Numeric};

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    string::{String, ToString},
    vec::Vec,
};

/// Builder for constructing JSON objects with a fluent API
///
/// This builder is optimized for embedded systems, allowing incremental
/// construction of JSON without temporary allocations.
///
/// # Examples
/// ```
/// use json_lib::embedded::ObjectBuilder;
///
/// let obj = ObjectBuilder::new()
///     .add_str("device", "sensor_01")
///     .add_i32("value", 42)
///     .add_f64("temperature", 23.5)
///     .build();
/// ```
pub struct ObjectBuilder {
    map: HashMap<String, Node>,
}

impl ObjectBuilder {
    /// Creates a new empty object builder
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Creates a builder with pre-allocated capacity
    ///
    /// Use this when you know how many fields you'll add to avoid reallocations
    #[cfg(feature = "std")]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Creates a builder with pre-allocated capacity (no-std version)
    #[cfg(not(feature = "std"))]
    pub fn with_capacity(_capacity: usize) -> Self {
        // BTreeMap doesn't support with_capacity
        Self::new()
    }

    /// Adds a string field
    pub fn add_str(mut self, key: &str, value: &str) -> Self {
        self.map
            .insert(key.to_string(), Node::Str(value.to_string()));
        self
    }

    /// Adds an i32 field
    pub fn add_i32(mut self, key: &str, value: i32) -> Self {
        self.map
            .insert(key.to_string(), Node::Number(Numeric::Int32(value)));
        self
    }

    /// Adds an i64 field
    pub fn add_i64(mut self, key: &str, value: i64) -> Self {
        self.map
            .insert(key.to_string(), Node::Number(Numeric::Integer(value)));
        self
    }

    /// Adds a u32 field
    pub fn add_u32(mut self, key: &str, value: u32) -> Self {
        self.map
            .insert(key.to_string(), Node::Number(Numeric::UInt32(value)));
        self
    }

    /// Adds a u64 field
    pub fn add_u64(mut self, key: &str, value: u64) -> Self {
        self.map
            .insert(key.to_string(), Node::Number(Numeric::UInteger(value)));
        self
    }

    /// Adds an f64 field
    pub fn add_f64(mut self, key: &str, value: f64) -> Self {
        self.map
            .insert(key.to_string(), Node::Number(Numeric::Float(value)));
        self
    }

    /// Adds a boolean field
    pub fn add_bool(mut self, key: &str, value: bool) -> Self {
        self.map.insert(key.to_string(), Node::Boolean(value));
        self
    }

    /// Adds a null field
    pub fn add_null(mut self, key: &str) -> Self {
        self.map.insert(key.to_string(), Node::None);
        self
    }

    /// Adds a pre-constructed Node
    pub fn add_node(mut self, key: &str, node: Node) -> Self {
        self.map.insert(key.to_string(), node);
        self
    }

    /// Adds an array field
    pub fn add_array(mut self, key: &str, array: Vec<Node>) -> Self {
        self.map.insert(key.to_string(), Node::Array(array));
        self
    }

    /// Adds an object field
    pub fn add_object(mut self, key: &str, object: HashMap<String, Node>) -> Self {
        self.map.insert(key.to_string(), Node::Object(object));
        self
    }

    /// Builds the final Node::Object
    pub fn build(self) -> Node {
        Node::Object(self.map)
    }
}

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing JSON arrays with a fluent API
///
/// # Examples
/// ```
/// use json_lib::embedded::ArrayBuilder;
///
/// let arr = ArrayBuilder::new()
///     .add_i32(1)
///     .add_i32(2)
///     .add_i32(3)
///     .build();
/// ```
pub struct ArrayBuilder {
    vec: Vec<Node>,
}

impl ArrayBuilder {
    /// Creates a new empty array builder
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Creates a builder with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Adds a string element
    pub fn add_str(mut self, value: &str) -> Self {
        self.vec.push(Node::Str(value.to_string()));
        self
    }

    /// Adds an i32 element
    pub fn add_i32(mut self, value: i32) -> Self {
        self.vec.push(Node::Number(Numeric::Int32(value)));
        self
    }

    /// Adds an i64 element
    pub fn add_i64(mut self, value: i64) -> Self {
        self.vec.push(Node::Number(Numeric::Integer(value)));
        self
    }

    /// Adds a u32 element
    pub fn add_u32(mut self, value: u32) -> Self {
        self.vec.push(Node::Number(Numeric::UInt32(value)));
        self
    }

    /// Adds a u64 element
    pub fn add_u64(mut self, value: u64) -> Self {
        self.vec.push(Node::Number(Numeric::UInteger(value)));
        self
    }

    /// Adds an f64 element
    pub fn add_f64(mut self, value: f64) -> Self {
        self.vec.push(Node::Number(Numeric::Float(value)));
        self
    }

    /// Adds a boolean element
    pub fn add_bool(mut self, value: bool) -> Self {
        self.vec.push(Node::Boolean(value));
        self
    }

    /// Adds a null element
    pub fn add_null(mut self) -> Self {
        self.vec.push(Node::None);
        self
    }

    /// Adds a pre-constructed Node
    pub fn add_node(mut self, node: Node) -> Self {
        self.vec.push(node);
        self
    }

    /// Builds the final Node::Array
    pub fn build(self) -> Node {
        Node::Array(self.vec)
    }
}

impl Default for ArrayBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for creating sensor data JSON structures
pub mod sensor {
    use super::*;

    /// Creates a simple sensor reading with timestamp
    ///
    /// # Examples
    /// ```
    /// use json_lib::embedded::sensor;
    ///
    /// let reading = sensor::simple_reading("temp_01", 23.5, 1699876543);
    /// ```
    pub fn simple_reading(device_id: &str, value: f64, timestamp: i64) -> Node {
        ObjectBuilder::new()
            .add_str("device", device_id)
            .add_f64("value", value)
            .add_i64("timestamp", timestamp)
            .build()
    }

    /// Creates a sensor reading with multiple values
    ///
    /// # Examples
    /// ```
    /// use json_lib::embedded::sensor;
    /// use json_lib::Node;
    ///
    /// let values = vec![
    ///     ("temperature", 23.5),
    ///     ("humidity", 45.2),
    /// ];
    /// let reading = sensor::multi_reading("env_01", &values, 1699876543);
    /// ```
    pub fn multi_reading(device_id: &str, values: &[(&str, f64)], timestamp: i64) -> Node {
        let mut builder = ObjectBuilder::new()
            .add_str("device", device_id)
            .add_i64("timestamp", timestamp);

        let mut readings = HashMap::new();
        for (key, value) in values {
            readings.insert(key.to_string(), Node::Number(Numeric::Float(*value)));
        }

        builder = builder.add_object("readings", readings);
        builder.build()
    }

    /// Creates a batch of sensor readings
    ///
    /// Useful for sending multiple readings at once to conserve bandwidth
    pub fn batch_readings(device_id: &str, readings: Vec<Node>) -> Node {
        ObjectBuilder::new()
            .add_str("device", device_id)
            .add_array("readings", readings)
            .build()
    }
}

/// Helper functions for configuration management
pub mod config {
    use super::*;

    /// Creates a simple configuration object
    ///
    /// # Examples
    /// ```
    /// use json_lib::embedded::config;
    ///
    /// let cfg = config::simple()
    ///     .add_str("wifi_ssid", "MyNetwork")
    ///     .add_i32("sample_rate", 1000)
    ///     .add_bool("debug", false)
    ///     .build();
    /// ```
    pub fn simple() -> ObjectBuilder {
        ObjectBuilder::new()
    }

    /// Extracts a string configuration value safely
    pub fn get_string<'a>(node: &'a Node, key: &str) -> Option<&'a str> {
        node.get(key).and_then(|n| n.as_str())
    }

    /// Extracts an integer configuration value safely
    pub fn get_i32(node: &Node, key: &str) -> Option<i32> {
        node.get(key).and_then(|n| {
            if let Node::Number(Numeric::Int32(i)) = n {
                Some(*i)
            } else {
                None
            }
        })
    }

    /// Extracts a boolean configuration value safely
    pub fn get_bool(node: &Node, key: &str) -> Option<bool> {
        node.get(key).and_then(|n| n.as_bool())
    }

    /// Extracts a float configuration value safely
    pub fn get_f64(node: &Node, key: &str) -> Option<f64> {
        node.get(key).and_then(|n| {
            if let Node::Number(Numeric::Float(f)) = n {
                Some(*f)
            } else {
                None
            }
        })
    }
}

/// Memory usage estimation helpers
pub mod memory {
    use super::*;
    use core::mem::size_of;

    /// Estimates the memory usage of a Node in bytes
    ///
    /// This is a rough estimate that includes the node itself and its immediate contents
    pub fn estimate_node_size(node: &Node) -> usize {
        match node {
            Node::Boolean(_) => size_of::<Node>(),
            Node::Number(_) => size_of::<Node>(),
            Node::None => size_of::<Node>(),
            Node::Str(s) => size_of::<Node>() + s.capacity(),
            Node::Array(arr) => {
                size_of::<Node>()
                    + size_of::<Node>() * arr.capacity()
                    + arr
                        .iter()
                        .map(|n| estimate_node_size(n) - size_of::<Node>())
                        .sum::<usize>()
            }
            Node::Object(map) => {
                size_of::<Node>()
                    + estimate_map_overhead(map.len())
                    + map
                        .iter()
                        .map(|(k, v)| k.capacity() + estimate_node_size(v) - size_of::<Node>())
                        .sum::<usize>()
            }
        }
    }

    /// Estimates HashMap/BTreeMap overhead
    fn estimate_map_overhead(len: usize) -> usize {
        // Rough estimate: 48 bytes base + 80 bytes per entry
        48 + (len * 80)
    }

    /// Returns the size of the Node enum
    pub fn node_size() -> usize {
        size_of::<Node>()
    }

    /// Returns the size of the Numeric enum
    pub fn numeric_size() -> usize {
        size_of::<Numeric>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_builder() {
        let obj = ObjectBuilder::new()
            .add_str("name", "test")
            .add_i32("value", 42)
            .add_bool("active", true)
            .build();

        assert!(matches!(obj, Node::Object(_)));
        if let Node::Object(map) = obj {
            assert_eq!(map.len(), 3);
            assert!(map.contains_key("name"));
            assert!(map.contains_key("value"));
            assert!(map.contains_key("active"));
        }
    }

    #[test]
    fn test_array_builder() {
        let arr = ArrayBuilder::new().add_i32(1).add_i32(2).add_i32(3).build();

        assert!(matches!(arr, Node::Array(_)));
        if let Node::Array(vec) = arr {
            assert_eq!(vec.len(), 3);
        }
    }

    #[test]
    fn test_sensor_simple_reading() {
        let reading = sensor::simple_reading("sensor_01", 23.5, 1699876543);

        if let Node::Object(map) = reading {
            assert_eq!(map.len(), 3);
            assert!(map.contains_key("device"));
            assert!(map.contains_key("value"));
            assert!(map.contains_key("timestamp"));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_config_helpers() {
        let cfg = config::simple()
            .add_str("name", "test_device")
            .add_i32("rate", 100)
            .add_bool("enabled", true)
            .build();

        assert_eq!(config::get_string(&cfg, "name"), Some("test_device"));
        assert_eq!(config::get_i32(&cfg, "rate"), Some(100));
        assert_eq!(config::get_bool(&cfg, "enabled"), Some(true));
        assert_eq!(config::get_string(&cfg, "missing"), None);
    }

    #[test]
    fn test_memory_estimation() {
        let node = Node::Str("test".to_string());
        let size = memory::estimate_node_size(&node);
        assert!(size > memory::node_size());

        let arr = Node::Array(vec![Node::None, Node::None]);
        let arr_size = memory::estimate_node_size(&arr);
        assert!(arr_size > memory::node_size());
    }
}
