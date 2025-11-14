use json_lib::{from_str, Node};
use std::str::FromStr;

fn main() {
    println!("=== Phase 8 API Improvements Demo ===\n");

    // 1. Index trait - bracket notation
    println!("1. Index Trait (bracket notation):");
    let json = from_str(r#"{"user": {"name": "Alice", "scores": [85, 92, 78]}}"#).unwrap();
    println!(
        "  json[\"user\"][\"name\"] = {:?}",
        json["user"]["name"].as_str()
    );
    println!(
        "  json[\"user\"][\"scores\"][0] = {:?}",
        json["user"]["scores"][0].as_i64()
    );
    println!();

    // 2. FromStr trait - parse() method
    println!("2. FromStr Trait (.parse() method):");
    let node: Node = r#"{"age": 30, "active": true}"#.parse().unwrap();
    println!("  Parsed using .parse(): {:?}", node);
    println!("  age = {:?}", node["age"].as_i64());
    println!();

    // 3. Convenience parsing functions
    println!("3. Convenience Parsing Functions:");
    let from_string = from_str(r#"{"method": "from_str"}"#).unwrap();
    println!("  from_str result: {:?}", from_string);

    let bytes = br#"{"method": "from_bytes"}"#;
    let from_byte_slice = json_lib::from_bytes(bytes).unwrap();
    println!("  from_bytes result: {:?}", from_byte_slice);
    println!();

    // 4. Default trait
    println!("4. Default Trait:");
    let default_node = Node::default();
    println!("  Node::default() = {:?}", default_node);
    println!("  Is null: {}", default_node.is_null());
    println!();

    // 5. take() method
    println!("5. take() Method:");
    let mut data = from_str(r#"{"x": 42, "y": 100}"#).unwrap();
    println!("  Before take: {:?}", data["x"]);

    if let Some(x_ref) = data.get_mut("x") {
        let old_value = x_ref.take();
        println!("  Taken value: {:?}", old_value);
        println!("  After take: {:?}", data["x"]);
        println!("  x is now null: {}", data["x"].is_null());
    }
    println!();

    // 6. Combining features - ergonomic JSON manipulation
    println!("6. Combined Example - Ergonomic JSON Manipulation:");
    let mut config: Node = r#"{
        "server": {
            "host": "localhost",
            "port": 8080
        },
        "users": ["alice", "bob", "charlie"]
    }"#
    .parse()
    .unwrap();

    println!(
        "  Server host: {}",
        config["server"]["host"].as_str().unwrap()
    );
    println!(
        "  Server port: {}",
        config["server"]["port"].as_i64().unwrap()
    );
    println!("  First user: {}", config["users"][0].as_str().unwrap());

    // Modify using bracket notation
    if let Some(port) = config["server"].get_mut("port") {
        *port = Node::from(9090);
    }
    println!(
        "  Updated port: {}",
        config["server"]["port"].as_i64().unwrap()
    );

    // Take and replace
    if let Some(host) = config["server"].get_mut("host") {
        let old_host = host.take();
        *host = Node::from("0.0.0.0");
        println!(
            "  Old host: {:?}, New host: {}",
            old_host.as_str(),
            host.as_str().unwrap()
        );
    }
    println!();

    // 7. Error handling improvements
    println!("7. Error Handling:");
    match Node::from_str(r#"{"broken": json}"#) {
        Ok(_) => println!("  Unexpectedly succeeded"),
        Err(e) => println!("  Parse error: {}", e),
    }
    println!();

    // 8. Default in struct
    #[derive(Default)]
    struct Config {
        settings: Node,
    }

    let cfg = Config::default();
    println!("8. Default in Struct:");
    println!("  Config.settings is null: {}", cfg.settings.is_null());

    println!("\n=== Demo Complete ===");
}
