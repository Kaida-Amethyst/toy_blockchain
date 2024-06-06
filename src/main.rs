// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;

// use block::Block;
// use transaction::Transaction;
use blockchain::BlockChain;

use crate::{
    block::Block,
    blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
};

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    println!("Tip block hash: {}", blockchain.get_tip_hash());
    let db: &sled::Db = blockchain.get_db();
    let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
    let tip_block_hash = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_block_hash = String::from_utf8(tip_block_hash.unwrap().to_vec()).unwrap();
    let tip_block_data = blocks_tree.get(tip_block_hash).unwrap();
    let tip_block = Block::deserialize(&tip_block_data.unwrap());
    tip_block.print();
}

mod tests;
