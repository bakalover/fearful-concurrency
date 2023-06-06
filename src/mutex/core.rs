use libc::{syscall, SYS_futex, FUTEX_WAIT, FUTEX_WAKE};
use std::sync::atomic::{AtomicU32, Ordering};
///Mutex's states
enum State {
    Unlocked,
    Locked,
    Waiting,
}

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

impl Mutex {
    pub fn new() -> Self {
        Mutex {
            futex_word: AtomicU32::new(get_st(State::Unlocked)),
        }
    }
    pub fn lock(&self) {
        match self.futex_word.compare_exchange(
            get_st(State::Unlocked),
            get_st(State::Locked),
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return,
            Err(_) => (),
        }
        loop {
            if self.futex_word.load(Ordering::Relaxed) == get_st(State::Waiting)
                || !(match self.futex_word.compare_exchange(
                    get_st(State::Locked),
                    get_st(State::Waiting),
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => true,
                    Err(_) => false,
                })
            {
                unsafe {
                    syscall(
                        SYS_futex,
                        &self.futex_word as *const AtomicU32,
                        FUTEX_WAIT,
                        2,
                        0,
                        0,
                        0,
                    );
                }
            }
            if !(match self.futex_word.compare_exchange(
                get_st(State::Unlocked),
                get_st(State::Waiting),
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => true,
                Err(_) => false,
            }) {
                return;
            }
        }
    }
    pub fn unlock(&self) {
        if self.futex_word.fetch_sub(1, Ordering::Relaxed) != get_st(State::Locked) {
            self.futex_word
                .store(get_st(State::Unlocked), Ordering::Relaxed);
            unsafe {
                syscall(
                    SYS_futex,
                    &self.futex_word as *const AtomicU32,
                    FUTEX_WAKE,
                    1,
                    0,
                    0,
                    0,
                );
            }
        };
    }
}
