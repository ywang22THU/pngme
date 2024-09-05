use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {
    pub fn new(byte: &[u8]) -> Self {
        ChunkType{
            bytes: [
                byte[0],
                byte[1],
                byte[2],
                byte[3]
            ]
        }
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }

    fn valid_char(c: u8) -> bool {
        c.is_ascii_lowercase() || c.is_ascii_uppercase()
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.bytes {
            if ChunkType::valid_char(byte) {
                continue
            }
            return false
        }
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0] & 0x20 == 0
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1] & 0x20 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 0x20 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & 0x20 != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType{bytes: value};
        if chunk_type.is_valid() {
            Ok(chunk_type)
        }
        else {
            Err("Error while creating ChunkType from [u8; 4]")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slice = s.as_bytes();
        let chunk_type = ChunkType::new(slice);
        if chunk_type.is_valid() {
            Ok(chunk_type)
        }
        else {
            Err("Error while creating ChunkType from Str")
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}",
               self.bytes[0] as char, self.bytes[1] as char,
               self.bytes[2] as char, self.bytes[3] as char)
    }
}

impl Default for ChunkType {
    fn default() -> Self {
        ChunkType{
            bytes: [0xff, 0xff, 0xff, 0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap_or_default();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap_or_default();
        let actual = ChunkType::from_str("RuSt").unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap_or_default();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap_or_default();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap_or_default();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap_or_default();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap_or_default();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap_or_default();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap_or_default();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap_or_default();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

