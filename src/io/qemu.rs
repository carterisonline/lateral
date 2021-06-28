use x86_64::instructions::port::Port;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExitCode {
    Success = 0b10,
    Failure = 0b11,
}

pub fn exit(code: ExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
}
