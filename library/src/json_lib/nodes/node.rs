use std::collections::HashMap;

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
    /// Represents a array of other nodes
    Array(Vec<Node>),
    /// Represents a object/map of string keys to node values
    Object(HashMap<String, Node>),
    /// Represents an empty or uninitialized node
    None,
}

/// Converts a vector of values into a array node
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

// Allow creating a array node from a static array literal, e.g., Node::from([1, 2, 3])
impl<T, const N: usize> From<[T; N]> for Node
where
    T: Into<Node>,
{
    fn from(value: [T; N]) -> Self {
        Node::Array(value.into_iter().map(|x| x.into()).collect())
    }
}

// Allow creating a object node from a static array of key-value pairs.
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
                assert_eq!(integer, Numeric::Integer(32));
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
                    Node::Number(i) => assert_eq!(*i, Numeric::Integer(2)),
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
}
