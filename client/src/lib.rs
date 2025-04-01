use bincode::{borrow_decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Decode, Encode, Debug)]
pub enum Message {
    Get(String),
    Set(String, String),
    Response(Option<String>),
}

pub struct KvStoreClient {
    address: String,
}

impl KvStoreClient {
    pub fn new(address: String) -> Self {
        KvStoreClient { address }
    }

    pub fn get(&self, key: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(&self.address)?;

        let get_message = Message::Get(key);
        let get_data = encode_to_vec(&get_message, bincode::config::standard()).unwrap();
        let get_len = get_data.len() as u32;
        let get_len_buf = get_len.to_be_bytes();

        stream.write_all(&get_len_buf)?;
        stream.write_all(&get_data)?;

        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;

        let mut data = vec![0u8; len];
        stream.read_exact(&mut data)?;

        let (response, _): (Message, usize) = borrow_decode_from_slice(&data, bincode::config::standard()).unwrap();
        match response {
            Message::Response(value) => Ok(value),
            _ => Err("Unexpected response".into()),
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(&self.address)?;

        let set_message = Message::Set(key, value);
        let set_data = encode_to_vec(&set_message, bincode::config::standard()).unwrap();
        let set_len = set_data.len() as u32;
        let set_len_buf = set_len.to_be_bytes();

        stream.write_all(&set_len_buf)?;
        stream.write_all(&set_data)?;

        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;

        let mut data = vec![0u8; len];
        stream.read_exact(&mut data)?;

        let (response, _): (Message, usize) = borrow_decode_from_slice(&data, bincode::config::standard()).unwrap();
        match response {
            Message::Response(None) => Ok(()),
            _ => Err("Unexpected response".into()),
        }
    }
}
