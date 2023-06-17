#![feature(sync_unsafe_cell)]
use std::{cell::SyncUnsafeCell, thread};

use fearconc::cond::Semaphore;

const CRIT: usize = 100_000;

#[test]
fn semaphore_simple() {
    let mut sem = Semaphore::<i32>::new(3);
    {
        let t = sem.acquire();
        sem.release(t);
    }

    {
        let t1 = sem.acquire();
        let t2 = sem.acquire();
        let t3 = sem.acquire();
        sem.release(t1);
        sem.release(t3);
        sem.release(t2);
    }

    //Lost ticket
    {
        let t1 = sem.acquire();
        let _t2 = sem.acquire();
        sem.release(t1);
    }
    for _ in 0..CRIT {
        let t = sem.acquire();
        sem.release(t);
    }
}

#[test]
fn mutex_like() {
    let sem = SyncUnsafeCell::new(Semaphore::<usize>::new(1));
    let counter = SyncUnsafeCell::new(0);
    for _ in 0..CRIT {
        thread::scope(|scope| {
            for _ in 0..5 {
                scope.spawn(|| unsafe {
                    let ticket = (*sem.get()).acquire();
                    *counter.get() += 1;
                    (*sem.get()).release(ticket);
                });
            }
        });
    }
    unsafe { assert_eq!(*counter.get(), CRIT * 5) }
}
#[test]
fn cycling_state() {
    let sem1 = SyncUnsafeCell::new(Semaphore::<usize>::new(1));
    let sem2 = SyncUnsafeCell::new(Semaphore::<usize>::new(0));
    let sem3 = SyncUnsafeCell::new(Semaphore::<usize>::new(0));
    let state = SyncUnsafeCell::new(2);
    for _ in 0..CRIT {
        thread::scope(|scope| unsafe {
            scope.spawn(|| {
                let ticket = (*sem1.get()).acquire();
                assert_eq!(*state.get(), 2);
                *state.get() = 1;
                assert_eq!(*state.get(), 1);
                (*sem3.get()).release(ticket);
            });
            scope.spawn(|| {
                let ticket = (*sem2.get()).acquire();
                assert_eq!(*state.get(), 3);
                *state.get() = 2;
                assert_eq!(*state.get(), 2);
                (*sem1.get()).release(ticket);
            });
            scope.spawn(|| {
                let ticket = (*sem3.get()).acquire();
                assert_eq!(*state.get(), 1);
                *state.get() = 3;
                assert_eq!(*state.get(), 3);
                (*sem2.get()).release(ticket);
            });
        });
    }
}
