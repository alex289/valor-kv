use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::sync::Mutex;

use postcard::{from_bytes, to_allocvec};

pub struct KvStore {
    data: Mutex<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }

    pub fn load_from_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        if reader.read_to_end(&mut buffer)? == 0 {
            // File is empty, initialize with an empty HashMap
            let mut store = self.data.lock().unwrap();
            *store = HashMap::new();
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let data: HashMap<String, String> = from_bytes(&contents)?;

        let mut store = self.data.lock().unwrap();
        *store = data;
        Ok(())
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.data.lock().unwrap();
        let mut file = File::create(path)?;
        let bytes = to_allocvec(&*data)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_set_and_get() {
        let store = KvStore::new();
        store.set("key1".to_string(), "value1".to_string());
        assert_eq!(store.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KvStore::new();
        assert_eq!(store.get("nonexistent"), None);
    }

    #[test]
    fn test_load_and_save() {
        let store = KvStore::new();
        let path = "test_data.json";

        store.set("key1".to_string(), "value1".to_string());
        store.set("key2".to_string(), "value2".to_string());

        store.save_to_file(path).unwrap();

        let new_store = KvStore::new();
        new_store.load_from_file(path).unwrap();

        assert_eq!(new_store.get("key1"), Some("value1".to_string()));
        assert_eq!(new_store.get("key2"), Some("value2".to_string()));

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_load_from_nonexistent_file() {
        let store = KvStore::new();
        let path = "nonexistent_file.bin";

        let result = store.load_from_file(path);

        assert!(result.is_ok());
        assert!(Path::new(path).exists());
        std::fs::remove_file(path).unwrap();
    }
}
