use std::sync::{Arc, Mutex};

/**
Blockchain transaction to add in blockchain register
*/
pub struct Transaction {
    id: u64,
    hash: String,
}

//Implementations

impl Transaction {
    pub fn new() -> Self {
        Self {
            id: 0,
            hash: String::new(),
        }
    }

    pub fn commit_transaction(&self, blockchain: Arc<Mutex<Blockchain>>) {}
}