use rust_caskdb::database::format::{
    decode_header,
    decode_kv,
    encode_header,
    encode_kv,
};

use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const HEADER_SIZE: usize = 12;

fn get_random_header() -> (u32, u32, u32) {
    let mut rng = rand::rng();

    (
        rng.random::<u32>(),
        rng.random::<u32>(),
        rng.random::<u32>(),
    )
}

fn get_random_kv() -> (u32, String, String, usize) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before the Unix epoch")
        .as_secs() as u32;

    let key = Uuid::new_v4().to_string();
    let val = Uuid::new_v4().to_string();

    let size = HEADER_SIZE + key.len() + val.len();

    (timestamp, key, val, size)
}

#[derive(Debug)]
struct Header {
    timestamp: u32,
    key_size: u32,
    val_size: u32,
}

#[derive(Debug)]
struct KeyValue {
    timestamp: u32,
    key: String,
    val: String,
    sz: usize,
}

fn header_test(tt: Header) {
    let data = encode_header(tt.timestamp, tt.key_size, tt.val_size);
    let (timestamp, key_size, val_size) = decode_header(&data);

    assert_eq!(tt.timestamp, timestamp);
    assert_eq!(tt.key_size, key_size);
    assert_eq!(tt.val_size, val_size);
}

#[test]
fn test_header_serialisation() {
    let tests = vec![
        Header {
            timestamp: 10,
            key_size: 10,
            val_size: 10,
        },
        Header {
            timestamp: 0,
            key_size: 0,
            val_size: 0,
        },
        Header {
            timestamp: 10_000,
            key_size: 10_000,
            val_size: 10_000,
        },
    ];

    for tt in tests {
        header_test(tt);
    }
}

#[test]
fn test_header_random() {
    for _ in 0..100 {
        let (timestamp, key_size, val_size) = get_random_header();

        let tt = Header {
            timestamp,
            key_size,
            val_size,
        };

        header_test(tt);
    }
}

#[test]
fn test_header_maximum_value() {
    /*
    The Python test tries to pass 2^32, which is too large for a four-byte
    unsigned integer.

    In Rust, encode_header accepts u32 values, so passing 2^32 is prevented
    by the compiler. The largest value we can test at runtime is u32::MAX.
    */

    let tt = Header {
        timestamp: u32::MAX,
        key_size: 5,
        val_size: 5,
    };

    header_test(tt);
}

fn kv_test(tt: KeyValue) {
    let (size, data) = encode_kv(tt.timestamp, &tt.key, &tt.val);
    let (timestamp, key, val) = decode_kv(&data);

    assert_eq!(tt.timestamp, timestamp);
    assert_eq!(tt.key, key);
    assert_eq!(tt.val, val);
    assert_eq!(tt.sz, size);
}

#[test]
fn test_kv_serialisation() {
    let tests = vec![
        KeyValue {
            timestamp: 10,
            key: "hello".to_string(),
            val: "world".to_string(),
            sz: HEADER_SIZE + 10,
        },
        KeyValue {
            timestamp: 0,
            key: String::new(),
            val: String::new(),
            sz: HEADER_SIZE,
        },
    ];

    for tt in tests {
        kv_test(tt);
    }
}

#[test]
fn test_kv_random() {
    for _ in 0..100 {
        let (timestamp, key, val, size) = get_random_kv();

        let tt = KeyValue {
            timestamp,
            key,
            val,
            sz: size,
        };

        kv_test(tt);
    }
}