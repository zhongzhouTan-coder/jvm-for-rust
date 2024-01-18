use crate::align_up;
use crate::model::{address::Address, block::Block};
use crate::model::block::LINE_SIZE;

pub struct ThreadLocalAllocator {
    block: Option<Block>,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl ThreadLocalAllocator {
    pub fn new() -> ThreadLocalAllocator {
        ThreadLocalAllocator {
            block: None,
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        let size: usize = align_up!(size, LINE_SIZE);
        let result = self.fast_allocate(size);
        result
    }

    fn fast_allocate(&mut self, size: usize) -> Address {
        assert!(
            self.block.is_some() && !self.bmp_cursor.is_null() && !self.bmp_limit.is_null(),
            "block, bmp cursor and bmp limit should not be null for fast allocation attempt."
        );
        if self.bmp_cursor + size <= self.bmp_limit {
            let result = self.bmp_cursor;
            self.bmp_cursor = self.bmp_cursor.plus(size);
            return result;
        }
        Address::zero()
    }

    fn slow_allocate(&mut self, size: usize) -> Address {
        if let Some(block) = &mut self.block {
            let result = block.allocate(size);
            (self.bmp_cursor, self.bmp_limit) = block.find_next_hole();
            return result;
        }
        Address::zero()
    }

    fn require_block_from_global(&mut self) {
        todo!()
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
