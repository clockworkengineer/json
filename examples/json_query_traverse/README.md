# JSON Query and Traversal

Demonstrates querying and traversing JSON structures with safe navigation, searching, filtering, and aggregation.

## Features Demonstrated

- **Safe navigation** with error handling and Option chaining
- **Array traversal** with iterators
- **Deep tree traversal** to find nested elements
- **Filtering** by criteria (price, tags, stock status)
- **Searching** within arrays and objects
- **Data extraction** from nested structures
- **Statistics and aggregation** (counting, averaging)
- **Path existence checking** for optional fields
- **Custom traversal** with callback functions

## Usage

```bash
cargo run
```

## Key Concepts

### Safe Navigation

```rust
// Chain Option methods for safe access
match root.get("catalog")
    .and_then(|c| c.get("name"))
    .and_then(|n| n.as_str()) {
    Some(name) => println!("Name: {}", name),
    None => println!("Not found"),
}
```

### Traversing Arrays

```rust
if let Some(categories) = root.get("catalog")
    .and_then(|c| c.get("categories"))
    .and_then(|c| c.as_array()) {
    
    for (idx, category) in categories.iter().enumerate() {
        if let Some(name) = category.get("name").and_then(|n| n.as_str()) {
            println!("Category {}: {}", idx + 1, name);
        }
    }
}
```

### Deep Traversal

```rust
fn collect_products<'a>(node: &'a Node, products: &mut Vec<&'a Node>) {
    if let Some(arr) = node.as_array() {
        for item in arr {
            if item.get("price").is_some() {
                products.push(item);
            }
            collect_products(item, products);
        }
    } else if let Some(obj) = node.as_object() {
        for (_key, value) in obj {
            collect_products(value, products);
        }
    }
}
```

### Filtering

```rust
// Filter by boolean field
for product in &all_products {
    let in_stock = product.get("in_stock")
        .and_then(|s| s.as_bool())
        .unwrap_or(false);
    
    if in_stock {
        // Process in-stock products
    }
}

// Filter by numeric criteria
for product in &all_products {
    if let Some(price) = product.get("price").and_then(|p| p.as_number()) {
        let price_val = match price {
            Numeric::Float(f) => *f,
            Numeric::Integer(i) => *i as f64,
            _ => 0.0,
        };
        
        if price_val > 1000.0 {
            // Process expensive products
        }
    }
}
```

### Aggregation

```rust
// Calculate average
let prices: Vec<f64> = all_products.iter()
    .filter_map(|p| p.get("price").and_then(|pr| pr.as_number()))
    .filter_map(|num| match num {
        Numeric::Float(f) => Some(*f),
        Numeric::Integer(i) => Some(*i as f64),
        _ => None,
    })
    .collect();

let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
```

## Learn More

- Library documentation: `json_lib::Node`
- Rust iterators and Option combinators
