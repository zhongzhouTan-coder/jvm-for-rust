use std::ops::{Index, IndexMut};

use crate::align_up;

use super::address::Address;

pub const BLOCK_SIZE: usize = 32 * 1024;
pub const LINE_SIZE: usize = 128;
const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

pub struct Block {
    block_mark: BlockMark,
    line_marks: LineMarks,
    base: Address,
}

pub struct LineMarks {
    base: Address,
}

#[repr(u8)]
#[derive(PartialEq)]
pub enum BlockMark {
    Free,
    Unavailable,
    Recyclable,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum LineMark {
    Free,
    Live,
}

impl Block {
    pub fn new(base: Address, line_mark_base: Address) -> Block {
        Block {
            block_mark: BlockMark::Free,
            line_marks: LineMarks::new(line_mark_base),
            base,
        }
    }

    fn mark_free(&mut self) {
        self.block_mark = BlockMark::Free;
    }

    fn mark_unavailable(&mut self) {
        self.block_mark = BlockMark::Unavailable;
    }

    fn mark_recyclable(&mut self) {
        self.block_mark = BlockMark::Recyclable;
    }

    fn mark_line(&mut self, index: usize) {
        self.line_marks[index] = LineMark::Live;
    }

    #[inline(always)]
    pub fn is_free(&self) -> bool {
        self.block_mark == BlockMark::Free
    }

    #[inline(always)]
    pub fn is_recyclable(&self) -> bool {
        self.block_mark == BlockMark::Recyclable
    }

    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        self.block_mark == BlockMark::Unavailable
    }

    pub fn mark_lines(&mut self, start: Address, end: Address) {
        assert!(
            start > self.base && end <= self.base.plus(BLOCK_SIZE),
            "invalid address range."
        );
        let start_index = start.diff(self.base) / LINE_SIZE;
        let end_index = end.diff(self.base) / LINE_SIZE;
        (start_index..end_index).for_each(|i| self.mark_line(i));
    }

    #[inline(always)]
    pub fn base_address(&self) -> Address {
        self.base
    }

    #[inline(always)]
    pub fn block_limit(&self) -> Address {
        self.base.plus(BLOCK_SIZE)
    }

    pub fn find_next_hole(&self) -> Option<(Address, Address)> {
        todo!("find next hole")
    }

    pub fn allocate(&mut self, size: usize) -> Option<Address> {
        let lines = align_up!(size, LINE_SIZE) / LINE_SIZE;
        let mut available_lines = 0;
        for index in 0..LINE_COUNT {
            if self.line_marks[index] == LineMark::Free {
                available_lines += 1;
                if available_lines == lines {
                    let start = index - lines;
                    (start..index).for_each(|i| self.mark_line(i));
                    return Some(self.base.plus(start * LINE_SIZE));
                }
            } else {
                available_lines = 0;
            }
        }
        None
    }
}

impl LineMarks {
    pub fn new(base: Address) -> LineMarks {
        LineMarks { base }
    }

    #[inline(always)]
    pub fn mark_line(&mut self, index: usize, mark: LineMark) {
        assert!(index < LINE_COUNT, "invalid line index.");
        self.base.plus(index).store(mark);
    }
}

impl Index<usize> for LineMarks {
    type Output = LineMark;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < LINE_COUNT, "invalid line index.");
        self.base.plus(index).load()
    }
}

impl IndexMut<usize> for LineMarks {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < LINE_COUNT, "invalid line index.");
        self.base.plus(index).load()
    }
}
