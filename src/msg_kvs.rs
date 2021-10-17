//! Implementation of KvStore interface with persistent storage using message
//! passing (std::mpsc).
//!
//! - the instance of a NetKvs shall spawn a worker thread that listens on the
//!   message queue receiving end.
//! - the client shall forward the method calls to the worker thread via the
//!   message queue sending-end.
//! - the key/value data should be stored in the filesystem by the worker thread.
//! - the key/value serialization method is up to the implementor.
//! 
//! The get/set/delete functions shall forward the parameters to the worker thread
//! via message passing (rust std::mpsc queue).
//!
//! Implementation hint:
//! The `get` function requires the server to send out a response thus we
//! require a bidirectional channel.
//! Rust mpsc queues are unidirectional, thus we should find a smart way to let
//! the server send a response to the client.
//! One pattern is to send the channel for the response in the request channel
//! along with the other request parameters.
//!
//! ```
//! fn get(&self, key: String) -> String {
//!     let (response_tx, response_rx) = channel();
//!     // we assume that `self` has already the request channel to communicate
//!     // with the worker thread...
//!     self.request_tx.send((key, response_tx)).unwrap();
//!     let value = response_rx.recv().unwrap()
//!     value
//! }
//! ```

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