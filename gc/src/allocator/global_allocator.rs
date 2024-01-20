use std::collections::LinkedList;

use crate::align_up;
use crate::model::block::Block;
use crate::model::block::{BLOCK_SIZE, LINE_SIZE};
use crate::model::line_map::LineMap;
use crate::utils::mmap::MemoryMap;

const DEFAULT_HEAP_SIZE: usize = 1024 * 1024;

pub struct GlobalAllocator {
    memory_map: MemoryMap,
    free_blocks: LinkedList<Block>,
    used_blocks: LinkedList<Block>,
    line_map: LineMap,
    total_blocks: usize,
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
            free_blocks: LinkedList::new(),
            used_blocks: LinkedList::new(),
            line_map,
            total_blocks: 0,
        }
    }

    pub fn require_block(&mut self) -> Block {
        if let Some(block) = self.free_blocks.pop_front() {
            return block;
        };
        self.require_block_from_system();
        if let Some(block) = self.free_blocks.pop_front() {
            return block;
        };
        self.collect();
        if let Some(block) = self.free_blocks.pop_front() {
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

    fn require_block_from_system(&mut self) {
        if let Some(address) = self.memory_map.allocate_memory(BLOCK_SIZE) {
            self.total_blocks += 1;
            let index = self.total_blocks - 1;
            let line_mark = self.line_map.block_line_marks(index);
            let block = Block::new(address, line_mark);
            self.free_blocks.push_back(block);
        } else {
            panic!("out of memory");
        }
    }
}
