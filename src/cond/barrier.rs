use super::condvar::CondVar;
use crate::guard::Guard;
use crate::mutex::Mutex;

pub struct Barrier {
    mutex: Mutex,
    counters: (usize, usize),
    all_bros: bool,
    cv: CondVar,
}

#[allow(dead_code)]
#[allow(clippy::while_immutable_condition)]
impl Barrier {
    pub fn new(participants: usize) -> Self {
        Barrier {
            mutex: Mutex::new(),
            counters: (participants, participants),
            all_bros: false,
            cv: CondVar::new(),
        }
    }
    pub fn arrive_wait(&mut self) {
        let _guard = Guard::new(&self.mutex);
        let bro = self.all_bros;
        self.counters.1 -= 1;

        if self.counters.1 == 0 {
            self.all_bros = !self.all_bros;
            self.counters.1 = self.counters.0;
            self.cv.notify_all();
        } else {
            while bro == self.all_bros {
                self.cv.wait(&self.mutex);
            }
        }
    }
}
