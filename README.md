# 🚌 MPSC Channel

Multi-producer, single-consumer FIFO queue communication.
This module provides message-based communication over channels.

A `Sender` is used to send data to a `Receiver`. Both senders are clone-able (multi-producer) such that many threads can send simultaneously to one receiver (single-consumer).

# Examples

Simple usage:

```rust
use std::thread;
use pd_mpsc::channel;

// Create a simple streaming channel.
let (mut tx, mut rx) = channel();
thread::spawn(move || {
    tx.send(100);
});
assert_eq!(rx.recv(), 100);
```

Shared usage:

```rust
use std::thread;
use pd_mpsc::channel;

// Create a shared channel that can be sent along from many threads
// where tx is the sending half (tx for transmission), and rx is the receiving
// half (rx for receiving).
let (tx, mut rx) = channel();
for i in 0..10 {
    let mut tx = tx.clone();
    thread::spawn(move || {
        tx.send(i);
    });
}

for _ in 0..10 {
    let j = rx.recv().unwrap();
    assert!(0 <= j && j < 10);
}
```
