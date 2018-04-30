# rusticblocks

An example of a small blockchain written in Rust. I've only just started learning the Rust programming language and this struck me as a nice little project to get me started. It is based on some python code that can be found here: [Snakecoin](https://github.com/schedutron/SnakeCoin)

I'm planning to add demo server and client code soon.

## Prerequisites

You need a computer with the rustc compiler and cargo installed.

## How to run example code

* git clone https://github.com/hacktor/rusticblocks.git
* cd rusticblocks
* cargo run --bin example

```
   Compiling blockchain v0.1.0 (file:///home/ruben/build/rust/rusticblocks)
    Finished dev [unoptimized + debuginfo] target(s) in 30.89 secs
     Running `target/debug/example`
{
  "index": 0,
  "timestamp": 0,
  "transactions": [
    {
      "timestamp": 0,
      "payload": "This is the Genesis Block"
    }
  ],
  "prevhash": 0,
  "myhash": 0
}
{
  "index": 1,
  "timestamp": 1525099218,
  "transactions": [
    {
      "timestamp": 1525099218,
      "payload": "Random string: bQj3b1LS6tfDOZxHsUC8sDdBeOWEh5KciZvTlLgoUkIjcPR3pmhOWUzLjuneo9Yd"
    },
    {
      "timestamp": 1525099218,
      "payload": "Random string: jL93EJK1EHEDOj0HE6vYkk27XSFktg4eYMEcbLQHjcgZ9AbkhYG82jngl4wmyclS"
    },
    {
      "timestamp": 1525099218,
      "payload": "Random string: cViMVnKTSykDuGfxJPJ9ZGHjNCiN2311MJGI3TTi1h0M9aEJpY0JtSx0MKuo77CQ"
    }
  ],
  "prevhash": 0,
  "myhash": 8778940246407823298
}
{
  "index": 2,
  "timestamp": 1525099218,
  "transactions": [
    {
      "timestamp": 1525099218,
      "payload": "Random string: H1KA4eGCHhDpW6Qm47e8FKHa9CVAPMMbFX29ylhphhtzAUaFdraI5mpSPLGEvTW8"
    },
    {
      "timestamp": 1525099218,
      "payload": "Random string: RTDUyV0bL1hmA48cgeVgxy4NOqnYXHA6BM6cjZG8Fq6EH9O1ks2uReFiQeQB2TVC"
    },
    {
      "timestamp": 1525099218,
      "payload": "Random string: tbzYCUPlvmTNXW8nMg47QD1dg17r7Hl3t2xEQ9ZDoo7SMEFCcNvewt4VyWp5m03r"
    }
  ],
  "prevhash": 8778940246407823298,
  "myhash": 18357583325446610174
}
{
  "index": 3,

  ....
```

## Authors

* **Ruben de Groot**

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

