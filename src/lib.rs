#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(llvm_asm)] // Should remove in favor of asm
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_harness"]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(const_fn_fn_ptr_basics)]

use thread::queue::ThreadQueue;

extern crate alloc as rust_alloc;

pub mod alloc;
pub mod cpu;
pub mod io;
pub mod mem;
pub mod test;
pub mod thread;
pub mod util;

pub static mut THREAD_QUEUE: ThreadQueue = ThreadQueue::new();

pub fn spawn_thread(thread: fn()) {
    unsafe {
        THREAD_QUEUE.push(thread);
    }
}

#[macro_export]
macro_rules! exit {
    ($id: expr, $code: literal) => {
        $crate::future::task::RETURN_CODES
            .lock()
            .insert($id.clone(), $code);
    };
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
bootloader::entry_point!(tests::main);
#[cfg(test)]
mod tests {
    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        super::init();
        super::test_harness();
        super::halt_loop();
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        super::test::panic(info)
    }
}

pub fn init() {
    cpu::gdt::init();
    cpu::interrupt::init_idt();
    unsafe { cpu::interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: rust_alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
