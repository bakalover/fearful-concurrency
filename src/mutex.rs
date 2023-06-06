pub mod bind;
pub mod core;
pub mod guard;

#[cfg(test)]
mod tests {

    use super::{core::Mutex, bind::RefProvider};
    use std::{thread, rc::Rc};
    #[test]
    pub fn sequential_locks() {
        let mutex = Mutex::new();
        let RefProvider = RefProvider::create_from(mutex);
        let mut arr: Vec<i32> = Vec::new();
        let mut handlers = vec![];

        let t1 = thread::spawn(|| {
            m.lock();
            {
                arr.push(0);
            }
            mutex.unlock();
        });
            handlers.push(handler);
    }
}
