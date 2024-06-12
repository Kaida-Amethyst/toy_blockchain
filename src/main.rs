// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;
mod utxo_set;

// use block::Block;
use blockchain::BlockChain;
use transaction::{TXOutput, Transaction};

use std::collections::HashMap;

// use crate::{
//     block::Block,
//     blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
// };

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    let transaction = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
    let _ = blockchain.mine_block(&[transaction]);

    let utxo: HashMap<String, Vec<TXOutput>> = blockchain.find_utxo();
    for (k, v) in utxo.iter() {
        println!("==============================");
        println!("txid: {}", k);
        for txo in v {
            println!("  TXOutput: {:?}", txo);
        }
    }
}

mod tests;
