/// Basic RAII implementation of mutex guard.
use super::mutex::Mutex;
pub struct Guard<'a> {
    mutex: &'a Mutex,
    is_locked: bool,
}

impl<'a> Guard<'a> {
    pub fn new(mutex: &'a Mutex) -> Self {
        mutex.lock();
        Guard {
            mutex,
            is_locked: true,
        }
    }

    pub fn lock(&mut self) {
        assert!(!self.is_locked);
        self.mutex.lock();
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        assert!(self.is_locked);
        self.is_locked = false;
        self.mutex.unlock()
    }
}

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        assert!(self.is_locked);
        self.mutex.unlock();
    }
}
