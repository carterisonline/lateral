use rust_alloc::string::String;
use rust_alloc::vec::Vec;

use crate::gui::lgtk::util::WidgetBuffer;
use crate::gui::lgtk::Size;
use crate::io::vga_buffer::{BgColor, ColorCode, FgColor, ScreenChar};

use super::Widget;

impl Widget for String {
    fn to_buffer(&self, container_size: Size, bgcolor: BgColor) -> Vec<Vec<ScreenChar>> {
        let mut output: Vec<Vec<ScreenChar>> = Vec::fill(
            ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::Black, bgcolor),
            },
            container_size,
        );

        let max = self.get_size(container_size);
        let mut x_pos = 0;
        let mut y_pos = 0;
        for word in self.split_ascii_whitespace() {
            if x_pos + word.len() > max.width {
                x_pos = 0;
                y_pos += 1;

                if y_pos >= max.height {
                    break;
                }
            }

            for c in word.chars() {
                output[y_pos][x_pos] = ScreenChar {
                    ascii_character: c as u8,
                    color_code: ColorCode::new(FgColor::Black, bgcolor),
                };

                x_pos += 1;
            }

            output[y_pos][x_pos] = ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::Black, bgcolor),
            };

            x_pos += 1;
        }

        output
    }

    fn get_padding(&self) -> Size {
        Size::square(2)
    }

    fn get_size_with_padding(&self, container_size: Size) -> Size {
        container_size
    }

    fn get_size(&self, container_size: Size) -> Size {
        container_size - 2
    }
}
