use crate::store::KvStore;
use bincode::{borrow_decode_from_slice, encode_to_vec, Decode, Encode};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Encode, Decode, Debug)]
pub enum Message {
    Get(String),
    Set(String, String),
    Response(Option<String>),
}

pub fn handle_client(
    mut stream: TcpStream,
    kv_store: Arc<KvStore>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut len_buf = [0u8; 4];
        if stream.read_exact(&mut len_buf).is_err() {
            break; // Connection closed
        }
        let len = u32::from_be_bytes(len_buf) as usize;

        let mut data = vec![0u8; len];
        if stream.read_exact(&mut data).is_err() {
            break; // Connection closed
        }

        let (message, _): (Message, usize) =
            borrow_decode_from_slice(&data, bincode::config::standard()).unwrap();

        let response = handle_message(message, &kv_store)?;

        let response_data = encode_to_vec(&response, bincode::config::standard()).unwrap();
        let response_len = response_data.len() as u32;
        let response_len_buf = response_len.to_be_bytes();

        stream.write_all(&response_len_buf)?;
        stream.write_all(&response_data)?;
    }
    Ok(())
}

fn handle_message(
    message: Message,
    kv_store: &Arc<KvStore>,
) -> Result<Message, Box<dyn std::error::Error>> {
    match message {
        Message::Get(key) => {
            let value = kv_store.get(&key);
            Ok(Message::Response(value))
        }
        Message::Set(key, value) => {
            kv_store.set(key, value);
            Ok(Message::Response(None))
        }
        _ => Ok(Message::Response(Some("Unknown message type".to_string()))),
    }
}
