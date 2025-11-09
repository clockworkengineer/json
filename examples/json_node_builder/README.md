# JSON Node Builder

Demonstrates programmatic JSON construction using the Node API with type conversions, indexing, and builder patterns.

## Features Demonstrated

- **Creating nodes** from primitive values (strings, integers, floats, booleans)
- **Building arrays** with mixed types
- **Building objects** with HashMap
- **Nested structures** (addresses, skills, employee records)
- **Indexing operations** with `[]` operator for arrays and objects
- **Safe navigation** with `get()`, `at()`, and `as_*()` methods
- **Mutable operations** on arrays and objects
- **Type conversions** for various numeric types (i8, i16, i32, i64, u8, u16, u32, u64, float)
- **Type checking** with `is_*()` methods

## Usage

```bash
cargo run
```

## Key Concepts

### Creating Nodes

```rust
// From primitives
let str_node = Node::from("Hello");
let int_node = Node::from(42i32);
let float_node = Node::from(3.14f64);
let bool_node = Node::from(true);
let null_node = Node::None;

// Arrays
let array = Node::Array(vec![
    Node::from(1),
    Node::from(2),
    Node::from(3),
]);

// Objects
let mut map = HashMap::new();
map.insert("name".to_string(), Node::from("Alice"));
map.insert("age".to_string(), Node::from(30));
let object = Node::Object(map);
```

### Accessing Data

```rust
// Array indexing
let value = array[0];

// Object indexing
let name = object["name"];

// Safe access
if let Some(city) = object.get("address")
    .and_then(|addr| addr.get("city")) {
    println!("City: {:?}", city.as_str());
}
```

### Modification

```rust
// Modify array
if let Some(arr) = numbers.as_array_mut() {
    arr.push(Node::from(4));
    arr[0] = Node::from(100);
}

// Modify object
if let Some(obj) = config.as_object_mut() {
    obj.insert("version".to_string(), Node::from("2.0"));
}
```

## Learn More

- Library documentation: `json_lib::Node`, `json_lib::Numeric`
