use data_encoding::HEXLOWER;

use crate::blockchain::BlockChain;
use crate::transaction::TXOutput;

pub const UTXO_TREE: &str = "chainstate";

pub struct UtxoSet {
    blockchain: BlockChain,
}

impl UtxoSet {
    pub fn new(blockchain: BlockChain) -> UtxoSet {
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
}
