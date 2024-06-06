// Toy Block Chain

mod block;
mod transaction;
mod utils;

use block::Block;
use transaction::Transaction;

fn main() {
    let tx = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
    let genesis_pre_hash = String::from("0x0");
    let bk = Block::new(genesis_pre_hash, vec![tx], 0);
    bk.print();
}

mod tests;
