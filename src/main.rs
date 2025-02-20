mod tests;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};

pub trait BlockData: Clone + std::fmt::Debug {
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    pub timestamp: u128,
    pub chars: String,
    pub version: String,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Block<T: BlockData> {
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: T,
    pub metadata: BlockMetadata,
}

impl<T: BlockData> Block<T> {
    fn generate_random_chars() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .filter(|random_char| random_char.is_ascii_alphabetic())
            .take(4)
            .map(|random_char| random_char as char)
            .collect()
    }

    pub fn new(index: u64, previous_hash: String, data: T) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
           
        let chars = Self::generate_random_chars();
       
        let metadata = BlockMetadata {
            timestamp,
            chars,
            version: "1.0".to_string(),
            additional_info: None,
        };

        let mut block = Block {
            index,
            previous_hash,
            hash: String::new(),
            data,
            metadata,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        format!("{:x}", md5::compute(format!(
            "{}{}{}{}{}",
            self.index,
            self.metadata.timestamp,
            self.previous_hash,
            self.data.to_string(),
            self.metadata.chars
        )))
    }

    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash()
    }
}

#[derive(Debug)]
pub struct Blockchain<T: BlockData> {
    pub chain: Vec<Block<T>>,  // Geändert zu pub für Tests
}

impl<T: BlockData> Blockchain<T> {
    pub fn new(genesis_data: T) -> Self {
        let genesis_block = Block::new(0, String::from("0"), genesis_data);
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn get_last_block(&self) -> &Block<T> {
        self.chain.last().expect("Blockchain has no blocks")
    }

    pub fn add_block(&mut self, data: T) {
        let last_block = self.get_last_block();
        let new_block = Block::new(
            last_block.index + 1,
            last_block.hash.clone(),
            data,
        );
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !current.is_valid() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringData(pub String);  // Geändert zu pub für Tests

impl BlockData for StringData {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

fn main() {
    let mut blockchain = Blockchain::new(StringData("Genesis Block".to_string()));
   
    blockchain.add_block(StringData("First Block".to_string()));
    blockchain.add_block(StringData("Second Block".to_string()));
    blockchain.add_block(StringData("Third Block".to_string()));
   
    println!("Blockchain: {:#?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_valid());
}