use rust_caskdb::database::memory_store::MemoryStorage;

#[test]
fn test_get() {
    let mut store = MemoryStorage::new();

    store.set("name".to_string(), "jojo".to_string());

    assert_eq!(store.get("name"), Some("jojo"));
}

#[test]
fn test_invalid_key() {
    let store = MemoryStorage::new();

    assert_eq!(store.get("some key"), None);
}

#[test]
fn test_close() {
    let mut store = MemoryStorage::new();

    store.close();
}