//! Parser module for processing JSON data
//! Implements parsing of JSON text into internal data structures
//! Supports standard JSON types and syntax validation

/// Parser configuration for resource limits
pub mod config;

/// Default parser implementation
/// Handles JSON parsing and error reporting functionality
pub mod default;

/// Parser statistics and profiling
#[cfg(feature = "alloc")]
pub mod stats;

/// JSON validation without allocation
pub mod validate;
