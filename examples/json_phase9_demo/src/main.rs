use json_lib::{json, Node, ParseError};
use std::collections::HashMap;

fn main() {
    println!("=== Phase 9 Advanced Features Demo ===\n");

    // 1. json! macro - The killer feature
    println!("1. json! Macro:");

    let simple = json!({
        "name": "Alice",
        "age": 30,
        "active": true
    });
    println!("  Simple object: {:?}", simple);
    println!("  Name: {}", simple["name"].as_str().unwrap());

    let nested = json!({
        "user": {
            "name": "Bob",
            "email": "bob@example.com",
            "preferences": {
                "theme": "dark",
                "notifications": true
            }
        },
        "scores": [85, 92, 78, 95],
        "metadata": {
            "version": "1.0",
            "timestamp": 1699999999
        }
    });
    println!(
        "  Nested access: user.name = {}",
        nested["user"]["name"].as_str().unwrap()
    );
    println!(
        "  Array access: scores[2] = {}",
        nested["scores"][2].as_i64().unwrap()
    );
    println!();

    // Using variables in json! macro
    let username = "Charlie";
    let score = 88;
    let user_data = json!({
        "username": username,
        "score": score,
        "timestamp": 1700000000
    });
    println!("  With variables: {:?}", user_data);
    println!();

    // 2. Structured ParseError
    println!("2. Structured ParseError:");

    let err1 = ParseError::syntax("Missing closing brace", Some(5), Some(10));
    println!("  Syntax error: {}", err1);

    let err2 = ParseError::unexpected_char('}', "comma or value");
    println!("  Unexpected char: {}", err2);

    let err3 = ParseError::limit_exceeded("array size", 1000);
    println!("  Limit exceeded: {}", err3);

    // Pattern matching on structured errors
    match ParseError::unexpected_eof("closing bracket") {
        ParseError::UnexpectedEof { expected } => {
            println!("  EOF error, expected: {}", expected);
        }
        _ => println!("  Other error"),
    }
    println!();

    // 3. Extended From implementations
    println!("3. Extended From Implementations:");

    // From fixed-size array
    let arr_node: Node = [1, 2, 3, 4, 5].into();
    println!("  From [i32; 5]: {:?}", arr_node);

    // From Vec of strings
    let vec_strings = vec!["hello", "world"];
    let vec_node: Node = vec_strings.into();
    println!("  From Vec<&str>: {:?}", vec_node);

    // From Option
    let some_val: Node = Some(42).into();
    let none_val: Node = Option::<i32>::None.into();
    println!("  From Some(42): {:?}", some_val);
    println!(
        "  From None: {:?} (is_null: {})",
        none_val,
        none_val.is_null()
    );

    // From HashMap
    let mut map = HashMap::new();
    map.insert("x".to_string(), 10);
    map.insert("y".to_string(), 20);
    let map_node: Node = map.into();
    println!("  From HashMap: {:?}", map_node);
    println!();

    // 4. as_array() / as_object() extractors (already existed!)
    println!("4. Direct Collection Access:");

    let array = json!([1, 2, 3, 4, 5]);
    if let Some(vec) = array.as_array() {
        println!("  Array length: {}", vec.len());
        println!("  Direct vec access: {:?}", vec);
    }

    let object = json!({"a": 1, "b": 2, "c": 3});
    if let Some(map) = object.as_object() {
        println!("  Object keys count: {}", map.len());
        for (key, value) in map {
            println!("    {}: {}", key, value.as_i64().unwrap());
        }
    }
    println!();

    // 5. Combining all features
    println!("5. Combined Example:");

    let config = json!({
        "server": {
            "host": "0.0.0.0",
            "port": 8080,
            "workers": 4
        },
        "database": {
            "url": "postgres://localhost/mydb",
            "pool_size": 10
        },
        "features": ["auth", "api", "websockets"],
        "debug": true
    });

    // Safe access with pattern matching
    if let Some(features) = config["features"].as_array() {
        println!("  Enabled features:");
        for feature in features {
            if let Some(name) = feature.as_str() {
                println!("    - {}", name);
            }
        }
    }

    // Direct HashMap manipulation
    if let Some(server) = config["server"].as_object() {
        println!("  Server configuration:");
        println!(
            "    Host: {}",
            server.get("host").and_then(|n| n.as_str()).unwrap()
        );
        println!(
            "    Port: {}",
            server.get("port").and_then(|n| n.as_i64()).unwrap()
        );
    }
    println!();

    // 6. json! macro with complex nesting
    println!("6. Complex JSON Construction:");

    // Build nested structure piece by piece
    let user1 = json!({
        "id": 1,
        "name": "Alice",
        "roles": ["admin", "user"]
    });

    let user2 = json!({
        "id": 2,
        "name": "Bob",
        "roles": ["user"]
    });

    let api_response = json!({
        "status": "success",
        "data": {
            "users": [user1, user2],
            "total": 2,
            "page": 1
        },
        "meta": {
            "timestamp": 1700000000,
            "version": "2.0"
        }
    });

    // Navigate deep structure
    if let Some(users) = api_response["data"]["users"].as_array() {
        println!("  Total users: {}", users.len());
        for (i, user) in users.iter().enumerate() {
            println!("  User {}: {}", i + 1, user["name"].as_str().unwrap());
            if let Some(roles) = user["roles"].as_array() {
                print!("    Roles: ");
                for role in roles {
                    print!("{} ", role.as_str().unwrap());
                }
                println!();
            }
        }
    }

    println!("\n=== Demo Complete ===");
}
