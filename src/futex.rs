use std::{arch::asm, sync::atomic::AtomicU32, u32::MAX};

pub(crate) struct Futex {}

impl Futex {
    pub fn sleep(flag: &AtomicU32) {
        let addr = std::ptr::addr_of!(*flag);
        unsafe {
            asm!(
                "mov rax, 202",
                "mov rdi, {0}",
                "mov rsi, 0",
                "mov rdx, 2",
                "mov rcx, 0",
                "mov r8, 0",
                "mov r9, 0",
                in(reg) addr
            );
        }
    }
    pub fn wake_one(flag: &AtomicU32) {
        let addr = std::ptr::addr_of!(*flag);
        unsafe {
            asm!(
                "mov rax, 202",
                "mov rdi, {0}",
                "mov rsi, 1",
                "mov rdx, 1",
                "mov rcx, 0",
                "mov r8, 0",
                "mov r9, 0",
                in(reg) addr
            );
        }
    }
    pub fn wake_all(flag: &AtomicU32) {
        let addr = std::ptr::addr_of!(*flag);
        let all = std::u32::MAX;
        unsafe {
            asm!(
                "mov rax, 202",
                "mov rdi, {0}",
                "mov rsi, 1",
                "mov rdx, {1:r}",
                "mov rcx, 0",
                "mov r8, 0",
                "mov r9, 0",
                in(reg) addr,
                in(reg) all,
            );
        }
    }
}
