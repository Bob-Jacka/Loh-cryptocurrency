//Declarations

/**
* Block in transaction
*/
pub struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    time: i64,
    txn_data: String,
    nonce: u64,
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

//Implementations

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
            time,
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
        self.time
    }
    pub fn get_txn_data(&self) -> &str {
        &self.txn_data
    }
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
    pub fn destroy(&mut self) {}
    pub fn add(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn get(&self, id: u64) -> Option<&Block> {
        self.blocks.iter().find(|b| b.id == id)
    }
}

fn main() {
    let blockchain = Blockchain::new();
}
