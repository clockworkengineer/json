use std::collections::HashMap;

/// A node in the JSON data structure that can represent different types of values.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// Represents a 64-bit signed integer value
    Boolean(bool),
    /// Represents a boolean value
    Integer(i64),
    /// Represents a string value
    Str(String),
    /// Represents a array of other nodes
    Array(Vec<Node>),
    /// Represents a dictionary/map of string keys to node values
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
impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Integer(value)
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

/// Converts a HashMap into a Dictionary node
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

// Allow creating a Dictionary node from a static array of key-value pairs.
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
    use super::{make_node, Node};
    use std::collections::HashMap;

    #[test]
    fn create_integer_works() {
        let variant = Node::Integer(32);
        match variant {
            Node::Integer(integer) => {
                assert_eq!(integer, 32);
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
                array.push(Node::Integer(32));
                assert_eq!(array.len(), 1);
                match array[0] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 32);
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
                array.push(Node::Integer(32));
                array.push(Node::Integer(33));
                array.push(Node::Integer(34));
                array.push(Node::Integer(35));
                array.push(Node::Integer(36));
                assert_eq!(array.len(), 5);
                match array[4] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 36);
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
    fn create_dictionary_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(dictionary) => {
                assert_eq!(dictionary.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn add_to_dictionary_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(mut dictionary) => {
                dictionary.insert(String::from("test"), Node::Integer(32));
                assert_eq!(dictionary.len(), 1);
                match dictionary["test"] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 32);
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
    fn add_multiple_to_dictionary_works() {
        let variant = Node::Object(HashMap::new());
        match variant {
            Node::Object(mut dictionary) => {
                dictionary.insert(String::from("test1"), Node::Integer(32));
                dictionary.insert(String::from("test2"), Node::Integer(33));
                dictionary.insert(String::from("test3"), Node::Integer(34));
                dictionary.insert(String::from("test4"), Node::Integer(35));
                dictionary.insert(String::from("test5"), Node::Integer(36));
                assert_eq!(dictionary.len(), 5);
                match dictionary["test5"] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 36);
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
            Node::Integer(integer) => {
                assert_eq!(integer, 32);
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
    fn make_a_dictionary_node_works() {
        let node = make_node(HashMap::<String, Node>::new());
        match node {
            Node::Object(dictionary) => {
                assert_eq!(dictionary.is_empty(), true);
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
                        Node::Integer(_) => (),
                        _ => assert_eq!(false, true),
                    }
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn mixed_array_literal_to_array_node_works() {
        let node = Node::from([Node::Integer(1), Node::Str("x".to_string()), Node::Integer(3)]);
        match node {
            Node::Array(array) => {
                assert_eq!(array.len(), 3);
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn array_literal_to_dictionary_node_works() {
        let node = make_node([("a", 1), ("b", 2)]);
        match node {
            Node::Object(map) => {
                assert_eq!(map.len(), 2);
                match map.get("b").unwrap() {
                    Node::Integer(i) => assert_eq!(*i, 2),
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
    fn mixed_dictionary_from_array_works() {
        let node = Node::from([
            ("int", Node::Integer(1)),
            ("str", Node::Str("test".to_string())),
            ("array", Node::Array(Vec::<Node>::new())),
        ]);
        match node {
            Node::Object(map) => {
                assert_eq!(map.len(), 3);
                assert!(matches!(map.get("int"), Some(Node::Integer(1))));
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
}
