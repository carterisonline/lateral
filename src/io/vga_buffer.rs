use core::fmt::{self, Display, Write};
use lazy_static::lazy_static;
use spin::Mutex;

pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FgColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BgColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Display for FgColor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", (*self as u8 + 0x80) as char)
    }
}

impl Display for BgColor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", (*self as u8 + 0x90) as char)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ColorCode(u8);

impl ColorCode {
    pub(crate) fn new(foreground: FgColor, background: BgColor) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub(crate) ascii_character: u8,
    pub(crate) color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    pub(crate) chars: [[ScreenChar; WIDTH]; HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    pub(crate) buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= WIDTH {
                    self.new_line();
                }

                let row = HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.chars() {
            match byte as u8 {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte as u8),
                0x80..=0x8f => {
                    self.color_code.0 = (self.color_code.0 & 0b11110000) | (byte as u8 - 0x80)
                }
                0x90..=0x9f => {
                    self.color_code.0 =
                        (self.color_code.0 & 0b00001111) | ((byte as u8 - 0x80) << 4)
                }
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    fn write_line(
        &mut self,
        text: &str,
        row: usize,
        col: usize,
        fgcolor: Option<FgColor>,
        bgcolor: Option<BgColor>,
    ) {
        for i in 0..text.len() {
            self.buffer.chars[row][col + i] = ScreenChar {
                ascii_character: text.chars().nth(i).unwrap() as u8,
                color_code: if fgcolor.is_none() || bgcolor.is_none() {
                    self.color_code
                } else {
                    ColorCode::new(fgcolor.unwrap(), bgcolor.unwrap())
                },
            }
        }
    }

    fn cls(&mut self) {
        for i in 0..self.buffer.chars.len() {
            for j in 0..self.buffer.chars[i].len() {
                self.buffer.chars[i][j] = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                }
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let item = Mutex::new(Writer {
            column_position: 0,
            color_code: ColorCode::new(FgColor::White, BgColor::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        });

        {
            let mut l = item.lock();
            l.cls();
        }

        item
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! write_line {
    ($text: expr, $x: expr, $y: expr) => {
        $crate::io::vga_buffer::_write($text, $x, $y, None, None)
    };

    ($text: expr, $x: expr, $y: expr, $fgcolor: expr, $bgcolor: expr) => {
        $crate::io::vga_buffer::_write($text, $x, $y, Some($fgcolor), Some($bgcolor))
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _write(
    text: &str,
    row: usize,
    col: usize,
    fgcolor: Option<FgColor>,
    bgcolor: Option<BgColor>,
) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_line(text, row, col, fgcolor, bgcolor);
    });
}
