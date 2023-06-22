use crate::{guard::Guard, mutex::Mutex};

use super::{
    condvar::CondVar,
    semaphore::{Semaphore as Sem, Ticket},
};
use std::{collections::VecDeque, usize};

//Limited sync queue
pub struct LSQueue<T> {
    producing: Sem<T>,
    consuming: Sem<T>,
    guard: Sem<T>,
    queue: VecDeque<T>,
}
impl<T> LSQueue<T> {
    pub fn new(cap: usize) -> Self {
        LSQueue {
            producing: Sem::new(cap),
            consuming: Sem::new(0),
            guard: Sem::new(1),
            queue: VecDeque::new(),
        }
    }
    pub fn put(&mut self, obj: T) {
        let ticket = Ticket::<T>::new(self.producing.acquire());

        let g = Ticket::<T>::new(self.guard.acquire());
        self.queue.push_front(obj);
        self.guard.release(g);

        self.consuming.release(ticket)
    }
    pub fn take(&mut self) -> T {
        let ticket = Ticket::<T>::new(self.consuming.acquire());

        let g = Ticket::<T>::new(self.guard.acquire());
        let obj = self.queue.pop_back().unwrap();
        self.queue.pop_back();
        self.guard.release(g);

        self.producing.release(ticket);
        obj
    }
}

//Unlimited Sync Queue;
pub struct USQueue<T> {
    buffer: std::collections::VecDeque<T>,
    mutex: Mutex,
    cond: CondVar,
    is_open: bool,
}

impl<T> Default for USQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> USQueue<T> {
    pub fn new() -> Self {
        USQueue {
            buffer: VecDeque::new(),
            mutex: Mutex::new(),
            cond: CondVar::new(),
            is_open: true,
        }
    }

    pub fn put(&mut self, val: T) -> bool {
        {
            let _guard = Guard::new(&self.mutex);
            if !self.is_open {
                return false;
            }
            self.buffer.push_back(val);
        }
        self.cond.notify_one();
        true
    }

    //Need better guard with manual lock toggle
    pub fn take(&mut self) -> Option<T> {
        self.mutex.lock();
        while self.buffer.is_empty() {
            if !self.is_open {
                return None;
            }
            self.cond.wait(&self.mutex);
        }
        let res = self.buffer.pop_front();
        self.mutex.unlock();
        res
    }

    pub fn close(&mut self) {
        {
            let _guard = Guard::new(&self.mutex);
            self.is_open = false;
        }
        self.cond.notify_all();
    }

    pub fn stop(&mut self) {
        {
            let _guard = Guard::new(&self.mutex);
            self.is_open = false;
            self.buffer.clear();
        }
        self.cond.notify_all();
    }
}
