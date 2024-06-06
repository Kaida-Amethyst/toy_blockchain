#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::blockchain::{BlockChain, BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY};
    use crate::transaction::Transaction;

    #[test]
    fn print_transactions() {
        let tx = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
        tx.print();
    }

    #[test]
    fn print_block() {
        let tx = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
        let genesis_pre_hash = String::from("0x0");
        let bk = Block::new(genesis_pre_hash, vec![tx], 0);
        bk.print();
    }

    #[test]
    fn create_blockchain() {
        let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
        println!("Tip block hash: {}", blockchain.get_tip_hash());
        let db: &sled::Db = blockchain.get_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
        let tip_block_hash = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_block_hash = String::from_utf8(tip_block_hash.unwrap().to_vec()).unwrap();
        let tip_block_data = blocks_tree.get(tip_block_hash).unwrap();
        let tip_block = Block::deserialize(&tip_block_data.unwrap());
        tip_block.print();
    }
}
