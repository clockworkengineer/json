//! Embedded Compatibility Test Library
//!
//! This test validates that the json_lib can be built for embedded systems
//! without the std library (no_std mode).
//!
//! Build with: cargo build --release -p embedded_test
//!
//! This compilation test demonstrates that the library compiles successfully
//! for no_std environments with only the alloc feature enabled.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use json_lib::embedded::{sensor, ArrayBuilder, ObjectBuilder};
use json_lib::{parse_with_config, stringify, BufferDestination, BufferSource, ParserConfig};

/// Test that sensor reading APIs work in no_std
pub fn test_sensor_reading() {
    let _reading = sensor::simple_reading("temp_01", 23.5, 1234567890);
}

/// Test that object builder works in no_std
pub fn test_object_builder() {
    let _config = ObjectBuilder::with_capacity(3)
        .add_str("device", "sensor_01")
        .add_i32("rate", 1000)
        .add_bool("enabled", true)
        .build();
}

/// Test that array builder works in no_std
pub fn test_array_builder() {
    let _arr = ArrayBuilder::with_capacity(5)
        .add_i32(1)
        .add_i32(2)
        .add_i32(3)
        .add_i32(4)
        .add_i32(5)
        .build();
}

/// Test that parsing works in no_std
pub fn test_parsing() {
    let json = br#"{"id":"sensor_01","value":23.5}"#;
    let config = ParserConfig::strict();
    let mut source = BufferSource::new(json);
    let _ = parse_with_config(&mut source, &config);
}

/// Test that stringification works in no_std
pub fn test_stringify() {
    let node = ObjectBuilder::new()
        .add_str("test", "value")
        .build();
    let mut dest = BufferDestination::new();
    let _ = stringify(&node, &mut dest);
}

/// Test that batching works in no_std
pub fn test_batch_readings() {
    let mut readings = Vec::new();
    for i in 0..10 {
        readings.push(sensor::simple_reading("sensor", 23.5, 1234567890 + i));
    }
    let _batch = sensor::batch_readings("device_01", readings);
}
