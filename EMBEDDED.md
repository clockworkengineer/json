# Embedded Systems Support

json_lib provides comprehensive support for embedded systems and resource-constrained devices.

## Quick Start

### Add to your embedded project:

```toml
[dependencies]
json_lib = { version = "0.1.6", default-features = false, features = ["alloc"] }
```

### Simple sensor reading example:

```rust
use json_lib::embedded::sensor;

let reading = sensor::simple_reading("temp_01", 23.5, 1234567890);
// Creates: {"sensor_id":"temp_01","value":23.5,"timestamp":1234567890}
```

## Key Features

### ✅ no_std Compatible
- Works with `default-features = false, features = ["alloc"]`
- Minimal dependencies
- Tested on embedded platforms

### ⚙️ Configurable Memory Limits
```rust
use json_lib::ParserConfig;

// For devices with 16-64KB RAM
let config = ParserConfig::strict();
// Max depth: 16, max string: 256 bytes, max array: 64, max object: 32

let mut source = BufferSource::new(json_data);
let node = parse_with_config(&mut source, &config)?;
```

### 🛠️ Builder APIs
```rust
use json_lib::embedded::{ObjectBuilder, ArrayBuilder};

let config = ObjectBuilder::with_capacity(3)
    .add_str("device", "sensor_01")
    .add_i32("rate", 1000)
    .add_bool("enabled", true)
    .build();
```

### 📊 Memory Estimation
```rust
use json_lib::embedded::memory;

let size = memory::estimate_node_size(&node);
println!("Memory usage: {} bytes", size);
```

## Platform Support

| Platform | RAM | Config | Status |
|----------|-----|--------|--------|
| ESP32 | 520KB | Default | ✅ Excellent |
| STM32F4 | 192KB | Default | ✅ Excellent |
| STM32F1 | 20KB | Strict | ✅ Good |
| nRF52 | 64KB | Strict | ✅ Good |
| ATmega328P | 2KB | Ultra-strict | ⚠️ Limited |

## Performance

**Parsing:** 468K ops/sec (small JSON), 114K ops/sec (medium JSON)

**Construction:** 968K ops/sec (objects), 3.9M ops/sec (arrays)

**Memory:** 56 bytes per Node, predictable allocation patterns

## Documentation

- **[Embedded Programming Guide](EMBEDDED_GUIDE.md)** - Comprehensive usage guide
- **[Validation Report](EMBEDDED_VALIDATION.md)** - Test results and platform recommendations
- **[IoT Sensor Example](examples/json_iot_sensor/)** - Practical patterns
- **[Benchmarks](examples/json_benchmarks/)** - Performance measurements

## Examples

### IoT Sensor Data
```rust
use json_lib::embedded::sensor;

// Single reading
let reading = sensor::simple_reading("temp_01", 23.5, 1234567890);

// Multi-value reading
let values = vec![("temp", 23.5), ("humidity", 45.2)];
let reading = sensor::multi_reading("env_01", &values, 1234567890);

// Batch for efficient transmission
let batch = sensor::batch_readings("device_id", vec![r1, r2, r3]);
```

### Configuration Management
```rust
use json_lib::embedded::config;

// Create configuration
let cfg = config::simple()
    .add_str("wifi_ssid", "Network")
    .add_i32("sample_rate", 1000)
    .add_bool("enabled", true)
    .build();

// Read configuration
let ssid = config::get_string(&cfg, "wifi_ssid");
let rate = config::get_i32(&cfg, "sample_rate");
```

## Memory Guidelines

**Rule of Thumb:** Allocate 3-5x the JSON text size for parsed structures.

**Example calculations:**
- Small (50 bytes JSON) → ~200 bytes RAM
- Medium (500 bytes JSON) → ~2KB RAM  
- Large (2KB JSON) → ~8KB RAM

## Best Practices

✅ **Do:**
- Use `ParserConfig` appropriate for your platform
- Pre-allocate with `with_capacity()` when size is known
- Batch sensor readings for transmission
- Estimate memory requirements during development

❌ **Don't:**
- Use `ParserConfig::unlimited()` on embedded
- Create deeply nested structures (>4 levels on constrained devices)
- Ignore parse errors
- Allocate in interrupt handlers

## Getting Started

1. **Read the [Embedded Guide](EMBEDDED_GUIDE.md)** for platform-specific advice
2. **Review the [IoT Example](examples/json_iot_sensor/)** for practical patterns
3. **Run [Benchmarks](examples/json_benchmarks/)** to measure performance
4. **Check [Validation Report](EMBEDDED_VALIDATION.md)** for your platform

## Support

For embedded-specific questions:
1. Check documentation first
2. Review examples
3. Open an issue with platform details (MCU, RAM, config)

## License

Same as json_lib - see main README.md
