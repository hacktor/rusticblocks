use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Debug)]
#[derive(Hash)]
pub struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    prevhash: u64,
    myhash: u64,
}
impl Block {
    fn somehash(b: &Block) -> u64 {
        let mut hasher = DefaultHasher::new();
        (b.index.to_string() + &b.timestamp.to_string() + &b.data.to_string() + &b.prevhash.to_string() + &b.myhash.to_string())
            .hash(&mut hasher);
        hasher.finish()
    }
    pub fn init() -> Block {
        Block {
            index: 0,
            timestamp: 0,
            prevhash: 0,
            myhash: 0,
            data: "This is the Rustic Genesis Block".to_string(),
        }
    }
    pub fn next(prev: &Block, data: &str) -> Block {
        Block {
            index: prev.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs(),
            data: data.to_string(),
            prevhash: prev.myhash,
            myhash: Block::somehash(prev),
        }
    }
}

