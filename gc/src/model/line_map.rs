use std::{mem::size_of, ops::Add};

use super::{address::Address, block::LINE_SIZE};

pub struct LineMap {
    base: Address,
    len: usize,
}

impl LineMap {
    pub fn new(line_size: usize) -> LineMap {
        let base = unsafe { libc::malloc(line_size * size_of::<u8>()) };
        LineMap {
            base: Address::new(base as usize),
            len: line_size,
        }
    }

    pub fn block_line_marks(&self, index: usize) -> Address {
        self.base.add(index * size_of::<u8>() * LINE_SIZE)
    }
}

impl Drop for LineMap {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.base.to_usize() as *mut libc::c_void);
        }
    }
}
