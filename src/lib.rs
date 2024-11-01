#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_harness"]
#![feature(alloc_error_handler)]
#![allow(undefined_naked_function_abi)]
#![allow(redundant_semicolons)] // weird rust-analyzer bug

use thread::queue::ThreadQueue;
use x86_64::instructions::port::Port;

use crate::io::vga_buffer::{BgColor, FgColor};

extern crate alloc as rust_alloc;

pub mod alloc;
pub mod cpu;
pub mod fs;
pub mod gui;
pub mod io;
pub mod mem;
pub mod syscall;
pub mod test;
pub mod thread;
pub mod time;
pub mod util;

pub static mut THREAD_QUEUE: ThreadQueue = ThreadQueue::new();

pub fn spawn_thread(thread: fn()) {
    unsafe {
        #[allow(static_mut_refs)]
        THREAD_QUEUE.push(thread);
    }
}

const MESSAGE: &str = "Kernel Version 0.2.1";

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

pub fn init() {
    cpu::gdt::init();
    cpu::interrupt::init_idt();
    unsafe { cpu::interrupt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    disable_cursor();
    startup_screen();
    time::rtc::init();
}

fn startup_screen() {
    let lateral_logo = [
        r#"88        db    888888 888888 88""Yb    db    88    "#,
        r#"88       dPYb     88   88__   88__dP   dPYb   88    "#,
        r#"88  .o  dP__Yb -- 88 - 88"" - 88"Yb - dP__Yb  88  .o"#,
        r#"88ood8 dP""""Yb   88   888888 88  Yb dP""""Yb 88ood8"#,
    ];

    x86_64::instructions::interrupts::without_interrupts(|| {
        for y in 0..io::vga_buffer::HEIGHT {
            for x in 0..io::vga_buffer::WIDTH {
                write_line!(" ", y, x, FgColor::White, BgColor::Black);
            }
        }

        let logo_y = 5;

        for (y, &line) in lateral_logo.iter().enumerate() {
            write_line!(line, logo_y + y, 14, FgColor::White, BgColor::Black);
        }

        write_line!(
            MESSAGE,
            logo_y + 8,
            (io::vga_buffer::WIDTH / 2) - (MESSAGE.len() / 2),
            FgColor::White,
            BgColor::Black
        )
    });
}

fn disable_cursor() {
    let mut cursor_1 = Port::new(0x3D4);
    let mut cursor_2 = Port::new(0x3D5);
    unsafe {
        cursor_1.write(0x0Au8);
        cursor_2.write(0x20u8);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: rust_alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
