use core::fmt;

/* # Transaction Module
 *
 * ## UXTO Model anc account model
 *
 *  In this project, we use UTXO Model
 *  What is UTXO?
 *  In real world, we are familiar with account model, which is like a bank account
 *  Suppose there is three persons, Alice Bob, and Charlie, they both have an account in the bank
 *  Now Alice want to transfer 10 coins to Bob and 5 coins to Charlie
 *  In Block chain, the speed of block generation is not that fast
 *  If the blockchain use account model, after ALice transfer 10 coins to Bob, the transaction is not confirmed
 *  She has to wait for the next block to be generated, which may cost 10 minutes, enen longer
 *  But in UTXO model, Alice may have many UTXOs, each UTXO conatins some coins
 *  So if she want to transfer 10 coins to Bob and 5 coins to Charlie, she can use two UTXOs at the same time
 *  That is why you can see the Transaction struct has two fields: vin and vout, which are vectors, not just an addrss
 */
use crate::utils::hex_encode;
use crate::utils::sha256_digest;
use serde::{Deserialize, Serialize};

/// UTXO input
/// fields:
///   - txid: Previous transaction ID, Notice that this is `Vec<u8>` instead of String
///           Because in rust, char is 4 bytes rather than 1 byte like C
///   - vout: Previous transaction output index
///   - signature: Signature of the transaction
///   - pub_key: Public key of the transaction
///
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

impl fmt::Debug for TXInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TXInput {{ txid: {:?}, vout: {:?}, signature: {:?}, pub_key: {:?} }}",
            hex_encode(&self.txid),
            self.vout,
            hex_encode(&self.signature),
            hex_encode(&self.pub_key)
        )
    }
}

/// UTXO output
/// fields:
///   - value: number of coins
///   - pub_key_hash: Public key hash
#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl fmt::Debug for TXOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TXOutput {{ value: {:?}, pub_key_hash: {:?} }}",
            self.value,
            hex_encode(&self.pub_key_hash)
        )
    }
}

/// Transaction Struct
/// fields:
///   - id: Transaction ID
///   - vin: Vector of UTXO input
///   - vout: Vector of UTXO output
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl TXInput {
    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    pub fn get_vout(&self) -> usize {
        self.vout
    }
}

impl TXOutput {
    /// Note: the parameter is `address`
    /// The `new` function will extract the public key hash from the address
    pub fn new(value: i32, address: &str) -> TXOutput {
        let mut output = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        // use bs58 to decode pub_key_hash from address
        // Note: it is pub_key_hash, not pub_key, so don't feel confused
        output.lock(address);
        output
    }

    fn lock(&mut self, address: &str) {
        let decode = bs58::decode(address).into_vec().unwrap();
        let pub_key_hash = &decode[1..decode.len() - 4];
        self.pub_key_hash = pub_key_hash.to_vec();
    }

    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl Transaction {
    /// function `new_coinbase_tx` is used when miner mined a new block, the root would reward the miner
    /// Since it has no input, so there is only one parameter, `to`
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let txout = TXOutput::new(10, to); // TODO: replace 10 with a variable
        let tx_input = TXInput::default(); // use default, because there is no input
        let mut tx = Transaction {
            id: Vec::new(),
            vin: vec![tx_input],
            vout: vec![txout],
        };
        tx.id = tx.hash();
        tx
    }

    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].pub_key.len() == 0
    }

    fn hash(&self) -> Vec<u8> {
        let tx_clone = self.clone();
        sha256_digest(&tx_clone.serialize().as_slice())
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    pub fn get_vout(&self) -> &[TXOutput] {
        &self.vout
    }

    pub fn get_vin(&self) -> &[TXInput] {
        &self.vin
    }

    pub fn print(&self) {
        println!("tx.id: {:?}", hex_encode(&self.id));
        println!("tx.vin: [");
        for item in self.vin.iter() {
            println!("  {{");
            println!("    txid: {:?}", hex_encode(&item.txid));
            println!("    vout: {:?}", item.vout);
            println!("    signature: {:?}", hex_encode(&item.signature));
            println!("    pub_key: {:?}", hex_encode(&item.pub_key));
            println!("  }}");
        }
        println!("]");

        println!("tx.vout: [");
        for item in self.vout.iter() {
            println!("  {{");
            println!("    value: {:?}", item.value);
            println!("    pub_key_hash: {:?}", hex_encode(&item.pub_key_hash));
            println!("  }}");
        }
        println!("]");
    }
}
