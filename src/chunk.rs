#![allow(dead_code)]
use std::fmt::Debug;
use std::fmt::Display;
use crate::chunk_type::ChunkType;
use crc32fast;

pub(crate) struct Chunk {
    chunk_type: ChunkType, 
    data: Vec<u8>,
    crc: u32
}

impl TryFrom<&[u8]> for Chunk {
  type Error = ();

  fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
      if data.len() < 12 {
          return Err(());
      }
      let chunk_type_data : [u8;4] = data[4..8].try_into().unwrap();
      let chunk_type = ChunkType::try_from(chunk_type_data)?;
      let mut chunk_data = Vec::new();
      chunk_data.extend_from_slice(&data[8..data.len() - 4]);
      let crc = u32::from_be_bytes(data[data.len() - 4..data.len()].try_into().unwrap());

      let chuck_type_with_data = [&chunk_type.bytes() as &[u8], &chunk_data].concat();
      let crc_check = crc32fast::hash(&chuck_type_with_data);
      if crc_check != crc {
          return Err(());
      }

      let result = Chunk { 
        chunk_type: chunk_type,
        data: chunk_data,
        crc: crc
      };
      Ok(result)
  }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.data))
    }   
}

impl Debug for Chunk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", String::from_utf8_lossy(&self.data))
  }   
}


impl Chunk {
    pub(crate) fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let data = data;
        let chuck_type_with_data = [&chunk_type.bytes(), data.as_slice()].concat();
        let crc = crc32fast::hash(&chuck_type_with_data);
        Chunk {
            chunk_type: chunk_type,
            data: data,
            crc: crc
        }
    }

    pub(crate) fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub(crate) fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn crc(&self) -> u32 {
        self.crc
    }

    pub(crate) fn data_as_string(&self) -> super::Result<String> {
        String::from_utf8(self.data.clone()).map_err(|err| 
            Box::new(err) as Box<dyn std::error::Error>)
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut result : Vec<u8> = Vec::new();
        result.extend_from_slice(&self.length().to_be_bytes());
        result.extend_from_slice(&self.chunk_type.bytes());
        result.extend_from_slice(&self.data);
        result.extend_from_slice(&self.crc.to_be_bytes());
        result
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
