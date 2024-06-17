// Toy Block Chain

mod block;
mod blockchain;
mod transaction;
mod utils;
mod utxo_set;
mod wallet;

// use block::Block;
// use blockchain::BlockChain;
// use transaction::{TXOutput, Transaction};
use utils::hex_encode;
// use utxo_set::UtxoSet;
//
// use std::collections::HashMap;
use wallet::Wallet;

// use crate::{
//     block::Block,
//     blockchain::{BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY},
// };

fn main() {
    // let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
    // let transaction = Transaction::new_coinbase_tx("hegtsodoucahjsubxg");
    //
    // let addr1 = "abxgtsunkodojahucd";
    // let addr2 = "hegtsodoucahjsubxg";

    let w1 = Wallet::new();
    let w2 = Wallet::new();

    let addr1 = w1.get_address();
    let addr2 = w2.get_address();

    let pub_key1 = w1.get_public_key();
    let pub_key2 = w2.get_public_key();

    let pub_key_hash1 = wallet::hash_pub_key(&pub_key1);
    let pub_key_hash2 = wallet::hash_pub_key(&pub_key2);

    let pub_key_hash_from_addr1 = bs58::decode(&addr1).into_vec().unwrap();
    let pub_key_hash_from_addr1 =
        &pub_key_hash_from_addr1[1..pub_key_hash_from_addr1.len() - 4].to_vec();

    let pub_key_hash_from_addr2 = bs58::decode(&addr2).into_vec().unwrap();
    let pub_key_hash_from_addr2 =
        &pub_key_hash_from_addr2[1..pub_key_hash_from_addr2.len() - 4].to_vec();

    let pub_key_hash1_str = hex_encode(&pub_key_hash1);
    let pub_key_hash2_str = hex_encode(&pub_key_hash2);

    let pub_key_hash_from_addr1_str = hex_encode(&pub_key_hash_from_addr1);
    let pub_key_hash_from_addr2_str = hex_encode(&pub_key_hash_from_addr2);

    assert_eq!(pub_key_hash1_str, pub_key_hash_from_addr1_str);
    assert_eq!(pub_key_hash2_str, pub_key_hash_from_addr2_str);
}

mod tests;
