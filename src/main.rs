#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lateral::test::runner)]
#![reexport_test_harness_main = "test_harness"]

#[cfg(not(test))]
bootloader::entry_point!(kernel::main);
#[cfg(test)]
bootloader::entry_point!(tests::main);

#[cfg(not(test))]
mod kernel {
    extern crate alloc as rust_alloc;
    use lateral::future::exec::Executor;
    use lateral::future::io::ps2;
    use lateral::future::task::Task;
    use lateral::mem::frame::BootInfoFrameAllocator;
    use lateral::mem::paging;
    use lateral::println;
    use x86_64::VirtAddr;

    pub fn main(boot_info: &'static bootloader::BootInfo) -> ! {
        lateral::init();

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { paging::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        lateral::alloc::heap::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");

        let mut executor = Executor::new();
        executor.spawn(Task::new(ps2::print_keypresses())); // new
        executor.run();

        println!("Hello World!");
        lateral::halt_loop();
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        println!("{}", info);
        lateral::halt_loop();
    }
}

#[cfg(test)]
mod tests {
    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        super::test_harness();
        loop {}
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        lateral::test::panic(info)
    }
}
