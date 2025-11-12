//! Parser statistics and profiling for embedded systems
//!
//! Provides metrics about memory usage, allocation patterns, and performance
//! to help tune parser configuration for resource-constrained devices.

#[cfg(feature = "std")]
use std::time::Instant;

/// Statistics collected during JSON parsing
///
/// These metrics help developers understand actual resource usage
/// and tune ParserConfig for their specific use cases.
#[derive(Debug, Clone, Default)]
pub struct ParseStats {
    /// Maximum nesting depth reached during parsing
    pub peak_depth: usize,

    /// Total number of Node allocations
    pub total_nodes: usize,

    /// Number of string values parsed
    pub string_count: usize,

    /// Total bytes in all string values
    pub string_bytes: usize,

    /// Number of array elements parsed
    pub array_elements: usize,

    /// Number of object key-value pairs parsed
    pub object_pairs: usize,

    /// Largest array size encountered
    pub max_array_size: usize,

    /// Largest object size encountered
    pub max_object_size: usize,

    /// Longest string encountered (bytes)
    pub max_string_length: usize,

    /// Estimated total memory usage in bytes
    /// This is an approximation based on Node sizes and allocations
    pub estimated_memory_bytes: usize,

    /// Parse duration in microseconds (only available with std)
    #[cfg(feature = "std")]
    pub parse_time_us: Option<u64>,
}

impl ParseStats {
    /// Creates a new empty statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Records entering a deeper nesting level
    pub fn enter_depth(&mut self, current_depth: usize) {
        if current_depth > self.peak_depth {
            self.peak_depth = current_depth;
        }
    }

    /// Records a string node allocation
    pub fn record_string(&mut self, len: usize) {
        self.total_nodes += 1;
        self.string_count += 1;
        self.string_bytes += len;
        if len > self.max_string_length {
            self.max_string_length = len;
        }
        // Estimate: Node enum (56 bytes) + String overhead (~24 bytes) + actual string data
        self.estimated_memory_bytes += 56 + 24 + len;
    }

    /// Records an array element
    pub fn record_array_element(&mut self, array_size: usize) {
        self.array_elements += 1;
        if array_size > self.max_array_size {
            self.max_array_size = array_size;
        }
    }

    /// Records an array node completion
    pub fn record_array(&mut self, size: usize) {
        self.total_nodes += 1;
        // Estimate: Node enum (56 bytes) + Vec overhead (~24 bytes) + element pointers
        self.estimated_memory_bytes += 56 + 24 + (size * 8);
    }

    /// Records an object key-value pair
    pub fn record_object_pair(&mut self, key_len: usize, object_size: usize) {
        self.object_pairs += 1;
        if object_size > self.max_object_size {
            self.max_object_size = object_size;
        }
        // Estimate: key string overhead + value pointer
        self.estimated_memory_bytes += 24 + key_len + 8;
    }

    /// Records an object node completion
    pub fn record_object(&mut self, size: usize) {
        self.total_nodes += 1;
        // Estimate: Node enum (56 bytes) + HashMap overhead (varies, ~96 bytes base + buckets)
        self.estimated_memory_bytes += 56 + 96 + (size * 32);
    }

    /// Records a simple node (number, boolean, null)
    pub fn record_simple_node(&mut self) {
        self.total_nodes += 1;
        // Estimate: Just the Node enum size
        self.estimated_memory_bytes += 56;
    }

    /// Returns a summary string of key statistics
    pub fn summary(&self) -> alloc::string::String {
        use alloc::format;
        format!(
            "ParseStats {{ depth: {}, nodes: {}, strings: {}, arrays: {}, objects: {}, memory: {} bytes }}",
            self.peak_depth,
            self.total_nodes,
            self.string_count,
            self.array_elements,
            self.object_pairs,
            self.estimated_memory_bytes
        )
    }

    /// Checks if parsed data fits within configuration limits
    pub fn fits_config(&self, config: &super::config::ParserConfig) -> bool {
        if let Some(max_depth) = config.max_depth {
            if self.peak_depth > max_depth {
                return false;
            }
        }
        if let Some(max_str_len) = config.max_string_length {
            if self.max_string_length > max_str_len {
                return false;
            }
        }
        if let Some(max_array) = config.max_array_size {
            if self.max_array_size > max_array {
                return false;
            }
        }
        if let Some(max_obj) = config.max_object_size {
            if self.max_object_size > max_obj {
                return false;
            }
        }
        true
    }
}

/// Timer for measuring parse duration (std only)
#[cfg(feature = "std")]
pub struct ParseTimer {
    start: Instant,
}

#[cfg(feature = "std")]
impl ParseTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::config::ParserConfig;

    #[test]
    fn test_stats_tracking() {
        let mut stats = ParseStats::new();

        stats.record_string(10);
        assert_eq!(stats.string_count, 1);
        assert_eq!(stats.string_bytes, 10);
        assert_eq!(stats.max_string_length, 10);

        stats.record_array_element(5);
        stats.record_array_element(5);
        assert_eq!(stats.array_elements, 2);
        assert_eq!(stats.max_array_size, 5);

        stats.record_object_pair(4, 2);
        assert_eq!(stats.object_pairs, 1);
        assert_eq!(stats.max_object_size, 2);

        assert!(stats.estimated_memory_bytes > 0);
    }

    #[test]
    fn test_depth_tracking() {
        let mut stats = ParseStats::new();

        stats.enter_depth(1);
        stats.enter_depth(3);
        stats.enter_depth(2);

        assert_eq!(stats.peak_depth, 3);
    }

    #[test]
    fn test_fits_config() {
        let mut stats = ParseStats::new();
        stats.peak_depth = 5;
        stats.max_string_length = 100;
        stats.max_array_size = 50;
        stats.max_object_size = 20;

        let config = ParserConfig::new();
        assert!(stats.fits_config(&config));

        let strict = ParserConfig::strict();
        assert!(stats.fits_config(&strict)); // All values fit within strict limits

        // Test exceeding limits
        stats.max_string_length = 300; // Exceeds strict's 256 byte limit
        assert!(!stats.fits_config(&strict));
    }

    #[test]
    fn test_summary() {
        let mut stats = ParseStats::new();
        stats.total_nodes = 10;
        stats.string_count = 3;

        let summary = stats.summary();
        assert!(summary.contains("nodes: 10"));
        assert!(summary.contains("strings: 3"));
    }
}
