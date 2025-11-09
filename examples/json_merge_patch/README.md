# JSON Merge and Patch

Demonstrates JSON merging and patching operations for configuration management, deep merging, and applying overlays.

## Features Demonstrated

- **Simple object merge** combining two objects
- **Deep merge** with nested objects (recursive merging)
- **Environment-specific configuration** overlays
- **Array replacement** behavior (arrays are replaced, not merged)
- **Cascading configuration layers** (defaults → user → runtime)
- **Conditional merging** based on content
- **Custom patch functions** with null-removal semantics

## Usage

```bash
cargo run
```

## Key Concepts

### Basic Merge

```rust
let mut base = Node::Object({
    let mut map = HashMap::new();
    map.insert("name".to_string(), Node::from("MyApp"));
    map.insert("version".to_string(), Node::from("1.0.0"));
    map
});

let overlay = Node::Object({
    let mut map = HashMap::new();
    map.insert("version".to_string(), Node::from("1.1.0"));
    map.insert("author".to_string(), Node::from("Alice"));
    map
});

base.merge(overlay);
// Result: {"name": "MyApp", "version": "1.1.0", "author": "Alice"}
```

### Deep Merge

When both objects have the same key:
- If both values are objects → recursively merge them
- Otherwise → value from overlay overwrites base

```rust
let mut config1 = parse_json(r#"
{
    "server": {
        "host": "localhost",
        "port": 8080
    }
}
"#);

let config2 = parse_json(r#"
{
    "server": {
        "port": 9090,
        "ssl": true
    }
}
"#);

config1.merge(config2);
// Result: {
//     "server": {
//         "host": "localhost",
//         "port": 9090,
//         "ssl": true
//     }
// }
```

### Environment Configurations

```rust
// Base configuration for all environments
let base_config = load_config("config.json");

// Environment-specific overrides
let mut final_config = base_config.deep_clone();
match environment {
    "development" => final_config.merge(load_config("config.dev.json")),
    "production" => final_config.merge(load_config("config.prod.json")),
    _ => {}
}
```

### Cascading Layers

```rust
let mut config = defaults;      // Layer 1: System defaults
config.merge(user_config);      // Layer 2: User preferences
config.merge(runtime_overrides);// Layer 3: Runtime settings
// Later layers override earlier ones
```

### Array Behavior

Arrays are **replaced**, not merged:

```rust
let mut node = {"items": [1, 2, 3]};
node.merge({"items": [10, 20]});
// Result: {"items": [10, 20]}  (not [1, 2, 3, 10, 20])
```

### Custom Patch with Null Removal

```rust
fn apply_patch(target: &mut Node, patch: &Node) {
    if let (Some(target_obj), Some(patch_obj)) = 
        (target.as_object_mut(), patch.as_object()) {
        
        for (key, value) in patch_obj {
            if value.is_null() {
                // null means remove the key
                target_obj.remove(key);
            } else {
                target_obj.entry(key.clone())
                    .and_modify(|existing| {
                        if existing.is_object() && value.is_object() {
                            existing.merge(value.clone());
                        } else {
                            *existing = value.clone();
                        }
                    })
                    .or_insert(value.clone());
            }
        }
    }
}
```

### Conditional Merge

```rust
// Only merge if certain conditions are met
let should_apply = target.get("status")
    .and_then(|s| s.as_str())
    .map(|s| s == "active")
    .unwrap_or(false);

if should_apply {
    target.merge(update);
}
```

## Use Cases

- **Configuration management**: Combine default, user, and runtime configurations
- **Feature flags**: Apply different settings based on environment
- **API responses**: Merge partial updates with existing data
- **Settings inheritance**: Override system defaults with user preferences
- **Multi-tenant applications**: Apply tenant-specific overrides to base configuration

## Important Notes

- **Arrays are replaced**: The entire array from the overlay replaces the base array
- **Null handling**: By default, `merge()` treats null as a value. Use custom patch functions for null-as-delete semantics
- **Deep clone**: Use `deep_clone()` when you need to preserve the original object

## Learn More

- Library documentation: `json_lib::Node::merge`, `json_lib::Node::deep_clone`
- [JSON Merge Patch (RFC 7386)](https://datatracker.ietf.org/doc/html/rfc7386)
