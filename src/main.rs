use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    signature: String,
    amount: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BlockData {
    transactions: Option<Vec<Transaction>>,
}

pub struct Block {
    pub id: u32,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub data: BlockData,
}

pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn generate_genesis_block(&mut self) {
        let genesis_block = Block {
            id: 0,
            hash: "0".to_string(),
            prev_hash: "0".to_string(),
            timestamp: 0,
            nonce: 0,
            data: BlockData { transactions: None },
        };

        self.blocks.push(genesis_block);
    }

    fn is_block_valid(&self, block: &Block) -> bool {
        let latest_block = self.blocks.last().expect("No blocks found");
        if block.id != latest_block.id + 1 {
            return false;
        }
        if block.prev_hash != latest_block.hash {
            return false;
        }
        true
    }

    fn add_block(&mut self, block: Block) {}

    fn calculate_hash(block: Block) {
        let block_data_in_json = serde_json::json!({
        "id": block.id,
            "prev_hash": block.prev_hash,
        "timestamp": block.timestamp,
        "nonce": block.nonce,
        "data": block.data
        });

        let mut hasher = Sha256::new();
        hasher.update(block_data_in_json.to_string().as_bytes());
    }

    fn stake() {}
}

fn main() {
    let mut app = App::new();

    if (app.blocks.len() < 1) {
        app.generate_genesis_block()
    }
}
