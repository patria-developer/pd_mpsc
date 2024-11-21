use std::sync::Arc;

use crate::Inner;

pub struct Receiver<T> {
    pub inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(value) => return value,
                None => {
                    queue = self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
}
