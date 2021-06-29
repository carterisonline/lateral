#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_harness"]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]


use future::task::TaskCache;

extern crate alloc as rust_alloc;

pub mod alloc;
pub mod cpu;
pub mod future;
pub mod io;
pub mod mem;
pub mod test;
pub mod util;

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub static mut TASK_CACHE: TaskCache = TaskCache::new();

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
