use std::marker::PhantomData;
use std::usize;

use super::condvar::CondVar;
use crate::guard::Guard;
use crate::mutex::core::Mutex;

pub struct Ticket<T> {
    phantom: PhantomData<T>,
}

impl<T> Ticket<T> {
    pub(super) fn new(_old: Ticket<T>) -> Self {
        Ticket {
            phantom: PhantomData,
        }
    }
}
pub struct Semaphore<T> {
    count: usize,
    mutex: Mutex,
    cv: CondVar,
    phantom: PhantomData<T>,
}

#[allow(clippy::while_immutable_condition)]
impl<T> Semaphore<T> {
    pub fn new(count: usize) -> Self {
        Semaphore {
            count,
            mutex: Mutex::new(),
            cv: CondVar::new(),
            phantom: PhantomData,
        }
    }
    #[deny(clippy::never_loop)]
    pub fn acquire(&mut self) -> Ticket<T> {
        let _guard = Guard::new(&self.mutex);
        while self.count == 0 {
            self.cv.wait(&self.mutex);
        }
        self.count -= 1;
        Ticket {
            phantom: PhantomData,
        }
    }

    pub fn release(&mut self, _ticket: Ticket<T>) {
        {
            let _guard = Guard::new(&self.mutex);
            self.count += 1;
        }
        self.cv.notify_one();
    }
}
