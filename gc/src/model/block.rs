use super::address::Address;

pub const BLOCK_SIZE: usize = 32 * 1024;
pub const LINE_SIZE: usize = 128;

pub struct Block {
    mark: BlockMark,
    line_mark_table: LineMarkTable,
    base: Address,
}

#[repr(u8)]
pub enum BlockMark {
    Free,
    Unavailable,
    Recyclable,
}

pub struct LineMarkTable {
    base: Address,
}

#[repr(u8)]
pub enum LineMark {
    Free,
    Live,
}

impl Block {
    /// mark a block state to free
    fn mark_free(&mut self) {
        self.mark = BlockMark::Free;
    }

    /// mark a block state to unavailable
    fn mark_unavailable(&mut self) {
        self.mark = BlockMark::Unavailable;
    }

    /// mark a block state to recyclable
    fn mark_recyclable(&mut self) {
        self.mark = BlockMark::Recyclable;
    }

    /// mark a line that a specific address is reside in
    fn mark_line(&mut self, address: Address) {
        self.validate_address(address);
        let line_offset = address.diff(self.base) / LINE_SIZE;
        self.line_mark_table.mark_line(line_offset);
    }

    #[inline(always)]
    fn validate_address(&self, address: Address) {
        assert!(
            self.base <= address && address < self.base + BLOCK_SIZE,
            "invalid address."
        );
    }
}

impl LineMarkTable {
    fn mark_line(&mut self, offset: usize) {
        self.base.plus(offset).store(LineMark::Live as u8);
    }
}
