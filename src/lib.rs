extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Hash, Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: u64,
    pub payload: String,
}
impl Clone for Transaction {
    fn clone(&self) -> Transaction {
        Transaction {
            timestamp: self.timestamp,
            payload: self.payload.to_string(),
        }
    }
}

#[derive(Hash, Serialize, Deserialize)]
pub struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    prevhash: u64,
    myhash: u64,
}
impl Block {
    fn somehash(text: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        text.to_string()
            .hash(&mut hasher);
        hasher.finish()
    }
    pub fn init() -> Block {
        let t = Transaction {
            timestamp: 0,
            payload: "This is the Genesis Block".to_owned(),
        };
        Block {
            index: 0,
            timestamp: 0,
            prevhash: 0,
            myhash: 0,
            transactions: vec![t],
        }
    }
    pub fn next(prev: &Block, t: Vec<Transaction>) -> Block {
        let index = prev.index + 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let json = serde_json::to_string(&t).expect("Transactions are not jsonifyable");
        Block {
            index: index,
            timestamp: timestamp,
            transactions: t,
            prevhash: prev.myhash,
            myhash: Block::somehash(&(index.to_string() + &timestamp.to_string() + &json + &prev.myhash.to_string())),
        }
    }
}

pub struct Blockchain {
    chain: Vec<Block>,
}
impl Blockchain {
    pub fn init() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
        }
    }
    pub fn getblock(&mut self, index: u64) -> Block {
        // normalize index, then we need a deep copy
        let index = if index > self.chain.len() as u64 { self.chain.len() as u64 } else { index }; 
        // must return Block; for now, always the last one
        let last = self.chain.last().expect("Last Should not be none");
        Block { 
            index: last.index,
            timestamp: last.timestamp,
            prevhash: last.prevhash,
            myhash: last.myhash,
            transactions: last.transactions.clone(),
        }
    }
    pub fn addblock(&mut self, block: Block) {
        self.chain.push(block);
    }
}
