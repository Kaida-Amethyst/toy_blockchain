// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;

// use block::Block;
// use transaction::Transaction;
use blockchain::BlockChain;

use crate::block::Block;

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    println!("Tip block hash: {}", blockchain.get_tip_hash());
    let db = blockchain.get_db();
    if let Ok(blocks_tree) = db.open_tree("blockchain") {
        if let Ok(block_serialied_data) = blocks_tree.get("tip_block_hash") {
            let block_deserialized: block::Block =
                Block::deserialize(block_serialied_data.unwrap().to_vec().as_slice());
            block_deserialized.print();
        } else {
            println!("Failed to get tip block hash");
        }
    } else {
        println!("Failed to open tree");
    }
}

mod tests;
