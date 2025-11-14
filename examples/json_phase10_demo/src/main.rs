use json_lib::{json, Node};

fn main() {
    println!("=== Phase 10 Quality of Life Improvements Demo ===\n");

    // 1. to_string_pretty() - Human-readable JSON
    println!("1. Pretty Printing:");
    let data = json!({
        "user": {
            "name": "Alice",
            "age": 30,
            "email": "alice@example.com"
        },
        "scores": [85, 92, 78, 95],
        "active": true
    });
    
    println!("  Compact: {}", data);
    println!("\n  Pretty (2 spaces):\n{}", data.to_string_pretty());
    println!("\n  Pretty (tabs):\n{}", data.to_string_with_indent("\t"));
    println!();

    // 2. Non-panicking Index - Safe access
    println!("2. Non-Panicking Index:");
    let obj = json!({"name": "Bob", "age": 25});
    
    // These don't panic, they return &Node::None
    println!("  obj[\"name\"]: {:?}", obj["name"].as_str());
    println!("  obj[\"missing\"] is_null: {}", obj["missing"].is_null());
    println!("  obj[\"missing\"][\"deep\"][\"path\"] is_null: {}", 
             obj["missing"]["deep"]["path"].is_null());
    
    let arr = json!([1, 2, 3]);
    println!("  arr[1]: {:?}", arr[1].as_i64());
    println!("  arr[99] is_null: {}", arr[99].is_null());
    
    // Indexing wrong type returns None
    let num = Node::from(42);
    println!("  number[\"key\"] is_null: {}", num["key"].is_null());
    println!();

    // 3. pointer() Methods - JSON Pointer as methods
    println!("3. JSON Pointer Methods:");
    let nested = json!({
        "user": {
            "profile": {
                "name": "Charlie",
                "address": {
                    "city": "NYC"
                }
            }
        },
        "scores": [10, 20, 30]
    });
    
    if let Some(name) = nested.pointer("/user/profile/name") {
        println!("  /user/profile/name: {}", name.as_str().unwrap());
    }
    
    if let Some(city) = nested.pointer("/user/profile/address/city") {
        println!("  /user/profile/address/city: {}", city.as_str().unwrap());
    }
    
    if let Some(score) = nested.pointer("/scores/1") {
        println!("  /scores/1: {}", score.as_i64().unwrap());
    }
    
    // Mutable pointer
    let mut data_mut = json!({"x": 10, "y": 20});
    if let Some(x) = data_mut.pointer_mut("/x") {
        *x = Node::from(100);
    }
    println!("  Modified x via pointer_mut: {}", data_mut["x"].as_i64().unwrap());
    println!();

    // 4. Convenience Constructors - Quick creation
    println!("4. Convenience Constructors:");
    let mut obj = Node::object();
    println!("  Node::object() is_object: {}", obj.is_object());
    
    let arr = Node::array();
    println!("  Node::array() is_array: {}", arr.is_array());
    
    let null = Node::null();
    println!("  Node::null() is_null: {}", null.is_null());
    
    // Build up an object
    obj.insert("name", "David");
    obj.insert("score", 88);
    println!("  Built object: {}", obj);
    println!();

    // 5. insert() Method - Direct insertion
    println!("5. insert() Method:");
    let mut config = Node::object();
    
    config.insert("host", "localhost");
    config.insert("port", 8080);
    config.insert("debug", true);
    
    println!("  Config: {}", config);
    
    // Can replace values
    let old = config.insert("port", 9090);
    println!("  Old port value: {:?}", old.unwrap().as_i64());
    println!("  New port: {}", config["port"].as_i64().unwrap());
    println!();

    // 6. Combined Example - Building complex structures
    println!("6. Combined Example:");
    let mut api_config = Node::object();
    
    // Build server config
    let mut server = Node::object();
    server.insert("host", "0.0.0.0");
    server.insert("port", 8080);
    api_config.insert("server", server);
    
    // Build database config
    let mut db = Node::object();
    db.insert("url", "postgres://localhost/mydb");
    db.insert("pool_size", 10);
    api_config.insert("database", db);
    
    // Add features array
    api_config.insert("features", vec!["auth", "api", "ws"]);
    
    println!("  API Config (pretty):\n{}", api_config.to_string_pretty());
    
    // Access with pointers
    if let Some(host) = api_config.pointer("/server/host") {
        println!("\n  Server host via pointer: {}", host.as_str().unwrap());
    }
    
    // Safe access to missing keys
    println!("  Missing key is_null: {}", api_config["missing"]["nested"].is_null());
    println!();

    // 7. Practical Example - Configuration management
    println!("7. Practical Configuration:");
    
    let mut app_config = json!({
        "app": {
            "name": "MyApp",
            "version": "1.0.0"
        },
        "logging": {
            "level": "info",
            "file": "/var/log/app.log"
        }
    });
    
    // Update nested values safely
    if let Some(level) = app_config.pointer_mut("/logging/level") {
        *level = Node::from("debug");
    }
    
    // Add new section
    let mut metrics = Node::object();
    metrics.insert("enabled", true);
    metrics.insert("interval", 60);
    app_config.insert("metrics", metrics);
    
    println!("{}", app_config.to_string_pretty());

    println!("\n=== Demo Complete ===");
}
