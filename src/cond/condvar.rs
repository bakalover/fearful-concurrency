use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
pub struct CondVar {
    flag: AtomicU32,
}

impl CondVar {
    pub fn wait(&self, mutex: &Mutex) {
        let cur = self.flag.load(Ordering::Relaxed);
        mutex.unlock();
        while self.flag.load(Ordering::Relaxed) == cur {
            Futex::sleep(&self.flag);
        }
        mutex.lock();
    }

    pub fn notify_one(&self) {
        self.flag.fetch_add(1, Ordering::Relaxed);
        Futex::wake_one(&self.flag);
    }

    pub fn notify_all(&self) {
        self.flag.fetch_add(1, Ordering::Relaxed);
        Futex::wake_all(&self.flag);
    }
}
