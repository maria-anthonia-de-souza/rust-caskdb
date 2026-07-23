use rust_caskdb::database::disk_store::DiskStorage;
use rust_caskdb::database::DatabaseError;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

/// Provides a temporary database path for disk-storage tests.
///
/// The temporary directory remains available while this struct exists.
/// When `clean_up` consumes the struct, the directory and database file
/// are deleted.
struct TempStorageFile {
    directory: TempDir,
    path: PathBuf,
}

impl TempStorageFile {
    /// Creates a temporary directory and returns a path for a database file.
    fn new() -> io::Result<Self> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("test.db");

        Ok(Self { directory, path })
    }

    /// Creates a temporary directory with a specific database filename.
    fn with_file_name(file_name: &str) -> io::Result<Self> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join(file_name);

        Ok(Self { directory, path })
    }

    /// Returns the database file path.
    fn path(&self) -> &Path {
        &self.path
    }

    /// Explicitly removes the temporary directory and its contents.
    fn clean_up(self) -> io::Result<()> {
        self.directory.close()
    }
}

fn test_values() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("crime and punishment", "dostoevsky"),
        ("anna karenina", "tolstoy"),
        ("war and peace", "tolstoy"),
        ("hamlet", "shakespeare"),
        ("othello", "shakespeare"),
        ("brave new world", "huxley"),
        ("dune", "frank herbert"),
    ])
}

#[test]
fn test_get() ->  Result<(), DatabaseError> {
    let temp_file = TempStorageFile::new()?;
    let mut store = DiskStorage::new(temp_file.path())?;

    store.set("name", "jojo")?;

    assert_eq!(store.get("name")?, Some("jojo".to_string()));

    store.close()?;
    temp_file.clean_up()?;

    Ok(())
}

#[test]
fn test_invalid_key() ->  Result<(), DatabaseError> {
    let temp_file = TempStorageFile::new()?;
    let mut store = DiskStorage::new(temp_file.path())?;

    assert_eq!(store.get("some key")?, None);

    store.close()?;
    temp_file.clean_up()?;

    Ok(())
}

/*
Python supports dictionary syntax:

    store["name"] = "jojo"
    store["name"]

Your current Rust DiskStorage API does not implement equivalent indexing
traits, so the direct Rust equivalent uses set and get.
*/
#[test]
fn test_store_api() ->  Result<(), DatabaseError> {
    let temp_file = TempStorageFile::new()?;
    let mut store = DiskStorage::new(temp_file.path())?;

    store.set("name", "jojo")?;

    assert_eq!(store.get("name")?, Some("jojo".to_string()));

    store.close()?;
    temp_file.clean_up()?;

    Ok(())
}

#[test]
fn test_persistence() ->  Result<(), DatabaseError> {
    let temp_file = TempStorageFile::new()?;
    let tests = test_values();

    {
        let mut store = DiskStorage::new(temp_file.path())?;

        for (key, value) in &tests {
            store.set(key, value)?;

            assert_eq!(
                store.get(key)?,
                Some((*value).to_string()),
            );
        }

        store.close()?;
    }

    {
        let mut store = DiskStorage::new(temp_file.path())?;

        for (key, value) in &tests {
            assert_eq!(
                store.get(key)?,
                Some((*value).to_string()),
            );
        }

        store.close()?;
    }

    temp_file.clean_up()?;

    Ok(())
}

#[test]
fn test_deletion() -> Result<(), DatabaseError> {
    let temp_file = TempStorageFile::new()?;
    let tests = test_values();

    {
        let mut store = DiskStorage::new(temp_file.path())?;

        for (key, value) in &tests {
            store.set(key, value)?;
        }

        // The Python version represents deletion by storing an empty string.
        for key in tests.keys() {
            store.set(key, "")?;
        }

        store.set("end", "yes")?;
        store.close()?;
    }

    {
        let mut store = DiskStorage::new(temp_file.path())?;

        for key in tests.keys() {
            assert_eq!(
                store.get(key)?,
                Some(String::new()),
            );
        }

        assert_eq!(
            store.get("end")?,
            Some("yes".to_string()),
        );

        store.close()?;
    }

    temp_file.clean_up()?;

    Ok(())
}

#[test]
fn test_get_existing_file() -> Result<(), DatabaseError> {
    let temp_file = TempStorageFile::with_file_name("temp.db")?;

    {
        let mut store = DiskStorage::new(temp_file.path())?;

        store.set("name", "jojo")?;

        assert_eq!(
            store.get("name")?,
            Some("jojo".to_string()),
        );

        store.close()?;
    }

    // Open the same database file again and check the key.
    {
        let mut store = DiskStorage::new(temp_file.path())?;

        assert_eq!(
            store.get("name")?,
            Some("jojo".to_string()),
        );

        store.close()?;
    }

    temp_file.clean_up()?;

    Ok(())
}