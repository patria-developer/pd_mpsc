mod receiver;
mod sender;
mod shared;

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use shared::{Inner, Shared};

pub use receiver::Receiver;
pub use sender::Sender;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[cfg(test)]
mod channel_tests {
    use std::thread;

    use super::*;

    #[test]
    fn send_single_thread() {
        let (mut tx, mut rx) = channel();
        tx.send(26);
        assert_eq!(rx.recv(), Some(26));
    }

    #[test]
    fn send_different_thread() {
        let (mut tx, mut rx) = channel();
        thread::spawn(move || {
            tx.send(26);
        });
        assert_eq!(rx.recv(), Some(26));
    }

    #[test]
    fn send_multi_thread() {
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
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(26);
    }
}
