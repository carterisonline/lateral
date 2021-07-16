use crate::io::logging::{kernel_error, kernel_warning};
use crate::print;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub fn init_ps2() {
    SCANCODE_QUEUE
        .try_init_once(|| ArrayQueue::new(100))
        .expect("ScancodeStream::new should only be called once");
}

/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            kernel_warning("Scancode queue full; dropping keyboard input");
        }
    } else {
        kernel_error("Scancode queue uninitialized");
    }

    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}
