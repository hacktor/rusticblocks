extern crate rusticblocks;

use rusticblocks::Block;

fn main() {
    // create the genesis block
    let mut block = Block::init();
    let mut index: u64 = 1;
    // now create the next 999 blocks
    loop {
        println!("{:?}", block);
        if index == 1000 { break; }
        block = Block::next(&block,&("This is block number ".to_string() + &index.to_string()));
        index = index + 1;
    }
}
