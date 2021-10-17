//! Groups together the testing code used by the various implementations.

use crate::kvs::KvStore;

// Should get previously stored value
pub fn get_stored_value<T: KvStore>(store: &mut T) {
    store.set("key1".to_owned(), "value1".to_owned());
    store.set("key2".to_owned(), "value2".to_owned());

    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
}

// Should overwrite existent value
pub fn overwrite_value<T: KvStore>(store: &mut T) {
    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));

    store.set("key1".to_owned(), "value2".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
}

// Should get `None` when getting a non-existent key
pub fn get_non_existent_value<T: KvStore>(store: &mut T) {
    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key2".to_owned()), None);
}

// Should remove a value by key.
pub fn remove_key<T: KvStore>(store: &mut T) {
    store.set("key1".to_owned(), "value1".to_owned());
    store.remove("key1".to_owned());
    assert_eq!(store.get("key1".to_owned()), None);
}