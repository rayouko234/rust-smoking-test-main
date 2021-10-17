// Ues one of your implementations and use it to implement
// a simple command line application using the "Clap" crate.
//
// Expected interface:
// $ kvs get <key>
// $ kvs set <key> <value>
// $ kvs remote <key>

mod kvs;
mod mem_kvs;
mod fs_kvs;
mod net_kvs;
mod msg_kvs;

// Include only for tests
#[cfg(test)]
mod tests;

use mem_kvs::MemStore;

fn main() {
    let _dummy = MemStore::new();
}
