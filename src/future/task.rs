extern crate alloc as rust_alloc;

use core::sync::atomic::{AtomicU64, Ordering};
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};
use rust_alloc::boxed::Box;
use rust_alloc::vec::Vec;

pub struct TaskCache {
    contents: Vec<Task>,
}

impl TaskCache {
    pub const fn new() -> TaskCache {
        TaskCache {
            contents: Vec::new(),
        }
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.contents.pop()
    }

    pub fn push(&mut self, task: Task) {
        self.contents.push(task);
    }

    pub fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }
}

pub struct Task {
    pub(super) id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }
    pub(super) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);
impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}