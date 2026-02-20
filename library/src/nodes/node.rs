#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap as HashMap, string::String, vec::Vec};

// Use smallvec for small arrays to reduce heap allocations
use smallvec::SmallVec;

#[cfg(feature = "std")]
use core::ops::{Index, IndexMut};

#[cfg(not(feature = "std"))]
use core::ops::{Index, IndexMut};

use core::fmt;
use core::mem;
use core::str::FromStr;

/// Represents different numeric types that can be stored in a JSON node
#[derive(Clone, Debug, PartialEq)]
pub enum Numeric {
    Integer(i64),  // 64-bit signed integer
    Float(f64),    // 64-bit floating point
    UInteger(u64), // 64-bit unsigned integer
    Byte(u8),      // 8-bit unsigned integer
    Int32(i32),    // 32-bit signed integer
    UInt32(u32),   // 32-bit unsigned integer
    Int16(i16),    // 16-bit signed integer
    UInt16(u16),   // 16-bit unsigned integer
    Int8(i8),      // 8-bit signed integer
}

/// A node in the JSON data structure that can represent different types of values.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// Represents a boolean value (true/false)
    Boolean(bool),
    /// Represents a numeric value (various integer and float types)
    Number(Numeric),
    /// Represents a string value
    Str(String),
    /// Represents an array of other nodes
    Array(Vec<Node>), // Internally, we may use SmallVec for construction
    /// Represents an object/map of string keys to node values
    Object(HashMap<String, Node>),
    /// Represents a null value or uninitialized node
    None,
}

impl Node {
        /// Creates a Node::Array from an iterator, using SmallVec for small arrays
        pub fn from_iter<I: IntoIterator<Item = Node>>(iter: I) -> Self {
            let mut small: SmallVec<[Node; 8]> = SmallVec::new();
            for item in iter {
                small.push(item);
            }
            Node::Array(small.into_vec())
        }

        /// Creates a Node::Array from a slice, using SmallVec for small arrays.
        pub fn from_slice(slice: &[Node]) -> Self {
            use core::mem::MaybeUninit;
            let mut small: SmallVec<[Node; 8]> = SmallVec::new();
            if slice.len() <= 8 {
                // For small slices, use stack allocation and clone
                for item in slice {
                    small.push(item.clone());
                }
                Node::Array(small.into_vec())
            } else {
                // For large slices, use Vec::from with clone
                Node::Array(slice.to_vec())
            }
        }

        /// Creates a Node::Array from a Vec<Node> without cloning (zero-copy).
        pub fn from_vec(vec: Vec<Node>) -> Self {
            Node::Array(vec)
        }

    /// Safely gets a value from an object by key without panicking
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Some(&Node)` if the node is an object and the key exists
    /// * `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut obj = HashMap::new();
    /// obj.insert("name".to_string(), Node::Str("Alice".to_string()));
    /// let node = Node::Object(obj);
    ///
    /// assert!(node.get("name").is_some());
    /// assert!(node.get("missing").is_none());
    /// ```
    pub fn get(&self, key: &str) -> Option<&Node> {
        match self {
            Node::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Safely gets a mutable reference to a value from an object by key
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Some(&mut Node)` if the node is an object and the key exists
    /// * `None` otherwise
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Node> {
        match self {
            Node::Object(map) => map.get_mut(key),
            _ => None,
        }
    }

    /// Safely gets a value from an array by index without panicking
    ///
    /// # Arguments
    /// * `index` - The array index
    ///
    /// # Returns
    /// * `Some(&Node)` if the node is an array and the index is valid
    /// * `None` otherwise
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::Array(vec![
    ///     Node::Str("first".to_string()),
    ///     Node::Str("second".to_string()),
    /// ]);
    ///
    /// assert!(node.at(0).is_some());
    /// assert!(node.at(10).is_none());
    /// ```
    pub fn at(&self, index: usize) -> Option<&Node> {
        match self {
            Node::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    /// Safely gets a mutable reference to a value from an array by index
    ///
    /// # Arguments
    /// * `index` - The array index
    ///
    /// # Returns
    /// * `Some(&mut Node)` if the node is an array and the index is valid
    /// * `None` otherwise
    pub fn at_mut(&mut self, index: usize) -> Option<&mut Node> {
        match self {
            Node::Array(arr) => arr.get_mut(index),
            _ => None,
        }
    }

    /// Returns true if this node is an object
    #[inline]
    pub fn is_object(&self) -> bool {
        matches!(self, Node::Object(_))
    }

    /// Returns true if this node is an array
    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, Node::Array(_))
    }

    /// Returns true if this node is a string
    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Node::Str(_))
    }

