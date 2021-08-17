use rust_alloc::boxed::Box;
use rust_alloc::string::{String, ToString};
use rust_alloc::vec::Vec;

use crate::io::vga_buffer::{BgColor, ColorCode, FgColor, ScreenChar, HEIGHT, WIDTH, WRITER};

use super::lgtk::widgets::Widget;
use super::lgtk::Size;

const DESKTOP_BG: BgColor = BgColor::Blue;
const START_BAR: usize = 14;

fn gradient_wallpaper() -> VGABuffer {
    let mut buffer = [[ScreenChar {
        ascii_character: b' ',
        color_code: ColorCode::new(FgColor::White, DESKTOP_BG),
    }; WIDTH]; HEIGHT];

    for i in 0u8..3 {
        buffer[START_BAR + i as usize] = [ScreenChar {
            ascii_character: 176 + i,
            color_code: ColorCode::new(FgColor::LightBlue, DESKTOP_BG),
        }; WIDTH];
    }

    for i in (START_BAR + 3)..HEIGHT {
        buffer[i as usize] = [ScreenChar {
            ascii_character: 219,
            color_code: ColorCode::new(FgColor::LightBlue, DESKTOP_BG),
        }; WIDTH];
    }

    buffer
}

fn window_with_borders(x: usize, y: usize, width: usize, height: usize) -> ScreenChar {
    if y == height || x == width {
        if y == 0 {
            ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::White, DESKTOP_BG),
            }
        } else if x == 0 {
            ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::White, BgColor::LightBlue),
            }
        } else {
            ScreenChar {
                ascii_character: 219,
                color_code: ColorCode::new(FgColor::Black, DESKTOP_BG),
            }
        }
    } else {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(FgColor::White, BgColor::LightGray),
        }
    }
}

type VGABuffer = [[ScreenChar; WIDTH]; HEIGHT];

struct SizedWidget<'a> {
    widget: Box<dyn Widget + 'a>,
    size: Size,
    y_pos: usize,
}

impl SizedWidget<'a> {
    fn new(widget: Box<dyn Widget + 'a>, width: usize, height: usize, y_pos: usize) -> Self {
        Self {
            widget,
            size: Size { width, height },
            y_pos,
        }
    }
}

pub struct Window<'a> {
    contents: Vec<Vec<ScreenChar>>,
    name: String,
    x_pos: usize,
    y_pos: usize,
    width: usize,
    height: usize,
    widgets: Vec<SizedWidget<'a>>,
    widget_height: usize,
}

impl Window<'a> {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        Self {
            // is this functional programming?   :O
            contents: (0..=height)
                .map(|y| {
                    (0..=width)
                        .map(|x| window_with_borders(x, y, width, height))
                        .collect::<Vec<ScreenChar>>()
                })
                .collect::<Vec<Vec<ScreenChar>>>(),
            width,
            height,
            name: name.to_string(),
            x_pos: WIDTH / 2 - (width / 2),
            y_pos: HEIGHT / 2 - (height / 2),
            widgets: Vec::new(),
            widget_height: 0,
        }
    }

    pub fn push_line(&mut self) {
        self.widget_height += 1;
    }

    pub fn push_widget<T>(&mut self, widget: T, height: usize)
    where
        T: Widget + 'a,
    {
        let padding = widget.get_padding().height;
        self.widgets.push(SizedWidget::new(
            box widget,
            self.width,
            height + padding * 2,
            self.widget_height,
        ));
        self.widget_height += height - 1;
    }

    pub fn set_text(&mut self, text: &str, x: usize, y: usize, x_max: usize) {
        let mut x_pos = x;
        let mut y_pos = y;
        for word in text.split_ascii_whitespace() {
            if x_pos + word.len() > x_max {
                x_pos = x;
                y_pos += 1;
            }

            for c in word.chars() {
                self.set_char(
                    x_pos,
                    y_pos,
                    ScreenChar {
                        ascii_character: c as u8,
                        color_code: ColorCode::new(FgColor::Black, BgColor::LightGray),
                    },
                );

                x_pos += 1;
            }

            self.set_char(
                x_pos,
                y_pos,
                ScreenChar {
                    ascii_character: b' ',
                    color_code: ColorCode::new(FgColor::Black, BgColor::LightGray),
                },
            );

            x_pos += 1;
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, c: ScreenChar) {
        self.contents[y][x] = c;
    }

    pub fn update_buffer(&mut self, vga_buffer: &mut VGABuffer) {
        for i in 0..self.widgets.len() {
            self.draw_widget(i);
        }

        for y in self.y_pos..=(self.y_pos + self.height) {
            if y >= HEIGHT {
                break;
            } else if y == 0 {
                continue;
            }

            for x in self.x_pos..=(self.x_pos + self.width) {
                if x >= WIDTH {
                    break;
                }

                vga_buffer[y][x] = self.contents[y - self.y_pos][x - self.x_pos];
            }
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    fn draw_widget(&mut self, wnum: usize) {
        let w = &self.widgets[wnum];
        let padding = w.widget.get_padding();
        for (y, row) in w.widget.to_buffer(w.size - padding).iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                self.contents[y + w.y_pos + padding.height][x + padding.width] = c.clone();
            }
        }
    }
}

pub struct Desktop<'a> {
    buffer: VGABuffer,
    windows: Vec<Window<'a>>,
    active_window: Option<usize>,
}

impl Desktop<'a> {
    pub fn new() -> Desktop<'a> {
        let mut buffer = gradient_wallpaper();

        buffer[0] = [ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(FgColor::White, BgColor::Black),
        }; WIDTH];

        Desktop {
            buffer,
            windows: Vec::new(),
            active_window: None,
        }
    }

    pub fn update_window(&mut self, window_num: usize) {
        self.windows[window_num].update_buffer(&mut self.buffer);
    }

    pub fn push_window(&mut self, window: Window<'a>) -> usize {
        self.windows.push(window);
        self.focus(self.windows.len() - 1);
        self.windows.len() - 1
    }

    pub fn display(&self) {
        x86_64::instructions::interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writer.buffer.chars = self.buffer;
        });
    }

    pub fn focus(&mut self, window_num: usize) {
        self.active_window = Some(window_num);
        let window = &self.windows[self.active_window.unwrap()];

        for i in 0..WIDTH {
            self.buffer[0][i] = ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(FgColor::White, BgColor::Black),
            }
        }

        for (i, c) in window.name.chars().enumerate() {
            self.buffer[0][i] = ScreenChar {
                ascii_character: c as u8,
                color_code: ColorCode::new(FgColor::White, BgColor::Black),
            }
        }
    }
}
