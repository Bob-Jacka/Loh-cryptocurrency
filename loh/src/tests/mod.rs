use rand::{random, Rng};
use crate::base_types::Object;
use crate::block::Block;
use crate::blockchain::Blockchain;

fn return_valid_block() -> Block {
    let mut block: Block = Block::new(
        random.next_u64(),
        "Hash".to_string(),
        "Previous_hash".to_string(),
        random.next_u64(),
        "Some_data".to_string(),
        3241,
    );
    block
}

fn return_multiple_valid_block() -> Vec<Block> {
    let mut blocks_vec: Block = Block::new()

    blocks_vec
}

fn return_invalid_block() -> Block {
    let mut block: Block = Block {
        ..Default::default()
    };
    block
}

#[test]
fn test_add_block1() {
    let mut blockchain = Blockchain::new();
    blockchain.add(return_valid_block());
    assert_eq!(blockchain.size(), 1);
}

#[test]
fn test_add_block2() {
    let mut blockchain = Blockchain::new();
    blockchain.add(Block::new());
    blockchain.add(Block::new());
    blockchain.add(Block::new());
    assert_eq!(blockchain.size(), 1);
}

#[test]
fn test_get_block() {
    let mut blockchain = Blockchain::new();
    blockchain.add(Block::new());
    assert_eq!(blockchain.get(0).unwrap().get_id(), 0);
}
