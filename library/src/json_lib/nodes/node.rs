use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub enum Numeric {
    Integer(i64),
    Float(f64),
    UInteger(u64),
    Byte(u8),
    Int32(i32),
    UInt32(u32),
    Int16(i16),
    UInt16(u16),
    Int8(i8),
}
/// A node in the JSON data structure that can represent different types of values.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// Represents a numeric value
    Boolean(bool),
    /// Represents a boolean value
    Number(Numeric),
    /// Represents a string value
    Str(String),
    /// Represents an array of other nodes
    Array(Vec<Node>),
    /// Represents a object/map of string keys to node values
    Object(HashMap<String, Node>),
    /// Represents an empty or uninitialized node
    None,
}

impl Index<usize> for Node {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Node::Array(arr) => &arr[index],
            _ => panic!("Cannot index non-array node with integer"),
        }
    }
}

impl Index<&str> for Node {
    type Output = Node;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Node::Object(map) => &map[key],
            _ => panic!("Cannot index non-object node with string"),
        }
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Node::Array(arr) => &mut arr[index],
            _ => panic!("Cannot index non-array node with integer"),
        }
    }
}

impl IndexMut<&str> for Node {
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

/// Converts an integer into an Integer node
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

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Number(Numeric::Int32(value))
    }
}

/// Converts a string slice into a Str node
impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Str(String::from(value))
    }
}

/// Converts a String into a Str node
impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Str(value)
    }
}

impl From<bool> for Node {
    fn from(value: bool) -> Self {
        Node::Boolean(value)
    }
}

/// Converts a HashMap into a object node
impl From<HashMap<String, Node>> for Node {
    fn from(value: HashMap<String, Node>) -> Self {
        Node::Object(value)
    }
}

// Allow creating an array node from a static array literal, e.g., Node::from([1, 2, 3])
impl<T, const N: usize> From<[T; N]> for Node
where
    T: Into<Node>,
{
    fn from(value: [T; N]) -> Self {
        Node::Array(value.into_iter().map(|x| x.into()).collect())
    }
}

