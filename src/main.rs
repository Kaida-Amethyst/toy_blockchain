// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;
mod utxo_set;
mod wallet;

// use block::Block;
use blockchain::BlockChain;
use transaction::{TXOutput, Transaction};
use utils::hex_encode;
use utxo_set::UtxoSet;

use std::collections::HashMap;

// use crate::{
//     block::Block,
//     blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
// };

fn main() {
    let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    let transaction = Transaction::new_coinbase_tx("hegtsodoucahjsubxg");
    let _ = blockchain.mine_block(&[transaction]);

    let utxo: HashMap<String, Vec<TXOutput>> = blockchain.find_utxo();
    for (k, v) in utxo.iter() {
        println!("==============================");
        println!("txid: {}", k);
        for txo in v {
            println!("  TXOutput: {:?}", txo);
        }
    }

    let utxo_set = UtxoSet::new(&blockchain);
    utxo_set.reindex();

    println!("\n==========utxo transaction=================\n");
    let transaction = Transaction::new_utxo_transactions(
        "hegtsodoucahjsubxg",
        "abxgtsunkodojahucd",
        8,
        &utxo_set,
    );
    transaction.print();
    let _ = blockchain.mine_block(&[transaction]);
    println!("\n=====Find Spendable=========================\n");

    utxo_set.reindex();
    let addr = "hegtsodoucahjsubxg".as_bytes();
    let decode = bs58::decode(addr).into_vec().unwrap();
    let pub_key_hash = &decode[1..decode.len() - 4];
    let spendable_outputs = utxo_set.find_spendable_outputs(pub_key_hash, 8);
    let pub_key_hash = pub_key_hash.to_vec();
    println!("pub_key_hash: {:?}", hex_encode(&pub_key_hash));
    println!("spendable_outputs: {:?}", spendable_outputs);
}

mod tests;
