use std::{cell::Cell, thread};

use fearconc::{
    self,
    mutex::{Mutex, Rp},
    guard::Guard
};
///Number of critical sections.
const CRIT: usize = 100_000;

///Special custom data type, that allows data races.
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

///Single mutex.
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

///Single mutex + data race.
#[test]
pub fn mutex_detach() {
    let counter = RaceCell::new(0);
    let mutex = Mutex::new();
    for _ in 0..CRIT {
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

///Multiple mutexes with special lifetimes.
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

/// Guard + data race (basically RAII version of "mutex_detach" test).
#[test]
pub fn guard() {
    let counter = RaceCell::new(0);
    let mutex = Mutex::new();
    for _ in 0..CRIT {
        thread::scope(|scope| {
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                counter.set(counter.get() + 1);
            });
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                counter.set(counter.get() + 1);
            });
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                counter.set(counter.get() - 1);
            });
        });
    }
    assert_eq!(counter.get(), CRIT as i32);
}

/// Providing references + data race. Using two Cells to check state on each step and "summary" state represented in "counter".
#[test]
pub fn providing() {
    let (mut state, mut counter) = (RaceCell::new(0), RaceCell::new(0));
    let state_provider = Rp::create_from(&mut state);
    let counter_provider = Rp::create_from(&mut counter);
    for _ in 0..CRIT {
        thread::scope(|scope| {
            scope.spawn(|| {
                let state_ref = Rp::acquire(&state_provider);
                let counter_ref = Rp::acquire(&counter_provider);
                state_ref.set(1);
                assert_eq!(state_ref.get(), 1);
                counter_ref.set(counter_ref.get() + 1);
            });
            scope.spawn(|| {
                let state_ref = Rp::acquire(&state_provider);
                let counter_ref = Rp::acquire(&counter_provider);
                state_ref.set(0);
                assert_eq!(state_ref.get(), 0);
                counter_ref.set(counter_ref.get() + 1);
            });
        });
    }
    assert_eq!(counter_provider.acquire().get(), (CRIT * 2) as i32);
}
