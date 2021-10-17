//! Implementation of KvStore interface with persistent storage using message
//! passing via network sockets.
//!
//! - the instance of a NetKvs shall spawn a worker thread that listens on a user defined port.
//! - the client shall forward the method calls to the worker thread via the socket interface.
//! - the key/value data should be stored in the filesystem by the worker thread.
//! - The key/value serialization method is up to the implementor.

#[cfg(test)]
mod tests {
    #[test]
    fn get_stored_value() {
        todo!();
    }

    #[test]
    fn overwrite_value() {
        todo!();
    }

    #[test]
    fn get_non_existent_value() {
        todo!();
    }

    #[test]
    fn remove_key() {
        todo!();
    }
}