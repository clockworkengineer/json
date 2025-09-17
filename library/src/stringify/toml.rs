use crate::io::traits::IDestination;
use crate::{Node, Numeric};
use std::collections::BTreeMap;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Object(dict) => stringify_object(dict, "", destination),
        _ => Err("TOML format requires a Object at the root level".to_string()),
    }
}

fn stringify_value(value: &Node, add_cr: bool, destination: &mut dyn IDestination) -> Result<(), String> {
    match value {
        Node::Str(s) => stringify_str(s, destination),
        Node::Boolean(b) => stringify_bool(b, destination),
        Node::Number(value) => stringify_number(value, destination),
        Node::Array(items) => stringify_array(items, destination)?,
        Node::None => destination.add_bytes("null"),
        Node::Object(_) => return Ok(()), // Handled separately for table syntax
    }
    if add_cr {
        destination.add_bytes("\n");
    }
    Ok(())
}
fn stringify_str(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");
    destination.add_bytes(s);
    destination.add_bytes("\"");
}

fn stringify_bool(b: &bool, destination: &mut dyn IDestination) {
    destination.add_bytes(&*b.to_string())
}

fn stringify_number(value: &Numeric, destination: &mut dyn IDestination) {
    match value {
        // Handles signed integer values
        Numeric::Integer(n) => destination.add_bytes(&n.to_string()),
        // Handles unsigned integer values
        Numeric::UInteger(n) => destination.add_bytes(&n.to_string()),
        // Handles floating point numbers
        Numeric::Float(f) => destination.add_bytes(&f.to_string()),
        // Handles 8-bit unsigned values (0-255)
        Numeric::Byte(b) => destination.add_bytes(&b.to_string()),
        // Handles 32-bit signed integers (-2^31 to 2^31-1)
        Numeric::Int32(i) => destination.add_bytes(&i.to_string()),
        // Handles 32-bit unsigned integers (0 to 2^32-1)
        Numeric::UInt32(u) => destination.add_bytes(&u.to_string()),
        // Fallback for any future numeric variants
        // If there are any other variants, add them here
        #[allow(unreachable_patterns)]
        _ => destination.add_bytes(&format!("{:?}", value)),
    }
}

fn stringify_array(items: &Vec<Node>, destination: &mut dyn IDestination) -> Result<(), String> {
    if items.is_empty() {
        destination.add_bytes("[]");
        return Ok(());
    }

    // Check first item's type
    let first_type = match &items[0] {
        Node::Str(_) => "string",
        Node::Boolean(_) => "boolean",
        Node::Number(_) => "number",
        Node::Array(_) => "array",
        Node::Object(_) => "object",
        Node::None => "null"
    };

    // Validate all items are same type
    for item in items {
        let item_type = match item {
            Node::Str(_) => "string",
            Node::Boolean(_) => "boolean",
            Node::Number(_) => "number",
            Node::Array(_) => "array",
            Node::Object(_) => "object",
            Node::None => "null"
        };
        if item_type != first_type {
            return Err("TOML arrays must contain elements of the same type".to_string());
        }
    }

    destination.add_bytes("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            destination.add_bytes(", ");
        }
        stringify_value(item, false, destination)?;
    }
    destination.add_bytes("]");
    Ok(())
}

