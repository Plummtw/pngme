// https://picklenerd.github.io/pngme_book/chapter_1.html
use std::fmt::Debug;
use std::str::FromStr;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub(crate) struct ChunkType {
    data: [u8; 4]
    // crc: u32,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(data: [u8; 4]) -> Result<Self, Self::Error> {
        let result = ChunkType { data };
        if result.is_all_letter() {
            Ok(result)
        } else {
            Err(())
        }
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = [0; 4];
        for (i, c) in s.chars().enumerate() {
            data[i] = c as u8;
        }
        let result = ChunkType { data };
        if result.is_all_letter() {
            Ok(result)
        } else {
            Err(())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }   
}

impl Debug for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", String::from_utf8_lossy(&self.data))
  }   
}

#[allow(dead_code)]
impl ChunkType {
    pub(crate) fn bytes(&self) -> [u8; 4] {
       self.data
    }
    fn is_all_letter(&self) -> bool {
        self.data.iter().all(|c| 
            c.is_ascii_uppercase() || 
            c.is_ascii_lowercase())
    }
    fn is_valid(&self) -> bool {
        self.is_all_letter() &&
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        (self.data[0] & 0b100000) == 0
    }
    fn is_public(&self) -> bool {
        (self.data[1] & 0b100000) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        (self.data[2] & 0b100000) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        (self.data[3] & 0b100000) == 0b100000
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