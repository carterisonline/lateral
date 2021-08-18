use core::sync::atomic::{AtomicU8, Ordering};

use rust_alloc::string::ToString;

use crate::gui::wm::{Axis, Direction};
use crate::gui::DESKTOP;
use crate::thread::ps2::OsChar;
use crate::{println, write_line};

/// Input modes are issued as the following: \
/// \
/// `0: Normal Mode` \
/// `1: Capture Mode`
pub static INPUTMODE: AtomicU8 = AtomicU8::new(0);

pub fn handle_input(scancode: OsChar) {
    let input_mode = INPUTMODE.load(Ordering::Relaxed);

    match input_mode {
        0 => match scancode {
            OsChar::Display(' ') => INPUTMODE.store(1, Ordering::Relaxed),
            OsChar::Display('w') => {
                let mut desktop = DESKTOP.write();
                desktop.change_focus(Direction::Up);
            }
            OsChar::Display('s') => {
                let mut desktop = DESKTOP.write();
                desktop.change_focus(Direction::Down);
            }
            OsChar::Display('a') => {
                let mut desktop = DESKTOP.write();
                desktop.change_focus(Direction::Left);
            }
            OsChar::Display('d') => {
                let mut desktop = DESKTOP.write();
                desktop.change_focus(Direction::Right);
            }
            OsChar::Display('W') => {
                let mut desktop = DESKTOP.write();
                let current_window = desktop.active_window.clone().unwrap();
                desktop.budge_window(current_window, Axis::Y, -1);
            }
            OsChar::Display('S') => {
                let mut desktop = DESKTOP.write();
                let current_window = desktop.active_window.clone().unwrap();
                desktop.budge_window(current_window, Axis::Y, 1);
            }
            OsChar::Display('A') => {
                let mut desktop = DESKTOP.write();
                let current_window = desktop.active_window.clone().unwrap();
                desktop.budge_window(current_window, Axis::X, -1);
            }
            OsChar::Display('D') => {
                let mut desktop = DESKTOP.write();
                let current_window = desktop.active_window.clone().unwrap();
                desktop.budge_window(current_window, Axis::X, 1);
            }
            _ => println!("Unhandled: {:?}", scancode),
        },
        1 => match scancode {
            OsChar::Display('\u{1b}') => INPUTMODE.store(0, Ordering::Relaxed),
            _ => println!("Unhandled: {:?}", scancode),
        },

        _ => unreachable!(),
    }

    let formatted = input_mode.to_string();
    let ref_formatted = formatted.as_str();
    write_line!(ref_formatted, 0, 0);

    while !crate::thread::ps2::SCANCODE_QUEUE.is_empty() {
        crate::thread::ps2::SCANCODE_QUEUE.pop();
    }
}
