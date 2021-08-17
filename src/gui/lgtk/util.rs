use rust_alloc::vec::Vec;

use crate::io::vga_buffer::ScreenChar;

use super::Size;

pub trait WidgetBuffer {
    fn fill(default_char: ScreenChar, size: Size) -> Self;
}

impl WidgetBuffer for Vec<Vec<ScreenChar>> {
    fn fill(default_char: ScreenChar, size: Size) -> Self {
        let mut output: Vec<Vec<ScreenChar>> = Vec::new();

        for _ in 0..size.height {
            let mut out = Vec::new();
            for _ in 0..size.width {
                out.push(default_char);
            }

            output.push(out);
        }

        output
    }
}
