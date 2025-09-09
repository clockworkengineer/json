use crate::io::traits::IDestination;
use crate::Node;

/// Converts a JSON node to TOML format and writes it to the destination
///
/// # Arguments
/// * `node` - The TOML node to convert
/// * `destination` - The output destination implementing IDestination
pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    fn write_value(value: &Node, destination: &mut dyn IDestination) {
        match value {
            Node::Str(s) => {
                destination.add_bytes("\"");
                destination.add_bytes(s);
                destination.add_bytes("\"");
            }
            Node::Number(n) => destination.add_bytes(&format!("{:?}", n)),
            Node::Boolean(b) => destination.add_bytes(&*b.to_string()),
            Node::None => destination.add_bytes("null"),
            Node::Array(arr) => {
                destination.add_bytes("[");
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        destination.add_bytes(", ");
                    }
                    write_value(item, destination);
                }
                destination.add_bytes("]");
            }
            Node::Object(_) => destination.add_bytes(""), // Handled separately
        }
    }

    fn write_table(prefix: &str, obj: &Node, destination: &mut dyn IDestination) {
        if let Node::Object(map) = obj {
            for (key, value) in map {
                match value {
                    Node::Object(_) => {
                        let new_prefix = if prefix.is_empty() {
                            key.to_string()
                        } else {
                            format!("{}.{}", prefix, key)
                        };
                        destination.add_bytes(&format!("\n[{}]\n", new_prefix));
                        write_table(&new_prefix, value, destination);
                    }
                    _ => {
                        if !prefix.is_empty() {
                            destination.add_bytes(&format!("{}.{} = ", prefix, key));
                        } else {
                            destination.add_bytes(&format!("{} = ", key));
                        }
                        write_value(value, destination);
                        destination.add_bytes("\n");
                    }
                }
            }
        }
    }

    if let Node::Object(_) = node {
        write_table("", node, destination);
    } else {
        write_value(node, destination);
    }
}
