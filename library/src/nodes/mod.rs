/// JSON Pointer support (RFC 6901)
#[cfg(feature = "json-pointer")]
pub mod json_pointer;

pub mod node;

/// JSON Schema validation (subset of Draft 7)
#[cfg(feature = "alloc")]
pub mod schema;

/// JSON Patch (RFC 6902)
#[cfg(feature = "json-pointer")]
pub mod patch;

/// JSON Merge Patch (RFC 7386)
#[cfg(feature = "alloc")]
pub mod merge_patch;
