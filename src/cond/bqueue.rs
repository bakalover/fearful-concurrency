use super::semaphore::{Semaphore as Sem, Ticket};
use std::{collections::VecDeque, usize};
pub struct BQueue<T> {
    producing: Sem,
    consuming: Sem,
    guard: Sem,
    queue: VecDeque<T>,
}

impl<T> BQueue<T> {
    pub fn new(cap: usize) -> Self {
        BQueue {
            producing: Sem::new(cap),
            consuming: Sem::new(0),
            guard: Sem::new(1),
            queue: VecDeque::new(),
        }
    }
    pub fn put(&self, obj: T) {
        let ticket = Ticket::<T>::new(self.producing.acquire());
        {
            let g = Ticket::<T>::new(self.guard.acquire());
            self.queue.push_front(obj);
            self.guard.release(g);
        }
        self.consuming.release(ticket)
    }
    pub fn take(&self) -> T {
        let ticket = Ticket::<T>::new(self.consuming.acquire());
        let g = Ticket::<T>::new(self.guard.acquire());
        let obj = self.queue.back().unwrap();
        self.guard.release(g);
        self.producing.release(ticket);
        obj
    }
}
