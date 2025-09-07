use std::fs::File;
use std::io::{Read, Result, Write};

/// Represents different Unicode text file formats with their corresponding byte order marks (BOM)
pub enum Format {
    Utf8,        // UTF-8 without BOM
    Utf8bom,     // UTF-8 with BOM (EF BB BF)
    Utf16le,     // UTF-16 Little Endian (FF FE)
    Utf16be,     // UTF-16 Big Endian (FE FF)
    Utf32le,     // UTF-32 Little Endian (FF FE 00 00)
    Utf32be,     // UTF-32 Big Endian (00 00 FE FF)
}

impl Format {
    /// Returns the byte order mark (BOM) bytes for each format
    fn get_bom(&self) -> &'static [u8] {
        match self {
            Format::Utf8 => &[],
            Format::Utf8bom => &[0xEF, 0xBB, 0xBF],
            Format::Utf16le => &[0xFF, 0xFE],
            Format::Utf16be => &[0xFE, 0xFF],
            Format::Utf32le => &[0xFF, 0xFE, 0x00, 0x00],
            Format::Utf32be => &[0x00, 0x00, 0xFE, 0xFF],
        }
    }
}

/// Detects the Unicode format of a text file by examining its byte order mark (BOM)
pub fn detect_format(filename: &str) -> Result<Format> {
    let mut file = File::open(filename)?;
    let mut bom_buffer = [0u8; 4];
    let bytes_read = file.read(&mut bom_buffer)?;

    let format = match &bom_buffer[..bytes_read] {
        [0xEF, 0xBB, 0xBF, ..] => Format::Utf8bom,
        [0xFE, 0xFF, ..] => Format::Utf16be,
        [0xFF, 0xFE, 0x00, 0x00] => Format::Utf32le,
        [0x00, 0x00, 0xFE, 0xFF] => Format::Utf32be,
        [0xFF, 0xFE, ..] => Format::Utf16le,
        _ => Format::Utf8
    };

    Ok(format)
}

/// Writes a string to a file in the specified Unicode format
pub fn write_file_from_string(filename: &str, content: &str, format: Format) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(format.get_bom())?;

    match format {
        Format::Utf8 | Format::Utf8bom => {
            file.write_all(content.as_bytes())?;
        }
        Format::Utf16le => {
            for c in content.encode_utf16() {
                file.write_all(&c.to_le_bytes())?;
            }
        }
        Format::Utf16be => {
            for c in content.encode_utf16() {
                file.write_all(&c.to_be_bytes())?;
            }
        }
        Format::Utf32le => {
            for c in content.chars() {
                file.write_all(&(c as u32).to_le_bytes())?;
            }
        }
        Format::Utf32be => {
            for c in content.chars() {
                file.write_all(&(c as u32).to_be_bytes())?;
            }
        }
    }
    Ok(())
}

