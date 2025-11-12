# JSON IoT Sensor Example

This example demonstrates practical patterns for using JSON in embedded IoT and sensor applications.

## Features Demonstrated

1. **Simple Sensor Readings** - Single value with timestamp
2. **Multi-Value Readings** - Multiple sensor values in one reading
3. **Batch Processing** - Efficient data transmission by batching
4. **Memory-Efficient Construction** - Using builders with capacity
5. **Configuration Management** - Device settings and parameters
6. **Memory-Limited Parsing** - Safe parsing with resource constraints
7. **Sensor Data Arrays** - Time-series data
8. **Complex Payloads** - Nested structures with status information
9. **Memory Analysis** - Calculating memory requirements

## Running the Example

```bash
cargo run --example json_iot_sensor
```

## Embedded Use Cases

### Temperature Sensor
```rust
let reading = sensor::simple_reading("temp_01", 23.5, timestamp);
```

### Environmental Monitor
```rust
let values = vec![
    ("temperature", 23.5),
    ("humidity", 45.2),
    ("pressure", 1013.25),
];
let reading = sensor::multi_reading("env_01", &values, timestamp);
```

### Motion Detector
```rust
let event = ObjectBuilder::new()
    .add_str("device", "motion_01")
    .add_bool("detected", true)
    .add_i64("timestamp", timestamp)
    .build();
```

## Memory Optimization Techniques

1. **Pre-allocate capacity** when size is known
2. **Reuse buffers** for repeated operations
3. **Use strict parsing** on constrained devices
4. **Batch readings** to reduce per-message overhead
5. **Choose appropriate types** (i32 vs i64, f32 vs f64)

## Platform-Specific Recommendations

### High-Power Devices (ESP32, STM32F4)
- Use default parser config
- No special memory constraints
- Can handle complex nested structures

### Low-Power Devices (STM32F1, nRF52)
- Use `ParserConfig::strict()`
- Pre-allocate buffers
- Batch transmissions to conserve power

### Ultra-Low-Power (ATmega, MSP430)
- Use `ParserConfig::strict().with_max_depth(Some(4))`
- Minimize JSON nesting
- Use fixed-size buffers
- Consider binary protocols for high-frequency data

## Common Patterns

### Device Configuration
```rust
let config = config::simple()
    .add_str("wifi_ssid", "Network")
    .add_i32("sample_rate", 1000)
    .add_bool("enabled", true)
    .build();
```

### Reading Configuration
```rust
let ssid = config::get_string(&config, "wifi_ssid");
let rate = config::get_i32(&config, "sample_rate");
let enabled = config::get_bool(&config, "enabled");
```

### Batched Transmission
```rust
let readings = vec![
    sensor::simple_reading("temp", 23.5, t1),
    sensor::simple_reading("temp", 23.7, t2),
    sensor::simple_reading("temp", 23.4, t3),
];
let batch = sensor::batch_readings("device_id", readings);
```
