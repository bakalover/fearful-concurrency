use std::usize;

use crate::mutex::core::Mutex;
use crate::mutex::guard::Guard;

pub struct Ticket<T>{}

impl<T> Ticket<T> {
    pub(super) fn new(old: Ticket<T>) -> Self{Ticket {  }}
}
pub struct Semaphore {
    count: usize,
    mutex: Mutex,
    cv: Condvar,
}


impl Semaphore {

    pub fn new(count: usize) -> Self{
        Semaphore { count, mutex: Mutex::new(), cv: CondVar::new() }
    }
    
    pub fn acquire(&self) -> Ticket{
        let guard = Guard::new(self.mutex);
        while(self.count == 0){
            cv.wait(guard);
        }
        self.count -= 1;
        Ticket {  }
    }

    pub fn release(&self, ticket:Ticket){
        {
            let _guard = Guard::new(self.mutex);
            self.count += 1;
        }
        cv.notify_one();
    }

}
