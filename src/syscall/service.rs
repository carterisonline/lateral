use core::arch::asm;

pub fn sleep(seconds: f64) {
    unsafe { asm!("sti") }; // Restore interrupts
    crate::time::rtc::sleep(seconds);
    unsafe { asm!("cli") }; // Disable interrupts
}

pub fn uptime() -> f64 {
    crate::time::uptime()
}

pub fn realtime() -> f64 {
    crate::time::realtime()
}
