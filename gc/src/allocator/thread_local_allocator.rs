use std::collections::LinkedList;

use crate::model::{address::Address, block::Block};

pub struct ThreadLocalAllocator {
    unavailable_blocks: LinkedList<Block>,
    recyclable_blocks: LinkedList<Block>,
    head_room: LinkedList<Block>,
    current_block: Option<Block>,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl ThreadLocalAllocator {
    pub fn new() -> ThreadLocalAllocator {
        ThreadLocalAllocator {
            unavailable_blocks: LinkedList::new(),
            recyclable_blocks: LinkedList::new(),
            head_room: LinkedList::new(),
            current_block: None,
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        todo!()
    }

    pub fn free(&self) {
        todo!()
    }
}

impl Drop for ThreadLocalAllocator {
    fn drop(&mut self) {
        todo!()
    }
}
