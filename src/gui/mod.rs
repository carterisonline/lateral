pub mod lgtk;
pub mod wm;

use lazy_static::lazy_static;
use rust_alloc::format;
use rust_alloc::string::String;
use spin::RwLock;

use crate::gui::wm::{Desktop, Window};
use crate::thread::yield_thread;
use crate::time::rtc::{nanowait, ticks};

use self::lgtk::widgets::header::Header;

macro_rules! window {
    (id: $id: ident name: $name: literal size: $width: literal x $height: literal position: ($x: literal, $y: literal) --- $($widget: ident($contents: expr), height: $widget_height: literal)*) => {
        let mut $id = Window::new($name, $width, $height);
        $(
            window!($widget ($contents), height: $widget_height, $id);
        );+

        $id.move_to($x, $y);
        // window!($($widget ($contents), $widget_height, $id),*);
    };
    ($widget: ident ($contents: expr), height: $widget_height: literal, $id: ident) => {
        $id.push_widget($widget::from($contents), $widget_height);
    };
}

lazy_static! {
    pub static ref DESKTOP: RwLock<Desktop<'static>> = RwLock::new(Desktop::new());
}

pub fn terminal() {
    window!(
        id: greeting_window
        name: "Lateral Welcome Message"
        size: 40 x 11
        position: (5, 6)
        ---
        Header ("Welcome to Lateral"), height: 3
        String ("Press <TAB> to access the command bar. For more information, run `system/help`."), height: 3
    );

    window!(
        id: test_window
        name: "Test Window"
        size: 20 x 4
        position: (47, 15)
        ---
        Header ("Test Window!!"), height: 2
    );

    window!(
        id: attention
        name: "Attention"
        size: 15 x 5
        position: (49, 8)
        ---
        String ("Hello :)"), height: 1
    );

    let mut desktop = DESKTOP.write();
    desktop.push_window(greeting_window); // Adds the window to the desktop, and returns the window number.
    desktop.push_window(test_window);
    desktop.push_window(attention);
    desktop.update_window(0);
    desktop.update_window(1);
    desktop.update_window(2);

    core::mem::drop(desktop);

    loop {
        let mut desktop = DESKTOP.write();

        //desktop.set_title(2, format!("{:?}", ticks()).as_str());
        //desktop.focus(2);
        desktop.redraw();
        desktop.display();
        core::mem::drop(desktop);
        yield_thread();
        nanowait(16_667);
    }

    /*
    let mut keyboard = KEYBOARD.lock();
    match SCANCODE_QUEUE.pop() {
        Ok(scancode) => match decode_scancode(&mut keyboard, scancode) {
            Some(OsChar::Display(character)) => match character {
                '\t' => {
                    palette_open = !palette_open;
                    if palette_open {
                        display_palette();
                    } else {
                        hide_palette();
                    }
                }
                _ => (), // print!("{}", character),
            },

            Some(OsChar::Special(code)) => {
                // println!("{:?}", code);
            }

            None => (),
        },

        Err(_) => (),
    }

    write_line!("Hello, world!", 20, 20, FgColor::White, BgColor::Red);*/
}
