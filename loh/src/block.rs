use crate::transaction::Transaction;
extern crate bincode;
use bincode::{deserialize, serialize};

/**
* Block in transaction
*/
pub struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    time_stamp: i64,
    transactions: Vec<Transaction>,
    txn_data: String,
    nonce: u64,
    height: usize,
}

pub struct ProofOfWork {
    blocks: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(blocks: Block) -> ProofOfWork {}
}

impl Block {
    pub fn new(
        id: u64,
        hash: String,
        previous_hash: String,
        time: i64,
        txn_data: String,
        nonce: u64,
    ) -> Block {
        Self {
            id,
            hash,
            previous_hash,
            time_stamp: time,
            txn_data,
            nonce,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn get_time(&self) -> i64 {
        self.time_stamp
    }

    pub fn get_txn_data(&self) -> &str {
        &self.txn_data
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec();
    }

    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap();
    }
}

pub fn new_block(pre_block: String, transactions: &[Vec<Transaction>], height: usize) -> Block {
    let mut block = Block {
        time_stamp: crate::current_timestamp(),
        previous_hash,
        hash: String::new(),
        transactions: transactions.to_vec(),
        nonce: 0,
        height,
    };
    block
}

pub fn generate_genesis_block() {}

pub fn hash_transaction(input: String) {}
