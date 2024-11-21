use std::sync::Arc;

use crate::Inner;

pub struct Sender<T> {
    pub inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(value);
    }
}
