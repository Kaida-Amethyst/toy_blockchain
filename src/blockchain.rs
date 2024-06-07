use crate::block::Block;
use crate::transaction::Transaction;
use sled::transaction::{self, TransactionResult};
/// BlockChain
use sled::{Db, Tree};
use std::env::current_dir;
use std::sync::{Arc, RwLock};

pub const DB_NAME: &str = "blockchain_data";
pub const BLOCKS_TREE_NAME: &str = "blockchain";
pub const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";

/// In BlockChain struct, we record two fileds:
///   1. tip_hash: the hash of the last block
///   2. db: sled::Db, the database to store the blockchain data
/// Once we want to iterator the blockchain, we chould search the block data from the db by the hash
pub struct BlockChain {
    tip_hash: Arc<RwLock<String>>, // the hash of the last block
    db: Db,
}

impl BlockChain {
    /// If we already have DB on current_dir, we will open it and get the tip block hash.
    /// If not, create a new DB and generate a genesis block.
    pub fn create_blockchain(genesis_address: &str) -> BlockChain {
        let db = sled::open(current_dir().unwrap().join(DB_NAME)).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
        let block_data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();

        let tip_hash = if block_data.is_none() {
            println!("Database not found, Create a new blockchain");
            println!("using address: {} as the genesis address", genesis_address);
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let genesis_block = Block::generate_genesis_block(coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &genesis_block);
            String::from(genesis_block.get_hash())
        } else {
            String::from_utf8(block_data.unwrap().to_vec()).unwrap()
        };

        BlockChain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx| {
            let _ = tx.insert(block_hash, block.clone());
            let _ = tx.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash);
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_best_height(&self) -> usize {
        // Read the tip block from the db
        let blocks_tree = self.db.open_tree(BLOCKS_TREE_NAME).unwrap();
        let tip_block_data = blocks_tree.get(self.get_tip_hash()).unwrap();
        let tip_block: Block = Block::deserialize(&tip_block_data.unwrap());
        tip_block.get_height()
    }

    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        let best_height = self.get_best_height();
        let block: Block = Block::new(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();
        let blocks_tree = self.db.open_tree(BLOCKS_TREE_NAME).unwrap();
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);
        block
    }
}
