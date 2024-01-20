use std::collections::LinkedList;

use crate::align_up;
use crate::model::block::LINE_SIZE;
use crate::model::{address::Address, block::Block};

use super::GLOBAL_ALLOCATOR;

pub struct ThreadLocalAllocator {
    unavailable_blocks: LinkedList<Block>,
    recyclable_blocks: LinkedList<Block>,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl ThreadLocalAllocator {
    pub fn new() -> ThreadLocalAllocator {
        ThreadLocalAllocator {
            unavailable_blocks: LinkedList::new(),
            recyclable_blocks: LinkedList::new(),
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<Address> {
        let size: usize = align_up!(size, LINE_SIZE);
        let mut result: Option<Address> = None;
        if !self.recyclable_blocks.is_empty() {
            result = self.fast_allocate(size);
        }
        if result.is_none() {
            result = self.slow_allocate(size);
        }
        if result.is_none() {
            let block = self.require_block_from_global();
            self.bmp_cursor = block.base_address();
            self.bmp_limit = block.block_limit();
            self.recyclable_blocks.push_front(block);
            result = self.fast_allocate(size);
        }
        result
    }

    fn fast_allocate(&mut self, size: usize) -> Option<Address> {
        assert!(
            !self.bmp_cursor.is_null() && !self.bmp_limit.is_null(),
            "bmp cursor and bmp limit should not be null for fast allocation attempt."
        );
        if self.bmp_cursor + size <= self.bmp_limit {
            let result = self.bmp_cursor;
            self.recyclable_blocks
                .front_mut()
                .unwrap()
                .mark_lines(self.bmp_cursor, self.bmp_cursor.plus(size));
            self.bmp_cursor = self.bmp_cursor.plus(size);
            return Some(result);
        }
        None
    }

    fn slow_allocate(&mut self, size: usize) -> Option<Address> {
        let mut result: Option<Address> = None;
        for block in self.recyclable_blocks.iter_mut() {
            if let Some(address) = block.allocate(size) {
                result = Some(address);
                break;
            }
        }
        while let Some(block) = self.recyclable_blocks.front() {
            if let Some((start, end)) = block.find_next_hole() {
                self.bmp_cursor = start;
                self.bmp_limit = end;
                break;
            }
            self.unavailable_blocks
                .push_back(self.recyclable_blocks.pop_front().unwrap());
        }
        result
    }

    fn require_block_from_global(&mut self) -> Block {
        GLOBAL_ALLOCATOR.lock().unwrap().require_block()
    }

    pub fn return_free_blocks_to_global(&mut self) {
        let mut free_blocks: LinkedList<Block> = LinkedList::new();
        let mut all_blocks: LinkedList<Block> = LinkedList::new();
        all_blocks.append(&mut self.recyclable_blocks);
        all_blocks.append(&mut self.unavailable_blocks);
        for block in all_blocks.into_iter() {
            if block.is_free() {
                free_blocks.push_back(block);
            } else if block.is_recyclable() {
                self.recyclable_blocks.push_back(block);
            } else if block.is_unavailable() {
                self.unavailable_blocks.push_back(block);
            } else {
                panic!("invalid block mark.");
            }
        }
        GLOBAL_ALLOCATOR
            .lock()
            .unwrap()
            .return_blocks(free_blocks.into_iter(), LinkedList::new().into_iter());
    }
}

impl Drop for ThreadLocalAllocator {
    fn drop(&mut self) {
        let mut used_blocks: LinkedList<Block> = LinkedList::new();
        used_blocks.append(&mut self.recyclable_blocks);
        used_blocks.append(&mut self.unavailable_blocks);
        GLOBAL_ALLOCATOR
            .lock()
            .unwrap()
            .return_blocks(LinkedList::new().into_iter(), used_blocks.into_iter());
    }
}
