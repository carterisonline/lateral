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
    use lateral::{print, println};

    pub fn main(_: &'static bootloader::BootInfo) -> ! {
        lateral::init();

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
