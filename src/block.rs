use std::fmt;
use chrono::Utc;
use crate::hash::generate_hash;

pub struct Block {
    pub timestamp: i64,
    pub last_hash: String,
    pub hash: String,
    pub data: String,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block({}, {}, {}, {})", self.timestamp, self.last_hash, self.hash, self.data)
    }
}

impl Block {
    pub fn _genesis() -> Self {
        Block {
            timestamp: 1,
            last_hash: String::from("-----"),
            hash: String::from("hash-one"),
            data: String::from(""),
        }
    }

    pub fn _mine_block(data: String, last_block: &Block) -> Self {
        let timestamp = Utc::now().timestamp();
        let last_hash = last_block.hash.clone();

        let hash = generate_hash(timestamp, &data, &last_hash);

        Block {
            timestamp,
            data,
            hash,
            last_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_block() -> Block {
        let timestamp = 2;
        let last_hash = String::from("foo-hash");
        let hash = String::from("bar-hash");
        let data = String::from("blockchain");
        
        let temp_block = Block {
            timestamp,
            last_hash,
            hash,
            data,
        };

        temp_block
    }

    #[test]
    fn has_timestamp() {
        let block = create_block();
        assert_eq!(2, block.timestamp)
    }

    #[test]
    fn has_last_hash() {
        let block = create_block();
        assert_eq!("foo-hash", block.last_hash)
    }

    #[test]
    fn has_hash() {
        let block = create_block();
        assert_eq!("bar-hash", block.hash)
    }

    #[test]
    fn has_data() {
        let block = create_block();
        assert_eq!("blockchain", block.data)
    }

    #[test]
    fn genesis_block() {
        let genesis = Block::_genesis();

        assert_eq!(1, genesis.timestamp);
        assert_eq!("-----", genesis.last_hash);
        assert_eq!("hash-one", genesis.hash);
        assert_eq!("", genesis.data);
    }

    #[test]
    fn mine_block() {
        let genesis = Block::_genesis();
        let data = String::from("minedBlock");

        let mined_block = Block::_mine_block(data.clone(), &genesis);

        assert_eq!(mined_block.last_hash, genesis.hash);
        assert_eq!(mined_block.data, data);
        assert!(!mined_block.hash.trim().is_empty());
        assert!(mined_block.timestamp > 0);
    }
}
