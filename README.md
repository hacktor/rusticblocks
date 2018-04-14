# rusticblocks

An example of a tiny blockchain written in Rust. I've only just started learning the Rust programming language and this struck me as a nice little project to get me started. It is based on some python code that can be found here: [Snakecoin](https://github.com/schedutron/SnakeCoin)

I'm planning to add demo server and client code soon.

## Prerequisites

You need a computer with the rustc compiler and cargo installed.

## How to run example code

* git clone https://github.com/hacktor/rusticblocks.git
* cd rusticblocks
* cargo run --bin example

     Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/example`
 Block { index: 0, timestamp: 0, data: "This is the Rustic Genesis Block", prevhash: 0, myhash: 0 }
 Block { index: 1, timestamp: 1523709837, data: "This is block number 1", prevhash: 0, myhash: 9392592759503862021 }
 Block { index: 2, timestamp: 1523709837, data: "This is block number 2", prevhash: 9392592759503862021, myhash: 5683024740449102550 }
 Block { index: 3, timestamp: 1523709837, data: "This is block number 3", prevhash: 5683024740449102550, myhash: 11846245409687764979 }
 Block { index: 4, timestamp: 1523709837, data: "This is block number 4", prevhash: 11846245409687764979, myhash: 7340726335388511881 }

## Authors

* **Ruben de Groot**

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

