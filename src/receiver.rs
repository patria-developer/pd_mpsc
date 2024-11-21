use std::{collections::VecDeque, sync::Arc};

use crate::shared::Shared;

pub struct Receiver<T> {
    pub shared: Arc<Shared<T>>,
    pub buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(value) = self.buffer.pop_front() {
            return Some(value);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(value) => {
                    std::mem::swap(&mut self.buffer, &mut inner.queue);
                    return Some(value);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}
