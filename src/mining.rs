use super::constants::DIFFICULTY_PREFIX;
use log::{error, info, warn};
pub mod blocks {
    use crate::constants;

    pub fn mine_block(
        id: u64,
        timestamp: i64,
        previous_hash: &String,
        data: &String,
    ) -> (u64, String) {
        info!("Mining block");

        let mut nonce = 0;

        loop {
            if nonce % 10e4 == 0 {
                info!("nonce: {}", nonce);
            }

            let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
            let binary_hash = hash_to_binary_representation(&hash);

            if binary_hash.starts_with(constantsts.DIFFICUL) {
                info!(
                    "mined! nonce: {}, hash: {}, binary_hash: {}",
                    nonce, hash, binary_hash
                );

                return (nonce, hex::encode(hash));
            }

            nonce += 1;
        }
    }

    pub fn calculate_hash(
        id: u64,
        timestamp: i64,
        previous_hash: &str,
        data: &str,
        nonce: u64,
    ) -> Vec<u8> {
        let hash_data = serde_json::json!({
            "id": id,
            "previous_hash": previous_hash,
            "data": data,
            "timestamp": timestamp,
            "nonce": nonce
        });
        let mut hasher = Sha256::new();
        hasher.update(hash_data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }
}
