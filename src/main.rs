// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;

// use block::Block;
use blockchain::BlockChain;
use transaction::Transaction;

use crate::{
    block::Block,
    blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
};

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    let transaction = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
    let block = blockchain.mine_block(&[transaction]);
    // check block and tip block in db
    println!("mined block: ");
    block.print();
    println!("\nTip block: ");
    let db = blockchain.get_db();
    let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
    let tip_blocks_hash = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
    let tip_blocks_hash = String::from_utf8(tip_blocks_hash.unwrap().to_vec()).unwrap();
    let tip_block: Block = Block::deserialize(&blocks_tree.get(tip_blocks_hash).unwrap().unwrap());
    tip_block.print();
}

mod tests;
