use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    fn send(&mut self, value: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(value);
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.pop_front()
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::new(VecDeque::default()),
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
