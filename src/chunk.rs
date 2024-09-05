use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};
use crc::Crc;
use crate::chunk_type::ChunkType;

pub fn bytes_to_word(bytes: &[u8]) -> u32 {
    let mut reader = BufReader::new(bytes);
    let mut buffer: [u8; 4] = [0, 0, 0, 0];
    reader.read_exact(&mut buffer).expect("");
    u32::from_be_bytes(buffer)
}

pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let feild = chunk_type.bytes().iter()
                           .chain(data.iter())
                           .copied().collect::<Vec<u8>>();
        Chunk{
            data_length: data.len() as u32,
            chunk_type: chunk_type,
            chunk_data: data,
            crc: Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&feild)
        }
    }

    pub fn length(&self) -> u32 {
        self.data_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data as &[u8]
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, ()> {
        Ok(String::from_utf8_lossy(&self.chunk_data).to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.data_length.to_be_bytes());
        bytes.extend(self.chunk_type.bytes());
        bytes.extend(self.chunk_data.iter());
        bytes.extend(self.crc.to_be_bytes());
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let length = value.len();
        if length < 12 {
            return Err("Too short data")
        }
        let chunk_length = bytes_to_word(&value[0..4]);
        if length != (12 + chunk_length) as usize {
            return Err("Not consistent length field")
        }
        let chunk_type = ChunkType::new(&value[4..8]);
        if !chunk_type.is_valid() {
            return Err("Invalid chunk type")
        }
        let chunk_data = Vec::from(&value[8..length - 4]);
        let crc = bytes_to_word(&value[length - 4..length]);
        let chunk = Chunk::new(chunk_type, chunk_data);
        if chunk.crc != crc {
            return Err("Invalid crc checksum")
        }
        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
        data_length: {}\n\
        chunk_type: {}\n\
        data: {:?}\n\
        crc: {}\n\
        ", self.data_length, self.chunk_type, self.chunk_data, self.crc)
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
