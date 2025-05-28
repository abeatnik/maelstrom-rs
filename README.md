# maelstrom-broadcast

A Rust implementation of the [Maelstrom](https://github.com/jepsen-io/maelstrom) broadcast challenge.

This project explores core distributed systems concepts such as gossip-based message dissemination, deduplication, and consistency across nodes. Each node participates in broadcasting and receiving messages, maintaining a local set of unique messages.

Built as part of the Codecrafters/Maelstrom distributed systems track.

## Features

- Gossip-style message propagation
- Duplicate message handling
- Read requests return known messages
- Topology awareness (static)

## Running

Requires [Maelstrom](https://github.com/jepsen-io/maelstrom) and a recent Rust toolchain.

```bash
cargo build --release
maelstrom test -w broadcast --bin ./target/release/maelstrom-broadcast \
  --node-count 5 --time-limit 20 --rate 10 --latency 100
