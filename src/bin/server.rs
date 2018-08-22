extern crate rusticblocks;
extern crate serde_json;
extern crate rocket;

use rusticblocks::{Blockchain, Block, Transaction};
use std::time::{UNIX_EPOCH, SystemTime};

fn main() {

    let mut blockchain: Blockchain = get_blockchain();

    // now create the next 10 blocks
    for x in 0..10 {
        let transactions = get_transactions();
        let lastblock = blockchain.getblock(x);
        let json = serde_json::to_string_pretty(&lastblock).expect("Block is not jsonifyable");
        let block = Block::next(&lastblock,transactions);
        println!("{}", json);
        blockchain.addblock(block);
    }
}

fn get_blockchain() -> Blockchain {
    // for later: return the blockchain from database
    // now: create the blockchain and the genesis block
    let mut blockchain: Blockchain = Blockchain::init();
    blockchain.addblock(Block::init()); 
    blockchain
}

fn get_transactions() -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();

    // we put 3 transactions into each block
    for _ in 0..3 {
        let random_string: String = "dummy".to_string();
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
