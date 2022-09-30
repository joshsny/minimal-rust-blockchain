use std::error;

use chrono::Utc;
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp() as u64,
            data: "genesis!".to_string(),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
        };

        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self
            .blocks
            .last()
            .expect("There are no blocks in the chain.");

        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("Could not add a block - invalid")
        }
    }

    fn is_block_valid(&self, block: &Block, previous_bloc: &Block) -> bool {
        let DIFFICULTY_PREFIX = "00";

        if block.previous_hash != previous_block.hash {
            warn!("block with id: {}, has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {}, has invalid difficulty", block.id);
            return false;
        } else if (block.id != previous_block.id + 1) {
            warn!("block with id: {}, has invalid id", block.id);
            return false;
        } else if hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }
}
