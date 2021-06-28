use core::panic::PanicInfo;
use core::str;

use crate::io::qemu;
use crate::serial_print;
use crate::serial_println;
use crate::util::byte::ByteCompare;

pub trait Testable {
    fn run(&self);
    fn run_should_panic(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        println_ok();
    }

    fn run_should_panic(&self) {
        serial_print!("{} ... ", core::any::type_name::<T>());
        self();
        println_failed();
        qemu::exit(qemu::ExitCode::Failure);
    }
}

// Test runner.
pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} test(s).", tests.len());
    for &test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success)
}

// Run single test.
pub fn run(test: &dyn Testable) {
    serial_println!("Running single test.");
    test.run();
    qemu::exit(qemu::ExitCode::Success)
}

// Run single should panic test.
pub fn run_should_panic(test: &dyn Testable) {
    serial_println!("Running should panic test.");
    test.run_should_panic();
}

// Test mode panic handler.
pub fn panic(info: &PanicInfo) -> ! {
    println_failed();
    serial_println!("\nError: {}", info);
    qemu::exit(qemu::ExitCode::Failure);
    crate::halt_loop();
}

// Test mode should panic handler.
pub fn panic_should_panic(_: &PanicInfo) -> ! {
    println_ok();
    qemu::exit(qemu::ExitCode::Success);
    crate::halt_loop();
}

// Test mode should panic with message handler.
pub fn panic_should_panic_with(info: &PanicInfo, message: &str) -> ! {
    if ByteCompare::starts_with(info, message) {
        println_ok();
        qemu::exit(qemu::ExitCode::Success);
    } else {
        println_failed();
        serial_println!(
            "\nPanic did not contain the expected message.\n{:>10}: {}\n{:>10}: {}",
            "Expected",
            message,
            "Info",
            info
        );
        qemu::exit(qemu::ExitCode::Failure);
    }
    crate::halt_loop();
}

fn println_ok() {
    serial_println!("ok");
}

fn println_failed() {
    serial_println!("failed");
}
