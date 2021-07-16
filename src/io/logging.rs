use crate::io::vga_buffer::{BgColor, FgColor};
use crate::println;

pub fn kernel_log(message: &str, fg: FgColor, bg: BgColor) {
    println!(
        "{}{} KERNEL {}{}  {}",
        FgColor::White,
        bg,
        fg,
        BgColor::Black,
        message
    );
}

pub fn kernel_fatal(message: &str) {
    kernel_log(message, FgColor::Red, BgColor::LightRed);
}

pub fn kernel_warning(message: &str) {
    kernel_log(message, FgColor::Yellow, BgColor::Brown);
}

pub fn kernel_info(message: &str) {
    kernel_log(message, FgColor::LightCyan, BgColor::Cyan);
}

pub fn kernel_event(message: &str) {
    kernel_log(message, FgColor::LightGreen, BgColor::Green);
}

pub fn kernel_error(message: &str) {
    kernel_log(message, FgColor::Pink, BgColor::Magenta);
}
