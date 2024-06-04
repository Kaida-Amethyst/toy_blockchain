#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;

    #[test]
    fn print_transactions() {
        let tx = Transaction::new_coinbse_tx("abxgtsunkodojahucd");
        tx.print();
    }
}
