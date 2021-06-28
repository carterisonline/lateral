#![no_std]
#![no_main]

// Entry point.
bootloader::entry_point!(main);
fn main(_: &'static bootloader::BootInfo) -> ! {
    lateral::init();
    lateral::test::run_should_panic(&tests::basic);
    lateral::test::run(&tests::test_breakpoint_exception);
    loop {}
}

// Panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    lateral::test::panic_should_panic(info)
}

mod tests {
    pub fn basic() {
        let actual = 1;
        assert_eq!(0, actual);
    }

    pub fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
