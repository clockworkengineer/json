//! Parser configuration for controlling memory usage and preventing resource exhaustion
//! Provides limits for parsing depth, string sizes, and collection sizes

/// Configuration for JSON parser to control resource usage
///
/// Default values are chosen for embedded systems with limited resources:
/// - Max depth: 32 levels (prevents stack overflow)
/// - Max string length: 4096 bytes (prevents excessive memory allocation)
/// - Max array size: 1024 elements
/// - Max object size: 256 key-value pairs
#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    /// Maximum nesting depth for objects and arrays
    /// Set to None for unlimited depth (not recommended for embedded systems)
    pub max_depth: Option<usize>,

    /// Maximum length of a string value in bytes
    /// Set to None for unlimited length (not recommended for embedded systems)
    pub max_string_length: Option<usize>,

    /// Maximum number of elements in an array
    /// Set to None for unlimited size (not recommended for embedded systems)
    pub max_array_size: Option<usize>,

    /// Maximum number of key-value pairs in an object
    /// Set to None for unlimited size (not recommended for embedded systems)
    pub max_object_size: Option<usize>,
}

impl ParserConfig {
    /// Creates a new parser configuration with default limits suitable for embedded systems
    ///
    /// Default limits:
    /// - Max depth: 32 levels
    /// - Max string length: 4096 bytes
    /// - Max array size: 1024 elements
    /// - Max object size: 256 key-value pairs
    pub fn new() -> Self {
        Self {
            max_depth: Some(32),
            max_string_length: Some(4096),
            max_array_size: Some(1024),
            max_object_size: Some(256),
        }
    }

    /// Creates a configuration with no limits (uses default behavior)
    ///
    /// Warning: This is not recommended for embedded systems as it can lead to
    /// stack overflow or excessive memory consumption
    pub fn unlimited() -> Self {
        Self {
            max_depth: None,
            max_string_length: None,
            max_array_size: None,
            max_object_size: None,
        }
    }

    /// Creates a very strict configuration for highly constrained embedded systems
    ///
    /// Strict limits:
    /// - Max depth: 16 levels
    /// - Max string length: 256 bytes
    /// - Max array size: 64 elements
    /// - Max object size: 32 key-value pairs
    pub fn strict() -> Self {
        Self {
            max_depth: Some(16),
            max_string_length: Some(256),
            max_array_size: Some(64),
            max_object_size: Some(32),
        }
    }

    /// Sets the maximum nesting depth
    pub fn with_max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Sets the maximum string length in bytes
    pub fn with_max_string_length(mut self, max_string_length: Option<usize>) -> Self {
        self.max_string_length = max_string_length;
        self
    }

    /// Sets the maximum array size
    pub fn with_max_array_size(mut self, max_array_size: Option<usize>) -> Self {
        self.max_array_size = max_array_size;
        self
    }

    /// Sets the maximum object size
    pub fn with_max_object_size(mut self, max_object_size: Option<usize>) -> Self {
        self.max_object_size = max_object_size;
        self
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ParserConfig::default();
        assert_eq!(config.max_depth, Some(32));
        assert_eq!(config.max_string_length, Some(4096));
        assert_eq!(config.max_array_size, Some(1024));
        assert_eq!(config.max_object_size, Some(256));
    }

    #[test]
    fn test_unlimited_config() {
        let config = ParserConfig::unlimited();
        assert_eq!(config.max_depth, None);
        assert_eq!(config.max_string_length, None);
        assert_eq!(config.max_array_size, None);
        assert_eq!(config.max_object_size, None);
    }

    #[test]
    fn test_strict_config() {
        let config = ParserConfig::strict();
        assert_eq!(config.max_depth, Some(16));
        assert_eq!(config.max_string_length, Some(256));
        assert_eq!(config.max_array_size, Some(64));
        assert_eq!(config.max_object_size, Some(32));
    }

    #[test]
    fn test_builder_pattern() {
        let config = ParserConfig::new()
            .with_max_depth(Some(10))
            .with_max_string_length(Some(100))
            .with_max_array_size(Some(50))
            .with_max_object_size(Some(25));

        assert_eq!(config.max_depth, Some(10));
        assert_eq!(config.max_string_length, Some(100));
        assert_eq!(config.max_array_size, Some(50));
        assert_eq!(config.max_object_size, Some(25));
    }
}
