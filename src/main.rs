// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;

// use block::Block;
use blockchain::BlockChain;
use transaction::Transaction;

// use crate::{
//     block::Block,
//     blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
// };

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    let transaction = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
    let block = blockchain.mine_block(&[transaction]);
    // check block and tip block in db
    println!("mined block: ");
    block.print();
    println!("\nVisit all blocks: ");
    let mut block_iterator = blockchain.iterator();
    while let Some(block) = block_iterator.next() {
        block.print();
    }
}

mod tests;
