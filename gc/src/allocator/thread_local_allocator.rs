use crate::align_up;
use crate::model::block::LINE_SIZE;
use crate::model::{address::Address, block::Block};

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
        if self.block.is_none() {
            let block = self.require_block_from_global();
            self.bmp_cursor = block.base_address();
            self.bmp_limit = block.block_limit();
            self.block = Some(block);
        }

        Address::zero()
    }

    fn require_block_from_global(&mut self) -> Block {
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
