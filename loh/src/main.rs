mod base_types;
mod block;
mod blockchain;
mod data_save;
mod node;
mod server;
mod tests;
mod transaction;
mod wallet;

use std::sync::{Arc, Mutex};
use std::thread;

/**
I want this function to be infinity loop for interaction with blockchain in console
*/
pub fn blockchain_net() {
    loop {
        //
    }
}

fn main() {
    let blockchain = blockchain::new();
    let v = Arc::new(Mutex::new(blockchain));

    let handle = thread::spawn(move || {
        let mut data = v.lock().unwrap();
    });

    handle.join().unwrap();
}
