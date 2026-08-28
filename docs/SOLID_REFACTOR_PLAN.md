# SOLID Architectural Refactoring Plan for `json_lib`

## Executive Summary

`json_lib` is a modular, high-performance JSON library for Rust. While the codebase is feature-rich and fast, over time several components have accumulated tight coupling, monolithic files, violated behavioral trait contracts, and mixed responsibilities. 

This document outlines a concrete architectural refactoring plan to align `json_lib` completely with the **SOLID Principles of Object-Oriented and API Design**:

- **S**ingle Responsibility Principle (SRP)
- **O**pen/Closed Principle (OCP)
- **L**iskov Substitution Principle (LSP)
- **I**nterface Segregation Principle (ISP)
- **D**ependency Inversion Principle (DIP)

---

## 1. Single Responsibility Principle (SRP)

*A class, struct, or module should have one, and only one, reason to change.*

### Current Violations

1. **Monolithic `nodes/node.rs` (66 KB, ~2,120 lines)**:
   - `Node` handles value representation, index resolution, numerical conversions, search/traversal, JSON Pointer navigation, JSON Patch operations, and schema validation.
2. **"Junk Drawer" `misc/mod.rs` (17.7 KB)**:
   - `misc/mod.rs` combines package version retrieval, pretty printing (`print`, `pretty_print`), whitespace stripping (`strip`), and statistics formatting into a single unorganized utility file.
3. **Overloaded `embedded.rs` (26.8 KB)**:
   - Mixes zero-copy string parsing, heapless arena allocation, fixed-capacity buffer sources, and text stringification into one file.

### Proposed Refactoring

#### 1.1 Modularize `nodes/`
Decompose `nodes/node.rs` into specialized sub-modules within `nodes/`:

- `nodes/types.rs`: Core `Node` and `Numeric` data enum definitions only.
- `nodes/accessors.rs`: Type-checking (`is_*`) and value coercion (`as_*`, `get`, `get_mut`).
- `nodes/indexing.rs`: Implementations of `Index<&str>`, `Index<usize>`, `IndexMut<&str>`, `IndexMut<usize>`.
- `nodes/traversal.rs`: Search, filter, walk, diff, and merge operations (`find`, `find_all`, `transform`, `diff`, `merge`).
- `nodes/convert.rs`: Standard `From` / `Into` trait implementations.

#### 1.2 Eliminate `misc/mod.rs`
Relocate functions to their proper home:
- Move `print` and `pretty_print` into `stringify/pretty.rs`.
- Move `strip` (whitespace stripping) into `io/utils.rs` or `parser/json5.rs`.
- Move `get_version` into `lib.rs` / `version.rs`.
- Deprecate `misc/mod.rs`.

#### 1.3 Decompose `embedded.rs`
Split `embedded.rs` into a dedicated `embedded/` module:
- `embedded/arena.rs`: Fixed-size arena memory allocator.
- `embedded/source.rs`: Static byte slice source.
- `embedded/parser.rs`: `no_std` zero-allocation parser routines.

---

## 2. Open/Closed Principle (OCP)

*Software entities should be open for extension, but closed for modification.*

### Current Violations

1. **Concrete Format Serializers (`stringify/`)**:
   - `default.rs`, `pretty.rs`, `yaml.rs`, `xml.rs`, `toml.rs`, and `bencode.rs` are written as rigid, freestanding functions. Adding a new output format (e.g., MessagePack, CBOR) requires authoring an entirely new module with duplicate AST iteration code.
2. **Hardcoded String Escaping Strategy**:
   - Escaping rules are hardcoded in `stringify/escape.rs` and duplicated across serializers.

### Proposed Refactoring

#### 2.1 Abstract `Serializer` and `FormatEncoder` Traits
Define generic serialization contracts in `stringify/traits.rs`:

```rust
pub trait FormatEncoder<D: IDestination> {
    fn encode_boolean(&mut self, val: bool, dst: &mut D) -> Result<(), String>;
    fn encode_number(&mut self, val: &Numeric, dst: &mut D) -> Result<(), String>;
    fn encode_string(&mut self, val: &str, dst: &mut D) -> Result<(), String>;
    fn encode_null(&mut self, dst: &mut D) -> Result<(), String>;
    fn begin_array(&mut self, dst: &mut D) -> Result<(), String>;
    fn end_array(&mut self, dst: &mut D) -> Result<(), String>;
    fn begin_object(&mut self, dst: &mut D) -> Result<(), String>;
    fn end_object(&mut self, dst: &mut D) -> Result<(), String>;
}
```

Implement concrete encoders:
- `JsonEncoder`
- `PrettyJsonEncoder`
- `YamlEncoder`
- `XmlEncoder`
- `TomlEncoder`
- `BencodeEncoder`

