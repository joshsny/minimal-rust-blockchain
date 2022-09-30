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
}
// impl App {
//     fn new() -> Self {
//         Self { blocks: vec![] }
//     }

//     fn genesis(&mut self) {
//         let genesis_block = Block {
//             id: 0,
//             timestamp: Utc::now().timestamp(),
//             previous_hash: String::from("genesis"),
//             data: String::from("genesis!"),
//             nonce: 2836,
//             hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
//         };
//         self.blocks.push(genesis_block);
//     }

// fn try_add_block(&mut self, block: Block) {
//     let latest_block = self.blocks.last().expect("there is at least one block");
//     if self.is_block_valid(&block, latest_block) {
//         self.blocks.push(block);
//     } else {
//         error!("could not add block - invalid");
//     }
// }
