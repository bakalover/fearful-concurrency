use crate::futex::Futex;
use crate::mutex::Mutex;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
#[derive(Default)]
pub struct CondVar {
    flag: AtomicU32,
}

impl CondVar {
    pub fn new() -> Self {
        CondVar {
            flag: AtomicU32::new(0),
        }
    }

    pub fn wait(&self, mutex: &Mutex) {
        let cur = self.flag.load(Ordering::SeqCst);
        mutex.unlock();
        while self.flag.load(Ordering::SeqCst) == cur {
            Futex::sleep(&self.flag);
        }
        mutex.lock();
    }

    pub fn notify_one(&self) {
        self.flag.fetch_add(1, Ordering::SeqCst);
        Futex::wake_one(&self.flag);
    }

    pub fn notify_all(&self) {
        self.flag.fetch_add(1, Ordering::SeqCst);
        Futex::wake_all(&self.flag);
    }
}
