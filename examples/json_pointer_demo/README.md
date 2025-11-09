# JSON Pointer Demo

Demonstrates JSON Pointer (RFC 6901) operations for navigating, querying, and modifying JSON structures.

## Features Demonstrated

- **Getting values** using JSON Pointer syntax (`/store/book/0/title`)
- **Setting values** and creating nested structures
- **In-place modifications** with `pointer_get_mut`
- **Removing values** from objects and arrays
- **Special character handling** in keys (using `~0` for `~` and `~1` for `/`)
- **Error handling** for invalid pointers and non-existent paths

## Usage

```bash
cargo run
```

## Key Concepts

### JSON Pointer Syntax
- Pointers start with `/`
- Each `/` separates reference tokens
- Array indices are numeric strings
- Special characters are escaped: `~0` for `~`, `~1` for `/`

### Examples from the Demo

```rust
// Get a value
pointer_get(&root, "/store/bicycle/color");

// Set a value
pointer_set(&mut root, "/store/bicycle/price", Node::Number(Numeric::Float(24.95)));

// Modify in place
if let Some(value) = pointer_get_mut(&mut root, "/store/book/0/price") {
    *value = Node::Number(Numeric::Float(9.95));
}

// Remove a value
pointer_remove(&mut root, "/store/book/0/price");
```

## Learn More

- [RFC 6901 - JSON Pointer](https://datatracker.ietf.org/doc/html/rfc6901)
- Library documentation: `json_lib::nodes::json_pointer`
