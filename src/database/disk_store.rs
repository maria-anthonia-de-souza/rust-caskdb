//! The `disk_store` module implements a key-value store persisted to disk.
//!
//! `DiskStorage` provides two basic operations:
//!
//! - `set`: stores a string key-value pair
//! - `get`: retrieves the value associated with a key
//!
//! All records are persisted to disk. During initialization, `DiskStorage`
//! loads the metadata for existing records into an in-memory index.
//!
//! If the database file is large, initialization may take longer because the
//! file must be scanned before the database can be used.
//!
//! # Example
//!
// ! ```ignore
// ! let mut disk = DiskStorage::new("books.db");
// !
// ! disk.set("othello", "shakespeare");
// ! let author = disk.get("othello");
// ! ```

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use dashmap::DashMap;
use super::DatabaseError;
use crate::database::format::FormatError;
use crate::database::{
   format::{HEADER_SIZE, KeyEntry, decode_header, decode_kv, encode_kv},
};
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DiskStorage {
    file_path: PathBuf,                //remember where the file is located
    file: File,                        //open connection to the file for reading, writting and seek
    keydir: DashMap<String, KeyEntry>, //which key is it for and where is the last record for this key stored in the file
}

impl DiskStorage {
    /// Creates or opens a disk-backed key-value store.
    ///
    /// `file_name` may be a file in the current directory or a full path.
    pub fn new<P: AsRef<Path>>(file_name: P) -> Result<Self, DatabaseError> {
        let file_path = file_name.as_ref().to_path_buf();

        let mut keydir = DashMap::new();

        if file_path.exists() {
            Self::init_keydir(&file_path, &mut keydir)?;
        }
        //open or create new db file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        Ok(Self {
            file_path,
            file,
            keydir,
        })
    }

   fn init_keydir(
    file_path: &Path,
    keydir: &mut DashMap<String, KeyEntry>,
) -> Result<(), DatabaseError> {
    let mut file = File::open(file_path)?;
    let mut position = 0u64;
    let mut header_buff = [0u8; HEADER_SIZE];

    loop {
        let bytes_read = file.read(&mut header_buff)?;

        // Zero bytes means we reached the normal end of the file.
        if bytes_read == 0 {
            break;
        }

        // Some header bytes exist, but not enough for a complete header.
        if bytes_read != HEADER_SIZE {
            return Err(DatabaseError::Format(
                FormatError::Header(bytes_read),
            ));
        }

        let (timestamp, key_size, val_size) =
            decode_header(&header_buff);

        let total_size =
            HEADER_SIZE as u32 + key_size + val_size;

        let mut key_buff = vec![0u8; key_size as usize];
        file.read_exact(&mut key_buff)?;

        let key = String::from_utf8(key_buff)
            .map_err(|_| {
                DatabaseError::Format(
                    FormatError::InvalidKey,
                )
            })?;

        let key_entry = KeyEntry {
            timestamp,
            position,
            total_size,
        };

        keydir.insert(key, key_entry);

        // Move our tracked position to the next record.
        position += total_size as u64;

        // The header and key were read already, so skip the value.
        file.seek(SeekFrom::Current(val_size as i64))?;
    }

    Ok(())
}

    /// Stores a key-value pair by appending a record to the database file.
    pub fn set(&mut self, key: &str, val: &str) -> std::io::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before the Unix epoch")
            .as_secs() as u32;

        let (total_size, encoded_bytes) = encode_kv(timestamp, key, val);
        let next_pos = self.file.seek(SeekFrom::End(0))?;
        //write bytes that are stored in memory to db file
        self.file.write_all(&encoded_bytes)?;

        self.keydir.insert(
            key.to_string(),
            KeyEntry {
                timestamp,
                position: next_pos,
                total_size: total_size as u32,
            },
        );

        Ok(())
    }

    /// Retrieves the value associated with a key.
    pub fn get(&mut self, key: &str) -> std::io::Result<Option<String>> {
        let entry = match self.keydir.get(key) {
            Some(entry) => entry,
            None => return Ok(None),
        };

        let position = entry.position;
        let total_size = entry.total_size;

        //move byte to the start of records position
        self.file.seek(SeekFrom::Start(position as u64))?;

        //temp memory used to hold the bytes read from a file
        //read_exact will replace the zeroes with bytes from the file
        let mut buffer = vec![0u8; total_size as usize];

        //read the record and write to buff
        self.file.read_exact(&mut buffer)?; //full encoded record 

        let (_timestamp, _key, val) = decode_kv(&buffer);

        Ok(Some(val))
    }

    /// Closes the database file.
    pub fn close(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
