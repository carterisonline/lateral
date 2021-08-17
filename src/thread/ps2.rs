use crate::cpu::interrupt::set_irq_handler;
use crate::io::logging::kernel_warning;
use crossbeam_queue::ArrayQueue;
use pc_keyboard::{
    layouts, DecodedKey, HandleControl, KeyCode, Keyboard, KeyboardLayout, ScancodeSet,
    ScancodeSet1,
};
use spin::Mutex;

use lazy_static::lazy_static;
use x86_64::instructions::port::Port;

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

lazy_static! {
    pub static ref SCANCODE_QUEUE: ArrayQueue<u8> = ArrayQueue::new(100);
}

pub fn init_ps2() {
    set_irq_handler(1, add_scancode);
    /*SCANCODE_QUEUE
    .try_init_once(|| Vec::new())
    .expect("ScancodeStream::new should only be called once");*/
}

fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    unsafe { port.read() }
}

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
fn add_scancode() {
    let scancode = read_scancode();

    if let Err(_) = SCANCODE_QUEUE.push(scancode) {
        kernel_warning("Scancode queue full; dropping keyboard input");
    }
}

pub enum OsChar {
    Display(char),
    Special(KeyCode),
}

pub fn decode_scancode<T: KeyboardLayout, U: ScancodeSet>(
    keyboard: &mut Keyboard<T, U>,
    scancode: u8,
) -> Option<OsChar> {
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => return Some(OsChar::Display(character)),
                DecodedKey::RawKey(thing) => return Some(OsChar::Special(thing)),
            }
        } else {
            return None;
        }
    } else {
        return None;
    }
}
