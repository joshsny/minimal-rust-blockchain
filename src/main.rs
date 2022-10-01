mod constants;
mod mining;
use chrono::prelude::*;
use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{Swarm, SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{stdin, AsyncBufReadExt, BufReader},
    select, spawn,
    sync::mpsc,
    time::sleep,
};

use mining::blocks::{calculate_hash, hash_to_binary_representation, mine_block};

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
    pub timestamp: i64,
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
            timestamp: Utc::now().timestamp(),
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
            log::error!("Could not add a block - invalid")
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {}, has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(constants::DIFFICULTY_PREFIX)
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

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }

            let first = chain.get(i - 1).expect("can get first block");

            let second = Arc::new(chain.get(i)).expect("can get current block");

            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    fn choose_chain(&self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let local_chain_valid = self.is_chain_valid(&local);
        let remote_chain_valid = self.is_chain_valid(&remote);

        match (local_chain_valid, remote_chain_valid) {
            (false, false) => panic!("Both local and remote chain are invalid."),
            (true, false) => local,
            (false, true) => remote,
            (true, true) => {
                if local.len() < remote.len() {
                    local
                } else {
                    remote
                }
            }
        }
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now();

        let (nonce, hash) = mine_block(id, now.timestamp(), &previous_hash, &data);

        Self {
            id,
            timestamp: now.timestamp(),
            nonce,
            previous_hash,
            hash,
            data,
        }
    }
}

// fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
//     info!("mining block...");
//     let mut nonce = 0;

//     loop {
//         if nonce % 100000 == 0 {
//             info!("nonce: {}", nonce);
//         }
//         let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
//         let binary_hash = hash_to_binary_representation(&hash);
//         if binary_hash.starts_with(DIFFICULTY_PREFIX) {
//             info!(
//                 "mined! nonce: {}, hash: {}, binary hash: {}",
//                 nonce,
//                 hex::encode(&hash),
//                 binary_hash
//             );
//             return (nonce, hex::encode(hash));
//         }
//         nonce += 1;
//     }
// }
