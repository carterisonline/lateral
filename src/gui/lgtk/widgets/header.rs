use rust_alloc::string::{String, ToString};
use rust_alloc::vec::Vec;

use crate::gui::lgtk::util::WidgetBuffer;
use crate::gui::lgtk::Size;
use crate::io::vga_buffer::{BgColor, ColorCode, FgColor, ScreenChar};

use super::Widget;

pub struct Header(String);

impl Header {
    pub fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl Widget for Header {
    fn to_buffer(&self, container_size: Size, bgcolor: BgColor) -> Vec<Vec<ScreenChar>> {
        let mut output: Vec<Vec<ScreenChar>> = Vec::fill(
            ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::Black, bgcolor),
            },
            container_size,
        );

        let start = container_size.width / 2 - self.0.len() / 2;

        self.0.chars().enumerate().for_each(|(i, c)| {
            output[0][start + i] = ScreenChar {
                ascii_character: c as u8,
                color_code: ColorCode::new(FgColor::White, bgcolor),
            };

            output[1][start + i] = ScreenChar {
                ascii_character: 223,
                color_code: ColorCode::new(FgColor::White, bgcolor),
            };
        });

        output
    }

    fn get_size_with_padding(&self, container_size: Size) -> Size {
        container_size
    }

    fn get_size(&self, container_size: Size) -> Size {
        container_size - 1
    }

    fn get_padding(&self) -> Size {
        Size::square(1)
    }
}