// Allow creating an object node from a static array of key-value pairs.
// e.g., Node::from([("a", 1), ("b", 2)])
impl<K, V, const N: usize> From<[(K, V); N]> for Node
where
    K: Into<String>,
    V: Into<Node>,
{
    fn from(value: [(K, V); N]) -> Self {
        let mut map: HashMap<String, Node> = HashMap::with_capacity(N);
        for (k, v) in value.into_iter() {
            map.insert(k.into(), v.into());
        }
        Node::Object(map)
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
    use super::{make_node, Node, Numeric};
    use std::collections::HashMap;

    #[test]
    fn create_integer_works() {
        let variant = Node::Number(Numeric::Int32(32));
        match variant {
            Node::Number(integer) => {
                assert_eq!(integer, Numeric::Int32(32));
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_string_works() {
        let variant = Node::Str(String::from("test"));
        match variant {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_array_works() {
        let variant = Node::Array(Vec::<Node>::new());
        match variant {
            Node::Array(array) => {
                assert_eq!(array.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn push_to_array_works() {
        let variant = Node::Array(Vec::<Node>::new());
        match variant {
            Node::Array(mut array) => {
                array.push(Node::Number(Numeric::Int32(32)));
                assert_eq!(array.len(), 1);
                match &array[0] {
                    Node::Number(integer) => {
                        assert_eq!(integer, &Numeric::Int32(32));
                    }
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn push_multiple_to_array_works() {
        let variant = Node::Array(Vec::<Node>::new());
        match variant {
            Node::Array(mut array) => {
                array.push(Node::Number(Numeric::Int32(32)));
                array.push(Node::Number(Numeric::Int32(33)));
                array.push(Node::Number(Numeric::Int32(34)));
                array.push(Node::Number(Numeric::Int32(35)));
                array.push(Node::Number(Numeric::Int32(36)));
                assert_eq!(array.len(), 5);
                match &array[4] {
                    Node::Number(integer) => {
                        assert_eq!(*integer, Numeric::Int32(36));
                    }
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_object_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(object) => {
                assert_eq!(object.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn add_to_object_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(mut object) => {
                object.insert(String::from("test"), Node::Number(Numeric::Int32(32)));
                assert_eq!(object.len(), 1);
                match &object["test"] {
                    Node::Number(integer) => {
                        assert_eq!(integer, &Numeric::Int32(32));
                    }
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn add_multiple_to_object_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(mut object) => {
                object.insert(String::from("test1"), Node::Number(Numeric::Int32(32)));
                object.insert(String::from("test2"), Node::Number(Numeric::Int32(33)));
                object.insert(String::from("test3"), Node::Number(Numeric::Int32(34)));
                object.insert(String::from("test4"), Node::Number(Numeric::Int32(35)));
                object.insert(String::from("test5"), Node::Number(Numeric::Int32(36)));
                assert_eq!(object.len(), 5);
                match &object["test5"] {
                    Node::Number(integer) => {
                        assert_eq!(*integer, Numeric::Int32(36));
                    }
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_an_integer_node_works() {
        let node = make_node(32);
        match node {
            Node::Number(integer) => {
                assert_eq!(integer, Numeric::Int32(32));
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_string_node_works() {
        let node = make_node("test");
        match node {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_array_node_works() {
        let node = make_node(Vec::<Node>::new());
        match node {
            Node::Array(array) => {
                assert_eq!(array.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_object_node_works() {
        let node = make_node(HashMap::<String, Node>::new());
        match node {
            Node::Object(object) => {
                assert_eq!(object.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }

    // New tests for static initializer arrays
    #[test]
    fn array_literal_to_array_node_works() {
        let node = make_node([1, 2, 3]);
        match node {
            Node::Array(array) => {

                for item in array {
                    match item {
                        Node::Number(_) => (),
                        _ => assert_eq!(false, true),
                    }
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn mixed_array_literal_to_array_node_works() {
        let node = Node::from([Node::Number(Numeric::Int32(1)), Node::Str("x".to_string()), Node::Number(Numeric::Int32(3))]);
        match node {
            Node::Array(array) => {
                assert_eq!(array.len(), 3);
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn array_literal_to_object_node_works() {
        let node = make_node([("a", 1), ("b", 2)]);
        match node {
            Node::Object(map) => {
                assert_eq!(map.len(), 2);
                match map.get("b").unwrap() {
                    Node::Number(i) => assert_eq!(*i, Numeric::Int32(2)),
                    _ => assert_eq!(false, true),
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn none_node_works() {
        let node = Node::None;
        match node {
            Node::None => (),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn mixed_object_from_array_works() {
        let node = Node::from([
            ("int", Node::Number(Numeric::Int32(1))),
            ("str", Node::Str("test".to_string())),
            ("array", Node::Array(Vec::<Node>::new())),
        ]);
        match node {
            Node::Object(map) => {
                assert_eq!(map.len(), 3);
                assert!(matches!(map.get("int"), Some(Node::Number(_))));
                assert!(matches!(map.get("str"), Some(Node::Str(_))));
                assert!(matches!(map.get("array"), Some(Node::Array(_))));
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn empty_array_to_array_works() {
        let node = Node::from([] as [i64; 0]);
        match node {
            Node::Array(array) => assert_eq!(array.len(), 0),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn empty_array_to_object_works() {
        let node = Node::from([] as [(String, Node); 0]);
        match node {
            Node::Object(map) => assert_eq!(map.len(), 0),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn create_boolean_works() {
        let variant = Node::Boolean(true);
        match variant {
            Node::Boolean(value) => {
                assert_eq!(value, true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn make_a_boolean_node_works() {
        let node = make_node(true);
        match node {
            Node::Boolean(value) => {
                assert_eq!(value, true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn test_float_conversion() {
        let num = Numeric::from(64.5);
        assert_eq!(num, Numeric::Float(64.5));
    }

    #[test]
    fn test_uint64_conversion() {
        let num = Numeric::from(64_u64);
        assert_eq!(num, Numeric::UInteger(64));
    }

    #[test]
    fn test_byte_conversion() {
        let num = Numeric::from(64_u8);
        assert_eq!(num, Numeric::Byte(64));
    }

    #[test]
    fn test_int32_conversion() {
        let num = Numeric::from(32_i32);
        assert_eq!(num, Numeric::Int32(32));
    }

    #[test]
    fn test_uint32_conversion() {
        let num = Numeric::from(32_u32);
        assert_eq!(num, Numeric::UInt32(32));
    }

    #[test]
    fn test_int16_conversion() {
        let num = Numeric::from(16_i16);
        assert_eq!(num, Numeric::Int16(16));
    }

    #[test]
    fn test_uint16_conversion() {
        let num = Numeric::from(16_u16);
        assert_eq!(num, Numeric::UInt16(16));
    }

    #[test]
    fn test_int8_conversion() {
        let num = Numeric::from(8_i8);
        assert_eq!(num, Numeric::Int8(8));
    }

    #[test]
    fn test_float_node_conversion() {
        let node = Node::Number(Numeric::Float(64.5));
        match node {
            Node::Number(num) => assert_eq!(num, Numeric::Float(64.5)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_mixed_numeric_array() {
        let node = Node::from([
            Node::Number(Numeric::Int32(1)),
            Node::Number(Numeric::Float(2.5)),
            Node::Number(Numeric::UInteger(3)),
            Node::Number(Numeric::Byte(4)),
        ]);
        match node {
            Node::Array(array) => assert_eq!(array.len(), 4),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_node_from_node() {
        let original = Node::Number(Numeric::Int32(42));
        let node: Node = original.clone().into();
        assert_eq!(node, original);
    }

    #[test]
    fn test_nested_numeric_array() {
        let node = Node::from([
            Node::from([
                Node::Number(Numeric::Int16(1)),
                Node::Number(Numeric::UInt16(2)),
                Node::Number(Numeric::Int8(3))
            ]),
            Node::from([
                Node::Number(Numeric::UInt32(4)),
                Node::Number(Numeric::Integer(5)),
                Node::Number(Numeric::Float(6.0))
            ])
        ]);
        match node {
            Node::Array(array) => {
                assert_eq!(array.len(), 2);
                match &array[0] {
                    Node::Array(inner) => assert_eq!(inner.len(), 3),
                    _ => assert_eq!(false, true),
                }
            },
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn complex_array_literal_to_array_node_works() {
        let node = Node::from([
            Node::from([1, 2, 3]),
            Node::from([("x", 10), ("y", 20)]),
            Node::Str("test".to_string())
        ]);
        match node {
            Node::Array(array) => {
                assert_eq!(array.len(), 3);
                match &array[0] {
                    Node::Array(inner_array) => assert_eq!(inner_array.len(), 3),
                    _ => assert_eq!(false, true),
                }
                match &array[1] {
                    Node::Object(obj) => assert_eq!(obj.len(), 2),
                    _ => assert_eq!(false, true),
                }
                match &array[2] {
                    Node::Str(s) => assert_eq!(s, "test"),
                    _ => assert_eq!(false, true),
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_array_indexing() {
        let array_node = Node::from([1, 2, 3]);
        match &array_node[0] {
            Node::Number(num) => assert_eq!(*num, Numeric::Int32(1)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_object_key_indexing() {
        let object_node = Node::from([("test", 42)]);
        match &object_node["test"] {
            Node::Number(num) => assert_eq!(*num, Numeric::Int32(42)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_mutable_array_indexing() {
        let mut array_node = Node::from([1, 2, 3]);
        array_node[1] = Node::Number(Numeric::Int32(42));
        match &array_node[1] {
            Node::Number(num) => assert_eq!(*num, Numeric::Int32(42)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_mutable_object_indexing() {
        let mut object_node = Node::from([("test", 1)]);
        object_node["test"] = Node::Number(Numeric::Int32(42));
        match &object_node["test"] {
            Node::Number(num) => assert_eq!(*num, Numeric::Int32(42)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_complex_literal_hashmap_to_dict_node_works() {
        let node = Node::from([
            ("array", Node::from([1, 2, 3])),
            ("object", Node::from([("x", 10), ("y", 20)])),
            ("string", Node::from("test")),
            ("number", Node::from(42)),
            ("boolean", Node::from(true)),
            ("nested", Node::from([
                ("inner_array", Node::from([4, 5, 6])),
                ("inner_object", Node::from([("a", 1), ("b", 2)]))
            ]))
        ]);

        match node {
            Node::Object(map) => {
                assert_eq!(map.len(), 6);

                match map.get("array").unwrap() {
                    Node::Array(arr) => assert_eq!(arr.len(), 3),
                    _ => assert_eq!(false, true),
                }

                match map.get("object").unwrap() {
                    Node::Object(obj) => assert_eq!(obj.len(), 2),
                    _ => assert_eq!(false, true),
                }

                match map.get("string").unwrap() {
                    Node::Str(s) => assert_eq!(s, "test"),
                    _ => assert_eq!(false, true),
                }

                match map.get("number").unwrap() {
                    Node::Number(_) => (),
                    _ => assert_eq!(false, true),
                }

                match map.get("boolean").unwrap() {
                    Node::Boolean(b) => assert_eq!(*b, true),
                    _ => assert_eq!(false, true),
                }

                match map.get("nested").unwrap() {
                    Node::Object(nested) => {
                        assert_eq!(nested.len(), 2);
                        match nested.get("inner_array").unwrap() {
                            Node::Array(arr) => assert_eq!(arr.len(), 3),
                            _ => assert_eq!(false, true),
                        }
                        match nested.get("inner_object").unwrap() {
                            Node::Object(obj) => assert_eq!(obj.len(), 2),
                            _ => assert_eq!(false, true),
                        }
                    },
                    _ => assert_eq!(false, true),
                }
            },
            _ => assert_eq!(false, true),
        }
    }
    
}
