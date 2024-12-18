#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_data(text: String) -> StringData {
        StringData(text)
    }

    #[test]
    fn test_blockchain_validity() {
        let mut blockchain = Blockchain::new(create_test_data("Genesis Block".to_string()));
        
        // Füge 100 Blöcke hinzu
        for i in 1..=100 {
            blockchain.add_block(create_test_data(format!("Block {}", i)));
        }

       
        for i in 1..blockchain.chain.len() {
            let previous_block = &blockchain.chain[i - 1];
            let current_block = &blockchain.chain[i];
            
            assert_eq!(current_block.previous_hash, previous_block.hash);
            
            assert_eq!(current_block.index, previous_block.index + 1);
            
            assert!(current_block.is_valid());
        }
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_generate_random_chars() {
        for _ in 0..100 {
            let block = Block::new(
                0,
                "test_hash".to_string(),
                create_test_data("test data".to_string())
            );
            
            
            let chars = &block.metadata.chars;
            assert_eq!(chars.len(), 4);
            assert!(chars.chars().all(|c| c.is_ascii_alphabetic()));
        }
    }

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            1,
            "previous_hash".to_string(),
            create_test_data("test data".to_string())
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "previous_hash");
        assert!(block.is_valid());
    }

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new(create_test_data("Genesis".to_string()));
        
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_data_integrity() {
        let mut blockchain = Blockchain::new(create_test_data("Genesis".to_string()));
        blockchain.add_block(create_test_data("Test Data".to_string()));

        let block = &blockchain.chain[1];
        assert_eq!(block.data.to_string(), "Test Data");
    }

    #[test]
    fn test_blockchain_manipulation_detection() {
        let mut blockchain = Blockchain::new(create_test_data("Genesis".to_string()));
        blockchain.add_block(create_test_data("Test Block".to_string()));

        // Simuliere eine Manipulation des ersten Blocks
        let manipulated_blockchain = {
            let mut chain = blockchain;
            let genesis_block = &mut chain.chain[0];
            let original_data = genesis_block.data.to_string();
            chain.chain[0] = Block::new(
                0,
                "0".to_string(),
                create_test_data("Manipulated Data".to_string())
            );
            assert_ne!(original_data, "Manipulated Data");
            chain
        };

        // Die Blockchain sollte nun als ungültig erkannt werden
        assert!(!manipulated_blockchain.is_valid());
    }
}