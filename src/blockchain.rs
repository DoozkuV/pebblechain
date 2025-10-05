use std::fmt;

use bytes::Bytes;

use crate::block::Block;

const DEFAULT_DIFFICULTY: u8 = 6;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    difficulty: u8,
}

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
