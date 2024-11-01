pub mod ps2;
pub mod queue;

use core::arch::{asm, naked_asm};
use core::ptr;

use rust_alloc::vec::Vec;

use crate::cpu::gdt::STACK_SIZE;
use crate::io::logging::*;
use crate::THREAD_QUEUE;

use self::queue::ThreadQueue;

const MAX_THREADS: usize = 32;
static mut RUNTIME: usize = 0;

#[derive(Debug, Default, Clone)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum State {
    Available,
    Running,
    Ready,
}

#[derive(Clone)]
struct Thread {
    stack: [u8; STACK_SIZE],
    ctx: ThreadContext,
    state: State,
}

impl Thread {
    fn new() -> Self {
        Thread {
            stack: [0_u8; STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            stack: [0_u8; STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
        };

        let mut threads = [base_thread].to_vec();
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|_| Thread::new()).collect();
        threads.append(&mut available_threads);

        Runtime {
            threads,
            current: 0,
        }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        kernel_event("All threads have exited.");
        loop {
            core::hint::spin_loop();
        }
    }

    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    fn t_yield(&mut self) -> bool {
        x86_64::instructions::interrupts::without_interrupts(|| {
            let mut pos = self.current;
            while self.threads[pos].state != State::Ready {
                pos += 1;
                if pos == self.threads.len() {
                    pos = 0;
                }
                if pos == self.current {
                    return false;
                }
            }

            if self.threads[self.current].state != State::Available {
                self.threads[self.current].state = State::Ready;
            }

            self.threads[pos].state = State::Running;
            let old_pos = self.current;
            self.current = pos;

            unsafe {
                let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
                let new: *const ThreadContext = &self.threads[pos].ctx;
                asm!(
                    "mov {old}, %rdi",
                    "mov {new}, %rsi",
                    old = in(reg) old,
                    new = in(reg) new,
                    // "r"(old), "r"(new)
                    options(att_syntax)
                );
                switch();
            }

            unsafe {
                #[allow(static_mut_refs)]
                for i in 0..THREAD_QUEUE.len() {
                    self.spawn(THREAD_QUEUE.contents[i]);
                }
                THREAD_QUEUE = ThreadQueue::new();
            }

            !self.threads.is_empty()
        })
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available thread.");

        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().add(size);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            ptr::write(s_ptr.offset(-16) as *mut usize, guard as usize);
            ptr::write(s_ptr.offset(-24) as *mut usize, skip as usize);
            ptr::write(s_ptr.offset(-32) as *mut usize, f as usize);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
        }
        available.state = State::Ready;
    }
}

#[naked]
fn skip() {
    unsafe {
        naked_asm!("");
    }
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    };
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}

//*  Replace with `asm!` or else

#[naked]
unsafe fn switch() {
    naked_asm!(
        "mov     %rsp, 0x00(%rdi)",
        "mov     %r15, 0x08(%rdi)",
        "mov     %r14, 0x10(%rdi)",
        "mov     %r13, 0x18(%rdi)",
        "mov     %r12, 0x20(%rdi)",
        "mov     %rbx, 0x28(%rdi)",
        "mov     %rbp, 0x30(%rdi)",
        "mov     0x00(%rsi), %rsp",
        "mov     0x08(%rsi), %r15",
        "mov     0x10(%rsi), %r14",
        "mov     0x18(%rsi), %r13",
        "mov     0x20(%rsi), %r12",
        "mov     0x28(%rsi), %rbx",
        "mov     0x30(%rsi), %rbp",
        "ret",
        options(att_syntax)
    );
}
