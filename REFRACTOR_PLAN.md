# JSON Library Refactor Plan: Size and Performance


## 1. Audit and Reduce Heap Allocations
### 1.1 Identify Hot Paths
- List all functions and modules involved in parsing, stringification, and Node construction.
- Profile code to confirm which paths are most performance-critical.

### 1.2 Audit Heap Allocations
- Search for all uses of `String`, `Vec`, and `HashMap` in hot paths.
- Document where heap allocations occur and why.

### 1.3 Replace with Stack or Small Allocations
- Evaluate replacing `String`/`Vec`/`HashMap` with stack-allocated buffers, `SmallVec`, or `SmallString` where feasible.
- Benchmark before/after for performance and memory impact.

### 1.4 Buffer Reuse
- Identify opportunities to reuse buffers in repeated operations (e.g., parsing, stringification, benchmarks).
- Refactor code to implement buffer reuse patterns.

### 1.5 Document and Review
- Summarize changes and improvements.
- Review with team for correctness and further optimization opportunities.


## 2. Avoid Unnecessary Cloning and Copying
### 2.1 Identify Cloning and Copying Hotspots
- Search for `.clone()`, `.to_string()`, and similar calls in performance-critical code.
- List all locations and functions where these are used.

### 2.2 Refactor to Use References or Moves
- Replace unnecessary clones/copies with references or move semantics where possible.
- Use Rust's ownership and borrowing to minimize allocations.

### 2.3 Use Consuming Methods
- Expand use of `.into_*()` and other consuming methods for zero-copy extraction.
- Refactor APIs to accept/return owned values where beneficial.

### 2.4 Benchmark and Review
- Benchmark before/after changes for performance impact.
- Review changes for correctness and further optimization.


## 3. Expand SIMD and Fast Path Optimizations
### 3.1 Audit Current SIMD Usage
- List all functions currently using SIMD acceleration.
- Identify parsing and validation functions that could benefit from SIMD.

### 3.2 Implement SIMD in More Functions
- Extend SIMD-accelerated routines to additional parsing and validation code.
- Ensure correctness and fallback for non-SIMD platforms.

### 3.3 Optimize Fast Paths
- Ensure fast paths for ASCII strings and simple numbers are implemented and used throughout.
- Profile code to confirm fast path effectiveness.

### 3.4 Inline Hot Functions
- Profile and inline hot functions with `#[inline(always)]` where beneficial.
- Benchmark to ensure inlining provides a measurable benefit.


## 4. Tune ParserConfig and API Defaults
### 4.1 Audit Current Defaults
- List all current defaults for depth, object/array size, and string length in ParserConfig and APIs.
- Identify which defaults are most relevant for embedded/low-memory use.

### 4.2 Set Conservative Defaults
- Adjust defaults for depth, object/array size, and string length to be more conservative where appropriate.
- Document rationale for each change.

### 4.3 Expose Configurable Parameters
- Ensure batch size and buffer reuse options are exposed in public APIs.
- Add documentation and examples for configuring these parameters.

### 4.4 Review and Test
- Test new defaults and configuration options in embedded/low-memory scenarios.
- Review with team and gather feedback.


## 5. Reduce Core Type Sizes
### 5.1 Audit Core Type Sizes
- Measure the size of the `Node` enum and other core types using tools like `std::mem::size_of`.
- List all large enum variants and optional fields.

### 5.2 Refactor Large Variants
- Use `Box` for large enum variants if overall size is excessive.
- Use `Option<Box<T>>` for optional large fields to reduce memory footprint.

### 5.3 Benchmark and Test
- Benchmark memory usage before and after refactoring.
- Test for correctness and performance impact.

### 5.4 Document Changes
- Document all changes and rationale.
- Review with team for further optimization opportunities.

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
