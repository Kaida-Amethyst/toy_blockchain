use data_encoding::HEXLOWER;
use std::collections::HashMap;

use crate::blockchain::BlockChain;
use crate::transaction::TXOutput;

pub const UTXO_TREE: &str = "chainstate";

pub struct UtxoSet<'a> {
    blockchain: &'a BlockChain,
}

impl<'a> UtxoSet<'a> {
    pub fn new(blockchain: &'a BlockChain) -> UtxoSet {
        UtxoSet { blockchain }
    }

    pub fn reindex(&self) {
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        let _ = utxo_tree.clear().unwrap();

        let utxo_map = self.blockchain.find_utxo();
        for (txid, outs) in utxo_map {
            let txid = HEXLOWER.decode(txid.as_bytes()).unwrap();
            let value = bincode::serialize(&outs).unwrap();
            let _ = utxo_tree.insert(txid.as_slice(), value).unwrap();
        }
    }

    pub fn find_spendable_outputs(
        &self,
        pub_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<usize>>) {
        let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();
        let mut accumulated = 0;
        let db = self.blockchain.get_db();
        let utxo_tree = db.open_tree(UTXO_TREE).unwrap();
        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let outs: Vec<TXOutput> = bincode::deserialize(v.to_vec().as_slice()).unwrap();
            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked_with_key(pub_key_hash) && accumulated < amount {
                    accumulated += out.get_value();
                    if unspent_outputs.contains_key(txid_hex.as_str()) {
                        unspent_outputs
                            .get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(idx);
                    } else {
                        unspent_outputs.insert(txid_hex.clone(), vec![idx]);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }
}
