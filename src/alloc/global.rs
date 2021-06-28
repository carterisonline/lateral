extern crate alloc as rust_alloc;

use core::ptr::null_mut;
use rust_alloc::alloc::{GlobalAlloc, Layout};

use super::block::FixedSizeBlockAllocator;
use super::lock::Locked;

#[global_allocator]
pub(super) static ALLOCATOR: Locked<FixedSizeBlockAllocator> =
    Locked::new(FixedSizeBlockAllocator::new());

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
