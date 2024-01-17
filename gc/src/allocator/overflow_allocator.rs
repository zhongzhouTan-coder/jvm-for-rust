use crate::allocator::GLOBAL_ALLOCATOR;
use crate::model::address::Address;
use crate::model::block::Block;

pub struct OverflowAllocator {
    overflow_block: Option<Block>,
    bmp_cursor: Address,
    bmp_limit: Address,
}

impl OverflowAllocator {
    pub fn new() -> OverflowAllocator {
        OverflowAllocator {
            overflow_block: None,
            bmp_cursor: Address::zero(),
            bmp_limit: Address::zero(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        todo!()
    }

    pub fn return_blocks(&mut self) {
        todo!()
    }
}

impl Drop for OverflowAllocator {
    fn drop(&mut self) {
        if let Some(block) = self.overflow_block.take() {
            let mut global_allocator = GLOBAL_ALLOCATOR.lock().unwrap();
            global_allocator.return_blocks(std::iter::once(block));
        }
    }
}