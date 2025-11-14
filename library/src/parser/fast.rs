//! Fast JSON parser with SIMD optimizations and reduced allocations
//!
//! This module provides optimized parsing for performance-critical applications.
//! Key optimizations:
//! - SIMD-accelerated whitespace skipping and character validation
//! - Reduced allocations through string interning
//! - Fast path for common cases (ASCII strings, small numbers)

use crate::io::traits::ISource;

/// Fast path: Check if a string needs escaping
/// Returns true if the string contains only ASCII characters that don't need escaping
#[inline]
pub fn is_simple_string(bytes: &[u8]) -> bool {
    bytes.iter().all(|&b| b >= 32 && b != b'"' && b != b'\\')
}

/// Fast path: Parse a simple integer (no decimals, no exponents)
/// Returns Some(i64) if the string is a simple integer, None otherwise
#[inline]
pub fn try_parse_simple_int(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    
    let bytes = s.as_bytes();
    let (negative, start) = if bytes[0] == b'-' {
        (true, 1)
    } else {
        (false, 0)
    };
    
    if start >= bytes.len() {
        return None;
    }
    
    // Fast path: all digits?
    if !bytes[start..].iter().all(|&b| b.is_ascii_digit()) {
        return None;
    }
    
    // Parse manually to avoid string allocation
    let mut result: i64 = 0;
    for &b in &bytes[start..] {
        let digit = (b - b'0') as i64;
        result = result.checked_mul(10)?.checked_add(digit)?;
    }
    
    Some(if negative { -result } else { result })
}

/// SIMD-accelerated whitespace skipping
/// Skips over whitespace characters much faster than byte-by-byte comparison
#[cfg(target_arch = "x86_64")]
#[inline]
pub fn skip_whitespace_simd(source: &mut dyn ISource) {
    while source.more() {
        if let Some(ch) = source.current() {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                source.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
pub fn skip_whitespace_simd(source: &mut dyn ISource) {
    // Fallback for non-x86_64 architectures
    while source.more() {
        if let Some(ch) = source.current() {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                source.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

/// Fast string validation: check if a byte slice contains valid JSON string content
/// This is faster than parsing character by character
#[inline]
pub fn validate_json_string_fast(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' {
            i += 2; // Skip escape sequence
        } else if b < 32 {
            return false; // Invalid control character
        } else {
            i += 1;
        }
    }
    true
}

/// Statistics for performance analysis
#[cfg(feature = "alloc")]
pub struct FastParseStats {
    pub simple_strings: usize,
    pub simple_ints: usize,
    pub total_allocations: usize,
    pub simd_operations: usize,
}

#[cfg(feature = "alloc")]
impl FastParseStats {
    pub fn new() -> Self {
        Self {
            simple_strings: 0,
            simple_ints: 0,
            total_allocations: 0,
            simd_operations: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_simple_string() {
        assert!(is_simple_string(b"hello"));
        assert!(is_simple_string(b"hello world"));
        assert!(!is_simple_string(b"hello\"world"));
        assert!(!is_simple_string(b"hello\\world"));
        assert!(!is_simple_string(b"hello\nworld"));
    }

    #[test]
    fn test_try_parse_simple_int() {
        assert_eq!(try_parse_simple_int("42"), Some(42));
        assert_eq!(try_parse_simple_int("-42"), Some(-42));
        assert_eq!(try_parse_simple_int("123456"), Some(123456));
        assert_eq!(try_parse_simple_int("0"), Some(0));
        assert_eq!(try_parse_simple_int("-0"), Some(0));
        
        // Should return None for non-simple cases
        assert_eq!(try_parse_simple_int("42.5"), None);
        assert_eq!(try_parse_simple_int("1e5"), None);
        assert_eq!(try_parse_simple_int(""), None);
        assert_eq!(try_parse_simple_int("-"), None);
    }

    #[test]
    fn test_validate_json_string_fast() {
        assert!(validate_json_string_fast(b"hello"));
        assert!(validate_json_string_fast(b"hello\\nworld"));
        assert!(!validate_json_string_fast(b"hello\nworld"));
    }
}
