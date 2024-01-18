use crate::align_up;

use super::address::Address;

const BLOCK_SIZE: usize = 32 * 1024;
pub const LINE_SIZE: usize = 128;
const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

pub struct Block {
    mark: BlockMark,
    line_marks: [LineMark; LINE_COUNT],
    base: Address,
}

#[repr(u8)]
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
    pub fn new(base: Address) -> Block {
        Block {
            mark: BlockMark::Free,
            line_marks: [LineMark::Free; LINE_COUNT],
            base,
        }
    }

    fn mark_free(&mut self) {
        self.mark = BlockMark::Free;
    }

    fn mark_unavailable(&mut self) {
        self.mark = BlockMark::Unavailable;
    }

    fn mark_recyclable(&mut self) {
        self.mark = BlockMark::Recyclable;
    }

    fn mark_line(&mut self, index: usize) {
        assert!(index < LINE_COUNT, "invalid line index.");
        self.line_marks[index] = LineMark::Live;
    }

    pub fn find_next_hole(&self) -> (Address, Address) {
        todo!("find next hole")
    }

    pub fn allocate(&mut self, size: usize) -> Address {
        let lines = align_up!(size, LINE_SIZE) / LINE_SIZE;
        let mut available_lines = 0;
        for index in 0..LINE_COUNT {
            if self.line_marks[index] == LineMark::Free {
                available_lines += 1;
                if available_lines == lines {
                    let start = index - lines;
                    (start..index).for_each(|i| self.mark_line(i));
                    return self.base.plus(start * LINE_SIZE);
                }
            } else {
                available_lines = 0;
            }
        }
        Address::zero()
    }
}
