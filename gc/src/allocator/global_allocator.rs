use std::collections::LinkedList;

use crate::align_up;
use crate::model::block::{BLOCK_SIZE, LINE_SIZE};
use crate::model::line_map::LineMap;
use crate::model::{address::Address, block::Block};
use crate::utils::mmap::MemoryMap;

const DEFAULT_HEAP_SIZE: usize = 1024 * 1024;
const WORD_SIZE: usize = 8;

pub struct GlobalAllocator {
    memory_map: MemoryMap,
    free_blocks: Vec<Block>,
    used_blocks: Vec<Block>,
    line_map: LineMap,
    committed_word_size: usize,
    limit_word_size: usize,
}

impl GlobalAllocator {
    pub fn initialize() -> GlobalAllocator {
        let heap_size: usize = std::env::var("HEAP_SIZE")
            .map(|s| s.parse().unwrap())
            .unwrap_or(DEFAULT_HEAP_SIZE);
        let heap_size = align_up!(heap_size, BLOCK_SIZE);
        let memory_map = MemoryMap::new(heap_size);
        let line_map = LineMap::new(heap_size / LINE_SIZE);
        GlobalAllocator {
            memory_map,
            free_blocks: Vec::new(),
            used_blocks: Vec::new(),
            line_map,
            committed_word_size: 0,
            limit_word_size: heap_size / WORD_SIZE,
        }
    }

    pub fn require_block(&mut self) -> Block {
        if let Some(block) = self.free_blocks.pop() {
            return block;
        };
        self.require_memory_from_system();
        if let Some(block) = self.free_blocks.pop() {
            return block;
        };
        self.collect();
        if let Some(block) = self.free_blocks.pop() {
            return block;
        };
        panic!("out of memory");
    }

    pub fn return_blocks<I>(&mut self, free_blocks: I, used_blocks: I)
    where
        I: Iterator<Item = Block>,
    {
        self.free_blocks.extend(free_blocks);
        self.used_blocks.extend(used_blocks);
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
