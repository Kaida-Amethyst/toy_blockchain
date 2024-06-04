use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

// fields:
//   - timestamp: Timestamp of the block
//   - pre_block_hash: Previous block hash
//   - hash: Current block hash
//   - transactions: Vector of transactions
//   - nonce: TODO: need more explanation
//   - height: Height of the block, TODO: need more explanation
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    timestamp: u64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new(pre_block_hash: String, transactions: Vec<Transaction>) -> Block {
        let block = Block {
            timestamp: 0,
            pre_block_hash,
            hash: String::new(),
            transactions,
            nonce: 0,  // TODO: Need to fix this
            height: 0, // TODO: Need to fix this
        };
        //
        // proof of work
        //
        block
    }

    pub fn print(&self) {
        println!("timestamp: {}", self.timestamp);
        println!("pre_block_hash: {}", self.pre_block_hash);
        println!("hash: {}", self.hash);
        println!("nonce: {}", self.nonce);
        println!("height: {}", self.height);
        for tx in &self.transactions {
            tx.print();
        }
    }
}
