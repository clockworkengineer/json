# JSON Parser Configuration Example

This example demonstrates how to use `ParserConfig` to control memory usage and prevent resource exhaustion when parsing JSON in embedded systems.

## Features Demonstrated

1. **Default Configuration** - Balanced settings for embedded systems
2. **Strict Configuration** - Ultra-constrained for low-memory devices
3. **Depth Limiting** - Prevent stack overflow from deeply nested structures
4. **String Length Limiting** - Control string memory allocations
5. **Array Size Limiting** - Bound vector allocations
6. **Object Size Limiting** - Bound map allocations
7. **Custom Configuration** - Tailored limits for specific use cases
8. **Memory Usage Estimation** - Calculate worst-case memory footprint

## Running the Example

```bash
cargo run --example json_parser_config
```

## Key Concepts

### Configuration Presets

- `ParserConfig::new()` - Default embedded-friendly limits
- `ParserConfig::strict()` - Highly constrained for ultra-low memory
- `ParserConfig::unlimited()` - No limits (desktop/server use only)

### Builder Pattern

```rust
let config = ParserConfig::new()
    .with_max_depth(Some(8))
    .with_max_string_length(Some(512))
    .with_max_array_size(Some(100))
    .with_max_object_size(Some(50));
```

### Safe Parsing

```rust
use json_lib::{parse_with_config, BufferSource, ParserConfig};

let config = ParserConfig::strict();
let mut source = BufferSource::new(json_data);
match parse_with_config(&mut source, &config) {
    Ok(node) => { /* Process node */ },
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## Platform Recommendations

### ESP32 (520 KB RAM)
```rust
let config = ParserConfig::new(); // Use defaults
```

### STM32F4 (192 KB RAM)
```rust
let config = ParserConfig::new()
    .with_max_depth(Some(16));
```

### STM32F1 (20 KB RAM)
```rust
let config = ParserConfig::strict();
```

### ATmega328P (2 KB RAM)
```rust
let config = ParserConfig::strict()
    .with_max_depth(Some(4))
    .with_max_string_length(Some(64))
    .with_max_array_size(Some(16))
    .with_max_object_size(Some(8));
```

## Error Messages

When limits are exceeded, you'll get descriptive errors:

- `"Maximum nesting depth of N exceeded"`
- `"Maximum string length of N bytes exceeded"`
- `"Maximum array size of N exceeded"`
- `"Maximum object size of N exceeded"`

## Memory Usage

The example includes calculations showing worst-case memory usage for different configurations, helping you choose appropriate limits for your platform's available RAM.