    /// Returns true if this node is a number
    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, Node::Number(_))
    }

    /// Returns true if this node is a boolean
    #[inline]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Node::Boolean(_))
    }

    /// Returns true if this node is None/null
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Node::None)
    }

    /// Returns an iterator over the keys of an object
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a".to_string(), Node::from(1));
    /// map.insert("b".to_string(), Node::from(2));
    /// let node = Node::Object(map);
    ///
    /// let keys: Vec<&str> = node.keys().unwrap().collect();
    /// assert_eq!(keys.len(), 2);
    /// ```
    #[inline]
    pub fn keys(&self) -> Option<impl Iterator<Item = &str>> {
        match self {
            Node::Object(map) => Some(map.keys().map(|s| s.as_str())),
            _ => None,
        }
    }

    /// Returns an iterator over the values if this is an object
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a".to_string(), Node::from(1));
    /// let node = Node::Object(map);
    /// let values: Vec<_> = node.object_values().unwrap().collect();
    /// assert_eq!(values.len(), 1);
    /// ```
    #[inline]
    pub fn object_values(&self) -> Option<impl Iterator<Item = &Node>> {
        match self {
            Node::Object(map) => Some(map.values()),
            _ => None,
        }
    }

    /// Returns a mutable iterator over object values
    #[inline]
    pub fn object_values_mut(&mut self) -> Option<impl Iterator<Item = &mut Node>> {
        match self {
            Node::Object(map) => Some(map.values_mut()),
            _ => None,
        }
    }

    /// Returns an iterator over array elements
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let arr = Node::Array(vec![Node::from(1), Node::from(2)]);
    /// let values: Vec<_> = arr.array_iter().unwrap().collect();
    /// assert_eq!(values.len(), 2);
    /// ```
    #[inline]
    pub fn array_iter(&self) -> Option<impl Iterator<Item = &Node>> {
        match self {
            Node::Array(vec) => Some(vec.iter()),
            _ => None,
        }
    }

    /// Returns a mutable iterator over array elements
    #[inline]
    pub fn array_iter_mut(&mut self) -> Option<impl Iterator<Item = &mut Node>> {
        match self {
            Node::Array(vec) => Some(vec.iter_mut()),
            _ => None,
        }
    }

    /// Returns the string value if this node is a Str, None otherwise
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Node::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the boolean value if this node is a Boolean, None otherwise
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Node::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the number value if this node is a Number, None otherwise
    #[inline]
    pub fn as_number(&self) -> Option<&Numeric> {
        match self {
            Node::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Converts the node to i64 if it's a numeric type
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::from(42);
    /// assert_eq!(node.as_i64(), Some(42));
    ///
    /// let node = Node::from(42.7);
    /// assert_eq!(node.as_i64(), Some(42));
    /// ```
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Node::Number(Numeric::Integer(n)) => Some(*n),
            Node::Number(Numeric::Int32(n)) => Some(*n as i64),
            Node::Number(Numeric::Int16(n)) => Some(*n as i64),
            Node::Number(Numeric::Int8(n)) => Some(*n as i64),
            Node::Number(Numeric::UInteger(n)) => Some(*n as i64),
            Node::Number(Numeric::UInt32(n)) => Some(*n as i64),
            Node::Number(Numeric::UInt16(n)) => Some(*n as i64),
            Node::Number(Numeric::Byte(n)) => Some(*n as i64),
            Node::Number(Numeric::Float(f)) => Some(*f as i64),
            _ => None,
        }
    }

    /// Converts the node to f64 if it's a numeric type
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::from(42.5);
    /// assert_eq!(node.as_f64(), Some(42.5));
    ///
    /// let node = Node::from(42);
    /// assert_eq!(node.as_f64(), Some(42.0));
    /// ```
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Node::Number(Numeric::Float(f)) => Some(*f),
            Node::Number(Numeric::Integer(n)) => Some(*n as f64),
            Node::Number(Numeric::Int32(n)) => Some(*n as f64),
            Node::Number(Numeric::Int16(n)) => Some(*n as f64),
            Node::Number(Numeric::Int8(n)) => Some(*n as f64),
            Node::Number(Numeric::UInteger(n)) => Some(*n as f64),
            Node::Number(Numeric::UInt32(n)) => Some(*n as f64),
            Node::Number(Numeric::UInt16(n)) => Some(*n as f64),
            Node::Number(Numeric::Byte(n)) => Some(*n as f64),
            _ => None,
        }
    }

    /// Converts the node to u64 if it's a numeric type
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::from(42u64);
    /// assert_eq!(node.as_u64(), Some(42));
    /// ```
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Node::Number(Numeric::UInteger(n)) => Some(*n),
            Node::Number(Numeric::UInt32(n)) => Some(*n as u64),
            Node::Number(Numeric::UInt16(n)) => Some(*n as u64),
            Node::Number(Numeric::Byte(n)) => Some(*n as u64),
            Node::Number(Numeric::Integer(n)) if *n >= 0 => Some(*n as u64),
            Node::Number(Numeric::Int32(n)) if *n >= 0 => Some(*n as u64),
            Node::Number(Numeric::Int16(n)) if *n >= 0 => Some(*n as u64),
            Node::Number(Numeric::Int8(n)) if *n >= 0 => Some(*n as u64),
            Node::Number(Numeric::Float(f)) if *f >= 0.0 => Some(*f as u64),
            _ => None,
        }
    }

    /// Returns the array reference if this node is an Array, None otherwise
    #[inline]
    pub fn as_array(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns a mutable array reference if this node is an Array, None otherwise
    #[inline]
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the object reference if this node is an Object, None otherwise
    #[inline]
    pub fn as_object(&self) -> Option<&HashMap<String, Node>> {
        match self {
            Node::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Returns a mutable object reference if this node is an Object, None otherwise
    #[inline]
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Node>> {
        match self {
            Node::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Consumes the node and returns the string if this is a Str variant
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::from("hello");
    /// assert_eq!(node.into_string(), Some("hello".to_string()));
    ///
    /// let node = Node::from(42);
    /// assert_eq!(node.into_string(), None);
    /// ```
    #[inline]
    pub fn into_string(self) -> Option<String> {
        match self {
            Node::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Consumes the node and returns the array if this is an Array variant
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::Array(vec![Node::from(1), Node::from(2)]);
    /// assert_eq!(node.into_array().map(|v| v.len()), Some(2));
    /// ```
    #[inline]
    pub fn into_array(self) -> Option<Vec<Node>> {
        match self {
            Node::Array(vec) => Some(vec),
            _ => None,
        }
    }

    /// Consumes the node and returns the object if this is an Object variant
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("key".to_string(), Node::from(42));
    /// let node = Node::Object(map);
    /// assert_eq!(node.into_object().map(|m| m.len()), Some(1));
    /// ```
    #[inline]
    pub fn into_object(self) -> Option<HashMap<String, Node>> {
        match self {
            Node::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Consumes the node and returns the number if this is a Number variant
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let node = Node::from(42);
    /// assert!(node.into_number().is_some());
    /// ```
    #[inline]
    pub fn into_number(self) -> Option<Numeric> {
        match self {
            Node::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Deep merges another node into this node
    ///
    /// For objects, recursively merges keys. If both have the same key:
    /// - If both values are objects, recursively merge them
    /// - Otherwise, the value from `other` overwrites the value in `self`
    ///
    /// For arrays, replaces the entire array with `other`
    /// For all other types, replaces `self` with `other`
    ///
    /// # Arguments
    /// * `other` - The node to merge into this node
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut obj1 = HashMap::new();
    /// obj1.insert("a".to_string(), Node::Str("old".to_string()));
    /// obj1.insert("b".to_string(), Node::Str("keep".to_string()));
    /// let mut node1 = Node::Object(obj1);
    ///
    /// let mut obj2 = HashMap::new();
    /// obj2.insert("a".to_string(), Node::Str("new".to_string()));
    /// obj2.insert("c".to_string(), Node::Str("added".to_string()));
    /// let node2 = Node::Object(obj2);
    ///
    /// node1.merge(node2);
    /// // node1 now has: {"a": "new", "b": "keep", "c": "added"}
    /// ```
    pub fn merge(&mut self, other: Node) {
        self.merge_ref(&other);
    }

    /// Efficiently merges another node into this node by reference.
    /// For objects, recursively merges keys. If both have the same key:
    /// - If both values are objects, recursively merge them
    /// - Otherwise, replaces the value
    /// For non-objects, replaces self with other.
    pub fn merge_ref(&mut self, other: &Node) {
        match (self, other) {
            (Node::Object(self_map), Node::Object(other_map)) => {
                // Fast path: if self_map is empty, just clone all from other_map
                if self_map.is_empty() {
                    self_map.extend(other_map.iter().map(|(k, v)| (k.clone(), v.clone())));
                    return;
                }
                for (key, other_value) in other_map {
                    match self_map.entry(key.clone()) {
                        std::collections::hash_map::Entry::Occupied(mut entry) => {
                            if entry.get().is_object() && other_value.is_object() {
                                entry.get_mut().merge_ref(other_value);
                            } else {
                                // Only clone if value is different
                                if entry.get() != other_value {
                                    entry.insert(other_value.clone());
                                }
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(other_value.clone());
                        }
                    }
                }
            }
            (this, other) => {
                if this != other {
                    *this = other.clone();
                }
            }
        }
    }

    /// Returns the length of an array or object, None for other types
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let arr = Node::Array(vec![Node::from(1), Node::from(2), Node::from(3)]);
    /// assert_eq!(arr.len(), Some(3));
    ///
    /// let obj = Node::Object(std::collections::HashMap::new());
    /// assert_eq!(obj.len(), Some(0));
    ///
    /// let num = Node::from(42);
    /// assert_eq!(num.len(), None);
    /// ```
    #[inline]
    pub fn len(&self) -> Option<usize> {
        match self {
            Node::Array(arr) => Some(arr.len()),
            Node::Object(map) => Some(map.len()),
            _ => None,
        }
    }

    /// Returns true if this node is an empty array or object
    ///
    /// Returns false for non-collection types
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let arr = Node::Array(vec![]);
    /// assert!(arr.is_empty());
    ///
    /// let num = Node::from(42);
    /// assert!(!num.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Node::Array(arr) => arr.is_empty(),
            Node::Object(map) => map.is_empty(),
            _ => false,
        }
    }

    /// Takes the value out of the Node, leaving Node::None in its place
    ///
    /// This is useful when you want to move a value out of a Node while
    /// leaving something valid behind.
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("x".to_string(), Node::from(42));
    /// let mut node = Node::Object(map);
    ///
    /// let old_value = node.get_mut("x").unwrap().take();
    /// assert_eq!(old_value, Node::from(42));
    /// assert_eq!(node.get("x").unwrap(), &Node::None);
    /// ```
    pub fn take(&mut self) -> Node {
        mem::replace(self, Node::None)
    }

    /// Creates a new empty object Node
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let obj = Node::object();
    /// assert!(obj.is_object());
    /// assert_eq!(obj.len(), Some(0));
    /// ```
    pub fn object() -> Self {
        Node::Object(HashMap::new())
    }

    /// Creates a new empty array Node
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let arr = Node::array();
    /// assert!(arr.is_array());
    /// assert_eq!(arr.len(), Some(0));
    /// ```
    pub fn array() -> Self {
        Node::Array(Vec::new())
    }

    /// Creates a new null Node
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let null = Node::null();
    /// assert!(null.is_null());
    /// ```
    pub fn null() -> Self {
        Node::None
    }

    /// Inserts a key-value pair into an object
    ///
    /// Returns None if the node is not an object, or the old value if the key existed.
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    ///
    /// let mut obj = Node::object();
    /// obj.insert("name", "Alice");
    /// assert_eq!(obj["name"].as_str(), Some("Alice"));
    /// ```
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<Node>) -> Option<Node> {
        match self {
            Node::Object(map) => map.insert(key.into(), value.into()),
            _ => None,
        }
    }

    /// Gets a value using JSON Pointer notation (RFC 6901)
    ///
    /// # Examples
    /// ```
    /// use json_lib::{json, Node};
    ///
    /// let data = json!({
    ///     "user": {
    ///         "name": "Alice"
    ///     }
    /// });
    ///
    /// assert_eq!(data.pointer("/user/name").unwrap().as_str(), Some("Alice"));
    /// ```
    #[cfg(feature = "json-pointer")]
    pub fn pointer(&self, path: &str) -> Option<&Node> {
        crate::nodes::json_pointer::get(self, path)
    }

    /// Gets a mutable value using JSON Pointer notation (RFC 6901)
    ///
    /// # Examples
    /// ```
    /// use json_lib::{json, Node};
    ///
    /// let mut data = json!({"x": 10});
    /// if let Some(x) = data.pointer_mut("/x") {
    ///     *x = Node::from(20);
    /// }
    /// assert_eq!(data["x"].as_i64(), Some(20));
    /// ```
    #[cfg(feature = "json-pointer")]
    pub fn pointer_mut(&mut self, path: &str) -> Option<&mut Node> {
        crate::nodes::json_pointer::get_mut(self, path)
    }

    /// Converts the Node to a pretty-printed JSON string
    ///
    /// # Examples
    /// ```
    /// use json_lib::{json, Node};
    ///
    /// let data = json!({"name": "Alice", "age": 30});
    /// let pretty = data.to_string_pretty();
    /// assert!(pretty.contains("\n"));
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_pretty(&self) -> String {
        self.to_string_with_indent("  ")
    }

    /// Converts the Node to a pretty-printed JSON string with custom indentation
    ///
    /// # Examples
    /// ```
    /// use json_lib::{json, Node};
    ///
    /// let data = json!({"x": 1});
    /// let pretty = data.to_string_with_indent("\t");
    /// assert!(pretty.contains("\t"));
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_with_indent(&self, indent: &str) -> String {
        use crate::io::destinations::buffer::Buffer;
        use crate::stringify::pretty::stringify_pretty;
        
        let mut dest = Buffer::new();
        stringify_pretty(self, &mut dest, indent).unwrap_or_default();
        dest.to_string()
    }
}

/// Default implementation returns Node::None
impl Default for Node {
    fn default() -> Self {
        Node::None
    }
}

// Static None node for non-panicking index operations
static NODE_NONE: Node = Node::None;

/// Implements array-style indexing for Node using integer indices
/// Returns &Node::None for invalid access instead of panicking
impl Index<usize> for Node {
    type Output = Node;

    /// Allows accessing array elements using array[index] syntax
    /// Returns &Node::None if not an array or index out of bounds
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Node::Array(arr) => arr.get(index).unwrap_or(&NODE_NONE),
            _ => &NODE_NONE,
        }
    }
}

/// Implements object-style indexing for Node using string keys
/// Returns &Node::None for invalid access instead of panicking
impl Index<&str> for Node {
    type Output = Node;

    /// Allows accessing object properties using object["key"] syntax
    /// Returns &Node::None if not an object or key doesn't exist
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Node::Object(map) => map.get(key).unwrap_or(&NODE_NONE),
            _ => &NODE_NONE,
        }
    }
}

/// Implements mutable array-style indexing for Node
impl IndexMut<usize> for Node {
    /// Allows modifying array elements using array[index] = value syntax
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Node::Array(arr) => &mut arr[index],
            _ => panic!("Cannot index non-array node with integer"),
        }
    }
}

/// Implements mutable object-style indexing for Node
/// Note: For mutable access, the key must exist. Use insert() to add new keys.
impl IndexMut<&str> for Node {
    /// Allows modifying object properties using object["key"] = value syntax
    /// Panics if not an object or key doesn't exist (use insert() for new keys)
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        match self {
            Node::Object(map) => map.get_mut(key).expect("Key does not exist. Use insert() to add new keys."),
            _ => panic!("Cannot mutably index non-object node with string"),
        }
    }
}

/// Converts a vector of values into an array node
impl<T: Into<Node>> From<Vec<T>> for Node {
    fn from(value: Vec<T>) -> Self {
        Node::Array(value.into_iter().map(|x| x.into()).collect())
    }
}

impl From<i64> for Numeric {
    fn from(value: i64) -> Self {
        Numeric::Integer(value)
    }
}

impl From<f64> for Numeric {
    fn from(value: f64) -> Self {
        Numeric::Float(value)
    }
}

impl From<u64> for Numeric {
    fn from(value: u64) -> Self {
        Numeric::UInteger(value)
    }
}

impl From<u8> for Numeric {
    fn from(value: u8) -> Self {
        Numeric::Byte(value)
    }
}

impl From<i32> for Numeric {
    fn from(value: i32) -> Self {
        Numeric::Int32(value)
    }
}

impl From<u32> for Numeric {
    fn from(value: u32) -> Self {
        Numeric::UInt32(value)
    }
}

impl From<i16> for Numeric {
    fn from(value: i16) -> Self {
        Numeric::Int16(value)
    }
}

impl From<u16> for Numeric {
    fn from(value: u16) -> Self {
        Numeric::UInt16(value)
    }
}

impl From<i8> for Numeric {
    fn from(value: i8) -> Self {
        Numeric::Int8(value)
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Number(Numeric::Integer(value))
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Str(String::from(value))
    }
}

impl From<f64> for Node {
    fn from(value: f64) -> Self {
        Node::Number(Numeric::Float(value))
    }
}

impl From<u64> for Node {
    fn from(value: u64) -> Self {
        Node::Number(Numeric::UInteger(value))
    }
}

impl From<u8> for Node {
    fn from(value: u8) -> Self {
        Node::Number(Numeric::Byte(value))
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Number(Numeric::Int32(value))
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::Number(Numeric::UInt32(value))
    }
}

impl From<i16> for Node {
    fn from(value: i16) -> Self {
        Node::Number(Numeric::Int16(value))
    }
}

impl From<u16> for Node {
    fn from(value: u16) -> Self {
        Node::Number(Numeric::UInt16(value))
    }
}

impl From<i8> for Node {
    fn from(value: i8) -> Self {
        Node::Number(Numeric::Int8(value))
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Boolean(value)
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Str(value)
    }
}

/// Convert from Option<T> where T: Into<Node>
impl<T: Into<Node>> From<Option<T>> for Node {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Node::None,
        }
    }
}

