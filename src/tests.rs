#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::blockchain::{BlockChain, BLOCKS_TREE_NAME, TIP_BLOCK_HASH_KEY};
    use crate::transaction::{TXOutput, Transaction};
    use crate::utils::hex_encode;
    use crate::utxo_set::UtxoSet;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::sync::Mutex;

    static TEST_MUTX: Mutex<()> = Mutex::new(());

    // rm -rf blockchain_data
    fn clean_db() {
        let _guard = TEST_MUTX.lock().unwrap();
        let db_path = Path::new("blockchain_data");
        if db_path.exists() {
            fs::remove_dir_all(db_path).unwrap();
        }
    }

    #[test]
    fn print_transactions() {
        let tx = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
        tx.print();
    }

    #[test]
    fn print_block1() {
        let tx = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
        let tx = vec![tx];
        let genesis_pre_hash = String::from("0x0");
        let bk = Block::new(genesis_pre_hash, &tx, 0);
        bk.print();
    }

    #[test]
    fn print_block2() {
        let tx = Transaction::new_coinbase_tx("Heobockchain");
        let tx = vec![tx];
        let genesis_pre_hash = String::from("0x12324567");
        let bk = Block::new(genesis_pre_hash, &tx, 0);
        bk.print();
    }

    #[test]
    fn create_blockchain() {
        let _guard = TEST_MUTX.lock().unwrap();

        let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
        println!("Tip block hash: {}", blockchain.get_tip_hash());
        let db: &sled::Db = blockchain.get_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
        let tip_block_hash = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_block_hash = String::from_utf8(tip_block_hash.unwrap().to_vec()).unwrap();
        let tip_block_data = blocks_tree.get(tip_block_hash).unwrap();
        let tip_block = Block::deserialize(&tip_block_data.unwrap());
        tip_block.print();

        // unlock guard
        drop(_guard);
        clean_db();
    }

    #[test]
    fn mine_block() {
        let _guard = TEST_MUTX.lock().unwrap();

        let blockchain = BlockChain::create_blockchain("bdsaowaappoqcvxhs");
        let transaction = Transaction::new_coinbase_tx("bdsaowaappoqcvxhs");
        let block = blockchain.mine_block(&[transaction]);
        // check block and tip block in db
        println!("mined block: ");
        block.print();
        println!("\nTip block: ");
        let db = blockchain.get_db();
        let blocks_tree = db.open_tree(BLOCKS_TREE_NAME).unwrap();
        let tip_blocks_hash = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_blocks_hash = String::from_utf8(tip_blocks_hash.unwrap().to_vec()).unwrap();
        let tip_block: Block =
            Block::deserialize(&blocks_tree.get(tip_blocks_hash).unwrap().unwrap());
        tip_block.print();

        // unlock guard
        drop(_guard);
        clean_db();
    }

    #[test]
    fn view_all_block() {
        let _guard = TEST_MUTX.lock().unwrap();
        let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
        let transaction = Transaction::new_coinbase_tx("abxgtsunkodojahucd");
        let block = blockchain.mine_block(&[transaction]);
        // check block and tip block in db
        println!("mined block: ");
        block.print();
        println!("\nVisit all blocks: ");
        let mut block_iterator = blockchain.iterator();
        while let Some(block) = block_iterator.next() {
            block.print();
        }

        // unlock guard
        drop(_guard);
        clean_db();
    }

    #[test]
    fn test_find_spendable() {
        let _guard = TEST_MUTX.lock().unwrap();
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

        println!("\n=====Find Spendable=========================\n");

        let utxo_set = UtxoSet::new(&blockchain);
        utxo_set.reindex();
        let addr = "hegtsodoucahjsubxg".as_bytes();
        let decode = bs58::decode(addr).into_vec().unwrap();
        let pub_key_hash = &decode[1..decode.len() - 4];
        let spendable_outputs = utxo_set.find_spendable_outputs(pub_key_hash, 8);
        let pub_key_hash = pub_key_hash.to_vec();
        println!("pub_key_hash: {:?}", hex_encode(&pub_key_hash));
        println!("spendable_outputs: {:?}", spendable_outputs);

        // unlock guard
        drop(_guard);
        clean_db();
    }

    #[test]
    fn test_utxo_transaction() {
        let _guard = TEST_MUTX.lock().unwrap();

        let blockchain = BlockChain::create_blockchain("abxgtsunkodojahucd");
        let transaction = Transaction::new_coinbase_tx("hegtsodoucahjsubxg");
        let _ = blockchain.mine_block(&[transaction]);

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

        // unlock guard
        drop(_guard);
        clean_db();
    }
}