fn stringify_object(dict: &std::collections::HashMap<String, Node>, prefix: &str, destination: &mut dyn IDestination) -> Result<(), String> {
    if dict.is_empty() {
        return Ok(());
    }

    let dict_sorted: BTreeMap<_, _> = dict.iter().collect();
    let mut tables = BTreeMap::new();
    let mut array_tables = BTreeMap::new();
    let mut is_first = true;

    // First pass - handle simple key-value pairs and collect tables
    for (key, value) in dict_sorted {
        match value {
            Node::Object(nested) => {
                tables.insert(key, nested);
            }
            Node::Array(items) => {
                if items.iter().all(|item| matches!(item, Node::Object(_))) {
                    // Collect arrays of tables
                    array_tables.insert(key, items);
                } else {
                    if !prefix.is_empty() && is_first {
                        destination.add_bytes("[");
                        destination.add_bytes(prefix);
                        destination.add_bytes("]\n");
                        is_first = false;
                    }
                    destination.add_bytes(key);
                    destination.add_bytes(" = ");
                    stringify_value(value, true, destination)?;
                }
            }
            _ => {
                if !prefix.is_empty() && is_first {
                    destination.add_bytes("[");
                    destination.add_bytes(prefix);
                    destination.add_bytes("]\n");
                    is_first = false;
                }
                destination.add_bytes(key);
                destination.add_bytes(" = ");
                stringify_value(value, true, destination)?;
            }
        }
    }

    // Second pass - handle nested tables
    for (key, nested) in tables {
        let new_prefix = if prefix.is_empty() {
            key.to_string()
        } else {
            format!("{}.{}", prefix, key)
        };
        stringify_object(nested, &new_prefix, destination)?;
    }

    // Third pass - handle arrays of tables
    let array_tables_sorted: BTreeMap<_, _> = array_tables.into_iter().collect();
    for (key, items) in array_tables_sorted {
        for item in items {
            if let Node::Object(nested) = item {
                let new_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                destination.add_bytes("[[");
                destination.add_bytes(&new_prefix);
                destination.add_bytes("]]\n");
                let nested_sorted: BTreeMap<_, _> = nested.iter().collect();
                for (inner_key, inner_value) in &nested_sorted {
                    match inner_value {
                        Node::Object(inner_nested) => {
                            let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                            stringify_object(inner_nested, &inner_prefix, destination)?;
                        }
                        Node::Array(inner_items) if inner_items.iter().all(|item| matches!(item, Node::Object(_))) => {
                            for inner_item in inner_items {
                                if let Node::Object(deepest) = inner_item {
                                    let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                                    stringify_object(deepest, &inner_prefix, destination)?;
                                }
                            }
                        }
                        _ => {
                            destination.add_bytes(inner_key);
                            destination.add_bytes(" = ");
                            stringify_value(inner_value, true, destination)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferDestination;
    use std::collections::{HashMap};

    #[test]
    fn test_stringify_array() {
        let mut dest = BufferDestination::new();
        let result = stringify(&Node::Array(vec![
            Node::Str("a".to_string()),
            Node::Number(Numeric::Float(1.0)),
            Node::Boolean(true)
        ]), &mut dest);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "TOML format requires a Object at the root level");
    }

    #[test]
    fn test_stringify_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Node::Str("value".to_string()));
        let mut dest = BufferDestination::new();
        let hashmap = map.into_iter().collect::<HashMap<_, _>>();
        let _ = stringify(&Node::Object(hashmap), &mut dest);
        assert_eq!(dest.to_string(), "key = \"value\"\n");
    }

    #[test]
    fn test_stringify_non_object_root() {
        let mut dest = BufferDestination::new();
        let result = stringify(&Node::Str("test".to_string()), &mut dest);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "TOML format requires a Object at the root level");
    }

    #[test]
    fn test_stringify_nested_object() {
        let mut inner = HashMap::new();
        inner.insert("inner_key".to_string(), Node::Str("value".to_string()));
        let inner_hashmap = inner.into_iter().collect::<HashMap<_, _>>();
        let mut outer = HashMap::new();
        outer.insert("outer".to_string(), Node::Object(inner_hashmap));
        let outer_hashmap = outer.into_iter().collect::<HashMap<_, _>>();
        let mut dest = BufferDestination::new();
        stringify(&Node::Object(outer_hashmap), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "[outer]\ninner_key = \"value\"\n");
    }
    // ...existing code...

    #[test]
    fn test_stringify_deeply_nested_object() {
        let mut level3 = HashMap::new();
        level3.insert("deep_key".to_string(), Node::Number(Numeric::Integer(123)));
        let level3 = Node::Object(level3);

        let mut level2 = HashMap::new();
        level2.insert("level3".to_string(), level3);
        let level2 = Node::Object(level2);

        let mut level1 = HashMap::new();
        level1.insert("level2".to_string(), level2);
        let level1 = Node::Object(level1);

        let mut root = HashMap::new();
        root.insert("level1".to_string(), level1);

        let mut dest = BufferDestination::new();
        stringify(&Node::Object(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "[level1.level2.level3]\ndeep_key = 123\n"
        );
    }

    #[test]
    fn test_stringify_object_with_multiple_nested_tables_and_values() {
        let mut address = HashMap::new();
        address.insert("city".to_string(), Node::Str("Paris".to_string()));
        address.insert("zip".to_string(), Node::Number(Numeric::Integer(75000)));

        let mut profile = HashMap::new();
        profile.insert("name".to_string(), Node::Str("Alice".to_string()));
        profile.insert("age".to_string(), Node::Number(Numeric::Integer(30)));
        profile.insert("address".to_string(), Node::Object(address));

        let mut root = HashMap::new();
        root.insert("profile".to_string(), Node::Object(profile));
        root.insert("active".to_string(), Node::Boolean(true));

        let mut dest = BufferDestination::new();
        stringify(&Node::Object(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "active = true\n[profile]\nage = 30\nname = \"Alice\"\n[profile.address]\ncity = \"Paris\"\nzip = 75000\n"
        );
    }

    #[test]
    fn test_stringify_simple_array_of_tables() {
        let mut table1 = HashMap::new();
        table1.insert("name".to_string(), Node::Str("First".to_string()));
        let mut table2 = HashMap::new();
        table2.insert("name".to_string(), Node::Str("Second".to_string()));

        let array = vec![Node::Object(table1), Node::Object(table2)];
        let mut root = HashMap::new();
        root.insert("items".to_string(), Node::Array(array));

        let mut dest = BufferDestination::new();
        stringify(&Node::Object(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "[[items]]\nname = \"First\"\n[[items]]\nname = \"Second\"\n"
        );
    }

    #[test]
    fn test_stringify_nested_array_of_tables() {
        let mut batter1 = HashMap::new();
        batter1.insert("id".to_string(), Node::Str("1001".to_string()));
        batter1.insert("type".to_string(), Node::Str("Regular".to_string()));

        let mut batter2 = HashMap::new();
        batter2.insert("id".to_string(), Node::Str("1002".to_string()));
        batter2.insert("type".to_string(), Node::Str("Chocolate".to_string()));

        let batters = vec![Node::Object(batter1), Node::Object(batter2)];
        let mut batters_obj = HashMap::new();
        batters_obj.insert("batter".to_string(), Node::Array(batters));

        let mut item = HashMap::new();
        item.insert("batters".to_string(), Node::Object(batters_obj));
        item.insert("name".to_string(), Node::Str("Cake".to_string()));

        let items = vec![Node::Object(item)];
        let mut root = HashMap::new();
        root.insert("items".to_string(), Node::Array(items));

        let mut dest = BufferDestination::new();
        stringify(&Node::Object(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "[[items]]\nname = \"Cake\"\n[[items.batters.batter]]\nid = \"1001\"\ntype = \"Regular\"\n[[items.batters.batter]]\nid = \"1002\"\ntype = \"Chocolate\"\n"
        );
    }

    #[test]
    fn test_stringify_complex_array_of_tables() {
        let mut code1 = HashMap::new();
        code1.insert("hex".to_string(), Node::Str("#000".to_string()));
        code1.insert("rgba".to_string(), Node::Array(vec![
            Node::Number(Numeric::Integer(0)),
            Node::Number(Numeric::Integer(0)),
            Node::Number(Numeric::Integer(0)),
            Node::Number(Numeric::Integer(1))
        ]));

        let mut color1 = HashMap::new();
        color1.insert("name".to_string(), Node::Str("black".to_string()));
        color1.insert("code".to_string(), Node::Object(code1));

        let colors = vec![Node::Object(color1)];
        let mut root = HashMap::new();
        root.insert("colors".to_string(), Node::Array(colors));

        let mut dest = BufferDestination::new();
        stringify(&Node::Object(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "[[colors]]\nname = \"black\"\n[colors.code]\nhex = \"#000\"\nrgba = [0, 0, 0, 1]\n"
        );
    }

}