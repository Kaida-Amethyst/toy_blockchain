#[cfg(test)]
mod tests {
    use crate::block::Block;
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
}
