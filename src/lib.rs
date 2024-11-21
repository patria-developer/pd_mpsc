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
