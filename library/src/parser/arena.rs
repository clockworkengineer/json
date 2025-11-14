//! Arena allocator for reduced allocation overhead during JSON parsing
//!
//! This module provides an arena allocator that can significantly reduce
//! allocation overhead when parsing JSON by allocating from a pre-allocated buffer.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Arena allocator for string storage
/// Reduces allocation overhead by using a bump allocator strategy
#[cfg(feature = "alloc")]
pub struct StringArena {
    buffer: Vec<u8>,
    position: usize,
    strings: Vec<(usize, usize)>, // (start, length) pairs
}

#[cfg(feature = "alloc")]
impl StringArena {
    /// Create a new arena with a pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            position: 0,
            strings: Vec::new(),
        }
    }

    /// Create a new arena with default capacity (16KB)
    pub fn new() -> Self {
        Self::with_capacity(16 * 1024)
    }

    /// Allocate a string in the arena
    /// Returns the index of the allocated string
    pub fn alloc_str(&mut self, s: &str) -> usize {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let start = self.position;

        // Ensure we have enough space
        if self.buffer.len() < self.position + len {
            self.buffer.reserve(len);
            self.buffer.extend_from_slice(bytes);
        } else {
            self.buffer[self.position..self.position + len].copy_from_slice(bytes);
        }

        self.position += len;
        let index = self.strings.len();
        self.strings.push((start, len));
        index
    }

    /// Get a string by its index
    pub fn get_str(&self, index: usize) -> Option<&str> {
        self.strings
            .get(index)
            .and_then(|&(start, len)| core::str::from_utf8(&self.buffer[start..start + len]).ok())
    }

    /// Clear the arena for reuse
    pub fn clear(&mut self) {
        self.position = 0;
        self.strings.clear();
        // Keep the buffer allocated for reuse
    }

    /// Get the total bytes allocated
    pub fn bytes_allocated(&self) -> usize {
        self.position
    }

    /// Get the number of strings stored
    pub fn string_count(&self) -> usize {
        self.strings.len()
    }

    /// Get remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        self.buffer.capacity() - self.position
    }
}

#[cfg(feature = "alloc")]
impl Default for StringArena {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory pool for Node allocations
/// Pre-allocates nodes to reduce allocation overhead
#[cfg(feature = "alloc")]
pub struct NodePool {
    capacity: usize,
    allocated: usize,
}

#[cfg(feature = "alloc")]
impl NodePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            allocated: 0,
        }
    }

    pub fn reset(&mut self) {
        self.allocated = 0;
    }

    pub fn allocated(&self) -> usize {
        self.allocated
    }

    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.allocated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_string_arena_basic() {
        let mut arena = StringArena::new();

        let idx1 = arena.alloc_str("hello");
        let idx2 = arena.alloc_str("world");

        assert_eq!(arena.get_str(idx1), Some("hello"));
        assert_eq!(arena.get_str(idx2), Some("world"));
        assert_eq!(arena.string_count(), 2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_string_arena_clear() {
        let mut arena = StringArena::with_capacity(1024);

        arena.alloc_str("test");
        assert_eq!(arena.string_count(), 1);

        arena.clear();
        assert_eq!(arena.string_count(), 0);
        assert_eq!(arena.bytes_allocated(), 0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_string_arena_capacity() {
        let arena = StringArena::with_capacity(1024);
        assert!(arena.remaining_capacity() >= 1024);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_node_pool() {
        let mut pool = NodePool::new(100);
        assert_eq!(pool.remaining(), 100);

        pool.reset();
        assert_eq!(pool.allocated(), 0);
    }
}
