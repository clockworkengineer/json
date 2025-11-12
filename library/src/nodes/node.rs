#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeMap as HashMap, string::String, vec::Vec};

#[cfg(feature = "std")]
use core::ops::{Index, IndexMut};

#[cfg(not(feature = "std"))]
use core::ops::{Index, IndexMut};

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
    Array(Vec<Node>),
    /// Represents an object/map of string keys to node values
    Object(HashMap<String, Node>),
    /// Represents a null value or uninitialized node
    None,
}

impl Node {
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
    pub fn is_object(&self) -> bool {
        matches!(self, Node::Object(_))
    }

    /// Returns true if this node is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Node::Array(_))
    }

    /// Returns true if this node is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Node::Str(_))
    }

    /// Returns true if this node is a number
    pub fn is_number(&self) -> bool {
        matches!(self, Node::Number(_))
    }

    /// Returns true if this node is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Node::Boolean(_))
    }

    /// Returns true if this node is None/null
    pub fn is_null(&self) -> bool {
        matches!(self, Node::None)
    }

    /// Returns the string value if this node is a Str, None otherwise
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Node::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Returns the boolean value if this node is a Boolean, None otherwise
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Node::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the number value if this node is a Number, None otherwise
    pub fn as_number(&self) -> Option<&Numeric> {
        match self {
            Node::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the array reference if this node is an Array, None otherwise
    pub fn as_array(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns a mutable array reference if this node is an Array, None otherwise
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Returns the object reference if this node is an Object, None otherwise
    pub fn as_object(&self) -> Option<&HashMap<String, Node>> {
        match self {
            Node::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Returns a mutable object reference if this node is an Object, None otherwise
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Node>> {
        match self {
            Node::Object(map) => Some(map),
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
        match (self, other) {
            (Node::Object(self_map), Node::Object(other_map)) => {
                for (key, other_value) in other_map {
                    self_map
                        .entry(key)
                        .and_modify(|self_value| {
                            if self_value.is_object() && other_value.is_object() {
                                self_value.merge(other_value.clone());
                            } else {
                                *self_value = other_value.clone();
                            }
                        })
                        .or_insert(other_value);
                }
            }
            (this, other) => *this = other,
        }
    }

    /// Creates a deep clone of this node
    /// This is equivalent to calling `.clone()` but provided for API consistency
    pub fn deep_clone(&self) -> Node {
        self.clone()
    }
}

/// Implements array-style indexing for Node using integer indices
impl Index<usize> for Node {
    type Output = Node;

    /// Allows accessing array elements using array[index] syntax
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Node::Array(arr) => &arr[index],
            _ => panic!("Cannot index non-array node with integer"),
        }
    }
}

/// Implements object-style indexing for Node using string keys
impl Index<&str> for Node {
    type Output = Node;

    /// Allows accessing object properties using object["key"] syntax
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Node::Object(map) => &map[key],
            _ => panic!("Cannot index non-object node with string"),
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
impl IndexMut<&str> for Node {
    /// Allows modifying object properties using object["key"] = value syntax
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        match self {
            Node::Object(map) => map.get_mut(key).expect("No such key exists"),
            _ => panic!("Cannot index non-object node with string"),
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

/// Helper functions to create a Node from any value that can be converted into a Node
pub fn make_node<T>(value: T) -> Node
where
    T: Into<Node>,
{
    value.into()
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
    #[should_panic(expected = "Cannot index non-array node with integer")]
    fn test_invalid_array_indexing() {
        let node = Node::Boolean(true);
        let _value = &node[0];
    }

    #[test]
    fn test_object_indexing() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::from(42));
        let obj = Node::Object(map);
        assert_eq!(obj["key"], Node::Number(Numeric::Int32(42)));
    }

    #[test]
    #[should_panic(expected = "Cannot index non-object node with string")]
    fn test_invalid_object_indexing() {
        let node = Node::Boolean(true);
        let _value = &node["key"];
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
    #[should_panic(expected = "Cannot index non-object node with string")]
    fn test_invalid_object_mut_indexing() {
        let mut node = Node::Boolean(true);
        node["key"] = Node::from(42);
    }

    #[test]
    #[should_panic(expected = "No such key exists")]
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
        println!("Size of HashMap<String, Node>: {} bytes", size_of::<HashMap<String, Node>>());
    }
}
