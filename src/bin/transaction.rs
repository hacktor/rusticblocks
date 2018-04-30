extern crate blockchain;
extern crate serde_json;

use std::time::{UNIX_EPOCH, SystemTime};
use blockchain::Transaction;

fn main() {
    let transaction = Transaction { 
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs(),
        payload: "First transaction".to_string(),
    };

    let j = serde_json::to_string(&transaction).expect("Transaction is not jsonifyable");

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}
