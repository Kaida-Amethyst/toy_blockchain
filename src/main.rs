// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;

// use block::Block;
// use transaction::Transaction;
use blockchain::BlockChain;

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    println!("Tip block hash: {}", blockchain.get_tip_hash());
}

mod tests;
