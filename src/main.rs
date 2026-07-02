mod database;
use database::format::encode_header;
use database::format::decode_header;


fn main() {
    let encoded = encode_header(10, 5, 5);
    println!("encoded length: {}", encoded.len());

    let (timestamp, key_size, val_size) = decode_header(&encoded);

    println!("timestamp: {}", timestamp);
    println!("key_size: {}", key_size);
    println!("val_size: {}", val_size);
}
