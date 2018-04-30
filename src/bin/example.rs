extern crate blockchain;
extern crate serde_json;
extern crate rand;

use blockchain::{Block, Transaction};
use std::time::{UNIX_EPOCH, SystemTime};
use rand::Rng;

fn main() {

    // create the genesis block
    let mut block = Block::init();
    let json = serde_json::to_string_pretty(&block).expect("Block is not jsonifyable");
    println!("{}", json);

    // now create the next 10 blocks
    for _ in 0..10 {
        let transactions = get_transactions();
        block = Block::next(&block,transactions);
        let json = serde_json::to_string_pretty(&block).expect("Block is not jsonifyable");
        println!("{}", json);
    }
}

fn get_transactions() -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();

    // we put 3 transactions into each block
    for _ in 0..3 {
        let random_string: String = rand::thread_rng().gen_ascii_chars().take(64).collect();
	    let t = Transaction {
	        timestamp: SystemTime::now()
	            .duration_since(UNIX_EPOCH)
	            .expect("Time went backwards")
	            .as_secs(),
	        payload: "Random string: ".to_string() + &random_string,
	    };
        transactions.push(t);
    }
    transactions
}
