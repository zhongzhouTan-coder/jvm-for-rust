use std::collections::LinkedList;

use crate::model::{address::Address, block::Block};
use crate::utils::mmap::MemoryMap;

const DEFAULT_HEAP_SIZE: usize = 1024 * 1024;
const WORD_SIZE: usize = 8;

pub struct GlobalAllocator {
    memory_map: MemoryMap,
    free_blocks: LinkedList<Block>,
    used_blocks: LinkedList<Block>,
    committed_word_size: usize,
    limit_word_size: usize,
}

impl GlobalAllocator {
    pub fn initialize() -> GlobalAllocator {
        let max_heap_size: usize = std::env::var("MAX_HEAP_SIZE")
            .map(|s| s.parse().unwrap())
            .unwrap_or(DEFAULT_HEAP_SIZE);
        let memory_map = MemoryMap::new(max_heap_size);
        GlobalAllocator {
            memory_map,
            free_blocks: LinkedList::new(),
            used_blocks: LinkedList::new(),
            committed_word_size: 0,
            limit_word_size: max_heap_size / WORD_SIZE,
        }
    }

    pub fn require_block(&mut self) -> Block {
        if let Some(block) = self.free_blocks.pop_front() {
            return block;
        };
        self.require_memory_from_system();
        if let Some(block) = self.free_blocks.pop_front() {
            return block;
        };
        self.collect();
        if let Some(block) = self.free_blocks.pop_front() {
            return block;
        };
        panic!("out of memory");
    }

    pub fn return_blocks<I>(&mut self, blocks: I)
    where
        I: Iterator<Item = Block>,
    {
        self.used_blocks.extend(blocks)
    }

    pub fn collect(&self) {
        todo!()
    }

    pub fn mark(obj_ref: Address) {
        todo!()
    }

    pub fn sweep() {
        todo!()
    }

    fn require_memory_from_system(&self) {}
}