/// Convert from fixed-size array [T; N]
impl<T: Into<Node> + Clone, const N: usize> From<[T; N]> for Node {
    fn from(arr: [T; N]) -> Self {
        Node::Array(arr.into_iter().map(|x| x.into()).collect())
    }
}

/// Convert from slice &[T]
impl<T: Into<Node> + Clone> From<&[T]> for Node {
    fn from(slice: &[T]) -> Self {
        Node::Array(slice.iter().cloned().map(|x| x.into()).collect())
    }
}

/// Convert from HashMap<String, T> where T: Into<Node>
impl<T: Into<Node>> From<HashMap<String, T>> for Node {
    fn from(map: HashMap<String, T>) -> Self {
        Node::Object(map.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

// Display implementations for better debugging
impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Numeric::Integer(n) => write!(f, "{}", n),
            Numeric::Float(n) => write!(f, "{}", n),
            Numeric::UInteger(n) => write!(f, "{}", n),
            Numeric::Byte(n) => write!(f, "{}", n),
            Numeric::Int32(n) => write!(f, "{}", n),
            Numeric::UInt32(n) => write!(f, "{}", n),
            Numeric::Int16(n) => write!(f, "{}", n),
            Numeric::UInt16(n) => write!(f, "{}", n),
            Numeric::Int8(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::None => write!(f, "null"),
            Node::Boolean(b) => write!(f, "{}", b),
            Node::Number(n) => write!(f, "{}", n),
            Node::Str(s) => write!(f, "\"{}\"", s),
            Node::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Node::Object(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in map {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

// TryFrom implementations for extracting values from Node
use core::convert::TryFrom;

impl TryFrom<Node> for String {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Str(s) => Ok(s),
            _ => Err("Node is not a string"),
        }
    }
}

impl TryFrom<Node> for Vec<Node> {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Array(vec) => Ok(vec),
            _ => Err("Node is not an array"),
        }
    }
}

impl TryFrom<Node> for HashMap<String, Node> {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Object(map) => Ok(map),
            _ => Err("Node is not an object"),
        }
    }
}