Now, new format support can be added by implementing `FormatEncoder` without modifying any core tree traversal logic.

---

## 3. Liskov Substitution Principle (LSP)

*Subtypes/Implementations must be substitutable for their base types/traits without breaking correctness.*

### Current Violations

1. **Flawed `ISource::reset()` Requirement (`io/traits.rs`)**:
   - `ISource` mandates `reset(&mut self)`. Network sockets, HTTP streams, and `stdin` streams cannot be reset to offset 0. Expecting `reset()` on all `ISource` implementations causes unexpected panics or erroneous behavior when using non-seekable streams.
2. **Flawed `IDestination::last()` and `clear()` Requirements**:
   - `IDestination` mandates `last(&self) -> Option<u8>` and `clear(&mut self)`. Non-seekable output streams (like raw sockets or file writers without buffering) cannot inspect their last written byte or clear their past output.
3. **UTF-8 Overhead in `ISource::current() -> Option<char>`**:
   - Reading `char` forces multi-byte decoding on every character read, breaking when stream chunks split UTF-8 sequences.

### Proposed Refactoring

#### 3.1 Correct Trait Behavioral Contracts
Update `io/traits.rs`:

```rust
pub trait ISource {
    fn next(&mut self);
    fn current(&mut self) -> Option<u8>; // Byte-level operations
    fn more(&mut self) -> bool;
}

pub trait ResetableSource: ISource {
    fn reset(&mut self);
}

pub trait IDestination {
    fn add_byte(&mut self, byte: u8);
    fn add_bytes(&mut self, bytes: &str);
}

pub trait BufferedDestination: IDestination {
    fn last(&self) -> Option<u8>;
    fn clear(&mut self);
}
```

---

## 4. Interface Segregation Principle (ISP)

*Clients should not be forced to depend on methods or interfaces they do not use.*

### Current Violations

1. **Fat `Node` API**:
   - `Node` exposes over 80 inherent methods. Consumers needing simple tree construction are exposed to complex JSON Pointer, JSON Patch, Schema, and filtering methods.
2. **Fat I/O Traits**:
   - Simple byte destinations must implement `last()` and `clear()`.

### Proposed Refactoring

#### 4.1 Extension Traits for Specialized `Node` Features
Extract non-core features into dedicated extension traits:

```rust
pub trait JsonPointerExt {
    fn pointer_get(&self, ptr: &str) -> Option<&Node>;
    fn pointer_set(&mut self, ptr: &str, val: Node) -> Result<(), String>;
    fn pointer_remove(&mut self, ptr: &str) -> Result<Option<Node>, String>;
}

pub trait JsonPatchExt {
    fn apply_patch(&mut self, patch: &Node) -> Result<(), String>;
}

pub trait JsonSchemaExt {
    fn validate_schema(&self, schema: &Node) -> Result<(), String>;
}
```

---

## 5. Dependency Inversion Principle (DIP)

*High-level modules should not depend on low-level modules. Both should depend on abstractions.*

### Current Violations

1. **Concrete Standard I/O Dependencies**:
   - Helper functions like `from_str` and `from_bytes` construct internal `BufferSource` instances directly. High-level conversion helper functions rely on concrete implementations rather than abstract traits.
2. **Parser Coupling to Resource Guard Limits**:
   - `parser/default.rs` couples limit enforcement directly to `ParserConfig` fields.

### Proposed Refactoring

#### 5.1 Abstract Parsing and Formatting Pipelines
- Standardize all high-level parse/stringify functions to accept generic parameters `S: ISource` and `D: IDestination` or dynamic trait objects `&mut dyn ISource` / `&mut dyn IDestination`.
- Introduce a `LimitPolicy` trait for custom parser resource guards (e.g. `DefaultLimitPolicy`, `UnlimitedPolicy`).

---

## Refactoring Roadmap & Phased Execution

| Phase | Target Module | Goal | SOLID Principles Addressed |
|---|---|---|---|
| **Phase 1** | `io/traits.rs` | Refactor `ISource` & `IDestination` byte contracts and split seekable/buffered traits | LSP, ISP, DIP |
| **Phase 2** | `nodes/` | Decompose `node.rs` into `types`, `accessors`, `indexing`, `traversal`, `convert` | SRP, ISP |
| **Phase 3** | `stringify/` | Implement `FormatEncoder` trait & generic `Serializer` | OCP, DIP |
| **Phase 4** | `misc/` & `embedded/` | Eliminate `misc/mod.rs` and modularize `embedded.rs` | SRP |
| **Phase 5** | `parser/` | Introduce `LimitPolicy` trait & decouple Lexer from Parser | DIP, OCP |

---
