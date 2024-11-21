use std::sync::Arc;

use crate::shared::Shared;

pub struct Sender<T> {
    pub shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(value);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}
