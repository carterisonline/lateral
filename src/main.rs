#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lateral::test::runner)]
#![reexport_test_harness_main = "test_harness"]
#![feature(generator_trait)]
#![feature(generators)]

#[cfg(not(test))]
bootloader::entry_point!(kernel::main);
#[cfg(test)]
bootloader::entry_point!(tests::main);
#[cfg(not(test))]
mod kernel {
    extern crate alloc as rust_alloc;
    use lateral::gui::terminal;
    use lateral::io::logging::kernel_fatal;
    use lateral::mem::frame::BootInfoFrameAllocator;
    use lateral::mem::paging;
    use lateral::thread::ps2::init_ps2;
    use lateral::thread::Runtime;
    use rust_alloc::format;
    use x86_64::VirtAddr;

    pub fn main(boot_info: &'static bootloader::BootInfo) -> ! {
        lateral::init();

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { paging::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        lateral::alloc::heap::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialization failed");

        init_ps2();

        let mut runtime = Runtime::new();

        runtime.init();
        runtime.spawn(terminal);
        runtime.run();
    }

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        kernel_fatal(format!("{}", info).as_str());
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
