/**
* Block in transaction
*/
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub time: i64,
    pub txn_data: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
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
    let mut blockchain = Blockchain { blocks: vec![] };
}
