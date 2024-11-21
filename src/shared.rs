use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Inner<T> {
    pub queue: VecDeque<T>,
    pub senders: usize,
}

pub struct Shared<T> {
    pub inner: Mutex<Inner<T>>,
    pub available: Condvar,
}
