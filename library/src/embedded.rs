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

    // ObjectBuilder — individual add_* methods
    #[test]
    fn test_object_builder_empty() {
        let obj = ObjectBuilder::new().build();
        if let Node::Object(map) = obj {
            assert_eq!(map.len(), 0);
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_object_builder_default() {
        let obj = ObjectBuilder::default().build();
        assert!(matches!(obj, Node::Object(_)));
    }

    #[test]
    fn test_object_builder_add_i64() {
        let obj = ObjectBuilder::new().add_i64("big", i64::MAX).build();
        if let Node::Object(map) = &obj {
            assert_eq!(
                map.get("big"),
                Some(&Node::Number(Numeric::Integer(i64::MAX)))
            );
        }
    }

    #[test]
    fn test_object_builder_add_u32() {
        let obj = ObjectBuilder::new().add_u32("u", u32::MAX).build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("u"), Some(&Node::Number(Numeric::UInt32(u32::MAX))));
        }
    }

    #[test]
    fn test_object_builder_add_u64() {
        let obj = ObjectBuilder::new().add_u64("u64", u64::MAX).build();
        if let Node::Object(map) = &obj {
            assert_eq!(
                map.get("u64"),
                Some(&Node::Number(Numeric::UInteger(u64::MAX)))
            );
        }
    }

    #[test]
    fn test_object_builder_add_f64() {
        let obj = ObjectBuilder::new().add_f64("pi", 3.14).build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("pi"), Some(&Node::Number(Numeric::Float(3.14))));
        }
    }

    #[test]
    fn test_object_builder_add_null() {
        let obj = ObjectBuilder::new().add_null("nothing").build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("nothing"), Some(&Node::None));
        }
    }

    #[test]
    fn test_object_builder_add_node() {
        let inner = Node::Boolean(true);
        let obj = ObjectBuilder::new().add_node("flag", inner).build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("flag"), Some(&Node::Boolean(true)));
        }
    }

    #[test]
    fn test_object_builder_add_array() {
        let arr = vec![
            Node::Number(Numeric::Int32(1)),
            Node::Number(Numeric::Int32(2)),
        ];
        let obj = ObjectBuilder::new().add_array("nums", arr).build();
        if let Node::Object(map) = &obj {
            assert!(matches!(map.get("nums"), Some(Node::Array(_))));
        }
    }

    #[test]
    fn test_object_builder_add_object() {
        let mut inner = HashMap::new();
        inner.insert("x".to_string(), Node::Boolean(false));
        let obj = ObjectBuilder::new().add_object("sub", inner).build();
        if let Node::Object(map) = &obj {
            assert!(matches!(map.get("sub"), Some(Node::Object(_))));
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_object_builder_with_capacity() {
        let obj = ObjectBuilder::with_capacity(4).add_str("k", "v").build();
        if let Node::Object(map) = obj {
            assert_eq!(map.len(), 1);
        }
    }

    #[test]
    fn test_object_builder_bool_false() {
        let obj = ObjectBuilder::new().add_bool("off", false).build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("off"), Some(&Node::Boolean(false)));
        }
    }

    #[test]
    fn test_object_builder_bool_true() {
        let obj = ObjectBuilder::new().add_bool("on", true).build();
        if let Node::Object(map) = &obj {
            assert_eq!(map.get("on"), Some(&Node::Boolean(true)));
        }
    }

    #[test]
    fn test_object_builder_overwrite_key() {
        let obj = ObjectBuilder::new().add_i32("x", 1).add_i32("x", 2).build();
        if let Node::Object(map) = &obj {
            // Last insert wins
            assert_eq!(map.get("x"), Some(&Node::Number(Numeric::Int32(2))));
        }
    }

    // ArrayBuilder — individual add_* methods
    #[test]
    fn test_array_builder_empty() {
        let arr = ArrayBuilder::new().build();
        if let Node::Array(v) = arr {
            assert_eq!(v.len(), 0);
        }
    }

    #[test]
    fn test_array_builder_default() {
        let arr = ArrayBuilder::default().build();
        assert!(matches!(arr, Node::Array(_)));
    }

    #[test]
    fn test_array_builder_with_capacity() {
        let arr = ArrayBuilder::with_capacity(8).add_i32(1).build();
        if let Node::Array(v) = arr {
            assert_eq!(v.len(), 1);
        }
    }

    #[test]
    fn test_array_builder_add_str() {
        let arr = ArrayBuilder::new().add_str("hello").build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Str("hello".to_string()));
        }
    }

    #[test]
    fn test_array_builder_add_i64() {
        let arr = ArrayBuilder::new().add_i64(i64::MIN).build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Number(Numeric::Integer(i64::MIN)));
        }
    }

    #[test]
    fn test_array_builder_add_u32() {
        let arr = ArrayBuilder::new().add_u32(u32::MAX).build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Number(Numeric::UInt32(u32::MAX)));
        }
    }

    #[test]
    fn test_array_builder_add_u64() {
        let arr = ArrayBuilder::new().add_u64(u64::MAX).build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Number(Numeric::UInteger(u64::MAX)));
        }
    }

    #[test]
    fn test_array_builder_add_f64() {
        let arr = ArrayBuilder::new().add_f64(2.718).build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Number(Numeric::Float(2.718)));
        }
    }

    #[test]
    fn test_array_builder_add_bool() {
        let arr = ArrayBuilder::new().add_bool(true).add_bool(false).build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Boolean(true));
            assert_eq!(v[1], Node::Boolean(false));
        }
    }

    #[test]
    fn test_array_builder_add_null() {
        let arr = ArrayBuilder::new().add_null().build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::None);
        }
    }

    #[test]
    fn test_array_builder_add_node() {
        let arr = ArrayBuilder::new()
            .add_node(Node::Number(Numeric::Byte(255)))
            .build();
        if let Node::Array(v) = arr {
            assert_eq!(v[0], Node::Number(Numeric::Byte(255)));
        }
    }

    #[test]
    fn test_array_builder_mixed_types() {
        let arr = ArrayBuilder::new()
            .add_null()
            .add_bool(true)
            .add_i32(-1)
            .add_str("x")
            .build();
        if let Node::Array(v) = arr {
            assert_eq!(v.len(), 4);
            assert_eq!(v[0], Node::None);
            assert_eq!(v[1], Node::Boolean(true));
            assert_eq!(v[2], Node::Number(Numeric::Int32(-1)));
            assert_eq!(v[3], Node::Str("x".to_string()));
        }
    }

    // sensor module
    #[test]
    fn test_sensor_simple_reading_values() {
        let reading = sensor::simple_reading("dev_01", 42.0, 1000);
        if let Node::Object(map) = &reading {
            assert_eq!(map.get("device"), Some(&Node::Str("dev_01".to_string())));
            assert_eq!(map.get("value"), Some(&Node::Number(Numeric::Float(42.0))));
            assert_eq!(
                map.get("timestamp"),
                Some(&Node::Number(Numeric::Integer(1000)))
            );
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_sensor_multi_reading() {
        let values = vec![("temp", 25.0), ("hum", 60.0)];
        let reading = sensor::multi_reading("dev_02", &values, 2000);
        if let Node::Object(map) = &reading {
            assert_eq!(map.get("device"), Some(&Node::Str("dev_02".to_string())));
            assert_eq!(
                map.get("timestamp"),
                Some(&Node::Number(Numeric::Integer(2000)))
            );
            assert!(matches!(map.get("readings"), Some(Node::Object(_))));
            if let Some(Node::Object(r)) = map.get("readings") {
                assert!(r.contains_key("temp"));
                assert!(r.contains_key("hum"));
            }
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_sensor_multi_reading_empty_values() {
        let reading = sensor::multi_reading("dev_03", &[], 0);
        if let Node::Object(map) = &reading {
            assert!(matches!(map.get("readings"), Some(Node::Object(_))));
        }
    }

    #[test]
    fn test_sensor_batch_readings() {
        let r1 = sensor::simple_reading("s1", 1.0, 100);
        let r2 = sensor::simple_reading("s2", 2.0, 200);
        let batch = sensor::batch_readings("gateway", vec![r1, r2]);
        if let Node::Object(map) = &batch {
            assert_eq!(map.get("device"), Some(&Node::Str("gateway".to_string())));
            if let Some(Node::Array(arr)) = map.get("readings") {
                assert_eq!(arr.len(), 2);
            } else {
                panic!("Expected array");
            }
        }
    }

    #[test]
    fn test_sensor_batch_readings_empty() {
        let batch = sensor::batch_readings("gw", vec![]);
        if let Node::Object(map) = &batch {
            if let Some(Node::Array(arr)) = map.get("readings") {
                assert_eq!(arr.len(), 0);
            }
        }
    }

    // config module
    #[test]
    fn test_config_get_string_missing_key() {
        let cfg = ObjectBuilder::new().add_str("a", "1").build();
        assert_eq!(config::get_string(&cfg, "missing"), None);
    }

    #[test]
    fn test_config_get_i32_missing_key() {
        let cfg = ObjectBuilder::new().add_i32("n", 5).build();
        assert_eq!(config::get_i32(&cfg, "missing"), None);
    }

    #[test]
    fn test_config_get_i32_wrong_type() {
        let cfg = ObjectBuilder::new().add_str("n", "not_a_number").build();
        assert_eq!(config::get_i32(&cfg, "n"), None);
    }

    #[test]
    fn test_config_get_bool_missing_key() {
        let cfg = ObjectBuilder::new().add_bool("flag", true).build();
        assert_eq!(config::get_bool(&cfg, "missing"), None);
    }

    #[test]
    fn test_config_get_bool_false() {
        let cfg = ObjectBuilder::new().add_bool("flag", false).build();
        assert_eq!(config::get_bool(&cfg, "flag"), Some(false));
    }

    #[test]
    fn test_config_get_f64() {
        let cfg = ObjectBuilder::new().add_f64("rate", 9.81).build();
        assert_eq!(config::get_f64(&cfg, "rate"), Some(9.81));
    }

    #[test]
    fn test_config_get_f64_missing_key() {
        let cfg = ObjectBuilder::new().add_f64("x", 1.0).build();
        assert_eq!(config::get_f64(&cfg, "missing"), None);
    }

    #[test]
    fn test_config_get_f64_wrong_type() {
        let cfg = ObjectBuilder::new().add_i32("n", 1).build();
        assert_eq!(config::get_f64(&cfg, "n"), None);
    }

    #[test]
    fn test_config_simple_returns_builder() {
        let cfg = config::simple()
            .add_str("ssid", "MyNet")
            .add_bool("dhcp", true)
            .build();
        assert_eq!(config::get_string(&cfg, "ssid"), Some("MyNet"));
        assert_eq!(config::get_bool(&cfg, "dhcp"), Some(true));
    }

    // memory module
    #[test]
    fn test_memory_node_size_nonzero() {
        assert!(memory::node_size() > 0);
    }

    #[test]
    fn test_memory_numeric_size_nonzero() {
        assert!(memory::numeric_size() > 0);
    }

    #[test]
    fn test_memory_null_size_equals_node_size() {
        assert_eq!(memory::estimate_node_size(&Node::None), memory::node_size());
    }

    #[test]
    fn test_memory_boolean_size_equals_node_size() {
        assert_eq!(
            memory::estimate_node_size(&Node::Boolean(true)),
            memory::node_size()
        );
    }

    #[test]
    fn test_memory_number_size_equals_node_size() {
        assert_eq!(
            memory::estimate_node_size(&Node::Number(Numeric::Integer(0))),
            memory::node_size()
        );
    }

    #[test]
    fn test_memory_string_larger_than_node_size() {
        let s = Node::Str("hello".to_string());
        assert!(memory::estimate_node_size(&s) >= memory::node_size());
    }

    #[test]
    fn test_memory_empty_array_size() {
        let arr = Node::Array(vec![]);
        let size = memory::estimate_node_size(&arr);
        assert!(size >= memory::node_size());
    }

    #[test]
    fn test_memory_object_larger_than_node() {
        let mut map = HashMap::new();
        map.insert("k".to_string(), Node::None);
        let obj = Node::Object(map);
        assert!(memory::estimate_node_size(&obj) > memory::node_size());
    }
}
