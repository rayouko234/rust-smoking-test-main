//! Trivial non persistent implementation of KvStore using an HashMap.

use crate::kvs::KvStore;
use std::collections::HashMap;

pub struct MemStore {
    map: HashMap<String, String>
}

impl MemStore {
    pub fn new() -> MemStore {
        MemStore {
            map: HashMap::new()
        }
    }
}

impl KvStore for MemStore {
    fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    fn set(&mut self, key: String, value: String) {
        let _ = self.map.insert(key, value);
    }

    fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests;

    #[test]
    fn get_stored_value() {
        let mut store = MemStore::new();
        tests::get_stored_value(&mut store);
    }

    #[test]
    fn overwrite_value() {
        let mut store = MemStore::new();
        tests::overwrite_value(&mut store);
    }

    #[test]
    fn get_non_existent_value() {
        let mut store = MemStore::new();
        tests::get_non_existent_value(&mut store);
    }

    #[test]
    fn remove_key() {
        let mut store = MemStore::new();
        crate::tests::remove_key(&mut store);
    }
}
