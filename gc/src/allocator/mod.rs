use std::cell::RefCell;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::model::address::Address;

use self::{global_allocator::GlobalAllocator, thread_local_allocator::ThreadLocalAllocator, overflow_allocator::OverflowAllocator};

mod global_allocator;
mod thread_local_allocator;
mod overflow_allocator;

static GLOBAL_ALLOCATOR: Lazy<Mutex<GlobalAllocator>> =
    Lazy::new(|| Mutex::new(GlobalAllocator::initialize()));

thread_local! {
    static THREAD_LOCAL_ALLOCATOR: RefCell<ThreadLocalAllocator> = RefCell::new(ThreadLocalAllocator::new());

    static OVERFLOW_ALLOCATOR: RefCell<OverflowAllocator> = RefCell::new(OverflowAllocator::new());
}

pub fn allocate(size: usize) -> Address {
    THREAD_LOCAL_ALLOCATOR.with(|allocator| {
        let mut allocator = allocator.borrow_mut();
        allocator.allocate(size)
    })
}

pub fn swap() {
    THREAD_LOCAL_ALLOCATOR.with(|allocator| {
        let mut allocator = allocator.borrow_mut();
        allocator.return_blocks();
    })
}