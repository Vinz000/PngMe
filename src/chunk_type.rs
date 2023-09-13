use std::convert;
use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_bit_zero(bit: u8, n: u8) -> bool {
        let mask = 1 << n;
        bit & mask == 0
    }

    fn is_critical(&self) -> bool {
        Self::is_bit_zero(self.bytes()[0], 5)
    }

    fn is_public(&self) -> bool {
        Self::is_bit_zero(self.bytes()[1], 5)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        Self::is_bit_zero(self.bytes()[2], 5)
    }

    fn is_safe_to_copy(&self) -> bool {
        !Self::is_bit_zero(self.bytes()[3], 5)
    }

    fn is_valid(&self) -> bool {
        self.bytes()[2].is_ascii_uppercase()
    }

    fn is_err(&self) -> bool {
        !self.is_valid()
    }

    fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }
}

impl convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if let Some(&byte) = bytes.iter().find(|&&byte| !byte.is_ascii()) {
            return Err(ChunkTypeError::InvalidByte(byte));
        }

        Ok(ChunkType { bytes })
    }
}

impl str::FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ChunkTypeError::WrongLength(s.len()));
        }

        let s_bytes = &s.as_bytes()[0..4];
        let bytes = <[u8; 4]>::try_from(s_bytes).unwrap();

        if let Some(&byte) = bytes.iter().find(|&&byte| !Self::is_valid_byte(byte)) {
            return Err(ChunkTypeError::InvalidByte(byte));
        }

        Ok(ChunkType { bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes_str = str::from_utf8(&self.bytes).unwrap();

        write!(f, "{}", bytes_str)
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidByte(u8),
    WrongLength(usize),
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidByte(byte) => write!(f, "Invalid Byte: {}", byte),
            Self::WrongLength(len) => write!(f, "Wrong length: {} (expected 4)", len),
        }
    }
}

impl Error for ChunkTypeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
