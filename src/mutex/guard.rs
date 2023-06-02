/// Basic RAII implementation Guard for objects that can lock some data
use super::core::Mutex;
pub struct Guard<'a> {
    mutex: &'a Mutex,
}

impl<'a> Guard<'a> {
    pub fn new(mutex: &'a Mutex) -> Self {
        mutex.lock();
        Guard { mutex: mutex }
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