impl TryFrom<Node> for i64 {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        node.as_i64()
            .ok_or("Node is not a number or cannot be converted to i64")
    }
}

impl TryFrom<&Node> for i64 {
    type Error = &'static str;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        node.as_i64()
            .ok_or("Node is not a number or cannot be converted to i64")
    }
}

impl TryFrom<Node> for f64 {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        node.as_f64()
            .ok_or("Node is not a number or cannot be converted to f64")
    }
}

impl TryFrom<&Node> for f64 {
    type Error = &'static str;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        node.as_f64()
            .ok_or("Node is not a number or cannot be converted to f64")
    }
}

impl TryFrom<Node> for bool {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Boolean(b) => Ok(b),
            _ => Err("Node is not a boolean"),
        }
    }
}

impl TryFrom<&Node> for bool {
    type Error = &'static str;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        node.as_bool().ok_or("Node is not a boolean")
    }
}

/// Helper functions to create a Node from any value that can be converted into a Node
pub fn make_node<T>(value: T) -> Node
where
    T: Into<Node>,
{
    value.into()
}

/// Implement FromStr to enable "string".parse::<Node>()
impl FromStr for Node {
    type Err = String;

    /// Parse a JSON string into a Node
    ///
    /// # Examples
    /// ```
    /// use json_lib::Node;
    /// use std::str::FromStr;
    ///
    /// let node = Node::from_str(r#"{"name": "Alice"}"#).unwrap();
    /// assert!(node.is_object());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::io::sources::buffer::Buffer as BufferSource;
        use crate::parser::default::parse;

