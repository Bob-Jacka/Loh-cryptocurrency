use crate::base_types::Object;
use crate::block::Block;
use crate::blockchain::Blockchain;

#[test]
fn test_add_block1() {
    let mut blockchain = Blockchain::new();
    blockchain.add(Block::new());
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
