use std::fmt;

pub struct Block {
    timestamp: String,
    last_hash: String,
    hash: String,
    data: String,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block({}, {}, {}, {})", self.timestamp, self.last_hash, self.hash, self.data)
    }
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            timestamp: String::from("1"),
            last_hash: String::from("-----"),
            hash: String::from("hash-one"),
            data: String::from(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_block() -> Block {
        let timestamp = String::from("a-date");
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
        assert_eq!("a-date", block.timestamp)
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
    fn has_genesis_block() {
        let genesis = Block::genesis();

        assert_eq!("1", genesis.timestamp);
        assert_eq!("-----", genesis.last_hash);
        assert_eq!("hash-one", genesis.hash);
        assert_eq!("", genesis.data);
    }
}
