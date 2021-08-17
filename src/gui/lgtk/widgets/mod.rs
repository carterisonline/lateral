use rust_alloc::vec::Vec;

use crate::io::vga_buffer::ScreenChar;

use super::Size;

pub mod header;
pub mod string;

pub trait Widget {
    fn to_buffer(&self, container_size: Size) -> Vec<Vec<ScreenChar>>;
    fn get_size_with_padding(&self, container_size: Size) -> Size;
    fn get_size(&self, container_size: Size) -> Size;
    fn get_padding(&self) -> Size;
}
