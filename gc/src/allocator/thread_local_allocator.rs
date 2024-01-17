use crate::align_up;
use crate::allocator::GLOBAL_ALLOCATOR;

use crate::model::block::LINE_SIZE;
use crate::model::{address::Address, block::Block};

pub struct ThreadLocalAllocator {
    recyclable_blocks: Vec<Block>,
    unavailable_blocks: Vec<Block>,
    block_index: isize,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl ThreadLocalAllocator {
    pub fn new() -> ThreadLocalAllocator {
        ThreadLocalAllocator {
            recyclable_blocks: Vec::new(),
            unavailable_blocks: Vec::new(),
            block_index: -1,
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        let mut result: Address = Address::zero();
        let size: usize = align_up!(size, LINE_SIZE);
        if self.block_index != -1 {
            assert!(
                !self.bmp_cursor.is_null() && !self.bmp_limit.is_null(),
                "bmp cursor and bmp limit should not be null for fast allocation attempt."
            );
            if self.bmp_cursor + size <= self.bmp_limit {
                result = self.bmp_cursor;
                self.bmp_cursor = self.bmp_cursor.plus(size);
                return result;
            }
            let mut block = &mut self.recyclable_blocks[self.block_index as usize];
            result = block.allocate(size);
        }

        result
    }

    pub fn return_blocks(&mut self) {
        todo!()
    }
}

impl Drop for ThreadLocalAllocator {
    fn drop(&mut self) {
        todo!()
    }
}
