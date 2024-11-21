mod receiver;
mod sender;
mod shared;

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use receiver::Receiver;
use sender::Sender;
use shared::Inner;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::new(VecDeque::default()),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
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
        assert_eq!(rx.recv(), 26);
    }

    #[test]
    fn send_multi_thread() {
        let (mut tx, mut rx) = channel();
        thread::spawn(move || {
            tx.send(26);
        });
        assert_eq!(rx.recv(), 26);
    }
}
