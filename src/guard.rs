/// Basic RAII implementation of mutex guard.
use super::mutex::Mutex;
pub struct Guard<'a> {
    mutex: &'a Mutex,
}

impl<'a> Guard<'a> {
    pub fn new(mutex: &'a Mutex) -> Self {
        mutex.lock();
        Guard { mutex }
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
