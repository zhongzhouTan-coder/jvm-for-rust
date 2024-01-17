use crate::align_up;
use crate::allocator::GLOBAL_ALLOCATOR;

use crate::model::{address::Address, block::Block};
use crate::model::block::LINE_SIZE;

pub struct ThreadLocalAllocator {
    current_block: Option<Block>,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl ThreadLocalAllocator {
    pub fn new() -> ThreadLocalAllocator {
        ThreadLocalAllocator {
            current_block: None,
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        if let Some(block) = &mut self.current_block {
            let lines = align_up!(size, LINE_SIZE) / LINE_SIZE;
            Address::zero()
        } else {
            todo!()
        }
    }

    pub fn return_blocks(&mut self) {
        todo!()
    }
}

impl Drop for ThreadLocalAllocator {
    fn drop(&mut self) {
        if let Some(block) = self.current_block.take() {
            let mut global_allocator = GLOBAL_ALLOCATOR.lock().unwrap();
            global_allocator.return_blocks(std::iter::once(block));
        }
    }
}
