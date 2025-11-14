use json_lib::{json, merge_patch, patch, schema, json5};

fn main() {
    println!("=== Phase 13 Advanced Features Demo ===\n");

    // 1. JSON Schema Validation
    println!("1. JSON Schema Validation:");
    
    let user_schema = json!({
        "type": "object",
        "required": ["name", "email", "age"],
        "properties": {
            "name": {
                "type": "string",
                "minLength": 1,
                "maxLength": 100
            },
            "email": {
                "type": "string",
                "minLength": 5
            },
            "age": {
                "type": "number",
                "minimum": 0,
                "maximum": 150
            },
            "role": {
                "type": "string",
                "enum": ["admin", "user", "guest"]
            }
        }
    });
    
    let validator = schema::SchemaValidator::new(user_schema);
    
    // Valid user
    let valid_user = json!({
        "name": "Alice",
        "email": "alice@example.com",
        "age": 30,
        "role": "admin"
    });
    
    match validator.validate(&valid_user) {
        Ok(_) => println!("  ✓ Valid user passed validation"),
        Err(errors) => {
            println!("  ✗ Validation errors:");
            for error in errors {
                println!("    - {}: {}", error.path, error.message);
            }
        }
    }
    
    // Invalid user (missing required field)
    let invalid_user = json!({
        "name": "Bob",
        "email": "bob@example.com"
    });
    
    match validator.validate(&invalid_user) {
        Ok(_) => println!("  ✓ User passed validation"),
        Err(errors) => {
            println!("  ✗ Invalid user - missing 'age' field:");
            for error in errors {
                println!("    - {}: {}", error.path, error.message);
            }
        }
    }
    
    // Invalid age
    let invalid_age = json!({
        "name": "Charlie",
        "email": "charlie@example.com",
        "age": 200
    });
    
    match validator.validate(&invalid_age) {
        Ok(_) => println!("  ✓ User passed validation"),
        Err(errors) => {
            println!("  ✗ Invalid age (200 > 150):");
            for error in errors {
                println!("    - {}: {}", error.path, error.message);
            }
        }
    }
    println!();

    // 2. JSON Patch (RFC 6902)
    println!("2. JSON Patch (RFC 6902):");
    
    let mut document = json!({
        "user": {
            "name": "Alice",
            "age": 30
        },
        "settings": {
            "theme": "dark"
        }
    });
    
    println!("  Original: {}", document);
    
    // Add operation
    let add_op = patch::PatchOp::Add {
        path: "/user/email".to_string(),
        value: json!("alice@example.com"),
    };
    patch::apply_operation(&mut document, &add_op).unwrap();
    println!("  After add email: {}", document);
    
    // Replace operation
    let replace_op = patch::PatchOp::Replace {
        path: "/user/age".to_string(),
        value: json!(31),
    };
    patch::apply_operation(&mut document, &replace_op).unwrap();
    println!("  After replace age: {}", document);
    
    // Copy operation
    let copy_op = patch::PatchOp::Copy {
        from: "/settings/theme".to_string(),
        path: "/user/preferred_theme".to_string(),
    };
    patch::apply_operation(&mut document, &copy_op).unwrap();
    println!("  After copy theme: {}", document);
    
    // Remove operation
    let remove_op = patch::PatchOp::Remove {
        path: "/settings".to_string(),
    };
    patch::apply_operation(&mut document, &remove_op).unwrap();
    println!("  After remove settings: {}", document);
    println!();

    // 3. JSON Merge Patch (RFC 7386)
    println!("3. JSON Merge Patch (RFC 7386):");
    
    let mut config = json!({
        "server": {
            "host": "localhost",
            "port": 8080,
            "ssl": false
        },
        "database": {
            "url": "postgres://localhost/db",
            "pool": 10
        }
    });
    
    println!("  Original config: {}", config);
    
    // Update server port and add timeout, remove ssl
    let patch = json!({
        "server": {
            "port": 9090,
            "ssl": null,
            "timeout": 30
        }
    });
    
    merge_patch::merge_patch(&mut config, &patch);
    println!("  After merge patch: {}", config);
    println!();

    // 4. Create Merge Patch
    println!("4. Create Merge Patch:");
    
    let source = json!({
        "name": "Alice",
        "age": 30,
        "email": "alice@old.com"
    });
    
    let target = json!({
        "name": "Alice",
        "age": 31,
        "phone": "555-1234"
    });
    
    let diff = merge_patch::create_merge_patch(&source, &target);
    println!("  Source: {}", source);
    println!("  Target: {}", target);
    println!("  Generated patch: {}", diff);
    
    // Apply the patch to verify
    let mut test = source.clone();
    merge_patch::merge_patch(&mut test, &diff);
    println!("  Applied patch result: {}", test);
    println!();

    // 5. JSON5 Comments
    println!("5. JSON5 Comments Support:");
    
    let json5_input = r#"{
        // Server configuration
        "host": "localhost",
        "port": 8080, // Default port
        
        /* Database settings
           with multi-line comment */
        "database": {
            "url": "postgres://localhost/mydb",
            "pool": 10
        }
    }"#;
    
    println!("  Input with comments:");
    println!("{}", json5_input);
    
    match json5::parse_json5(json5_input) {
        Ok(parsed) => {
            println!("\n  Parsed successfully:");
            println!("  {}", parsed);
            println!("  host: {}", parsed["host"].as_str().unwrap());
            println!("  port: {}", parsed["port"].as_i64().unwrap());
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // 6. Array Schema Validation
    println!("6. Array Schema Validation:");
    
    let array_schema = json!({
        "type": "array",
        "items": {
            "type": "number",
            "minimum": 0,
            "maximum": 100
        },
        "minItems": 2,
        "maxItems": 5,
        "uniqueItems": true
    });
    
    let validator = schema::SchemaValidator::new(array_schema);
    
    let valid_array = json!([10, 20, 30]);
    match validator.validate(&valid_array) {
        Ok(_) => println!("  ✓ Valid array [10, 20, 30]"),
        Err(_) => println!("  ✗ Array validation failed"),
    }
    
    let invalid_array = json!([10, 20, 30, 40, 50, 60]);
    match validator.validate(&invalid_array) {
        Ok(_) => println!("  ✓ Array passed validation"),
        Err(errors) => {
            println!("  ✗ Too many items [10, 20, 30, 40, 50, 60]:");
            for error in errors {
                println!("    - {}", error.message);
            }
        }
    }
    println!();

    // 7. Combined Example: API with Validation and Patching
    println!("7. Combined Example: API Request Processing:");
    
    // Define API request schema
    let request_schema = json!({
        "type": "object",
        "required": ["method", "path"],
        "properties": {
            "method": {
                "type": "string",
                "enum": ["GET", "POST", "PUT", "DELETE"]
            },
            "path": {
                "type": "string",
                "minLength": 1
            },
            "body": {
                "type": "object"
            }
        }
    });
    
    let validator = schema::SchemaValidator::new(request_schema);
    
    // Valid request
    let request = json!({
        "method": "POST",
        "path": "/api/users",
        "body": {
            "name": "Alice",
            "email": "alice@example.com"
        }
    });
    
    println!("  Request: {}", request);
    match validator.validate(&request) {
        Ok(_) => println!("  ✓ Request validated"),
        Err(e) => println!("  ✗ Validation failed: {:?}", e),
    }
    
    // Apply patch to modify request
    let mut modified_request = request.clone();
    let patch_op = patch::PatchOp::Replace {
        path: "/method".to_string(),
        value: json!("PUT"),
    };
    patch::apply_operation(&mut modified_request, &patch_op).unwrap();
    
    println!("  Modified request: {}", modified_request);
    match validator.validate(&modified_request) {
        Ok(_) => println!("  ✓ Modified request still valid"),
        Err(_) => println!("  ✗ Modified request invalid"),
    }

    println!("\n=== Demo Complete ===");
}
