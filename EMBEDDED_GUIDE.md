# Embedded Systems Programming Guide

This guide provides best practices and patterns for using the json_lib library on embedded systems and resource-constrained devices.

## Table of Contents

1. [Platform Requirements](#platform-requirements)
2. [Memory Management](#memory-management)
3. [Parser Configuration](#parser-configuration)
4. [Efficient JSON Construction](#efficient-json-construction)
5. [Error Handling](#error-handling)
6. [Platform-Specific Examples](#platform-specific-examples)
7. [Performance Optimization](#performance-optimization)
8. [Power Consumption](#power-consumption)

## Platform Requirements

### Minimum Requirements

- **RAM**: 8KB minimum (with strict configuration)
- **Flash**: 20KB for core library + selected features
- **CPU**: Any 32-bit MCU (Cortex-M0+, RISC-V, etc.)
- **Rust**: MSRV 1.88.0
- **alloc**: Required (need heap allocator)

### Tested Platforms

| Platform | RAM | Status | Recommended Config |
|----------|-----|--------|-------------------|
| ESP32 | 520KB | ✅ Excellent | Default |
| STM32F4 | 192KB | ✅ Excellent | Default |
| STM32F1 | 20KB | ✅ Good | Strict |
| nRF52 | 64KB | ✅ Good | Strict or Custom |
| ATmega328P | 2KB | ⚠️ Limited | Ultra-strict |

## Memory Management

### Understanding Memory Usage

The library uses heap allocation for dynamic JSON structures. Key sizes:

```rust
use json_lib::embedded::memory;

// Core type sizes
println!("Node: {} bytes", memory::node_size());      // 56 bytes
println!("Numeric: {} bytes", memory::numeric_size()); // 16 bytes
```

### Memory Calculation Example

For a configuration object:
```json
{
  "device": "sensor_01",
  "rate": 1000,
  "enabled": true
}
```

Approximate memory:
- Base Node: 56 bytes
- HashMap overhead: ~150 bytes
- 3 keys (strings): ~60 bytes
- 3 values: ~170 bytes
- **Total: ~436 bytes**

### Memory Estimation API

```rust
use json_lib::embedded::memory;

let node = /* your JSON structure */;
let size = memory::estimate_node_size(&node);
println!("Estimated memory: {} bytes", size);
```

### Memory Budgeting

**Rule of Thumb**: Allocate 3-5x the JSON text size for parsed structures.

Example: 100-byte JSON → 300-500 bytes RAM

## Parser Configuration

### Configuration Presets

#### Default (Embedded-Friendly)
```rust
use json_lib::ParserConfig;

let config = ParserConfig::new();
// Max depth: 32
// Max string: 4KB
// Max array: 1024
// Max object: 256
```

**Best for**: ESP32, STM32F4, devices with >64KB RAM

#### Strict (Low-Memory)
```rust
let config = ParserConfig::strict();
// Max depth: 16
// Max string: 256 bytes
// Max array: 64
// Max object: 32
```

**Best for**: STM32F1, nRF52, devices with 16-64KB RAM

#### Ultra-Strict (Ultra-Low-Memory)
```rust
let config = ParserConfig::strict()
    .with_max_depth(Some(8))
    .with_max_string_length(Some(64))
    .with_max_array_size(Some(16))
    .with_max_object_size(Some(8));
```

**Best for**: ATmega328P, MSP430, devices with <16KB RAM

### Custom Configuration

Tune based on your specific JSON schemas:

```rust
// IoT sensor device with simple JSON
let config = ParserConfig::new()
    .with_max_depth(Some(4))           // Shallow nesting
    .with_max_string_length(Some(128))  // Short strings
    .with_max_array_size(Some(50))      // Small batches
    .with_max_object_size(Some(20));    // Few fields

// Configuration file parser
let config = ParserConfig::new()
    .with_max_depth(Some(8))
    .with_max_string_length(Some(512))  // Longer config values
    .with_max_array_size(Some(100))
    .with_max_object_size(Some(50));
```

### Worst-Case Memory Calculation

With strict config:
```
Per nesting level:
  - Max string: 256 bytes
  - Max array: 64 × 56 = 3,584 bytes
  - Max object: 32 × 80 ≈ 2,560 bytes
  - Level total: ~6.4KB

Total (16 levels): ~102KB worst case
```

## Efficient JSON Construction

### Use Builders

**Bad** (multiple allocations):
```rust
let mut map = HashMap::new();
map.insert("key1".to_string(), Node::Str("value1".to_string()));
map.insert("key2".to_string(), Node::Number(Numeric::Int32(42)));
let node = Node::Object(map);
```

**Good** (fluent API, pre-allocation):
```rust
use json_lib::embedded::ObjectBuilder;

let node = ObjectBuilder::with_capacity(2)
    .add_str("key1", "value1")
    .add_i32("key2", 42)
    .build();
```

### Sensor Reading Pattern

```rust
use json_lib::embedded::sensor;

// Simple reading
let reading = sensor::simple_reading("temp_01", 23.5, timestamp);

// Multi-value reading
let values = vec![("temp", 23.5), ("humidity", 45.2)];
let reading = sensor::multi_reading("env_01", &values, timestamp);

// Batch for transmission
let batch = sensor::batch_readings("device_id", vec![r1, r2, r3]);
```

### Configuration Management

```rust
use json_lib::embedded::config;

// Creating configuration
let cfg = config::simple()
    .add_str("wifi_ssid", "Network")
    .add_i32("sample_rate", 1000)
    .add_bool("enabled", true)
    .build();

// Reading configuration
let ssid = config::get_string(&cfg, "wifi_ssid");
let rate = config::get_i32(&cfg, "sample_rate");
let enabled = config::get_bool(&cfg, "enabled");
```

## Error Handling

### Safe Parsing

Always handle parse errors:

```rust
use json_lib::{parse_with_config, ParserConfig, BufferSource};

let config = ParserConfig::strict();
let mut source = BufferSource::new(json_data);

match parse_with_config(&mut source, &config) {
    Ok(node) => {
        // Process node
    }
    Err(e) => {
        // Handle error - log, retry, use defaults
        eprintln!("Parse error: {}", e);
    }
}
```

### Safe Access

Use safe methods instead of indexing:

```rust
// Safe object access
if let Some(value) = node.get("key") {
    // Use value
}

// Safe array access
if let Some(elem) = node.at(0) {
    // Use element
}

// Type-safe extraction
if let Some(s) = node.as_str() {
    // Use string
}
```

## Platform-Specific Examples

### ESP32 (520KB RAM)

Full features, no restrictions:

```rust
use json_lib::{parse, stringify, Node, BufferSource, BufferDestination};

// Parse any JSON
let mut source = BufferSource::new(json_data);
let node = parse(&mut source)?;

// Create complex structures
let sensor_data = create_complex_payload();

// Stringify
let mut dest = BufferDestination::new();
stringify(&sensor_data, &mut dest)?;
```

### STM32F103 (20KB RAM)

Use strict config, simple structures:

```rust
use json_lib::{parse_with_config, ParserConfig, embedded::sensor};

// Strict parsing
let config = ParserConfig::strict();
let mut source = BufferSource::new(json_data);
let node = parse_with_config(&mut source, &config)?;

// Simple sensor readings
let reading = sensor::simple_reading("temp", value, timestamp);
```

### ATmega328P (2KB RAM)

Ultra-strict, minimal JSON:

```rust
use json_lib::{ParserConfig, embedded::ObjectBuilder};

// Ultra-strict config
let config = ParserConfig::strict()
    .with_max_depth(Some(4))
    .with_max_string_length(Some(64))
    .with_max_object_size(Some(8));

// Minimal structures
let data = ObjectBuilder::with_capacity(3)
    .add_str("id", "s1")
    .add_i32("v", value)
    .add_i64("t", timestamp)
    .build();
```

## Performance Optimization

### 1. Pre-allocate Buffers

```rust
// Reuse destination buffer
let mut dest = BufferDestination::new();
for reading in readings {
    dest.clear();
    stringify(&reading, &mut dest)?;
    transmit(dest.as_bytes());
}
```

### 2. Batch Operations

```rust
// Instead of sending 10 messages
for reading in readings {
    send_json(reading);  // 10 transmissions
}

// Batch into one message
let batch = sensor::batch_readings("device", readings);
send_json(batch);  // 1 transmission
```

### 3. Choose Appropriate Types

```rust
// For small values, use smaller types
ObjectBuilder::new()
    .add_i32("temperature", temp)  // i32 instead of i64
    .add_u32("battery_mv", 3300)   // u32 sufficient for mV
    .build()
```

### 4. Minimize Nesting

**Bad** (3 levels deep):
```json
{"data": {"sensor": {"value": 23.5}}}
```

**Good** (1 level):
```json
{"sensor": "temp_01", "value": 23.5}
```

### 5. Use Fixed Schemas

Design JSON schemas that fit your memory limits:

```rust
// Know your schema size
const MAX_READING_SIZE: usize = 512;

// Design to fit
let reading = ObjectBuilder::with_capacity(5)
    // ... max 5 fields
    .build();
```

## Power Consumption

### Low-Power Patterns

#### 1. Batch Before Transmit

```rust
// Collect readings in deep sleep intervals
let mut batch = Vec::new();
for _ in 0..10 {
    let reading = take_reading();
    batch.push(reading);
    deep_sleep(60_000); // 1 minute
}

// Wake up, batch, and transmit once
let payload = sensor::batch_readings("device", batch);
transmit_json(payload);
```

#### 2. Minimize JSON Size

Shorter JSON = less transmission time = less power:

```rust
// Use short keys
ObjectBuilder::new()
    .add_str("d", "device_01")  // "d" instead of "device_id"
    .add_f64("t", 23.5)         // "t" instead of "temperature"
    .add_i64("ts", timestamp)    // "ts" instead of "timestamp"
    .build()
```

#### 3. Pre-compute JSON

```rust
// Pre-compute static parts at initialization
let header = ObjectBuilder::new()
    .add_str("device", "sensor_01")
    .add_str("location", "room_A")
    .build();

// Only add dynamic data before transmission
```

## Best Practices Summary

### Do's ✅

1. Use `ParserConfig` appropriate for your platform
2. Pre-allocate with `with_capacity()` when size is known
3. Use embedded utility builders
4. Handle all errors gracefully
5. Batch data for efficient transmission
6. Estimate memory requirements during development
7. Test with realistic payloads
8. Monitor stack usage
9. Profile actual RAM usage
10. Keep JSON schemas simple and flat

### Don'ts ❌

1. Don't use `ParserConfig::unlimited()` on embedded
2. Don't create deeply nested structures (>4 levels)
3. Don't ignore parse errors
4. Don't allocate in interrupts
5. Don't use large string values unnecessarily
6. Don't create JSON during time-critical operations
7. Don't forget to test worst-case scenarios
8. Don't assume unlimited stack depth
9. Don't use f64 when f32 suffices
10. Don't over-engineer - keep it simple

## Troubleshooting

### Out of Memory

**Symptoms**: Allocation failures, crashes, resets

**Solutions**:
1. Use stricter `ParserConfig`
2. Reduce `max_array_size` and `max_object_size`
3. Simplify JSON structures
4. Increase heap size in linker script
5. Check for memory leaks

### Stack Overflow

**Symptoms**: Hard faults, random crashes during parsing

**Solutions**:
1. Reduce `max_depth` in `ParserConfig`
2. Increase stack size in linker script
3. Flatten JSON structures
4. Test with deeply nested inputs

### Parse Errors

**Symptoms**: `parse_with_config()` returns `Err`

**Check**:
1. JSON syntax validity
2. Parser config limits not exceeded
3. String encoding (UTF-8)
4. Buffer size sufficient

## Further Resources

- [IoT Sensor Example](../examples/json_iot_sensor/)
- [Parser Config Example](../examples/json_parser_config/)
- [API Documentation](https://docs.rs/json_lib)
- [GitHub Issues](https://github.com/clockworkengineer/json/issues)

## Support

For embedded-specific questions or issues:
1. Check this guide first
2. Review the examples
3. Open an issue on GitHub
4. Provide platform details (MCU, RAM, config used)
