pub mod format;
pub mod memory_store; 
pub mod disk_store;

use std::io;
use crate::database::format::FormatError;

#[derive(Debug)]
pub enum DatabaseError {
    Io(io::Error),
    Format(FormatError),
}

impl From<io::Error> for DatabaseError {
    fn from(error: io::Error) -> Self {
        DatabaseError::Io(error)
    }
}