# JSON Library Refactor Plan: Size and Performance

## 1. Audit and Reduce Heap Allocations
- Review all uses of `String`, `Vec`, and `HashMap` in hot paths (parsing, stringification, Node construction).
- Replace with stack-allocated buffers or `SmallVec`/`SmallString` where possible.
- Reuse buffers in repeated operations (e.g., parsing, stringification, benchmarks).

## 2. Avoid Unnecessary Cloning and Copying
- Audit for `.clone()`, `.to_string()`, and similar calls in performance-critical code.
- Use references or move semantics where possible.
- Expand use of `.into_*()` consuming methods for zero-copy extraction.

## 3. Expand SIMD and Fast Path Optimizations
- Extend SIMD-accelerated routines to more parsing and validation functions.
- Ensure fast paths for ASCII strings and simple numbers are used throughout.
- Profile and inline hot functions with `#[inline(always)]` where beneficial.

## 4. Tune ParserConfig and API Defaults
- Set conservative defaults for depth, object/array size, and string length for embedded/low-memory use.
- Expose configuration for batch size and buffer reuse in APIs.

## 5. Reduce Core Type Sizes
- Check the size of the `Node` enum and other core types.
- Use `Box` for large enum variants if overall size is excessive.
- Use `Option<Box<T>>` for optional large fields.

## 6. Batch Operations and Buffer Pooling
- Pool or cache frequently used objects in benchmarks and high-throughput scenarios.
- Add examples and documentation for memory-efficient patterns (pre-allocating, reusing builders).

## 7. Documentation and Examples
- Add more usage examples for memory-efficient and high-performance patterns.
- Document best practices for embedded and high-throughput use cases.

## 8. Continuous Profiling and Benchmarking
- Regularly profile with real data to identify new bottlenecks.
- Expand and automate benchmarks for parsing, construction, and stringification.

---

**Next Steps:**
- Assign owners for each area above.
- Schedule code audits and targeted refactors.
- Track progress and measure improvements with benchmarks.
