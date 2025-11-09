# JSON Unicode Files

Demonstrates Unicode file handling with BOM (Byte Order Mark) detection and multi-format read/write capabilities.

## Features Demonstrated

- **Writing files** in different Unicode formats (UTF-8, UTF-16, UTF-32)
- **BOM handling** for UTF-8, UTF-16 LE/BE, UTF-32 LE/BE
- **Automatic format detection** when reading files
- **Round-trip conversion** between formats
- **FileSource/FileDestination** with Unicode content
- **Error handling** for file operations

## Usage

```bash
cargo run
```

## Key Concepts

### Unicode Formats Supported

| Format | BOM Bytes | Description |
|--------|-----------|-------------|
| UTF-8 (no BOM) | None | Variable-length (1-4 bytes), most common |
| UTF-8 with BOM | EF BB BF | UTF-8 with byte order mark |
| UTF-16 LE | FF FE | 16-bit little-endian |
| UTF-16 BE | FE FF | 16-bit big-endian |
| UTF-32 LE | FF FE 00 00 | 32-bit little-endian |
| UTF-32 BE | 00 00 FE FF | 32-bit big-endian |

### Writing Files

```rust
use json_lib::{write_file_from_string, Format};

let content = r#"{"greeting": "Hello, 世界!"}"#;

// Write as UTF-8
write_file_from_string("output.json", content, Format::Utf8)?;

// Write as UTF-16 LE
write_file_from_string("output.json", content, Format::Utf16le)?;
```

### Detecting Format

```rust
use json_lib::detect_format;

match detect_format("input.json") {
    Ok(format) => println!("Detected format: {:?}", format),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Reading Files

```rust
use json_lib::read_file_to_string;

// Automatically handles BOM and converts to UTF-8 string
let content = read_file_to_string("input.json")?;
```

### Round-Trip Conversion

```rust
use json_lib::{read_file_to_string, write_file_from_string, Format};

// Read from any format
let content = read_file_to_string("input.json")?;

// Write to different format
write_file_from_string("output.json", &content, Format::Utf16le)?;
```

### Using with Parser

```rust
use json_lib::{read_file_to_string, parse, BufferSource};

// Read file with automatic format detection
let content = read_file_to_string("config.json")?;

// Parse JSON
let mut source = BufferSource::new(content.as_bytes());
let node = parse(&mut source)?;
```

## Use Cases

- **Internationalization**: Handle JSON files with various international characters
- **Legacy systems**: Work with files that use UTF-16 or UTF-32
- **Cross-platform compatibility**: Ensure proper BOM handling across different systems
- **Format conversion**: Convert between Unicode formats while preserving content

## Learn More

- Library documentation: `json_lib::file::file`
- [Unicode Standard](https://unicode.org/standard/standard.html)
- [Byte Order Mark (BOM)](https://en.wikipedia.org/wiki/Byte_order_mark)
