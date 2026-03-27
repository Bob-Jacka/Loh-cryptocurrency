use std::sync::{Arc, Mutex};

/**
Blockchain transaction to add in blockchain register
*/
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

pub struct TXOutput {
    value: i32,
    pub_key: Vec<u8>,
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
