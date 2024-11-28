#[cfg(test)]
mod tests {
    use crate::{Blockchain, Block}; // Importiere Blockchain und Block aus dem übergeordneten Modul

    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new();
        
        for i in 1..=100 {
            blockchain.add_block(format!("Block {}", i));
        }

        for i in 1..blockchain.chain.len() {
            let previous_block = &blockchain.chain[i - 1];
            let current_block = &blockchain.chain[i];
            assert_eq!(current_block.previous_hash, previous_block.hash);
        }
    }

    #[test]
    fn test_generate_random_chars() {
        for _ in 0..100 {
            let chars = Block::generate_random_chars();
            assert_eq!(chars.len(), 4);
            assert!(chars.chars().all(|c| c.is_ascii_alphabetic()));
        }
    }
}