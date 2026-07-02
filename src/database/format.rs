use std::io::Read;

// use std::process::ExitCode;
use bincode;
use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};

// timestamp 4bytes + keysize 4bytes + valsize 4bytes
pub const HEADER_SIZE: usize = 12;

//KeyEntry lives in memory

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEntry {
    pub timestamp: u32,
    pub position: u32,
    pub total_size: u32,
}

//lives on disk
#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Header {
    timestamp: u32,
    key_size: u32,
    val_size: u32,
}

//returns data represented by growable list of bytes
pub fn encode_header(timestamp: u32, key_size: u32, val_size: u32) -> Vec<u8> {
    let header = Header {
        timestamp,
        key_size,
        val_size,
    };

    //stores u32 exactly as 4 bytes
    let config = config::standard().with_fixed_int_encoding();

    let encoded: Vec<u8> = bincode::serde::encode_to_vec(header, config).unwrap();

    println!("{}", encoded.len());

    return encoded;
}
// //takes in and reads bytes (does not own or write)
pub fn decode_header(data: &[u8]) -> (u32, u32, u32) {
    let config = config::standard().with_fixed_int_encoding();

    let (header, _bytes_read): (Header, usize) =
        bincode::serde::borrow_decode_from_slice(data, config).unwrap();

    return (header.timestamp, header.key_size, header.val_size);
}

pub fn encode_kv(timestamp: u32, key: &str, val: &str) -> (usize, Vec<u8>) {
    let key_size = key.len();
    let val_size = val.len();

    let mut encoded: Vec<u8> = encode_header(
        timestamp,
        key_size.try_into().unwrap(),
        val_size.try_into().unwrap(),
    );

    encoded.extend_from_slice(key.as_bytes());

    encoded.extend_from_slice(val.as_bytes());

    let total_size = encoded.len();

    return (total_size, encoded);
}

pub fn decode_kv(data: &[u8]) -> (u32, String, String) {

    //decode header
    let (timestamp, key_size,val_size) = decode_header(data);

    let key_size = key_size as usize; //memory index needs to be usize 
    let val_size = val_size as usize; 

    //slicing header from key and val
    let key_start = HEADER_SIZE; 
    let key_end = key_size + key_start;
    let val_start = key_end;
    let val_end = val_start + val_size;

    let key_bytes = &data[key_start..key_end];
    let val_bytes = &data[val_start..val_end]; 

    //bytes to string 
    let key = str::from_utf8(key_bytes).unwrap().to_owned(); 
    let val = str::from_utf8(val_bytes).unwrap().to_owned();


    (timestamp, key, val)
  
}
