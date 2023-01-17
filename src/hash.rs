use sha256::digest;

pub fn generate_hash(timestamp: i64, data: &String, last_hash: &String) -> String {
    return digest(format!("{}{}{}", timestamp, data, last_hash));
}

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use super::*;

    #[test]
    fn generates_a_hash_string() {
        let genesis = Block::_genesis();
        let output = generate_hash(genesis.timestamp, &genesis.data, &genesis.last_hash);

        assert_eq!(output, "56a946b0aeca28d96bede89a1163cdd532a1904d304c2d66c8be6b799db23d6c");
    }
}
