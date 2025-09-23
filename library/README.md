# json_lib

A lightweight, modular JSON toolkit for Rust with pluggable I/O sources/destinations, a simple in-memory Node tree, and
multiple serializers (JSON, YAML, XML, Bencode, TOML). Designed for small binaries, predictable behavior, and easy
embedding.

- Core Node type representing JSON structures
- Parser to build Node trees from streams
- Stringifiers to JSON, YAML, XML, Bencode, and TOML
- File and in-memory buffer I/O abstractions
- Pretty-printing utilities
- Unicode-aware file helpers (BOM detection/handling)

Minimum supported Rust version: 1.88.0

## Features at a glance

- Node and Numeric types for representing JSON values
- Parse and stringify with streaming-friendly traits
- FileSource/FileDestination and BufferSource/BufferDestination
- Pretty printer for human-friendly output
- Convert a Node to alternate formats (YAML, XML, Bencode, TOML)
- Unicode file format detection and read/write helpers

## Installation

Add the library to a workspace member or use a path dependency:
```
toml
# Cargo.toml
[dependencies]
json_lib = { path = "library" }
```
If publishing to crates.io or using a Git dependency, adjust accordingly.

## Quick start

Parse a file, then pretty-print to another file:
```
rust
use json_lib::{FileSource, FileDestination, parse};
use json_lib::json_lib::misc::print;
use std::path::Path;

fn main() -> Result<(), String> {
let input = "data.json";
let mut src = FileSource::new(input).map_err(|e| e.to_string())?;
let node = parse(&mut src).map_err(|e| e.to_string())?;

    let output = Path::new(input).with_extension("pretty.json");
    let mut dst = FileDestination::new(output.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;

    // 4-space indentation
    print(&node, &mut dst, 4, 0);
    Ok(())
}
```
Build a Node in memory and stringify to a buffer:
```
rust
use json_lib::{Node, Numeric, stringify, BufferDestination};

fn main() {
let node = Node::Object(vec![
("name".into(), Node::Str("example".into())),
("count".into(), Node::Number(Numeric::Integer(3))),
("items".into(), Node::Array(vec![
Node::Boolean(true),
Node::None,
Node::Str("text".into())
])),
].into_iter().collect());

    let mut buf = BufferDestination::new();
    stringify(&node, &mut buf);
    let json = buf.to_string();
    println!("{}", json);
}
```
Convert a Node to YAML/XML/Bencode:
```
rust
use json_lib::{Node, Numeric, to_yaml, to_xml, to_bencode, BufferDestination};

fn main() {
let node = Node::Array(vec![
Node::Number(Numeric::Integer(1)),
Node::Number(Numeric::Integer(2)),
Node::Number(Numeric::Integer(3)),
]);

    let mut yaml = BufferDestination::new();
    to_yaml(&node, &mut yaml);
    println!("YAML:\n{}", yaml.to_string());

    let mut xml = BufferDestination::new();
    to_xml(&node, &mut xml);
    println!("XML:\n{}", xml.to_string());

    let mut bencode = BufferDestination::new();
    to_bencode(&node, &mut bencode);
    println!("Bencode:\n{}", bencode.to_string());
}
```
Read/write text files with Unicode BOM handling:
```
rust
use json_lib::{Format, detect_format, read_file_to_string, write_file_from_string};

fn main() -> Result<(), String> {
// Detect BOM/format
let fmt = detect_format("input.txt").unwrap_or(Format::UTF8);
let content = read_file_to_string("input.txt")?;

    // Write as UTF-8 (no BOM) regardless of input
    write_file_from_string("output.txt", &content, Format::UTF8)
}
```
## API overview

- Version
  - version() -> &'static str

- I/O sources and destinations
  - FileSource / FileDestination
  - BufferSource / BufferDestination

- Node model
  - Node: Object, Array, Str, Number(Numeric), Boolean, None
  - Numeric: Integer, UInteger, Float, Byte, Int8, Int16, UInt16, Int32, UInt32

- Parsing and stringifying
  - parse(&mut Source) -> Result<Node, String>
  - stringify(&Node, &mut Destination)
  - Pretty print: json_lib::misc::print(&Node, &mut Destination, indent, current_indent)

- Alternative format encoders
  - to_yaml(&Node, &mut Destination)
  - to_xml(&Node, &mut Destination)
  - to_bencode(&Node, &mut Destination)
  - to_toml(&Node, &mut Destination)

- Unicode-aware file helpers
  - detect_format(path) -> Result<Format, String>
  - read_file_to_string(path) -> Result<String, String>
  - write_file_from_string(path, &str, Format) -> Result<(), String>

## Example: maintaining a JSON-backed sequence

A typical workflow:
- Read from a file using FileSource
- Parse into a Node
- Mutate the Node in memory
- Write back to a file with stringify and FileDestination

This pattern works for configuration files, logs, and machine-generated data.

## Design goals

- Small, explicit API focused on Node-based manipulation
- Works with streams and simple abstractions
- Deterministic and predictable output
- Easy to integrate for CLI tools and small services

## Roadmap ideas

- Optional parse/stringify options (e.g., key sorting, non-ASCII escaping)
- Streaming parser and incremental writer
- JSON Pointer, Patch, and Merge Patch utilities
- Canonical JSON and stable hashing
- Optional serde interop and async I/O

If you’re interested in any of these, contributions are welcome.

## License

This project is licensed under the terms of the LICENSE file included in the repository.

## Contributing

- Open issues for bugs and feature requests
- PRs with tests are welcome
- Please format and lint your code before submitting

## Support

If you run into issues or have questions, please open an issue in the repository.
```
