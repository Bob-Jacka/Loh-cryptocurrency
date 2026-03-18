use std::sync::Mutex;
use crate::base_types::{Object, SynBlock};
use crate::block::Block;

/**
Main class for blockchain entity
*/
pub struct Blockchain {
    blocks: SynBlock,
}

impl Object for Blockchain {
    fn new() -> Self {
        Self {
            blocks: SynBlock::new(Mutex::new(vec![])),
        }
    }

    fn destroy(&mut self) {
        self.blocks.lock().unwrap().clear();
    }
}

impl Blockchain {
    pub fn add(&mut self, block: Block) {
        self.blocks.lock().unwrap().push(block);
    }

    pub fn get(&self, id: u64) -> Option<&Block> {
        (self.blocks.lock().unwrap()).iter().find(|b| b.id == id)
    }

    pub fn size(&self) -> usize {
        self.blocks.lock().unwrap().len()
    }
}

pub fn create_blockchain() {}

pub fn update_block_tree() {}

pub fn new_blockchain() {}

pub fn add_block() {}