/// Reads a text file and returns its content as a String, handling different Unicode formats
pub fn read_file_to_string(filename: &str) -> Result<String> {
    let mut content = String::new();
    let format = detect_format(filename)?;
    let mut file = File::open(filename)?;

    /// Helper function to read and skip over the BOM bytes
    fn read_and_skip_bom(file: &mut File, size: usize) -> Result<()> {
        let mut buf = vec![0u8; size];
        file.read_exact(&mut buf)
    }

    /// Helper function to process UTF-16 encoded files
    fn process_utf16(file: &mut File, is_be: bool) -> Result<String> {
        read_and_skip_bom(file, 2)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let content = String::from_utf16(
            &bytes.chunks(2)
                .map(|chunk| if is_be {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                } else {
                    u16::from_le_bytes([chunk[0], chunk[1]])
                })
                .collect::<Vec<u16>>()
        ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(content.replace("\r\n", "\n"))
    }

    /// Helper function to process UTF-32 encoded files
    fn process_utf32(file: &mut File, is_be: bool) -> Result<String> {
        read_and_skip_bom(file, 4)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let content = bytes.chunks(4)
            .map(|chunk| if is_be {
                u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            } else {
                u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
            })
            .map(|cp| char::from_u32(cp).unwrap_or('\u{FFFD}'))
            .collect::<String>();

        Ok(content.replace("\r\n", "\n"))
    }

    match format {
        Format::Utf8bom => {
            read_and_skip_bom(&mut file, 3)?;
            file.read_to_string(&mut content)?;
        }
        Format::Utf16be => return process_utf16(&mut file, true),
        Format::Utf16le => return process_utf16(&mut file, false),
        Format::Utf32be => return process_utf32(&mut file, true),
        Format::Utf32le => return process_utf32(&mut file, false),
        Format::Utf8 => {
            file.read_to_string(&mut content)?;
        }
    }

    Ok(content.replace("\r\n", "\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Creates a test file with the specified BOM and content
    fn create_test_file(filename: &str, bom: &[u8]) -> Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(bom)?;
        file.write_all(b"test content")?;
        Ok(())
    }

    #[test]
    fn test_utf8() -> Result<()> {
        create_test_file("test_utf8.txt", &[])?;
        assert!(matches!(detect_format("test_utf8.txt")?, Format::Utf8));
        fs::remove_file("test_utf8.txt")?;
        Ok(())
    }

    #[test]
    fn test_utf8_bom() -> Result<()> {
        create_test_file("test_utf8_bom.txt", &[0xEF, 0xBB, 0xBF])?;
        assert!(matches!(detect_format("test_utf8_bom.txt")?, Format::Utf8bom));
        fs::remove_file("test_utf8_bom.txt")?;
        Ok(())
    }

    #[test]
    fn test_utf16_le() -> Result<()> {
        create_test_file("test_utf16le.txt", &[0xFF, 0xFE])?;
        assert!(matches!(detect_format("test_utf16le.txt")?, Format::Utf16le));
        fs::remove_file("test_utf16le.txt")?;
        Ok(())
    }

    #[test]
    fn test_utf16_be() -> Result<()> {
        create_test_file("test_utf16be.txt", &[0xFE, 0xFF])?;
        assert!(matches!(detect_format("test_utf16be.txt")?, Format::Utf16be));
        fs::remove_file("test_utf16be.txt")?;
        Ok(())
    }

    #[test]
    fn test_utf32_le() -> Result<()> {
        create_test_file("test_utf32le.txt", &[0xFF, 0xFE, 0x00, 0x00])?;
        assert!(matches!(detect_format("test_utf32le.txt")?, Format::Utf32le));
        fs::remove_file("test_utf32le.txt")?;
        Ok(())
    }

    #[test]
    fn test_utf32_be() -> Result<()> {
        create_test_file("test_utf32be.txt", &[0x00, 0x00, 0xFE, 0xFF])?;
        assert!(matches!(detect_format("test_utf32be.txt")?, Format::Utf32be));
        fs::remove_file("test_utf32be.txt")?;
        Ok(())
    }

    #[test]
    fn test_formatted_testfile021() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile021.json")?, Format::Utf8));
        Ok(())
    }
    #[test]
    fn test_formatted_testfile022() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile022.json")?, Format::Utf8bom));
        Ok(())
    }

    #[test]
    fn test_formatted_testfile023() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile023.json")?, Format::Utf16be));
        Ok(())
    }

    #[test]
    fn test_formatted_testfile024() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile024.json")?, Format::Utf16le));
        Ok(())
    }

    #[test]
    fn test_formatted_testfile025() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile025.json")?, Format::Utf32be));
        Ok(())
    }

    #[test]
    fn test_formatted_testfile026() -> Result<()> {
        assert!(matches!(detect_format("../files/formatted/testfile026.json")?, Format::Utf32le));
        Ok(())
    }

    fn verify_file_content(filename: &str, expected: &str) -> Result<()> {
        let content = read_file_to_string(filename)?;
        assert_eq!(content, expected);
        Ok(())
    }

    #[test]
    fn test_read_utf8_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile021.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_read_utf8bom_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile022.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_read_utf16be_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile023.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_read_utf16le_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile024.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_read_utf32be_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile025.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_read_utf32le_file() -> Result<()> {
        verify_file_content("../files/formatted/testfile026.json", "[true  , \"Out of time\",  7.89043e+18, true]")
    }

    #[test]
    fn test_write_utf8() -> Result<()> {
        let test_content = "Test UTF-8 content";
        write_file_from_string("test_write_utf8.txt", test_content, Format::Utf8)?;
        assert_eq!(read_file_to_string("test_write_utf8.txt")?, test_content);
        fs::remove_file("test_write_utf8.txt")?;
        Ok(())
    }

    #[test]
    fn test_write_utf8bom() -> Result<()> {
        let test_content = "Test UTF-8 BOM content";
        write_file_from_string("test_write_utf8bom.txt", test_content, Format::Utf8bom)?;
        assert_eq!(read_file_to_string("test_write_utf8bom.txt")?, test_content);
        fs::remove_file("test_write_utf8bom.txt")?;
        Ok(())
    }

    #[test]
    fn test_write_utf16le() -> Result<()> {
        let test_content = "Test UTF-16LE content";
        write_file_from_string("test_write_utf16le.txt", test_content, Format::Utf16le)?;
        assert_eq!(read_file_to_string("test_write_utf16le.txt")?, test_content);
        fs::remove_file("test_write_utf16le.txt")?;
        Ok(())
    }

    #[test]
    fn test_write_utf16be() -> Result<()> {
        let test_content = "Test UTF-16BE content";
        write_file_from_string("test_write_utf16be.txt", test_content, Format::Utf16be)?;
        assert_eq!(read_file_to_string("test_write_utf16be.txt")?, test_content);
        fs::remove_file("test_write_utf16be.txt")?;
        Ok(())
    }

    #[test]
    fn test_write_utf32le() -> Result<()> {
        let test_content = "Test UTF-32LE content";
        write_file_from_string("test_write_utf32le.txt", test_content, Format::Utf32le)?;
        assert_eq!(read_file_to_string("test_write_utf32le.txt")?, test_content);
        fs::remove_file("test_write_utf32le.txt")?;
        Ok(())
    }

    #[test]
    fn test_write_utf32be() -> Result<()> {
        let test_content = "Test UTF-32BE content";
        write_file_from_string("test_write_utf32be.txt", test_content, Format::Utf32be)?;
        assert_eq!(read_file_to_string("test_write_utf32be.txt")?, test_content);
        fs::remove_file("test_write_utf32be.txt")?;
        Ok(())
    }
    #[test]
    fn test_write_formatted_testfile021() -> Result<()> {
        let test_content = "[true  , \"Out of time\",  7.89043e+18, true]";
        write_file_from_string("../files/formatted/testfile021.json", test_content, Format::Utf8)?;
        assert_eq!(read_file_to_string("../files/formatted/testfile021.json")?, test_content);
        Ok(())
    }
    #[test]
    fn test_write_formatted_testfile022() -> Result<()> {
        let test_content = "[true  , \"Out of time\",  7.89043e+18, true]";
        write_file_from_string("../files/formatted/testfile022.json", test_content, Format::Utf8bom)?;
        assert_eq!(read_file_to_string("../files/formatted/testfile022.json")?, test_content);
        Ok(())
    }
}