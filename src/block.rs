use crate::transaction::Transaction;
use crate::{utils::hex_encode, utils::sha256_digest};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sled::IVec;

const DIFFICULTY: usize = 2;

// fields:
//   - timestamp: Timestamp of the block
//   - pre_block_hash: Previous block hash
//   - hash: Current block hash
//   - transactions: Vector of transactions
//   - nonce: miner need modify this value to get a hash that less than target
//   - height: Height of the block, it is the index of the block in the chain
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
    pub fn new(pre_block_hash: String, transactions: Vec<Transaction>, height: usize) -> Block {
        let mut block = Block {
            timestamp: 0,
            pre_block_hash,
            hash: String::new(),
            transactions,
            nonce: 0,
            height,
        };
        // Proof of Work
        // The miner need modify the nonce from 0 to N,
        //   until the hash of the block is less than target
        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        block
    }

    pub fn generate_genesis_block(coinbase_tx: Transaction) -> Block {
        Block::new(String::from("None"), vec![coinbase_tx], 0)
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn serialize(&self) -> Vec<u8> {
        if let Ok(serialized) = bincode::serialize(&self) {
            serialized.to_vec()
        } else {
            panic!("Failed to serialize block");
        }
    }

    pub fn deserialize(data: &[u8]) -> Block {
        if let Ok(deserialized) = bincode::deserialize(data) {
            deserialized
        } else {
            panic!("Failed to deserialize block");
        }
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut tx_hashes = Vec::new();
        for tx in &self.transactions {
            tx_hashes.extend(tx.get_id());
        }
        sha256_digest(&tx_hashes)
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

impl From<Block> for IVec {
    fn from(block: Block) -> IVec {
        let bytes = block.serialize();
        Self::from(bytes)
    }
}

struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new(block: Block) -> ProofOfWork {
        let target = BigInt::from(1) << (256 - DIFFICULTY);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(DIFFICULTY.to_be_bytes()); // TODO: why add difficulty?
        data_bytes.extend(nonce.to_be_bytes());
        return data_bytes;
    }

    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash: Vec<u8>;
        println!("Target: {}, Mining block.......", self.target);
        loop {
            let data = self.prepare_data(nonce);
            hash = sha256_digest(&data);
            let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
            if hash_int < self.target {
                println!("Found hash: {}", hex_encode(&hash));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        return (nonce, hex_encode(&hash));
    }
}
