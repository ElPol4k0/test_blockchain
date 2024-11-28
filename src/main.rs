pub mod tests;
use serde::{Serialize, Deserialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    previous_hash: String,
    hash: String,
    data: String,
    chars: String,
}

impl Block {
   pub fn generate_random_chars() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .filter(|random_char| random_char.is_ascii_alphabetic())
            .take(4)
            .map(|random_char| random_char  as char)
            .collect()

    }

    fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let chars = Block::generate_random_chars();
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &data, &chars);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
            chars
        }
    }
    fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str, chars: &str) -> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}", index, timestamp, previous_hash, data, chars)))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PersoenlicheDaten {
    vorname: String,
    nachname: String,
    geburtsdatum: String,
    geburtsort: String,
    familienstand: String,
    kinder: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockData {
    persoenlicheDaten: PersoenlicheDaten,
}


#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {

    fn add_block_from_file(&mut self, file_path: &str) {
        println!("Reading data from file: {}", file_path);
        let file_content = fs::read_to_string(file_path).expect("Unable to read file");
        let block_data: BlockData = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    
        let data = format!(
            /* "Name: {} {}, Geburtsdatum: {}, Geburtsort: {}, Familienstand: {}, Kinder: {}", */
            "{} {} {} {} {} {}",
            block_data.persoenlicheDaten.vorname,
            block_data.persoenlicheDaten.nachname,
            block_data.persoenlicheDaten.geburtsdatum,
            block_data.persoenlicheDaten.geburtsort,
            block_data.persoenlicheDaten.familienstand,
            block_data.persoenlicheDaten.kinder
        );
    
        self.add_block(data);
    }



    fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn get_last_block(&self) -> &Block {
        self.chain.last().expect("Blockchain has no blocks")
    }

    fn add_block(&mut self, data: String) {
        let last_block = self.get_last_block();
        let new_block = Block::new(
            last_block.index + 1,
            last_block.hash.clone(),
            data,
        );
        self.chain.push(new_block);
    }

    // Validierung der Blockchain
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.previous_hash,
                &current.data,
                &current.chars,
            ) {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block_from_file("C:/Users/PawelWiercioch/Documents/Rust/test_blockchain/src/data/data.json");
/*     blockchain.add_block(String::from("Second Block"));
    blockchain.add_block(String::from("Third Block")); */

    for _ in 1..=5 {
        let last_block_data = blockchain.get_last_block().data.clone();
        blockchain.add_block(last_block_data);
    }
    
    

    println!("Blockchain: {:#?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_valid());
}
