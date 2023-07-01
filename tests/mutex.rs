#![feature(sync_unsafe_cell)]
use std::{cell::SyncUnsafeCell, thread};

use fearconc::{
    self,
    guard::Guard,
    mutex::{Mutex, Rp},
};

///Number of critical sections.
const CRIT: usize = 10_;

///Single mutex.
#[test]
fn m_single() {
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
fn m_multi() {
    let counter = SyncUnsafeCell::new(0);
    let mutex = Mutex::new();
    for _ in 0..CRIT {
        thread::scope(|scope| {
            scope.spawn(|| {
                mutex.lock();
                unsafe {
                    *counter.get() += 1;
                }
                mutex.unlock();
            });
            scope.spawn(|| {
                mutex.lock();
                unsafe {
                    *counter.get() += 1;
                }
                mutex.unlock();
            });
            scope.spawn(|| {
                mutex.lock();
                unsafe {
                    *counter.get() -= 1;
                }
                mutex.unlock();
            });
        });
    }
    assert_eq!(unsafe { *counter.get() }, CRIT as i32);
}

///Multiple mutexes with special lifetimes.
#[test]
fn m_multi_scope() {
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
fn m_guard() {
    let counter = SyncUnsafeCell::new(0);
    let mutex = Mutex::new();
    for _ in 0..CRIT {
        thread::scope(|scope| {
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                unsafe { *counter.get() += 1 };
            });
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                unsafe { *counter.get() += 1 };
            });
            scope.spawn(|| {
                let _guard = Guard::new(&mutex);
                unsafe { *counter.get() -= 1 };
            });
        });
    }
    assert_eq!(unsafe { *counter.get() }, CRIT as i32);
}

/// Providing references + data race. Using two Cells to check state on each step and "summary" state represented in "counter".
#[test]
fn m_reference() {
    let (mut state, mut counter) = (SyncUnsafeCell::new(0), SyncUnsafeCell::new(0));
    let state_provider = Rp::create_from(&mut state);
    let counter_provider = Rp::create_from(&mut counter);
    for _ in 0..CRIT {
        thread::scope(|scope| {
            scope.spawn(|| {
                let state_ref = Rp::acquire(&state_provider);
                let counter_ref = Rp::acquire(&counter_provider);
                unsafe { *state_ref.get() = 1 };
                assert_eq!(unsafe { *state_ref.get() }, 1);
                unsafe {
                    *counter_ref.get() += 1;
                }
            });
            scope.spawn(|| {
                let state_ref = Rp::acquire(&state_provider);
                let counter_ref = Rp::acquire(&counter_provider);
                unsafe { *state_ref.get() = 0 };
                assert_eq!(unsafe { *state_ref.get() }, 0);
                unsafe {
                    *counter_ref.get() += 1;
                }
            });
        });
    }
    assert_eq!(
        unsafe { *counter_provider.acquire().get() },
        (CRIT * 2) as i32
    );
}
