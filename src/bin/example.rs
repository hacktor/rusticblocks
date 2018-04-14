extern crate rusticblocks;

use rusticblocks::Block;

fn main() {
    // create the genesis block
    let mut block = Block::init();
    // now create the next 999 blocks
    for x in 1..999 {
        println!("{:?}", block);
        block = Block::next(&block,&("This is block number ".to_string() + &x.to_string()));
    }
}
