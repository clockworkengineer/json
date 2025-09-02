use std::collections::HashMap;
use std::ops::{Index, IndexMut};

/// Represents different numeric types that can be stored in a JSON node
#[derive(Clone, Debug, PartialEq)]
pub enum Numeric {
    Integer(i64),    // 64-bit signed integer
    Float(f64),      // 64-bit floating point
    UInteger(u64),   // 64-bit unsigned integer
    Byte(u8),        // 8-bit unsigned integer
    Int32(i32),      // 32-bit signed integer
    UInt32(u32),     // 32-bit unsigned integer
    Int16(i16),      // 16-bit signed integer
    UInt16(u16),     // 16-bit unsigned integer
    Int8(i8),        // 8-bit signed integer
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

// Numeric type conversion implementations
// Each implementation converts a specific numeric type to the Numeric enum

impl From<i64> for Numeric {
    fn from(value: i64) -> Self {
        Numeric::Integer(value)
    }
}

// ... [remaining Numeric implementations]

// Node type conversion implementations
// Each implementation converts a specific type to a Node

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Number(Numeric::Integer(value))
    }
}

// ... [remaining Node implementations]

/// Helper function to create a Node from any value that can be converted into a Node
pub fn make_node<T>(value: T) -> Node
where
    T: Into<Node>,
{
    value.into()
}

// Test module containing comprehensive tests for Node functionality
#[cfg(test)]
mod tests {
    // ... [test implementations remain unchanged]
}