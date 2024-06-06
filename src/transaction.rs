// Implement Transaction
// UTXO Model
use crate::utils::hex_encode;
use crate::utils::sha256_digest;
use serde::{Deserialize, Serialize};

// UTXO input
// fields:
//   - txid: Previous transaction ID, Notice that this is Vec<u8> instead of String
//           Because in rust, char is 4 bytes rather than 1 byte like C
//   - vout: Previous transaction output index
//   - signature: Signature of the transaction
//   - pub_key: Public key of the transaction
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
struct TXInput {
    txid: Vec<u8>,
    vout: i32,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

// fields:
//   - value: number of coins
//   - pub_key_hash: Public key hash
#[derive(Clone, Serialize, Deserialize, Debug)]
struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

// fields:
//   - id: Transaction ID
//   - vin: Vector of UTXO input
//   - vout: Vector of UTXO output
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl TXOutput {
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
}

impl Transaction {
    // Used when miner mined a new block, the root would reward the miner
    // Since it has no input, so there is only one parameter, `to`
    pub fn new_coinbse_tx(to: &str) -> Transaction {
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

    fn hash(&self) -> Vec<u8> {
        let tx_clone = self.clone();
        sha256_digest(&tx_clone.serialize().as_slice())
    }

    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
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
