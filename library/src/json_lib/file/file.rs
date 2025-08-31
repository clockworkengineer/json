use std::fs::File;
use std::io::{Read, Result, Write};

pub enum Format {
    Utf8,
    Utf8bom,
    Utf16le,
    Utf16be,
    Utf32le,
    Utf32be,
}

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

pub fn read_file_to_string(filename: &str) -> Result<String> {
    let mut content = String::new();
    let format = detect_format(filename)?;
    let mut file = File::open(filename)?;
    match format {
        Format::Utf8bom => {
            let mut buf = [0u8; 3];
            file.read_exact(&mut buf)?;
        }
        Format::Utf16be => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            content = String::from_utf16(
                &bytes.chunks(2)
                    .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                    .collect::<Vec<u16>>()
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            return Ok(content.replace("\r\n", "\n"));
        }
        Format::Utf16le => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            content = String::from_utf16(
                &bytes.chunks(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect::<Vec<u16>>()
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            return Ok(content.replace("\r\n", "\n"));
        }
        Format::Utf32be => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let code_points = bytes.chunks(4)
                .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect::<Vec<u32>>();
            content = code_points.into_iter()
                .map(|cp| char::from_u32(cp).unwrap_or('\u{FFFD}'))
                .collect::<String>();
            return Ok(content.replace("\r\n", "\n"));
        }
        Format::Utf32le => {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            let code_points = bytes.chunks(4)
                .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect::<Vec<u32>>();
            content = code_points.into_iter()
                .map(|cp| char::from_u32(cp).unwrap_or('\u{FFFD}'))
                .collect::<String>();
            return Ok(content.replace("\r\n", "\n"));
        }
        Format::Utf8 => {}
    }
    file.read_to_string(&mut content)?;

    // Convert CRLF to LF
    Ok(content.replace("\r\n", "\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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
}