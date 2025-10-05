use std::{fmt, time::SystemTime, u64};

use bytes::Bytes;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

/// Represents a single block within the blockchain
#[derive(Debug)]
struct Block {
    timestamp: SystemTime,
    previous_hash: Bytes,
    hash: Bytes,
    data: String,
    nonce: u64,
}

impl Block {
    pub fn new(data: String, previous_hash: Bytes, difficulty: u8) -> Self {
        let timestamp = SystemTime::now();
        let (nonce, hash) = Block::mine_block(&data, &previous_hash, &timestamp, difficulty);

        Self {
            timestamp,
            previous_hash,
            hash,
            data,
            nonce,
        }
    }

    fn mine_block(
        data: &str,
        previous_hash: &Bytes,
        timestamp: &SystemTime,
        difficulty: u8,
    ) -> (u64, Bytes) {
        let prefix = "0".repeat(difficulty as usize);

        (0..u64::MAX)
            .into_par_iter()
            .find_map_any(|nonce| {
                let hash = Block::compute_hash_static(&data, &previous_hash, &timestamp, nonce);
                let hash_hex = hex::encode(&hash);
                if hash_hex.starts_with(&prefix) {
                    Some((nonce, hash.into()))
                } else {
                    None
                }
            })
            .expect("No valid nonce found")
    }

    pub fn compute_hash(&self) -> Bytes {
        Self::compute_hash_static(&self.data, &self.previous_hash, &self.timestamp, self.nonce)
    }

    fn compute_hash_static(
        data: &str,
        previous_hash: &Bytes,
        timestamp: &SystemTime,
        nonce: u64,
    ) -> Bytes {
        let mut hasher = Sha256::new();
        hasher.update(previous_hash);
        hasher.update(data.as_bytes());
        hasher.update(
            &timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_le_bytes(),
        );
        hasher.update(nonce.to_le_bytes());
        Bytes::from_owner(hasher.finalize())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "data: {}\nprev_hash: {}\nhash: {}\nnonce: {}",
            self.data,
            hex::encode(&self.previous_hash),
            hex::encode(&self.hash),
            self.nonce,
        )
    }
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: u8,
}

const DEFAULT_DIFFICULTY: u8 = 6;

impl Blockchain {
    pub fn new() -> Self {
        Blockchain::from_difficulty(DEFAULT_DIFFICULTY)
    }

    pub fn from_difficulty(difficulty: u8) -> Self {
        Self {
            blocks: vec![Block::new(
                "Genesis Block".to_string(),
                Bytes::new(),
                difficulty,
            )],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        self.blocks
            .push(Block::new(data, prev_block.hash.clone(), self.difficulty));
    }

    pub fn is_chain_valid(&self) -> bool {
        self.blocks
            .windows(2)
            .all(|b| b[1].hash == b[1].compute_hash() && b[1].previous_hash == b[0].hash)
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "difficulty: {}", self.difficulty)?;
        for (i, block) in self.blocks.iter().enumerate() {
            writeln!(f, "--- Block {} ---\n{}", i, block)?;
        }
        Ok(())
    }
}
