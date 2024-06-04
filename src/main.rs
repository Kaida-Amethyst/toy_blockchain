// Toy Block Chain

mod transaction;
mod utils;

use transaction::Transaction;

fn main() {
    let tx = Transaction::new_coinbse_tx("abxgtsunkodojahucd");
    tx.print();
}

mod tests;
