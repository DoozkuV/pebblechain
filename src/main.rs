use std::time::SystemTime;

use sha2::{Digest, Sha256};

// TODO: Consider converting to use Bytes crate instead of u8 arrays for
// more efficient memory usage
#[derive(Debug)]
struct Block {
    pub timestamp: SystemTime,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
    pub data: String,
}

impl Block {
    pub fn new(data: String, previous_hash: [u8; 32]) -> Self {
        let timestamp = SystemTime::now();
        let hash = Block::_compute_hash(&data, &previous_hash, &timestamp);

        Self {
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        Block::_compute_hash(&self.data, &self.previous_hash, &self.timestamp)
    }

    fn _compute_hash(data: &str, previous_hash: &[u8; 32], timestamp: &SystemTime) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(previous_hash);
        hasher.update(data);
        hasher.update(
            &timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_le_bytes(),
        );
        hasher.finalize().into()
    }
}

#[derive(Debug)]
struct Blockchain(Vec<Block>);

impl Blockchain {
    fn new() -> Self {
        Self(vec![Block::new("Genesis Block".to_string(), [0; 32])])
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.0.last().unwrap();
        self.0.push(Block::new(data, prev_block.hash));
    }

    fn is_chain_valid(&self) -> bool {
        self.0
            .windows(2)
            .all(|b| b[1].hash == b[1].compute_hash() && b[1].previous_hash == b[0].hash)
    }
}

fn main() {
    println!("Hello, world!");

    let mut blockchain = Blockchain::new();

    blockchain.add_block("Hello World!".to_string());
    blockchain.add_block("Goodbye world!".to_string());

    println!("blockchain valid: {}", blockchain.is_chain_valid());

    println!("Blockchain status: {:?}", blockchain);
}
