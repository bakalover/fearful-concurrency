use std::{
    arch::asm,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::futex::Futex;
///Mutex's states.
///
#[derive(Clone, Copy)]
enum State {
    Unlocked,
    Locked,
    Waiting,
}
///Transform mutex state to futex memory cell size.
fn get_st(st: State) -> u32 {
    match st {
        State::Unlocked => 0,
        State::Locked => 1,
        State::Waiting => 2,
    }
}

pub struct Mutex {
    futex_word: AtomicU32,
}

impl Default for Mutex {
    fn default() -> Self {
        Mutex::new()
    }
}

impl Mutex {
    #[must_use]
    pub fn new() -> Self {
        Mutex {
            futex_word: AtomicU32::new(get_st(State::Unlocked)),
        }
    }
    fn cmpxchg(&self, cur: State, needed: State) -> bool {
        self.futex_word
            .compare_exchange(
                get_st(cur),
                get_st(needed),
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_ok()
    }
    pub fn lock(&self) {
        if self.cmpxchg(State::Unlocked, State::Locked) {
            return;
        }
        loop {
            if self.futex_word.load(Ordering::Relaxed) == get_st(State::Waiting)
                || !self.cmpxchg(State::Locked, State::Waiting)
            {
                Futex::sleep(&self.futex_word);
            }
            if self.cmpxchg(State::Unlocked, State::Waiting) {
                break;
            }
        }
    }

    ///Unlock operation causes waking up SINGLE thread.
    pub fn unlock(&self) {
        if self.futex_word.fetch_sub(1, Ordering::Relaxed) != get_st(State::Locked) {
            self.futex_word
                .store(get_st(State::Unlocked), Ordering::Relaxed);
            Futex::wake_one(&self.futex_word);
        };
    }
}
