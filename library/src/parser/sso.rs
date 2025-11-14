//! Small String Optimization (SSO) for reducing heap allocations
//!
//! This module provides a SmallString type that stores short strings inline
//! to avoid heap allocations for common small strings like keys in JSON objects.

use core::fmt;

/// Size of inline storage (23 bytes + 1 byte for length on 64-bit systems)
/// This allows strings up to 23 bytes to be stored without heap allocation
const INLINE_CAPACITY: usize = 23;

/// A string that stores small strings inline to avoid heap allocations
#[cfg(feature = "alloc")]
pub enum SmallString {
    /// Inline storage for strings up to INLINE_CAPACITY bytes
    Inline {
        len: u8,
        data: [u8; INLINE_CAPACITY],
    },
    /// Heap-allocated storage for larger strings
    Heap(alloc::string::String),
}

#[cfg(feature = "alloc")]
impl SmallString {
    /// Create a new SmallString from a string slice
    pub fn new(s: &str) -> Self {
        let bytes = s.as_bytes();
        if bytes.len() <= INLINE_CAPACITY {
            let mut data = [0u8; INLINE_CAPACITY];
            data[..bytes.len()].copy_from_slice(bytes);
            SmallString::Inline {
                len: bytes.len() as u8,
                data,
            }
        } else {
            SmallString::Heap(s.to_string())
        }
    }

    /// Get the string as a &str
    pub fn as_str(&self) -> &str {
        match self {
            SmallString::Inline { len, data } => {
                core::str::from_utf8(&data[..*len as usize]).unwrap()
            }
            SmallString::Heap(s) => s.as_str(),
        }
    }

    /// Get the length of the string
    pub fn len(&self) -> usize {
        match self {
            SmallString::Inline { len, .. } => *len as usize,
            SmallString::Heap(s) => s.len(),
        }
    }

    /// Check if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the string is stored inline (no heap allocation)
    pub fn is_inline(&self) -> bool {
        matches!(self, SmallString::Inline { .. })
    }

    /// Convert to a regular String
    pub fn into_string(self) -> alloc::string::String {
        match self {
            SmallString::Inline { .. } => self.as_str().to_string(),
            SmallString::Heap(s) => s,
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for SmallString {
    fn from(s: &str) -> Self {
        SmallString::new(s)
    }
}

#[cfg(feature = "alloc")]
impl From<alloc::string::String> for SmallString {
    fn from(s: alloc::string::String) -> Self {
        if s.len() <= INLINE_CAPACITY {
            SmallString::new(&s)
        } else {
            SmallString::Heap(s)
        }
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "alloc")]
impl fmt::Debug for SmallString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SmallString({:?})", self.as_str())
    }
}

#[cfg(feature = "alloc")]
impl PartialEq for SmallString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

#[cfg(feature = "alloc")]
impl Clone for SmallString {
    fn clone(&self) -> Self {
        match self {
            SmallString::Inline { len, data } => SmallString::Inline {
                len: *len,
                data: *data,
            },
            SmallString::Heap(s) => SmallString::Heap(s.clone()),
        }
    }
}

/// Statistics about SmallString usage
#[cfg(feature = "alloc")]
pub struct SmallStringStats {
    pub inline_count: usize,
    pub heap_count: usize,
    pub total_bytes_saved: usize,
}

#[cfg(feature = "alloc")]
impl SmallStringStats {
    pub fn new() -> Self {
        Self {
            inline_count: 0,
            heap_count: 0,
            total_bytes_saved: 0,
        }
    }

    pub fn record_string(&mut self, s: &SmallString) {
        if s.is_inline() {
            self.inline_count += 1;
            // Each inline string saves approximately 24 bytes (String header overhead)
            self.total_bytes_saved += 24;
        } else {
            self.heap_count += 1;
        }
    }

    pub fn inline_percentage(&self) -> f64 {
        let total = self.inline_count + self.heap_count;
        if total == 0 {
            0.0
        } else {
            (self.inline_count as f64 / total as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_small_string_inline() {
        let s = SmallString::new("hello");
        assert!(s.is_inline());
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len(), 5);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_small_string_heap() {
        let long = "this is a very long string that exceeds inline capacity";
        let s = SmallString::new(long);
        assert!(!s.is_inline());
        assert_eq!(s.as_str(), long);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_small_string_boundary() {
        // Test string exactly at inline capacity
        let s = SmallString::new("12345678901234567890123");
        assert!(s.is_inline());
        assert_eq!(s.len(), 23);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_small_string_stats() {
        let mut stats = SmallStringStats::new();

        let s1 = SmallString::new("short");
        stats.record_string(&s1);

        let s2 = SmallString::new("this is a very long string that needs heap allocation");
        stats.record_string(&s2);

        assert_eq!(stats.inline_count, 1);
        assert_eq!(stats.heap_count, 1);
        assert!(stats.inline_percentage() > 0.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_small_string_equality() {
        let s1 = SmallString::new("test");
        let s2 = SmallString::new("test");
        assert_eq!(s1, s2);
    }
}