        let mut source = BufferSource::new(s.as_bytes());
        parse(&mut source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_conversions() {
        assert_eq!(Numeric::from(42i64), Numeric::Integer(42));
        assert_eq!(Numeric::from(3.14f64), Numeric::Float(3.14));
        assert_eq!(Numeric::from(42u64), Numeric::UInteger(42));
        assert_eq!(Numeric::from(42u8), Numeric::Byte(42));
        assert_eq!(Numeric::from(42i32), Numeric::Int32(42));
        assert_eq!(Numeric::from(42u32), Numeric::UInt32(42));
        assert_eq!(Numeric::from(42i16), Numeric::Int16(42));
        assert_eq!(Numeric::from(42u16), Numeric::UInt16(42));
        assert_eq!(Numeric::from(42i8), Numeric::Int8(42));
    }

    #[test]
    fn test_node_numeric_conversions() {
        assert_eq!(Node::from(42i64), Node::Number(Numeric::Integer(42)));
        assert_eq!(Node::from(3.14f64), Node::Number(Numeric::Float(3.14)));
        assert_eq!(Node::from(42u64), Node::Number(Numeric::UInteger(42)));
        assert_eq!(Node::from(42u8), Node::Number(Numeric::Byte(42)));
        assert_eq!(Node::from(42i32), Node::Number(Numeric::Int32(42)));
        assert_eq!(Node::from(42u32), Node::Number(Numeric::UInt32(42)));
        assert_eq!(Node::from(42i16), Node::Number(Numeric::Int16(42)));
        assert_eq!(Node::from(42u16), Node::Number(Numeric::UInt16(42)));
        assert_eq!(Node::from(42i8), Node::Number(Numeric::Int8(42)));
    }

    #[test]
    fn test_node_string_conversions() {
        assert_eq!(Node::from("test"), Node::Str("test".to_string()));
        assert_eq!(
            Node::from("test".to_string()),
            Node::Str("test".to_string())
        );
    }

    #[test]
    fn test_node_bool_conversion() {
        assert_eq!(Node::from(true), Node::Boolean(true));
        assert_eq!(Node::from(false), Node::Boolean(false));
    }

    #[test]
    fn test_node_vec_conversion() {
        let vec = vec![1, 2, 3];
        let node = Node::from(vec);
        match node {
            Node::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], Node::Number(Numeric::Int32(1)));
                assert_eq!(arr[1], Node::Number(Numeric::Int32(2)));
                assert_eq!(arr[2], Node::Number(Numeric::Int32(3)));
            }
            _ => panic!("Expected Array node"),
        }
    }

    #[test]
    fn test_array_indexing() {
        let arr = Node::Array(vec![Node::from(1), Node::from(2)]);
        assert_eq!(arr[0], Node::Number(Numeric::Int32(1)));
        assert_eq!(arr[1], Node::Number(Numeric::Int32(2)));
    }

    #[test]
    fn test_invalid_array_indexing() {
        let node = Node::Boolean(true);
        // Non-panicking - returns &Node::None
        assert!(node[0].is_null());
    }

    #[test]
    fn test_object_indexing() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::from(42));
        let obj = Node::Object(map);
        assert_eq!(obj["key"], Node::Number(Numeric::Int32(42)));
    }

    #[test]
    fn test_invalid_object_indexing() {
        let node = Node::Boolean(true);
        // Non-panicking - returns &Node::None
        assert!(node["key"].is_null());
    }

    #[test]
    fn test_array_mut_indexing() {
        let mut arr = Node::Array(vec![Node::from(1), Node::from(2)]);
        arr[0] = Node::from(42);
        assert_eq!(arr[0], Node::Number(Numeric::Int32(42)));
    }

    #[test]
    #[should_panic(expected = "Cannot index non-array node with integer")]
    fn test_invalid_array_mut_indexing() {
        let mut node = Node::Boolean(true);
        node[0] = Node::from(42);
    }

    #[test]
    fn test_object_mut_indexing() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::from(42));
        let mut obj = Node::Object(map);
        obj["key"] = Node::from(100);
        assert_eq!(obj["key"], Node::Number(Numeric::Int32(100)));
    }

    #[test]
    #[should_panic(expected = "Cannot mutably index non-object node with string")]
    fn test_invalid_object_mut_indexing() {
        let mut node = Node::Boolean(true);
        node["key"] = Node::from(42);
    }

    #[test]
    #[should_panic(expected = "Key does not exist")]
    fn test_object_mut_indexing_nonexistent_key() {
        let mut obj = Node::Object(HashMap::new());
        obj["nonexistent"] = Node::from(42);
    }

    #[test]
    fn test_make_node() {
        assert_eq!(make_node(42), Node::Number(Numeric::Int32(42)));
        assert_eq!(make_node("test"), Node::Str("test".to_string()));
        assert_eq!(make_node(true), Node::Boolean(true));
    }
    #[test]
    fn test_make_node_vec() {
        let vec = vec![1, 2, 3];
        assert_eq!(
            make_node(vec),
            Node::Array(vec![
                Node::Number(Numeric::Int32(1)),
                Node::Number(Numeric::Int32(2)),
                Node::Number(Numeric::Int32(3))
            ])
        );
    }

    #[test]
    fn test_enum_sizes() {
        use core::mem::size_of;
        println!("Size of Node: {} bytes", size_of::<Node>());
        println!("Size of Numeric: {} bytes", size_of::<Numeric>());
        println!("Size of String: {} bytes", size_of::<String>());
        println!("Size of Vec<Node>: {} bytes", size_of::<Vec<Node>>());
        println!(
            "Size of HashMap<String, Node>: {} bytes",
            size_of::<HashMap<String, Node>>()
        );
    }
}
