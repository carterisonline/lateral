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
    use lateral::mem::frame::BootInfoFrameAllocator;
    use lateral::mem::paging;
    use lateral::println;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    pub fn main(boot_info: &'static bootloader::BootInfo) -> ! {
        lateral::init();

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
        let mut mapper = unsafe { paging::init(phys_mem_offset) };
        let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

        // map an unused page
        let page = Page::containing_address(VirtAddr::new(0));
        paging::create_example_mapping(page, &mut mapper, &mut frame_allocator);

        // write the string `New!` to the screen through the new mapping
        let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
        unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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
