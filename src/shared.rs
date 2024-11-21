use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

pub struct Inner<T> {
    pub queue: Mutex<VecDeque<T>>,
    pub available: Condvar,
}
