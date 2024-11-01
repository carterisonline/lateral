use rust_alloc::vec::Vec;

pub struct ThreadQueue {
    pub contents: Vec<fn()>,
}

impl Default for ThreadQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadQueue {
    pub const fn new() -> ThreadQueue {
        ThreadQueue {
            contents: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<fn()> {
        self.contents.pop()
    }

    pub fn push(&mut self, thread: fn()) {
        self.contents.push(thread);
    }

    pub fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }
}
