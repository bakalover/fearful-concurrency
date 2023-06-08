pub mod bind;
pub mod core;
pub mod guard;

#[cfg(test)]
mod tests {

    use super::{bind::RefProvider, core::Mutex, guard::Guard};
    use std::{cell::Cell, thread};

    const CRIT: usize = 100000;
    struct RaceCell {
        val: Cell<i32>,
    }

    unsafe impl Sync for RaceCell {}

    impl RaceCell {
        pub fn new(val: i32) -> Self {
            RaceCell {
                val: Cell::new(val),
            }
        }
        pub fn get(&self) -> i32 {
            self.val.get()
        }
        pub fn set(&self, val: i32) {
            self.val.set(val);
        }
    }
    #[test]
    pub fn mutex_join() {
        let mut counter = 0;
        let mutex = Mutex::new();
        for _ in 0..CRIT {
            mutex.lock();
            counter += 1;
            mutex.unlock();
        }
        assert_eq!(counter, CRIT);
    }

    #[test]
    pub fn mutex_detach() {
        let counter = RaceCell::new(0);
        let mutex = Mutex::new();
        for i in 0..CRIT {
            thread::scope(|scope| {
                scope.spawn(|| {
                    mutex.lock();
                    counter.set(counter.get() + 1);
                    mutex.unlock();
                });
                scope.spawn(|| {
                    mutex.lock();
                    counter.set(counter.get() + 1);
                    mutex.unlock();
                });
                scope.spawn(|| {
                    mutex.lock();
                    counter.set(counter.get() - 1);
                    mutex.unlock();
                });
            });
        }
        assert_eq!(counter.get(), CRIT as i32);
    }

    #[test]
    pub fn mutex_multi_join() {
        let mut arr: Vec<i32> = vec![];
        let mutex1 = Mutex::new();
        let mutex2 = Mutex::new();
        for _ in 0..CRIT {
            thread::scope(|scope_th1| {
                scope_th1.spawn(|| {
                    mutex1.lock();
                    arr.push(1);
                    thread::scope(|scope_th2| {
                        scope_th2.spawn(|| {
                            mutex2.lock();
                            arr.push(2);
                            mutex2.unlock();
                        });
                    });
                    arr.push(1);
                    mutex1.unlock();
                });
            });
        }
        assert_eq!(arr.len(), CRIT * 3);
        assert_eq!(arr.into_iter().filter(|x| *x == 1).count(), CRIT * 2);
    }

    #[test]
    pub fn guard() {
        let counter = RaceCell::new(0);
        let mutex = Mutex::new();
        for i in 0..CRIT {
            thread::scope(|scope| {
                scope.spawn(|| {
                    let _guard = Guard::new(&mutex);
                    counter.set(counter.get() + 1);
                    drop(_guard);
                });
                scope.spawn(|| {
                    let _guard = Guard::new(&mutex);
                    counter.set(counter.get() + 1);
                    drop(_guard);
                });
                scope.spawn(|| {
                    let _guard = Guard::new(&mutex);
                    counter.set(counter.get() - 1);
                    drop(_guard);
                });
            });
        }
        assert_eq!(counter.get(), CRIT as i32);
    }

    #[test]
    pub fn providing() {
        let mut state = RaceCell::new(0);

        let resource_provider = RefProvider::create_from(&mut state);
        for _ in 0..10 {
            thread::scope(|scope| {
                scope.spawn(|| {
                    let state_ref = RefProvider::acquire(&resource_provider);
                    state_ref.set(1);
                    assert_eq!(state_ref.get(), 1);
                });
                scope.spawn(|| {
                    let state_ref = RefProvider::acquire(&resource_provider);
                    state_ref.set(0);
                    assert_eq!(state_ref.get(), 0);
                });
            });
        }
    }
}
